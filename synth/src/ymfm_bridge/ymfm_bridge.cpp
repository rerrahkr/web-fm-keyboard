// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

#include "ymfm_bridge.hpp"

#include <memory>
#include <mutex>

#include "synth/src/lib.rs.h"

namespace {

ymfm::ymfm_interface interface;

using LockGuardMutex = std::lock_guard<std::mutex>;

std::unique_ptr<ymfm::ym2608> ym2608;
std::mutex ym2608_mutex;
constexpr std::uint32_t kYm2608Clock{3993600 * 2};
}

bool ym2608_create() {
  LockGuardMutex guard{ym2608_mutex};

  if (ym2608) {
    return false;
  }

  ym2608 = std::make_unique<ymfm::ym2608>(interface);
  ym2608->set_fidelity(ymfm::OPN_FIDELITY_MIN);
  ym2608->reset();

  return true;
}

bool ym2608_destroy() {
  LockGuardMutex guard{ym2608_mutex};
  if (!ym2608) {
    return false;
  }

  ym2608.reset();

  return true;
}

void ym2608_reset() {
  LockGuardMutex guard{ym2608_mutex};
  
  ym2608->reset();
}

std::uint32_t ym2608_sample_rate() {
  LockGuardMutex guard{ym2608_mutex};

  if (!ym2608) {
    return 0;
  }

  return ym2608->sample_rate(kYm2608Clock);
}

std::uint32_t ym2608_clock() {
  return kYm2608Clock;
}

std::uint8_t ym2608_read_low(std::uint8_t addr) {
  LockGuardMutex guard{ym2608_mutex};

  if (!ym2608) {
    return 0;
  }

  ym2608->write_address(addr);
  return ym2608->read_data();
}

std::uint8_t ym2608_read_high(std::uint8_t addr) {
  LockGuardMutex guard{ym2608_mutex};

  if (!ym2608) {
    return 0;
  }

  ym2608->write_address_hi(addr);
  return ym2608->read_data_hi();
}

void ym2608_write_low(std::uint8_t addr, std::uint8_t data) {
  LockGuardMutex guard{ym2608_mutex};

  if (!ym2608) {
    return;
  }

  ym2608->write_address(addr);
  ym2608->write_data(data);
}

void ym2608_write_high(std::uint8_t addr, std::uint8_t data) {
  LockGuardMutex guard{ym2608_mutex};

  if (!ym2608) {
    return;
  }

  ym2608->write_address_hi(addr);
  ym2608->write_data_hi(data);
}

void ym2608_generate(TwoChannelBuffer& buffer, std::uint32_t num_samples) {
  LockGuardMutex guard{ym2608_mutex};

  if (!ym2608) {
    return;
  }

  ymfm::ym2608::output_data output_data;
  for (std::uint32_t i = 0; i < num_samples; ++i) {
    ym2608->generate(&output_data);

    // Raise the volume.
    buffer.left[i] = output_data.data[0] << 1;
    buffer.right[i] = output_data.data[1] << 1;
  }
}
