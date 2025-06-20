// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

#[cxx::bridge]
pub(crate) mod ffi {

    #![allow(dead_code)]
    struct TwoChannelBuffer {
        usize: usize,
        left: Vec<i16>,
        right: Vec<i16>,
    }

    unsafe extern "C++" {
        include!("synth/src/ymfm_bridge/ymfm_bridge.hpp");

        fn ym2608_create() -> bool;
        fn ym2608_destroy() -> bool;
        fn ym2608_reset();
        fn ym2608_sample_rate() -> u32;
        fn ym2608_clock() -> u32;
        fn ym2608_read_low(addr: u8) -> u8;
        fn ym2608_read_high(addr: u8) -> u8;
        fn ym2608_write_low(addr: u8, data: u8);
        fn ym2608_write_high(addr: u8, data: u8);
        fn ym2608_generate(buffer: &mut TwoChannelBuffer, num_samples: u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ym2608() {
        assert!(ffi::ym2608_create());
        assert_eq!(ffi::ym2608_sample_rate(), 166400);

        ffi::ym2608_reset();

        ffi::ym2608_write_low(0x00, 0xff);
        assert_eq!(ffi::ym2608_read_low(0x00), 0xff);

        const BUFFER_SIZE: usize = 1024;
        let mut buffer = ffi::TwoChannelBuffer {
            usize: BUFFER_SIZE,
            left: vec![0; BUFFER_SIZE],
            right: vec![0; BUFFER_SIZE],
        };

        ffi::ym2608_generate(&mut buffer, 512);

        assert!(ffi::ym2608_destroy());
    }
}
