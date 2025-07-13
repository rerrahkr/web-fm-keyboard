// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

#include <emscripten/bind.h>
#include <emscripten/val.h>

#include <algorithm>
#include <array>
#include <list>
#include <mutex>
#include <unordered_set>
#include <utility>
#include <variant>
#include <vector>

#include "chip/ym2608.hpp"
#include "instrument.hpp"
#include "keyboard.hpp"
#include "miniaudio.h"

namespace {
// Chip instance.
using VariantChip = std::variant<synth::chip::Ym2608>;
VariantChip chip_instance;

// Temporary buffer.
constexpr std::size_t kInitialBufferSize{0x10000};
std::vector<float> tmp_buffer[2] = {
    std::vector<float>(kInitialBufferSize, 0.f),
    std::vector<float>(kInitialBufferSize, 0.f),
};

// Resampling settings.
std::uint32_t sampling_rate{44100};
ma_resampler left_resampler, right_resampler;

// Keyboard management.
synth::Keyboard keyboard{6};

// Command management.
struct NoteOnCommand {
  std::uint8_t ch;
  synth::Note note;
};

struct NoteOffCommand {
  std::uint8_t ch;
  synth::Note note;
};

struct SetInstrumentCommand {
  synth::FmInstrument instrument;
};

using Command =
    std::variant<NoteOnCommand, NoteOffCommand, SetInstrumentCommand>;
std::list<Command> command_memory{};

struct CommandVisitor {
  void operator()(const NoteOnCommand& command) const {
    std::visit([command](auto& chip) { chip.NoteOn(command.ch, command.note); },
               chip_instance);
  }

  void operator()(const NoteOffCommand& command) const {
    std::visit(
        [command](auto& chip) { chip.NoteOff(command.ch, command.note); },
        chip_instance);
  }

  void operator()(const SetInstrumentCommand& command) const {
    std::visit(
        [command](auto& chip) { chip.SetInstrument(command.instrument); },
        chip_instance);
  }
};

CommandVisitor command_visitor{};

// Mutex.
std::mutex chip_input_mutex, chip_output_mutex;
}  // namespace

namespace synth {
enum class ChipType {
  Ym2608,
};

bool SetSamplingRate(std::uint32_t rate) {
  std::lock_guard<std::mutex> in_lock(chip_input_mutex);
  std::lock_guard<std::mutex> out_lock(chip_output_mutex);

  ma_resampler_uninit(&left_resampler, nullptr);
  ma_resampler_uninit(&right_resampler, nullptr);

  const auto internal_rate = std::visit(
      [](const auto& chip) { return chip.sampling_rate(); }, chip_instance);

  ma_resampler_config config = ma_resampler_config_init(
      ma_format_f32, 1, internal_rate, rate, ma_resample_algorithm_linear);

  if (ma_resampler_init(&config, nullptr, &left_resampler) != MA_SUCCESS ||
      ma_resampler_init(&config, nullptr, &right_resampler) != MA_SUCCESS) {
    return false;
  }

  sampling_rate = rate;
  return true;
}

bool SwitchChip(ChipType type, std::uint32_t rate) {
  std::lock_guard<std::mutex> in_lock(chip_input_mutex);
  std::lock_guard<std::mutex> out_lock(chip_output_mutex);

  switch (type) {
    case ChipType::Ym2608:
      if (!std::holds_alternative<synth::chip::Ym2608>(chip_instance)) {
        chip_instance.emplace<synth::chip::Ym2608>();
      }
      break;

    default:
      // Unsupported chip type.
      return false;
  }

  // Reset note-on memory.
  const auto num_channels = std::visit(
      [](const auto& chip) { return chip.num_channels(); }, chip_instance);
  keyboard = Keyboard(num_channels);

  return SetSamplingRate(rate);
}

bool ChangeChip(ChipType type) { return SwitchChip(type, sampling_rate); }

bool Initialize() { return ChangeChip(ChipType::Ym2608); }

bool Deinitialize() {
  std::lock_guard<std::mutex> lock(chip_output_mutex);

  ma_resampler_uninit(&left_resampler, nullptr);
  ma_resampler_uninit(&right_resampler, nullptr);

  return true;
}

void Reset() {
  std::visit([](auto& chip) { chip.Reset(); }, chip_instance);
}

void NoteOn(const Note& note) {
  std::lock_guard<std::mutex> lock(chip_input_mutex);

  const auto result = keyboard.NoteOn(note);

  if (!result) {
    return;
  }

  std::uint8_t ch = result->first;
  if (const auto note_off_note = result->second) {
    command_memory.push_back(NoteOffCommand{ch, note_off_note.value()});
  }

  command_memory.push_back(NoteOnCommand{ch, note});
}

void NoteOff(const Note& note) {
  std::lock_guard<std::mutex> lock(chip_input_mutex);

  const auto result = keyboard.NoteOff(note);

  if (result) {
    command_memory.push_back(NoteOffCommand{result.value(), note});
  }
}

void SetInstrument(const FmInstrument& instrument) {
  std::lock_guard<std::mutex> lock(chip_input_mutex);

  command_memory.push_back(SetInstrumentCommand{instrument});
}

void Generate(emscripten::val left_buffer, emscripten::val right_buffer,
              std::uint32_t num_samples) {
  {
    std::lock_guard<std::mutex> lock(chip_input_mutex);

    while (!command_memory.empty()) {
      std::visit(command_visitor, command_memory.front());
      command_memory.pop_front();
    }
  }

  {
    std::lock_guard<std::mutex> lock(chip_output_mutex);

    // Convert SharedArrayBuffer to Float32Array views.
    float* left_ptr =
        reinterpret_cast<float*>(left_buffer.as<std::uintptr_t>());
    float* right_ptr =
        reinterpret_cast<float*>(right_buffer.as<std::uintptr_t>());

    ma_uint64 required_input_samples = 0;
    ma_uint64 output_samples = static_cast<ma_uint64>(num_samples);
    if (ma_resampler_get_required_input_frame_count(
            &left_resampler, output_samples, &required_input_samples) !=
        MA_SUCCESS) {
      return;
    }

    if (required_input_samples > tmp_buffer[0].size()) {
      tmp_buffer[0].resize(required_input_samples, 0.f);
      tmp_buffer[1].resize(required_input_samples, 0.f);
    }

    std::visit(
        [required_input_samples](auto& chip) {
          chip.Generate(tmp_buffer[0].data(), tmp_buffer[1].data(),
                        static_cast<std::uint32_t>(required_input_samples));
        },
        chip_instance);

    ma_resampler_process_pcm_frames(&left_resampler, tmp_buffer[0].data(),
                                    &required_input_samples, left_ptr,
                                    &output_samples);
    ma_resampler_process_pcm_frames(&right_resampler, tmp_buffer[1].data(),
                                    &required_input_samples, right_ptr,
                                    &output_samples);
  }
}
}  // namespace synth

EMSCRIPTEN_BINDINGS(synth_module) {
  using namespace synth;

  emscripten::enum_<ChipType>("ChipType").value("Ym2608", ChipType::Ym2608);

  emscripten::enum_<NoteName>("NoteName")
      .value("C", NoteName::C)
      .value("Cs", NoteName::Cs)
      .value("D", NoteName::D)
      .value("Eb", NoteName::Eb)
      .value("E", NoteName::E)
      .value("F", NoteName::F)
      .value("Fs", NoteName::Fs)
      .value("G", NoteName::G)
      .value("Gs", NoteName::Gs)
      .value("A", NoteName::A)
      .value("Bb", NoteName::Bb)
      .value("B", NoteName::B);

  emscripten::value_object<Note>("Note")
      .field("name", &Note::name)
      .field("octave", &Note::octave);

  emscripten::value_array<std::array<FmOperator, 4>>("FmOperatorArray")
      .element(emscripten::index<0>())
      .element(emscripten::index<1>())
      .element(emscripten::index<2>())
      .element(emscripten::index<3>());

  emscripten::value_object<FmOperator>("FmOperator")
      .field("ar", &FmOperator::ar)
      .field("dr", &FmOperator::dr)
      .field("sr", &FmOperator::sr)
      .field("rr", &FmOperator::rr)
      .field("sl", &FmOperator::sl)
      .field("tl", &FmOperator::tl)
      .field("ks", &FmOperator::ks)
      .field("ml", &FmOperator::ml)
      .field("dt", &FmOperator::dt)
      .field("ssg_eg", &FmOperator::ssg_eg)
      .field("am", &FmOperator::am);

  emscripten::value_object<FmInstrument>("FmInstrument")
      .field("al", &FmInstrument::al)
      .field("fb", &FmInstrument::fb)
      .field("op", &FmInstrument::op)
      .field("lfo_freq", &FmInstrument::lfo_freq)
      .field("ams", &FmInstrument::ams)
      .field("pms", &FmInstrument::pms);

  emscripten::function("initialize", &Initialize);
  emscripten::function("deinitialize", &Deinitialize);
  emscripten::function("reset", &Reset);
  emscripten::function("changeChip", &ChangeChip);
  emscripten::function("setSamplingRate", &SetSamplingRate);
  emscripten::function("noteOn", &NoteOn);
  emscripten::function("noteOff", &NoteOff);
  emscripten::function("setInstrument", &SetInstrument);
  emscripten::function("generate", &Generate);
}
