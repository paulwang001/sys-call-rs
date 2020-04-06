#![feature(asm)]

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[path = "platform/linux-x86_64.rs"]
pub mod platform;

#[macro_export]
macro_rules! sys_call {
    ($num:ident) => {
        $crate::platform::sys_call0($num)
    };
    ($num:expr, $arg1:expr) => {
        $crate::platform::sys_call1($num, $arg1)
    };
    ($num:expr, $arg1:expr, $arg2:expr) => {
        $crate::platform::sys_call2($num, $arg1, $arg2)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::platform::sys_call3($num, $arg1, $arg2, $arg3)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::platform::sys_call4($num, $arg1, $arg2, $arg3, $arg4)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        $crate::platform::sys_call5($num, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        $crate::platform::sys_call6($num, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const WRITE: usize = 1;
        const STDOUT: usize = 1;
        let buf = b"Hello World!\n";
        unsafe {
            sys_call!(WRITE, STDOUT, buf.as_ptr() as usize, buf.len());
        }
    }
}
