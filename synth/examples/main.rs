// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

use std::{fs::File, io::BufWriter};
use synth::*;

fn stream(writer: &mut hound::WavWriter<BufWriter<File>>, length: usize) {
    const BUF_SIZE: usize = 0x10000;
    let (mut left, mut right) = ([0; BUF_SIZE], [0; BUF_SIZE]);

    const BLOCK: usize = 2000;

    for _ in 0..length {
        generate(left.as_mut_ptr(), right.as_mut_ptr(), BLOCK);

        for i in 0..BLOCK {
            writer.write_sample(left[i]).unwrap();
            writer.write_sample(right[i]).unwrap();
        }
    }
}

fn main() {
    let tone = FmTone {
        al: 0x04,
        fb: 0x07,
        op: [
            FmOperator {
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
            },
            FmOperator {
                ar: 0x1f,
                dr: 10,
                sr: 0x00,
                rr: 0x07,
                sl: 0x01,
                tl: 0,
                ks: 0x00,
                ml: 4,
                dt: 0x00,
                am: false,
                ssg_eg: 0x00,
            },
            FmOperator {
                ar: 0x1f,
                dr: 0x00,
                sr: 0x00,
                rr: 0x00,
                sl: 0x00,
                tl: 21,
                ks: 0x00,
                ml: 4,
                dt: 0x03,
                am: false,
                ssg_eg: 0x00,
            },
            FmOperator {
                ar: 0x1f,
                dr: 10,
                sr: 0x00,
                rr: 0x07,
                sl: 0x01,
                tl: 0x00,
                ks: 0x00,
                ml: 0x04,
                dt: 0x03,
                am: false,
                ssg_eg: 0x00,
            },
        ],
        lfo_freq: 0,
        ams: 0,
        pms: 0,
    };

    const SAMPLE_RATE: u32 = 44_100;

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut wav_writer = hound::WavWriter::create("sound.wav", spec).unwrap();

    create(SAMPLE_RATE as f64);

    stream(&mut wav_writer, 20);

    set_tone(&tone);

    note_on(0, Note::C(3));

    stream(&mut wav_writer, 50);

    note_off(0);

    stream(&mut wav_writer, 20);

    destroy();

    wav_writer.finalize().unwrap();
}
