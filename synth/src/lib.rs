// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

mod buffer;
mod note;
mod tone;

mod ymfm_bridge;

pub use note::*;
pub use tone::*;

use buffer::TwoChannelBuffer;
use std::sync::Mutex;
use ymfm_bridge::ffi::*;

static RESAMPLED_BUF: Mutex<Option<TwoChannelBuffer>> = Mutex::new(None);

fn ym2608_write_high_and_low(addr: u8, data: u8) {
    ym2608_write_low(addr, data);
    ym2608_write_high(addr, data);
}

pub fn create(sample_rate: f64) -> bool {
    if !ym2608_create() {
        return false;
    }

    ym2608_reset();

    let mut resampled_buf = RESAMPLED_BUF.lock().unwrap();
    *resampled_buf = Some(TwoChannelBuffer::new(
        ym2608_sample_rate().into(),
        sample_rate,
    ));

    // YM2608 mode.
    ym2608_write_low(0x29, 0x80);

    // Panning.
    for i in 0..=2 {
        ym2608_write_high_and_low(0xb4 + i, 0xc0);
    }

    true
}

pub fn set_tone(tone: &FmTone) {
    ym2608_write_low(0x24, tone.value_lfo_freq());

    for ch_offset in 0..=2 {
        ym2608_write_high_and_low(0xb0 + ch_offset, tone.value_fb_al());
        const CENTER_PANNING: u8 = 0xc0;
        ym2608_write_high_and_low(0xb4 + ch_offset, CENTER_PANNING | tone.value_ams_pms());
    }

    const OP_OFFSET: [u8; 4] = [0x00, 0x08, 0x04, 0x0c];
    for (op, op_offset) in tone.op.iter().zip(OP_OFFSET.iter()) {
        for ch_offset in 0..=2 {
            ym2608_write_high_and_low(0x30 + ch_offset + op_offset, op.value_dt_ml());
            ym2608_write_high_and_low(0x40 + ch_offset + op_offset, op.value_tl());
            ym2608_write_high_and_low(0x50 + ch_offset + op_offset, op.value_ks_ar());
            ym2608_write_high_and_low(0x60 + ch_offset + op_offset, op.value_am_dr());
            ym2608_write_high_and_low(0x70 + ch_offset + op_offset, op.value_sr());
            ym2608_write_high_and_low(0x80 + ch_offset + op_offset, op.value_sl_rr());
            ym2608_write_high_and_low(0x90 + ch_offset + op_offset, op.value_ssg_eg());
        }
    }
}

const SLOT_FLAGS: u8 = 0xf0;
const HIGH_CH_NOTE_ON_FLAG: u8 = 0x04;

pub fn note_on(ch: u8, note: Note) {
    if ch < 3 {
        ym2608_write_low(0xa4 + ch, note.to_f_number_high_block());
        ym2608_write_low(0xa0 + ch, note.to_f_number_low());
        ym2608_write_low(0x28, SLOT_FLAGS | ch);
    } else if ch < 6 {
        let high_ch = ch - 2;
        ym2608_write_high(0xa4 + high_ch, note.to_f_number_high_block());
        ym2608_write_high(0xa0 + high_ch, note.to_f_number_low());
        ym2608_write_low(0x28, SLOT_FLAGS | HIGH_CH_NOTE_ON_FLAG | high_ch);
    }
}

pub fn note_off(ch: u8) {
    if ch < 3 {
        ym2608_write_low(0x28, !SLOT_FLAGS & ch);
    } else if ch < 6 {
        let high_ch = ch - 2;
        ym2608_write_low(0x28, !SLOT_FLAGS & (HIGH_CH_NOTE_ON_FLAG | high_ch));
    }
}

pub fn generate(left_ptr: *mut i16, right_ptr: *mut i16, size: usize) {
    let mut resampled_buf = RESAMPLED_BUF.lock().unwrap();

    let Some(ref mut buf) = *resampled_buf else {
        return;
    };

    let available_count = buf.available_sample_count();
    if available_count < size {
        let frame_size = buf.needed_frame_size(size - available_count);
        ym2608_generate(buf, frame_size.try_into().unwrap());
        buf.end_frame();
    }

    _ = buf.pop(left_ptr, right_ptr, size);
}

pub fn destroy() -> bool {
    if !ym2608_destroy() {
        return false;
    }

    let mut resampled_buf = RESAMPLED_BUF.lock().unwrap();
    *resampled_buf = None;

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f64 = 44_100.0;

    #[test]
    fn test_synth() {
        const OPERATOR: FmOperator = FmOperator {
            ar: 0x1f,
            dr: 0x00,
            sr: 0x00,
            rr: 0x00,
            sl: 0x00,
            tl: 28,
            ks: 0x00,
            ml: 0x04,
            dt: 0x00,
            am: false,
            ssg_eg: 0x00,
        };

        const TONE: FmTone = FmTone {
            al: 0x04,
            fb: 0x07,
            op: [OPERATOR; 4],
            lfo_freq: 0,
            ams: 0,
            pms: 0,
        };

        assert_eq!(create(SAMPLE_RATE), true);

        set_tone(&TONE);

        note_on(0, Note::C(4));

        const BUF_SIZE: usize = 100;
        let (mut left_buf, mut right_buf) = ([0; BUF_SIZE], [0; BUF_SIZE]);
        generate(left_buf.as_mut_ptr(), right_buf.as_mut_ptr(), BUF_SIZE);

        note_off(0);

        assert_eq!(destroy(), true);
    }

    #[test]
    fn test_synth_multiple_call() {
        assert_eq!(create(SAMPLE_RATE), true);
        assert_eq!(create(SAMPLE_RATE), false);

        assert_eq!(destroy(), true);
        assert_eq!(destroy(), false);
    }
}
