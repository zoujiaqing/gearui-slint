// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! # Composed Controls
//!
//! Higher-level controls that combine multiple primitive and interactive controls.
//! These provide complete user interface components like buttons, checkboxes, etc.

pub mod button;
pub mod progress_indicator;
pub mod spinner;

pub use button::Button;
pub use progress_indicator::{ProgressIndicator, ProgressIndicatorItem};
pub use spinner::{Spinner, SpinnerItem};
