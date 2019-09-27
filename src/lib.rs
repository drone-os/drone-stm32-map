//! STM32 peripheral mappings for Drone, an Embedded Operating System.
//!
//! This crate uses
//! [CMSIS-SVD](https://arm-software.github.io/CMSIS_5/SVD/html/index.html)
//! files provided by [STMicroelectronics](https://www.st.com/) to automatically
//! generate Drone register and interrupt bindings. However only the
//! corresponding Reference Manual is the single source of truth. A difference
//! between this crate bindings and the Reference Manual is considered a
//! bug. Fixing such a bug is *not a breaking change*.
//!
//! This crate re-exports the contents of [`drone_cortex_m::map`] module and is
//! a drop-in replacement for it.
//!
//! # Supported Devices
//!
//! |                                                     Device name / Cargo feature | Core name | Reference manual |
//! |-------------|-----------------------|--------------------------------------------------------------------------|
//! | `stm32f100` | ARM® Cortex®-M3 r1p1  | [RM0041](https://www.st.com/resource/en/reference_manual/cd00246267.pdf) |
//! | `stm32f101` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) |
//! | `stm32f102` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) |
//! | `stm32f103` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) |
//! | `stm32f107` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) |
//! | `stm32l4x1` | ARM® Cortex®-M4F r0p1 | [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940.pdf) |
//! | `stm32l4x2` | ARM® Cortex®-M4F r0p1 | [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940.pdf) |
//! | `stm32l4x3` | ARM® Cortex®-M4F r0p1 | [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940.pdf) |
//! | `stm32l4x5` | ARM® Cortex®-M4F r0p1 | [RM0351](https://www.st.com/resource/en/reference_manual/dm00083560.pdf) |
//! | `stm32l4x6` | ARM® Cortex®-M4F r0p1 | [RM0351](https://www.st.com/resource/en/reference_manual/dm00083560.pdf) |
//! | `stm32l4r5` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) |
//! | `stm32l4s5` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) |
//! | `stm32l4r7` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) |
//! | `stm32l4s7` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) |
//! | `stm32l4r9` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) |
//! | `stm32l4s9` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) |
//!
//! **NOTE** Exactly one cargo feature should be selected based on the device
//! model.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-stm32-map/0.10)
//!
//! The API documentation intentionally skips auto-generated [`reg`] and [`thr`]
//! bindings. Otherwise it would use several gigabytes of space and would be
//! very slow to render in a browser. One should refer to the Reference Manual
//! instead. And to get an idea of what the API looks like on the Drone side,
//! look at the [`drone_cortex_m::map`] module documentation.
//!
//! # Usage
//!
//! Place the following to the Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! drone-stm32-map = { version = "0.10.1", features = [...] }
//! ```

#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![no_std]

pub mod periph;
pub mod reg;
pub mod thr;

pub use drone_stm32_map_pieces::stm32_reg_tokens;
