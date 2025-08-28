pub mod b1;
pub mod common;

pub use ffi::*;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wrapper.hpp");

        fn init_channel_factory(network_interface: &CxxString);
    }
}
