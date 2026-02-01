// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

/*!
# Rectangle Control - Simplified 3-Layer Architecture

A basic implementation of Rectangle control using GearUI's 3-layer architecture.
This is a simplified version focused on getting the architecture working.

## 3-Layer Architecture Implementation

### Layer 1: RectangleItem (Item trait implementation)
Core rendering, layout, and event handling logic.

### Layer 2: Rectangle (View control wrapper)
Public API with builder pattern.

### Layer 3: GearView Implementation (Placeholder)
Conversion methods for future ItemTree/Component integration.

## Field Mapping to Slint Rectangle
| GearUI Field | Slint Field | Type  | Default Value |
|-------------|-------------|-------|---------------|
| background  | background  | Brush | transparent   |

## Usage Examples

### Basic Usage
```rust
use i_slint_gearui::Rectangle;
use i_slint_core::graphics::{Brush, Color};

let rect = Rectangle::new()
    .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)));
```
*/

use crate::View;
use const_field_offset::FieldOffsets;
use core::pin::Pin;
use i_slint_core::{
    ItemVTable_static,
    Property,
    declare_item_vtable,
    graphics::Brush,
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer, RenderRectangle},
    item_tree::ItemTreeRc, // Fixed: ItemTreeRc is in item_tree module
    items::{Item, ItemConsts, ItemRc, ItemVTable, RenderingResult},
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    window::WindowAdapterRc,
};
use i_slint_core_macros::*;
use std::rc::Rc;

// Required for VTable macro
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Layer 1: Core Rectangle Item Implementation
///
/// This implements the Item trait and provides the actual rendering/layout logic.
/// Field names and default values exactly match Slint's Rectangle specification.
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct RectangleItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Background color/brush (corresponds to Slint: background)
    /// Default: transparent
    pub background: Property<Brush>,
    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for RectangleItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {
        // No initialization required for basic rectangle
    }

    fn layout_info(
        self: Pin<&Self>,
        _orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        // Rectangle has no intrinsic size - it adapts to its container
        LayoutInfo::default()
    }

    fn input_event_filter_before_children(
        self: Pin<&Self>,
        _event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventFilterResult {
        InputEventFilterResult::ForwardAndIgnore
    }

    fn input_event(
        self: Pin<&Self>,
        _event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventResult {
        InputEventResult::EventIgnored
    }

    fn key_event(
        self: Pin<&Self>,
        _event: &KeyEvent,
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
        _event: &FocusEvent,
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
        // 🎯 调试：检查 Rectangle render 方法是否被调用
        println!("🎯 RectangleItem::render called! size: {:?}", size);
        let background_pin = unsafe { Pin::new_unchecked(&self.get_ref().background) };
        println!("🎯   Background: {:?}", background_pin.get());

        // Delegate to the RenderRectangle trait implementation
        println!("🎯   Calling backend.draw_rectangle...");
        backend.draw_rectangle(self, self_rc, size, &self.cached_rendering_data);
        println!("🎯   backend.draw_rectangle completed!");
        RenderingResult::ContinueRenderingChildren
    }

    fn bounding_rect(
        self: Pin<&Self>,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
        geometry: LogicalRect,
    ) -> LogicalRect {
        geometry
    }

    fn clips_children(self: Pin<&Self>) -> bool {
        false
    }
}

impl RenderRectangle for RectangleItem {
    fn background(self: Pin<&Self>) -> Brush {
        // Fixed: Use proper Pin API for Property access
        let pinned = unsafe { Pin::new_unchecked(&self.get_ref().background) };
        pinned.get()
    }
}

impl ItemConsts for RectangleItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUIRectangleVTable() -> GearUIRectangleVTable for RectangleItem
}

/// Layer 2: Rectangle View Control Wrapper
///
/// This is a simplified public API for basic Rectangle usage.
/// Full integration with Slint systems will be implemented in future phases.
#[derive(Clone)]
pub struct Rectangle {
    // Store the background directly for now
    background: Brush,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    /// Create a new Rectangle with default properties
    pub fn new() -> Self {
        Self {
            background: Brush::default(), // transparent
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 50.0,
        }
    }

    /// Set the background brush
    pub fn with_background(mut self, background: Brush) -> Self {
        println!("🎯 Rectangle::with_background called!");
        println!("🎯   Input background: {:?}", background);
        self.background = background;
        println!("🎯   Stored background: {:?}", self.background);
        self
    }

    /// Get the current background brush
    pub fn background(&self) -> &Brush {
        &self.background
    }

    /// Set x position
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Set y position
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Set width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Create a RectangleItem with this Rectangle's properties
    ///
    /// This is the core integration method for creating actual Items.
    pub fn create_item(&self) -> RectangleItem {
        let item = RectangleItem::default();

        // 🎯 设置位置和尺寸 - 使用用户设置的值
        item.x.set(LogicalLength::new(self.x));
        item.y.set(LogicalLength::new(self.y));
        item.width.set(LogicalLength::new(self.width));
        item.height.set(LogicalLength::new(self.height));

        item.background.set(self.background.clone());
        item
    }

    /// Create a shared Rc<RectangleItem> with this Rectangle's properties
    ///
    /// This is useful for shared ownership scenarios.
    pub fn create_item_rc(&self) -> Rc<RectangleItem> {
        Rc::new(self.create_item())
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}

/// Layer 3: View Implementation - Unified Architecture
///
/// This provides the complete API integration for Rectangle controls.
impl crate::View for Rectangle {
    type ItemType = RectangleItem;

    /// Create a new RectangleItem with current properties
    fn create_item(self) -> Self::ItemType {
        let item = RectangleItem::default();

        // 🎯 Debug output for creation (before moving)
        println!("🎯 Rectangle::create_item (View trait): background = {:?}", self.background);

        // Apply properties using .set() method for Property types
        item.background.set(self.background.clone());

        // 🎯 Verify the background was set correctly
        let background_pin = unsafe { Pin::new_unchecked(&item.background) };
        println!("🎯   After setting: item.background = {:?}", background_pin.get());

        item
    }

    /// 🎯 **关键修复**：Rectangle 使用 CompositeItemTree 架构
    ///
    /// 与 Button 控件保持一致，创建带 WindowItem 根节点的结构
    fn into_item_tree(self) -> Option<ItemTreeRc> {
        println!("🎯 Rectangle::into_item_tree: Creating CompositeItemTree with WindowItem root");

        // 创建 WindowItem 作为根节点
        let window_item = i_slint_core::items::WindowItem::default();

        // 🎯 **关键修复**：让 Rectangle 转移属性到 WindowItem
        self.transfer_properties_to_window(&window_item);

        // 创建 Rectangle 控件
        let rect_item = self.create_item();

        // 创建 CompositeItemTree：WindowItem(index 0) + Rectangle(index 1)
        let tree_rc =
            crate::item_tree_integration::create_composite_item_tree(window_item, rect_item);

        println!("✅ Rectangle::into_item_tree: CompositeItemTree created successfully");
        Some(tree_rc)
    }

    /// 🎯 **关键功能**：将 Rectangle 的背景色转移到 WindowItem
    ///
    /// 这样 Rectangle 的背景色会成为整个窗口的背景色
    fn transfer_properties_to_window(&self, window_item: &i_slint_core::items::WindowItem) {
        println!("🎯 Rectangle::transfer_properties_to_window called!");
        println!("🎯   Rectangle background: {:?}", self.background);

        // 设置窗口背景色为 Rectangle 的背景色
        // 这是一个强大的特性：Rectangle 可以控制整个窗口的外观
        window_item.background.set(self.background.clone());

        println!("✅ Window background set to Rectangle background!");
    }

    // Phase 3: Use default implementation that properly handles ItemTree -> Component conversion
    // The default trait implementation will call self.into_item_tree() and wrap it in GearUIComponent
}

/// Helper functions for creating Rectangle items and integrations
pub mod helpers {
    use super::*;

    /// Create a red rectangle (useful for testing/debugging)
    pub fn red_rect() -> Rectangle {
        use i_slint_core::graphics::Color;
        Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
    }

    /// Create a blue rectangle
    pub fn blue_rect() -> Rectangle {
        use i_slint_core::graphics::Color;
        Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(0, 100, 255)))
    }

    /// Create a green rectangle
    pub fn green_rect() -> Rectangle {
        use i_slint_core::graphics::Color;
        Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(0, 200, 100)))
    }
}
