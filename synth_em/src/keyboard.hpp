// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

#pragma once

#include <algorithm>
#include <list>
#include <optional>
#include <unordered_map>

#include "note.hpp"

namespace synth {
class Keyboard {
 public:
  explicit Keyboard(std::size_t num_polyphony) {
    std::generate_n(
        std::back_inserter(channels_), num_polyphony,
        [ch = static_cast<std::uint8_t>(0)]() mutable { return ch++; });
  }

  std::optional<std::pair<std::uint8_t, std::optional<Note>>> NoteOn(
      const Note& note) {
    std::optional<Note> popped;

    if (map_.find(note) != map_.end()) {
      return std::nullopt;
    } else if (channels_.empty()) {
      popped = std::move(queue_.back());
      queue_.pop_back();
      std::uint8_t ch = map_[popped.value()].second;
      channels_.push_back(ch);
      map_.erase(popped.value());
    }

    queue_.push_front(value);
    std::uint8_t ch = channels_.front();
    map_[value] = std::make_pair(queue_.begin(), ch);
    channels_.pop_front();

    return std::make_pair(ch, std::move(popped));
  }

  std::optional<std::uint8_t> NoteOff(const Note& note) {
    auto it = map_.find(note);
    if (it == map_.end()) {
      return std::nullopt;
    }

    std::uint8_t ch = it->second.second;
    queue_.erase(note);
    channels_.push_back(ch);
    map_.erase(note);

    return ch;
  }

 private:
  std::list<Note> queue_;
  std::list<std::uint8_t> channels_;
  std::unordered_map<Note, std::pair<std::list<Note>::iterator, std::uint8_t>>
      map_;
};
}  // namespace synth
