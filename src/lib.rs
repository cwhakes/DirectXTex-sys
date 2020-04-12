#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use winapi::shared::dxgiformat::DXGI_FORMAT;

include!("bindings.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sanity() {
        unsafe {
            use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT;
            use super::*;

            assert!(!IsCompressed(DXGI_FORMAT_R32G32B32A32_FLOAT));
        }
    }
}
