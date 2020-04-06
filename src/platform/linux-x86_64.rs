#[inline(always)]
pub unsafe fn sys_call0(mut num: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         :
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call1(mut num: usize, arg1: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call2(mut num: usize, arg1: usize, arg2: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call3(mut num: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call4(
    mut num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3) "{r10}"(arg4)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call5(
    mut num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3) "{r10}"(arg4) "{r8}"(arg5)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}

#[inline(always)]
pub unsafe fn sys_call6(
    mut num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    asm!("syscall"
         : "+{rax}"(num)
         : "{rdi}"(arg1) "{rsi}"(arg2) "{rdx}"(arg3) "{r10}"(arg4) "{r8}"(arg5) "{r9}"(arg6)
         : "rcx", "r11", "memory"
         : "volatile");
    num
}
