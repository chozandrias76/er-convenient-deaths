use std::error::Error;

use crash_handler::{CrashContext, CrashEventResult, CrashHandler, make_crash_event};
use pelite::pe64::PeObject;
use std::ptr;
use tracing_panic::panic_hook;
use eldenring_util::{program::Program, system::wait_for_system_init};
pub const DLL_PROCESS_ATTACH: u32 = 1;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        setup();

        std::thread::spawn(|| {
            // Give the CRT init a bit of leeway
            wait_for_system_init(5000).expect("System initialization timed out");

            init().expect("Could not start the DLL after the game would be ready");
        });
    }
    true
}

fn setup() {
    let rolling_log = tracing_appender::rolling::never("./", "er_auto_arc.log");
    tracing_subscriber::fmt().with_writer(rolling_log).init();
    std::panic::set_hook(Box::new(panic_hook));

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
}

fn init() -> Result<(), Box<dyn Error>> {
    let program: Program<'_> = unsafe { Program::current() };
    let pe_image: &[u8] = program.image();
    // let offsets: [isize; 3] = [0x81f270, 0x81f271, 0x81f272];
    // let full_replacement = [0xc3, 0x90, 0x90];
    // let expected_existing_bytes = [0x48, 0x8b, 0xc4];

    // for (i, &offset) in offsets.iter().enumerate() {
    //     // Calculate the target memory address
    //     let target_address = unsafe { pe_image.as_ptr().offset(offset) as *mut u8 };

    //     // Check if the target address is valid
    //     if target_address.is_null() {
    //         return Err(format!("Invalid target address at offset: {:x}", offset).into());
    //     }

    //     // Check that the target address has the expected opcode
    //     let val = unsafe { *target_address };
    //     if val != expected_existing_bytes[i] {
    //         return Err(format!(
    //             "Unexpected opcode at target address (offset {:x}): {:x}",
    //             offset, val
    //         )
    //         .into());
    //     }
    // }

    // let target_address = unsafe { pe_image.as_ptr().offset(offsets[0]) as *mut u8 };

    // unsafe {
    //     for (i, &byte) in full_replacement.iter().enumerate() {
    //         ptr::write(target_address.add(i), byte);
    //     }
    // }
    tracing::info!(
        "ER Auto Arc code injected successfully at the following offsets: {:x?}",
        offsets
    );
    Ok(())
}