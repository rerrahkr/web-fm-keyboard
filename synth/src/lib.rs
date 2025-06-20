// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

mod ymfm_bridge;

use std::fmt;
use ymfm_bridge::ffi::*;

fn ym2608_write_high_and_low(addr: u8, data: u8) {
    ym2608_write_low(addr, data);
    ym2608_write_high(addr, data);
}

pub fn create() {
    if !ym2608_create() {
        panic!("Failed to create YM2608 instance");
    }
    ym2608_reset();

    // YM2608 mode.
    ym2608_write_low(0x29, 0x80);

    // Panning.
    for i in 0..=2 {
        ym2608_write_high_and_low(0xb4 + i, 0xc0);
    }
}

pub struct FmTone {
    pub al: u8,
    pub fb: u8,
    pub op: [FmOperator; 4],

    pub lfo_freq: u8,
    pub ams: u8,
    pub pms: u8,
}

impl FmTone {
    fn value_fb_al(&self) -> u8 {
        (self.fb & 0x7) << 3 | (self.al & 0x07)
    }

    fn value_lfo_freq(&self) -> u8 {
        self.lfo_freq & 0xf
    }

    fn value_ams_pms(&self) -> u8 {
        (self.ams & 0x3) << 4 | (self.pms & 0x7)
    }
}

pub struct FmOperator {
    pub ar: u8,
    pub dr: u8,
    pub sr: u8,
    pub rr: u8,
    pub sl: u8,
    pub tl: u8,
    pub ks: u8,
    pub ml: u8,
    pub dt: u8,
    pub am: bool,
    pub ssg_eg: u8,
}

impl FmOperator {
    fn value_dt_ml(&self) -> u8 {
        (self.dt & 0x7) << 4 | (self.ml & 0x8)
    }

    fn value_ks_ar(&self) -> u8 {
        (self.ks & 0x3) << 6 | (self.ar & 0x1f)
    }

    fn value_am_dr(&self) -> u8 {
        (u8::from(self.am) << 7) | (self.dr & 0x1f)
    }

    fn value_sl_rr(&self) -> u8 {
        (self.sl & 0x7) << 4 | (self.rr & 0x7)
    }

    fn value_sr(&self) -> u8 {
        self.sr & 0x1f
    }

    fn value_ssg_eg(&self) -> u8 {
        self.ssg_eg & 0xf
    }

    fn value_tl(&self) -> u8 {
        self.tl & 0x3f
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

pub enum Note {
    C(u8),
    Cs(u8),
    D(u8),
    Eb(u8),
    E(u8),
    F(u8),
    Fs(u8),
    G(u8),
    Gs(u8),
    A(u8),
    Bb(u8),
    B(u8),
}

impl Note {
    fn octave(&self) -> u8 {
        match self {
            Note::C(octave)
            | Note::Cs(octave)
            | Note::D(octave)
            | Note::Eb(octave)
            | Note::E(octave)
            | Note::F(octave)
            | Note::Fs(octave)
            | Note::G(octave)
            | Note::Gs(octave)
            | Note::A(octave)
            | Note::Bb(octave)
            | Note::B(octave) => *octave,
        }
    }

    fn to_f_number(&self) -> u16 {
        match self {
            Note::C(_) => 0x016d,
            Note::Cs(_) => 0x0183,
            Note::D(_) => 0x0199,
            Note::Eb(_) => 0x01b1,
            Note::E(_) => 0x01cc,
            Note::F(_) => 0x01e8,
            Note::Fs(_) => 0x0205,
            Note::G(_) => 0x0223,
            Note::Gs(_) => 0x0243,
            Note::A(_) => 0x0266,
            Note::Bb(_) => 0x028a,
            Note::B(_) => 0x02b1,
        }
    }

    fn to_f_number_low(&self) -> u8 {
        (self.to_f_number() & 0xff) as u8
    }

    fn to_f_number_high_block(&self) -> u8 {
        ((self.octave() & 0x07) << 3) | ((self.to_f_number() >> 8) & 0x07) as u8
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Note::C(_) => "C",
            Note::Cs(_) => "C#",
            Note::D(_) => "D",
            Note::Eb(_) => "Eb",
            Note::E(_) => "E",
            Note::F(_) => "F",
            Note::Fs(_) => "F#",
            Note::G(_) => "G",
            Note::Gs(_) => "G#",
            Note::A(_) => "A",
            Note::Bb(_) => "Bb",
            Note::B(_) => "B",
        };

        write!(f, "{}{}", name, self.octave())
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
        panic!("Failed to destroy YM2608 instance");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_display() {
        assert_eq!(Note::C(2).to_string(), "C2");
        assert_eq!(Note::Gs(5).to_string(), "G#5");
    }

    #[test]
    fn test_note_to_f_number_block() {
        assert_eq!(Note::C(0).to_f_number_low(), 0x6d);
        assert_eq!(Note::C(0).to_f_number_high_block(), 0x01);

        assert_eq!(Note::Bb(5).to_f_number_low(), 0x8a);
        assert_eq!(Note::Bb(5).to_f_number_high_block(), 0x2a);
    }
}
