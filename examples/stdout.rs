use sys_call::sys_call;

fn main() {
    const WRITE: usize = 1;
    const STDOUT: usize = 1;
    let buf = b"Hello World!\n";

    unsafe {
        sys_call!(WRITE, STDOUT, buf.as_ptr() as usize, buf.len());
    }
}
