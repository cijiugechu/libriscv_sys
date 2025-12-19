use libriscv_sys::*;

fn main() {
    unsafe {
        let mut options: RISCVOptions = std::mem::zeroed();
        libriscv_set_defaults(&mut options);
    }
}
