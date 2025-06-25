// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

use std::fmt;

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
    pub(crate) fn octave(&self) -> u8 {
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

    pub(crate) fn to_f_number(&self) -> u16 {
        match self {
            Note::C(_) => 0x0266,
            Note::Cs(_) => 0x028b,
            Note::D(_) => 0x02b2,
            Note::Eb(_) => 0x02db,
            Note::E(_) => 0x0307,
            Note::F(_) => 0x0334,
            Note::Fs(_) => 0x0365,
            Note::G(_) => 0x0398,
            Note::Gs(_) => 0x03ce,
            Note::A(_) => 0x0406,
            Note::Bb(_) => 0x0442,
            Note::B(_) => 0x0480,
        }
    }

    pub(crate) fn to_f_number_low(&self) -> u8 {
        (self.to_f_number() & 0xff) as u8
    }

    pub(crate) fn to_f_number_high_block(&self) -> u8 {
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
        assert_eq!(Note::C(0).to_f_number_low(), 0x66);
        assert_eq!(Note::C(0).to_f_number_high_block(), 0x02);

        assert_eq!(Note::Bb(5).to_f_number_low(), 0x42);
        assert_eq!(Note::Bb(5).to_f_number_high_block(), 0x2c);
    }
}
