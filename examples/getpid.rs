fn main() {
    let pid = unsafe { linux_sys::getpid() };

    let msg = format!("{}\n", pid);
    let msg_raw = msg.as_bytes();
    let msg_ptr = msg_raw.as_ptr() as *const linux_sys::void;
    let msg_size = msg_raw.len() * core::mem::size_of::<u8>();

    let written = unsafe { linux_sys::write(1, msg_ptr, msg_size) };
    if written < 0 {
        panic!("failed to write");
    }
}