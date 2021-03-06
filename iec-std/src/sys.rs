#![cfg(all(not(test), target_arch = "wasm32"))]

use core::{fmt::Write, panic::PanicInfo};

use crate::intrinsics::{self, wasm_log_level_LOG_ERROR as LOG_ERROR};
use arrayvec::ArrayString;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    let mut buffer: ArrayString<[u8; 512]> = ArrayString::new();

    unsafe {
        let _ = write!(buffer, "{}", info);
        let msg = buffer.as_str();

        let file = info.location().map(|l| l.file()).unwrap_or("<unknown>");
        let line = info.location().map(|l| l.line()).unwrap_or(0);

        let _ = intrinsics::wasm_log(
            LOG_ERROR,
            file.as_ptr() as *const _,
            file.len() as _,
            line as _,
            msg.as_ptr() as *const _,
            msg.len() as _,
        );

        core::arch::wasm32::unreachable()
    }
}
