// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! # Interactive Controls
//!
//! 交互式控件模块，包含用户可以操作的控件

pub mod checkbox;
pub mod slider;
pub mod switch;
pub mod switch_composite;
pub mod touch_area;

pub use checkbox::CheckBox;
pub use slider::Slider;
pub use switch::{Switch, SwitchItem};
pub use switch_composite::SwitchComposite;
pub use touch_area::{TouchArea, TouchAreaItem};
