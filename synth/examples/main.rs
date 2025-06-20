// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

use synth::*;

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

    create();

    set_tone(&tone);

    note_on(0, Note::C(4));
    note_off(0);

    destroy();
}
