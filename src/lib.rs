use std::{error::Error, fs::{self, File}, sync::Mutex};
use serde::{Deserialize, Serialize};

use crash_handler::{CrashContext, CrashEventResult, CrashHandler, make_crash_event};
use eldenring_util::{program::Program, system::wait_for_system_init};
use pelite::pe64::PeObject;
use toml::to_string_pretty;
use std::ptr;
pub const DLL_PROCESS_ATTACH: u32 = 1;

struct Command {
    name: String,
    offset: usize,
    replacement: Vec<u8>,
    expected_existing_bytes: Vec<u8>,
}

impl Command {
    fn new(
        name: String,
        offset: usize,
        replacement_instructions: Vec<u8>,
        expected_existing_bytes: Vec<u8>,
    ) -> Self {
        Command {
            name,
            offset,
            replacement: replacement_instructions,
            expected_existing_bytes,
        }
    }
}


#[derive(Deserialize, Serialize)]
#[derive(Default)]
struct Config {
    keep_runes_on_death: bool,
    keep_rune_arc_on_death: bool,
    quicker_deaths: bool,
}

#[unsafe(no_mangle)]
/// # DllMain Entry Point
///
/// This is the entry point for the DLL. It is called by the operating system when the DLL is loaded or unloaded.
///
/// ## Parameters
/// - `_hmodule`: A handle to the DLL module. This parameter is unused in this implementation.
/// - `reason`: The reason code for the call. This implementation only handles `DLL_PROCESS_ATTACH` (value `1`).
///
/// ## Behavior
/// When the `reason` is `DLL_PROCESS_ATTACH`, the following actions are performed:
/// 1. The `setup` function is called to initialize logging, crash handling, and other setup tasks.
/// 2. A new thread is spawned to:
///    - Wait for the system to initialize using `wait_for_system_init`.
///    - Call the `init` function to perform the main DLL initialization logic.
///
/// ## Safety
/// This function is marked as `unsafe` because it interacts with low-level system APIs and performs operations
/// that require careful handling, such as spawning threads and modifying memory.
///
/// ## Returns
/// Always returns `true` to indicate successful execution.
/// ```
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        let config = setup().unwrap();

        std::thread::spawn(|| {
            // Give the CRT init a bit of leeway
            wait_for_system_init(5000).expect("System initialization timed out");

            init(config).expect("Could not start the DLL after the game would be ready");
        });
    }
    true
}

fn setup() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "./er_convenient_deaths.toml";
    let config: Config = if let Ok(config_content) = fs::read_to_string(config_path) {
        toml::from_str(&config_content)?
    } else {
        let config: Config = Default::default();
        let toml_content: String = to_string_pretty(&config)?;
        fs::write(config_path, toml_content)?;
        config
    };
    let log_file = File::create("./er_convenient_deaths.log")?;
    let subscriber = tracing_subscriber::fmt()
        .with_writer(Mutex::new(log_file))
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("Application panicked: {}", panic_info);
    }));

    #[allow(unsafe_code)]
    let handler = CrashHandler::attach(unsafe {
        make_crash_event(move |context: &CrashContext| {
            tracing::error!(
                "Exception: {:x} at {:x}",
                context.exception_code,
                (*(*context.exception_pointers).ExceptionRecord).ExceptionAddress as usize
            );

            CrashEventResult::Handled(true)
        })
    })
    .unwrap();
    std::mem::forget(handler);

    Ok(config)
}

fn init(config: Config) -> Result<(), Box<dyn Error>> {
    let program: Program<'_> = Program::current();
    let pe_image: &[u8] = program.image();

    let commands: [Command; 3] = [
        Command::new(
            "Skip losing runes on death".to_string(),
            0x5fc1a0,
            vec![0xc3, 0x90, 0x90],
            vec![0x48, 0x85, 0xd2],
        ),
        Command::new(
            "If death happens, you have rune arc set to 1, and rune arc should be set to 0, set it to 1 instead.".to_string(),
            0x25e299,
            vec![0x66, 0xC7, 0x81, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00],
            vec![0x66, 0xC7, 0x81, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00],
        ),
        Command::new(
            "Replace Jump instruction for death timer".to_string(),
            0x5A7193,
            vec![0xeb, 0x06],
            vec![0x74, 0x06],
        ),
    ];

    for (i, command) in commands.iter().enumerate() {
        // Check if the command should be skipped based on the config
        let enabled = match i {
            0 => config.keep_runes_on_death,
            1 => config.keep_rune_arc_on_death,
            2 => config.quicker_deaths,
            _ => false,
        };

        if !enabled {
            tracing::info!("Skipping command '{}'", command.name);
            continue;
        }
        let target_address = unsafe { pe_image.as_ptr().add(command.offset) as *mut u8 };

        // Check if the target address is valid
        if target_address.is_null() {
            tracing::warn!(
                "Invalid target address for command '{}', offset: {:x}",
                command.name,
                command.offset
            );
            continue;
        }

        // Check that the target address has the expected opcode
        let mut is_valid = true;
        for (i, &expected_byte) in command.expected_existing_bytes.iter().enumerate() {
            let actual_byte = unsafe { *target_address.add(i) };
            if actual_byte != expected_byte {
                tracing::warn!(
                    "Unexpected opcode for command '{}' at offset {:x}: expected {:x}, found {:x}",
                    command.name,
                    command.offset + i,
                    expected_byte,
                    actual_byte
                );
                is_valid = false;
                break;
            }
        }

        if !is_valid {
            continue;
        }

        // Apply the replacement bytes
        unsafe {
            for (i, &replacement_byte) in command.replacement.iter().enumerate() {
                ptr::write(target_address.add(i), replacement_byte);
            }
        }

        tracing::info!(
            "Successfully applied command '{}' at offset {:x}",
            command.name,
            command.offset
        );
    }

    tracing::info!("ER Auto Arc code injection completed.");
    Ok(())
}
