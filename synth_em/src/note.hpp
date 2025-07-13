// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

#pragma once

#include <cstdint>

namespace synth {
enum class NoteName : int { C, Cs, D, Eb, E, F, Fs, G, Gs, A, Bb, B };

struct Note {
  NoteName name;
  std::uint8_t octave;

  bool operator==(const Note& other) const noexcept {
    return name == other.name && octave == other.octave;
  }

  bool operator!=(const Note& other) const noexcept {
    return !(*this == other);
  }
};
}  // namespace synth

namespace std {
template <>
struct hash<synth::Note> {
  std::size_t operator()(const synth::Note& note) const {
    return static_cast<int>(note.name) + (note.octave << 4);
  }
};
}  // namespace std
