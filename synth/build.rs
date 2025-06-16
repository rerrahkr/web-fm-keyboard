// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

fn main() {
    cxx_build::bridge("src/ymfm_bridge.rs")
        .file("src/ymfm_bridge/ymfm_bridge.cpp")
        .file("lib/ymfm/src/ymfm_opn.cpp")
        .file("lib/ymfm/src/ymfm_adpcm.cpp")
        .file("lib/ymfm/src/ymfm_ssg.cpp")
        .include("lib/ymfm/src")
        .std("c++20")
        .flag("-Wno-unused-parameter")
        .compile("synth");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ymfm_bridge");
    println!("cargo:rerun-if-changed=lib/ymfm/src");
}
