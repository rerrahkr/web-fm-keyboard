// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

pub struct FmTone {
    pub al: u8,
    pub fb: u8,
    pub op: [FmOperator; 4],

    pub lfo_freq: u8,
    pub ams: u8,
    pub pms: u8,
}

impl FmTone {
    pub(crate) fn value_fb_al(&self) -> u8 {
        (self.fb & 0x7) << 3 | (self.al & 0x07)
    }

    pub(crate) fn value_lfo_freq(&self) -> u8 {
        self.lfo_freq & 0xf
    }

    pub(crate) fn value_ams_pms(&self) -> u8 {
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
    pub(crate) fn value_dt_ml(&self) -> u8 {
        (self.dt & 0x07) << 4 | (self.ml & 0x0f)
    }
    pub(crate) fn value_ks_ar(&self) -> u8 {
        (self.ks & 0x3) << 6 | (self.ar & 0x1f)
    }

    pub(crate) fn value_am_dr(&self) -> u8 {
        (u8::from(self.am) << 7) | (self.dr & 0x1f)
    }

    pub(crate) fn value_sl_rr(&self) -> u8 {
        (self.sl & 0x0f) << 4 | (self.rr & 0x0f)
    }

    pub(crate) fn value_sr(&self) -> u8 {
        self.sr & 0x1f
    }

    pub(crate) fn value_ssg_eg(&self) -> u8 {
        self.ssg_eg & 0x0f
    }

    pub(crate) fn value_tl(&self) -> u8 {
        self.tl & 0x3f
    }
}
