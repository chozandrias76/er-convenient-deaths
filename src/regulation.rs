use region::Protection;
use retour::static_detour;
use std::mem::transmute;

#[cfg(target_os = "windows")]
use winapi::um::wincon::SetConsoleTextAttribute;

#[cfg(target_os = "windows")]
use winapi::um::processenv::GetStdHandle;
#[cfg(target_os = "windows")]
use winapi::um::winbase::STD_OUTPUT_HANDLE;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

use crate::util::match_instruction_pattern;
use log::info;

#[cfg(target_os = "windows")]
fn set_console_color(color: u16) {
    unsafe {
        let handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle != INVALID_HANDLE_VALUE {
            SetConsoleTextAttribute(handle, color);
        }
    }
}

const REGBIN_CHECK_FLAG_SETTER_PATTERN: &str = concat!(
    // MOV RAX, qword ptr [RBX+0x8]
    "01001... 10001011 01000011 00001000",
    // MOV [RAX+0xC8], RCX
    "01001... 10001001 10001000 11001000 00000000 00000000 00000000",
    // CMP [???], CL
    "00111000 00001101 ........ ........ ........ ........",
    // JNZ [???]
    "01110101 ........",
    // CALL [???]
    "11101000 ........ ........ ........ ........",
    // MOV [RegBinFlags + 0], AL
    "10001000 00000101 ........ ........ ........ ........",
    // MOV [RegBinFlags + 1], AL
    "10001000 00000101 ........ ........ ........ ........",
    // MOV [RegBinFlags + 2], AL
    "[10001000 00000101 ........ ........ ........ ........]",
);

pub fn hook() {
    let safety_flag_initializer_va = match_instruction_pattern(REGBIN_CHECK_FLAG_SETTER_PATTERN)
        .map(|m: crate::util::PatternResult| m.captures.first().map(|c: &crate::util::PatternCapture| c.location as *mut u8))
        .flatten()
        .expect("Could not find the regbin check flag setter");

    #[cfg(target_os = "windows")]
    {
        set_console_color(0x02); // Set text color to green
        info!("\n\n\tAttached DLL for Auto Arc\n\n");
        set_console_color(0x07); // Reset text color to default
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("Attached DLL for Auto Arc");
    }

    unsafe {
        region::protect(safety_flag_initializer_va, 1, Protection::READ_WRITE_EXECUTE)
            .expect("Could not change memory protection for flag initializer");

        // XOR in stead of MOV so we clear out the flag
        *safety_flag_initializer_va = 0x30;
    }
}
