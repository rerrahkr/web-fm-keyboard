// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

mod note;
mod tone;

mod ymfm_bridge;

pub use note::*;
pub use tone::*;

use ymfm_bridge::ffi::*;

fn ym2608_write_high_and_low(addr: u8, data: u8) {
    ym2608_write_low(addr, data);
    ym2608_write_high(addr, data);
}

pub fn create() {
    if !ym2608_create() {
        // panic!("Failed to create YM2608 instance");
        return;
    }
    ym2608_reset();

    // YM2608 mode.
    ym2608_write_low(0x29, 0x80);

    // Panning.
    for i in 0..=2 {
        ym2608_write_high_and_low(0xb4 + i, 0xc0);
    }
}

pub fn set_tone(tone: &FmTone) {
    ym2608_write_low(0x24, tone.value_lfo_freq());

    for ch_offset in 0..=2 {
        ym2608_write_high_and_low(0xb0 + ch_offset, tone.value_fb_al());
        const CENTER_PANNING: u8 = 0xc0;
        ym2608_write_high_and_low(0xb4 + ch_offset, CENTER_PANNING | tone.value_ams_pms());
    }

    const OP_OFFSET: [u8; 4] = [0x00, 0x04, 0x08, 0x0c];
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
        ym2608_write_low(0x28, !SLOT_FLAGS | ch);
    } else if ch < 6 {
        let high_ch = ch - 2;
        ym2608_write_low(0x28, !SLOT_FLAGS | HIGH_CH_NOTE_ON_FLAG | high_ch);
    }
}

pub fn destroy() {
    if !ym2608_destroy() {
        // panic!("Failed to destroy YM2608 instance");
    }
}
