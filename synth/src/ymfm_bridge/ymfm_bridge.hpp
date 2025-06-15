// SPDX-FileCopyrightText: 2025 Rerrah
// SPDX-License-Identifier: MIT

#pragma once

#include <cstdint>

#include "ymfm_opn.h"

struct TwoChannelBuffer;

/// All functions are thread-safe.

  /**
   * @brief Create YM2608 instance.
   * @return @c true if an instance was created successfully, @c false if one already exists.
   */
  bool ym2608_create();

  /**
   * @brief Destroy YM2608 instance.
   * @return @c true if an instance was destroyed successfully, @c false if no instance exists.
   */
  bool ym2608_destroy();

  /**
   * @brief Reset the YM2608 instance.
   */
  void ym2608_reset();

  /**
   * @brief Get the sample rate.
   * @return The sample rate of YM2608 chip calculated by its clock.
   */
  std::uint32_t ym2608_sample_rate();

  /**
   * @brief Read data from the YM2608 in A1 = 0 mode.
   * @param[in] addr the register address.
   * @return The data read from the specified register.
   * @note If the YM2608 instance does not exist, it returns 0.
   */
  std::uint8_t ym2608_read_low(std::uint8_t addr);

  /**
   * @brief Read data from the YM2608 in A1 = 1 mode.
   * @param[in] addr the register address.
   * @return The data read from the specified register.
   * @note If the YM2608 instance does not exist, it returns 0.
   */
  std::uint8_t ym2608_read_high(std::uint8_t addr);

  /**
   * @brief Write data to the YM2608 in A1 = 0 mode.
   * @param[in] addr the register address.
   * @param[in] data the data to write.
   */
  void ym2608_write_low(std::uint8_t addr, std::uint8_t data);

  /**
   * @brief Write data to the YM2608 in A1 = 1 mode.
   * @param[in] addr the register address.
   * @param[in] data the data to write.
   */
  void ym2608_write_high(std::uint8_t addr, std::uint8_t data);

  /**
   * @brief Generate samples.
   * @param[in] buffer The buffer to fill with generated samples.
   * @param[in] num_samples The number of samples to generate.
   * @note The buffer must have enough space for num_samples samples.
   */
  void ym2608_generate(TwoChannelBuffer& buffer, std::uint32_t num_samples);
