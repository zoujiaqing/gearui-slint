// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

/*!
# Image Control - Primitive

A 1:1 translation of Slint's ImageItem element into native Rust.
This provides the same functionality as the built-in Image primitive.

## Field Mapping to Slint ImageItem
All fields match exactly with Slint's ImageItem element definition:

| GearUI Field           | Slint Field            | Type                | Default Value        |
|------------------------|------------------------|---------------------|----------------------|
| source                 | source                 | Image               | empty                |
| width                  | width                  | LogicalLength       | 0px                  |
| height                 | height                 | LogicalLength       | 0px                  |
| image_fit              | image-fit              | ImageFit            | fill                 |
| image_rendering        | image-rendering        | ImageRendering      | smooth               |
| colorize               | colorize               | Brush               | transparent          |

## ItemTree Integration
- Provides `into_item_tree()` for integration into ItemTree structures
- Implements full Slint Item trait for seamless rendering
- Full compatibility with Slint's internal architecture

## Usage Example
```rust
use i_slint_gearui::Image;

let image = Image::new()
    .with_source(image_data)
    .with_image_fit(ImageFit::Contain);
```
*/

use const_field_offset::FieldOffsets;
use core::pin::Pin;
use i_slint_core::{
    Coord, ItemVTable_static, Property, declare_item_vtable,
    graphics::{Brush, Image as SlintImage},
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer, RenderImage},
    items::{
        ImageFit, ImageHorizontalAlignment, ImageRendering, ImageTiling, ImageVerticalAlignment,
        Item, ItemConsts, ItemRc, ItemVTable, RenderingResult,
    },
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    window::{WindowAdapter, WindowAdapterRc},
};
use i_slint_core_macros::*;
use std::rc::Rc;

// Required for VTable macro and types
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Native Rust ImageItem - 1:1 translation of Slint's ImageItem element
///
/// All field names and default values exactly match Slint's ImageItem specification.
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct ImageItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,
    /// Control width (corresponds to Slint: width)
    pub width: Property<LogicalLength>,
    /// Control height (corresponds to Slint: height)
    pub height: Property<LogicalLength>,

    /// Image source (corresponds to Slint: source)
    pub source: Property<SlintImage>,
    /// Image fitting mode (corresponds to Slint: image-fit)
    pub image_fit: Property<ImageFit>,
    /// Image rendering quality (corresponds to Slint: image-rendering)
    pub image_rendering: Property<ImageRendering>,
    /// Color overlay mask (corresponds to Slint: colorize)
    pub colorize: Property<Brush>,
    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for ImageItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {}

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        let natural_size = self.source().size();
        LayoutInfo {
            preferred: match orientation {
                _ if natural_size.width == 0 || natural_size.height == 0 => 0 as Coord,
                Orientation::Horizontal => natural_size.width as Coord,
                Orientation::Vertical => {
                    natural_size.height as Coord * self.width().get() / natural_size.width as Coord
                }
            },
            ..Default::default()
        }
    }

    fn input_event_filter_before_children(
        self: Pin<&Self>,
        _: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventFilterResult {
        InputEventFilterResult::ForwardAndIgnore
    }

    fn input_event(
        self: Pin<&Self>,
        _: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventResult {
        InputEventResult::EventIgnored
    }

    fn key_event(
        self: Pin<&Self>,
        _: &KeyEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        KeyEventResult::EventIgnored
    }

    fn capture_key_event(
        self: Pin<&Self>,
        _event: &KeyEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        KeyEventResult::EventIgnored
    }

    fn focus_event(
        self: Pin<&Self>,
        _: &FocusEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> FocusEventResult {
        FocusEventResult::FocusIgnored
    }

    fn render(
        self: Pin<&Self>,
        backend: &mut &mut dyn ItemRenderer,
        self_rc: &ItemRc,
        size: LogicalSize,
    ) -> RenderingResult {
        (*backend).draw_image(self, self_rc, size, &self.cached_rendering_data);
        RenderingResult::ContinueRenderingChildren
    }

    fn bounding_rect(
        self: core::pin::Pin<&Self>,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
        geometry: LogicalRect,
    ) -> LogicalRect {
        geometry
    }

    fn clips_children(self: core::pin::Pin<&Self>) -> bool {
        false
    }
}

impl RenderImage for ImageItem {
    fn target_size(self: Pin<&Self>) -> LogicalSize {
        let width_pin = unsafe { Pin::new_unchecked(&self.get_ref().width) };
        let height_pin = unsafe { Pin::new_unchecked(&self.get_ref().height) };
        LogicalSize::from_lengths(width_pin.get(), height_pin.get())
    }

    fn source(self: Pin<&Self>) -> SlintImage {
        let source_pin = unsafe { Pin::new_unchecked(&self.get_ref().source) };
        source_pin.get()
    }

    fn source_clip(self: Pin<&Self>) -> Option<i_slint_core::graphics::IntRect> {
        None
    }

    fn image_fit(self: Pin<&Self>) -> ImageFit {
        let fit_pin = unsafe { Pin::new_unchecked(&self.get_ref().image_fit) };
        fit_pin.get()
    }

    fn rendering(self: Pin<&Self>) -> ImageRendering {
        let rendering_pin = unsafe { Pin::new_unchecked(&self.get_ref().image_rendering) };
        rendering_pin.get()
    }

    fn colorize(self: Pin<&Self>) -> Brush {
        let colorize_pin = unsafe { Pin::new_unchecked(&self.get_ref().colorize) };
        colorize_pin.get()
    }

    fn alignment(self: Pin<&Self>) -> (ImageHorizontalAlignment, ImageVerticalAlignment) {
        Default::default()
    }

    fn tiling(self: Pin<&Self>) -> (ImageTiling, ImageTiling) {
        Default::default()
    }
}

impl ItemConsts for ImageItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<
        ImageItem,
        CachedRenderingData,
    > = ImageItem::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUIImageVTable() -> GearUIImageVTable for ImageItem
}

/// High-level Image control with Builder pattern API
#[derive(Clone)]
pub struct Image {
    // Store properties directly for simple builder pattern
    source: SlintImage,
    width: LogicalLength,
    height: LogicalLength,
    x: LogicalLength,
    y: LogicalLength,
    image_fit: ImageFit,
    image_rendering: ImageRendering,
    colorize: Brush,
}

impl Image {
    /// Create a new Image with default values
    pub fn new() -> Self {
        Self {
            source: SlintImage::default(),
            width: LogicalLength::new(0.0),
            height: LogicalLength::new(0.0),
            x: LogicalLength::new(0.0),
            y: LogicalLength::new(0.0),
            image_fit: ImageFit::Fill,
            image_rendering: ImageRendering::Smooth,
            colorize: Brush::default(),
        }
    }

    /// Builder pattern: Set image source
    pub fn with_source(mut self, source: SlintImage) -> Self {
        self.source = source;
        self
    }

    /// Builder pattern: Set control width
    pub fn with_width(mut self, width: LogicalLength) -> Self {
        self.width = width;
        self
    }

    /// Builder pattern: Set control height
    pub fn with_height(mut self, height: LogicalLength) -> Self {
        self.height = height;
        self
    }

    /// Builder pattern: Set image fitting mode
    pub fn with_image_fit(mut self, fit: ImageFit) -> Self {
        self.image_fit = fit;
        self
    }

    /// Builder pattern: Set rendering quality
    pub fn with_image_rendering(mut self, rendering: ImageRendering) -> Self {
        self.image_rendering = rendering;
        self
    }

    /// Builder pattern: Set color overlay
    pub fn with_colorize(mut self, colorize: Brush) -> Self {
        self.colorize = colorize;
        self
    }

    // ItemRc conversion will be implemented when ItemTree integration is complete
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::View for Image {
    type ItemType = ImageItem;

    fn create_item(self) -> Self::ItemType {
        let item = ImageItem {
            x: Property::new(self.x),
            y: Property::new(self.y),
            width: Property::new(self.width),
            height: Property::new(self.height),
            source: Property::new(self.source),
            image_fit: Property::new(self.image_fit),
            image_rendering: Property::new(self.image_rendering),
            colorize: Property::new(self.colorize),
            cached_rendering_data: CachedRenderingData::default(),
        };

        item
    }

    fn transfer_properties_to_window(&self, _window_item: &i_slint_core::items::WindowItem) {
        // Image doesn't transfer properties to window
    }

    fn into_item_tree(self) -> Option<i_slint_core::item_tree::ItemTreeRc> {
        // 🎯 **关键修复**：使用 CompositeItemTree 而不是 SingleItemTree
        // Slint 要求 ItemTree 的第0个 Item 必须是 WindowItem

        println!("🎯 Image::into_item_tree: Creating CompositeItemTree with WindowItem root");

        // 创建 WindowItem 作为根节点
        let window_item = i_slint_core::items::WindowItem::default();

        // 🎯 **属性转移**：设置窗口背景为白色，表示 Image 控件存在
        let bg_color = i_slint_core::graphics::Color::from_rgb_u8(255, 255, 255); // 白色背景
        window_item.background.set(i_slint_core::graphics::Brush::SolidColor(bg_color));

        println!("🎯 Image white background transferred to WindowItem");

        // 创建 Image 控件作为子节点
        let image_item = self.create_item();

        // 创建 CompositeItemTree：WindowItem (index 0) + Image (index 1)
        let tree_rc =
            crate::item_tree_integration::create_composite_item_tree(window_item, image_item);
        Some(tree_rc)
    }
}
