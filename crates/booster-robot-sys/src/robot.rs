pub mod b1;
pub mod common;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper.hpp");

        fn init_channel_factory(network_interface: &CxxString);
    }
}
