use sys_call::sys_call;

fn main() {
    const WRITE: isize = 1;
    const STDOUT: isize = 1;
    let buf = b"Hello World!\n";

    unsafe {
        sys_call!(WRITE, STDOUT, buf.as_ptr() as isize, buf.len() as isize);
    }
}
