// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

/*!
# Primitives Module

Basic drawing elements that form the foundation of all other controls.
These are the lowest-level controls that directly interface with Slint's rendering system.

## Phase 1A Status - MVP Rectangle Only

Currently only Rectangle is fully working and exported.
Other primitives will be enabled as compilation issues are resolved.

## Available Controls

- **Rectangle** ✅ - Filled shapes with background colors
- **Text** 🚧 - Text rendering (compilation fixes in progress)
- **Image** 🚧 - Image display (compilation fixes in progress)
*/

// Core primitive controls
pub mod rectangle;
pub use rectangle::Rectangle;

// Text control - now with View implementation!
pub mod text;
pub use text::Text;

pub mod image;
pub use image::Image;

pub mod clip;
pub use clip::{Clip, ClipItem};

pub mod opacity;
pub use opacity::{Opacity, OpacityItem};
