# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### Unreleased
- [added] Add support for 'otgfs' on STM32F401 and STM32F411
- [added] Add support for 'uart' on STM32F4 series

### v0.13.0 (2020-11-28)

- [removed] Removed interrupt bindings

### v0.12.3 (2020-09-07)

- [added] Added support for STM32F303

### v0.12.1 (2020-05-03)

- [fixed] Fixed `RCC_PLLCFGR` and `RCC_CFGR` mappings for STM32F4

### v0.12.0 (2020-05-01)

- [added] Wrote tests for peripheral macros
- [added] Added `i2c` peripheral mappings for STM32F4 family

### v0.11.1 (2019-11-27)

- [fixed] Fixed `uart` peripheral mappings for STM32L4/STM32L4+
- [fixed] Fixed `dma`, `tim` peripheral mappings for STM32F4

### v0.11.0 (2019-11-06)

- [added] Added STM32F4 family support with `adc`, `dma`, `exti`, `gpio`, `tim`
  peripheral mappings
- [changed] Using `stm2_mcu` config flag to specify the MCU model
- [changed] Extracted `drone-svd` crate

### v0.10.1 (2019-09-27)

- [fixed] Fixed API documentation by moving to self-hosted
  https://api.drone-os.com
