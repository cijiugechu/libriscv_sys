//! FFI bindings to libriscv, a fast RISC-V sandbox emulator.
//!
//! This crate provides low-level bindings to the libriscv C API.
//! For a higher-level Rust interface, consider using a wrapper crate.
//!
//! # Example
//!
//! ```no_run
//! use libriscv_sys::*;
//! use std::ptr;
//!
//! unsafe {
//!     let mut options: RISCVOptions = std::mem::zeroed();
//!     libriscv_set_defaults(&mut options);
//!     
//!     // Load ELF binary...
//!     // let machine = libriscv_new(elf_data.as_ptr() as *const _, elf_data.len() as u32, &mut options);
//! }
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_defaults() {
        unsafe {
            let mut options: RISCVOptions = std::mem::zeroed();
            libriscv_set_defaults(&mut options);
            
            // Check that defaults are set
            assert!(options.max_memory > 0);
            assert!(options.stack_size > 0);
            assert_eq!(options.strict_sandbox, 1); // true
        }
    }

    #[test]
    fn test_strerror() {
        unsafe {
            let msg = libriscv_strerror(0);
            assert!(!msg.is_null());
            
            let msg = libriscv_strerror(RISCV_ERROR_TYPE_MACHINE_TIMEOUT);
            assert!(!msg.is_null());
        }
    }
}
