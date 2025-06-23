// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

use crate::buffer::TwoChannelBuffer;

#[cxx::bridge]
pub(crate) mod ffi {
    extern "Rust" {
        type TwoChannelBuffer;

        fn push(self: &mut TwoChannelBuffer, left_sample: i32, right_sample: i32);
    }

    #[allow(dead_code)]
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
    use crate::ymfm_bridge::ffi::ym2608_sample_rate;

    use super::*;

    #[test]
    fn test_ym2608() {
        assert_eq!(ffi::ym2608_create(), true);

        let sample_rate = ffi::ym2608_sample_rate();
        assert_eq!(sample_rate, 166400);

        ffi::ym2608_reset();

        ffi::ym2608_write_low(0x00, 0xff);
        assert_eq!(ffi::ym2608_read_low(0x00), 0xff);

        let mut buffer = TwoChannelBuffer::new(ym2608_sample_rate().into(), sample_rate.into());
        let frame_size = buffer.needed_frame_size(512);

        ffi::ym2608_generate(&mut buffer, frame_size.try_into().unwrap());

        buffer.end_frame();

        assert_eq!(ffi::ym2608_destroy(), true);
    }

    #[test]
    fn test_ym2608_multiple_call() {
        assert_eq!(ffi::ym2608_create(), true);
        assert_eq!(ffi::ym2608_create(), false);

        assert_eq!(ffi::ym2608_destroy(), true);
        assert_eq!(ffi::ym2608_destroy(), false);
    }
}
