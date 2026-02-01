// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

/*!
# TouchArea Control - 按照 Text 成功模式实现

## 架构说明
- **Layer 1**: TouchAreaItem (impl Item) - 与 Slint 系统直接交互
- **Layer 2**: TouchArea (impl View) - 用户友好的构建器模式接口

## Slint 属性映射
完全按照 Slint TouchArea 元素的属性映射：
- enabled: 是否启用
- mouse-cursor: 鼠标光标
- pressed: 是否按下 (输出属性)
- has-hover: 是否悬停 (输出属性)

## 使用示例
```rust
TouchArea::new()
    .with_enabled(true)
    .with_mouse_cursor(MouseCursor::Hand);
```
*/

use crate::View;
use const_field_offset::FieldOffsets;
use core::pin::Pin;
use i_slint_core::{
    Callback, ItemVTable_static, Property, declare_item_vtable,
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer},
    items::{Item, ItemConsts, ItemRc, ItemVTable, MouseCursor, RenderingResult, VoidArg},
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    window::WindowAdapterRc,
};
use i_slint_core_macros::*;
use std::rc::Rc;

// Required for VTable macro
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Layer 1: Core TouchArea Item Implementation
///
/// 实现 Item trait，提供实际的交互处理逻辑。
/// 字段名和默认值完全匹配 Slint 的 TouchArea 规范。
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct TouchAreaItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Whether the touch area is enabled (corresponds to Slint: enabled)
    pub enabled: Property<bool>,
    /// Whether the touch area is currently pressed (corresponds to Slint: pressed)
    pub pressed: Property<bool>,
    /// Whether the mouse is hovering over the touch area (corresponds to Slint: has_hover)
    pub has_hover: Property<bool>,
    /// X coordinate of the mouse (corresponds to Slint: mouse_x)
    pub mouse_x: Property<LogicalLength>,
    /// Y coordinate of the mouse (corresponds to Slint: mouse_y)
    pub mouse_y: Property<LogicalLength>,
    /// X coordinate of the pressed point (corresponds to Slint: pressed_x)
    pub pressed_x: Property<LogicalLength>,
    /// Y coordinate of the pressed point (corresponds to Slint: pressed_y)
    pub pressed_y: Property<LogicalLength>,
    /// Mouse cursor to display (corresponds to Slint: mouse_cursor)
    pub mouse_cursor: Property<MouseCursor>,
    /// Callback triggered on click (corresponds to Slint: clicked)
    pub clicked: Callback<VoidArg>,
    /// Callback triggered on double click (corresponds to Slint: double_clicked)
    pub double_clicked: Callback<VoidArg>,
    /// Callback triggered on move (corresponds to Slint: moved)
    pub moved: Callback<VoidArg>,
    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for TouchAreaItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {
        // No initialization required for basic touch area
    }

    fn layout_info(
        self: Pin<&Self>,
        _orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        // ✅ 标准 Slint 模式：TouchArea 没有内在尺寸
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
        _backend: &mut &mut dyn ItemRenderer,
        _self_rc: &ItemRc,
        _size: LogicalSize,
    ) -> RenderingResult {
        // ✅ 标准 Slint 模式：TouchArea 是不可见的
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

impl ItemConsts for TouchAreaItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUITouchAreaVTable() -> GearUITouchAreaVTable for TouchAreaItem
}

/// Layer 2: TouchArea View Control Wrapper
///
/// 用户友好的公共 API，采用建造者模式。
#[derive(Clone)]
pub struct TouchArea {
    // Store properties directly for simple builder pattern
    enabled: bool,
    mouse_cursor: MouseCursor,
}

impl TouchArea {
    /// Create a new TouchArea with default properties
    pub fn new() -> Self {
        Self { enabled: true, mouse_cursor: MouseCursor::Default }
    }

    /// Builder pattern: Set whether the touch area is enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builder pattern: Set the mouse cursor to display
    pub fn with_mouse_cursor(mut self, cursor: MouseCursor) -> Self {
        self.mouse_cursor = cursor;
        self
    }

    /// Get current enabled state
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get current mouse cursor
    pub fn mouse_cursor(&self) -> MouseCursor {
        self.mouse_cursor
    }

    /// Create a TouchAreaItem with this TouchArea's properties
    pub fn create_item(&self) -> TouchAreaItem {
        let item = TouchAreaItem::default();

        // ✅ 标准 Slint 模式：设置属性值
        item.enabled.set(self.enabled);
        item.mouse_cursor.set(self.mouse_cursor);

        item
    }

    /// Create a shared Rc<TouchAreaItem> with this TouchArea's properties
    pub fn create_item_rc(&self) -> Rc<TouchAreaItem> {
        Rc::new(self.create_item())
    }
}

impl Default for TouchArea {
    fn default() -> Self {
        Self::new()
    }
}

/// Layer 3: View Implementation - 统一架构
impl View for TouchArea {
    type ItemType = TouchAreaItem;

    fn create_item(self) -> Self::ItemType {
        // ✅ 标准 Slint 模式：创建 Item 并设置属性
        let item = TouchAreaItem::default();

        // Set position and size properties to defaults
        item.width.set(LogicalLength::new(0.0));
        item.height.set(LogicalLength::new(0.0));
        item.x.set(LogicalLength::new(0.0));
        item.y.set(LogicalLength::new(0.0));

        // Set TouchArea-specific properties
        item.enabled.set(self.enabled);
        item.pressed.set(false);
        item.has_hover.set(false);
        item.mouse_x.set(LogicalLength::new(0.0));
        item.mouse_y.set(LogicalLength::new(0.0));
        item.pressed_x.set(LogicalLength::new(0.0));
        item.pressed_y.set(LogicalLength::new(0.0));
        item.mouse_cursor.set(self.mouse_cursor);

        item
    }

    fn transfer_properties_to_window(&self, _window_item: &i_slint_core::items::WindowItem) {
        // TouchArea doesn't transfer properties to window
    }

    fn into_item_tree(self) -> Option<i_slint_core::item_tree::ItemTreeRc> {
        // 🎯 **关键修复**：使用 CompositeItemTree 而不是 SingleItemTree
        // Slint 要求 ItemTree 的第0个 Item 必须是 WindowItem

        println!("🎯 TouchArea::into_item_tree: Creating CompositeItemTree with WindowItem root");

        // 创建 WindowItem 作为根节点
        let window_item = i_slint_core::items::WindowItem::default();

        // 🎯 **属性转移**：设置窗口背景为透明，表示 TouchArea 控件存在但不可见
        let bg_color = i_slint_core::graphics::Color::from_argb_u8(0, 0, 0, 0); // 完全透明
        window_item.background.set(i_slint_core::graphics::Brush::SolidColor(bg_color));

        println!("🎯 TouchArea transparent background transferred to WindowItem");

        // 创建 TouchArea 控件作为子节点
        let touch_area_item = self.create_item();

        // 创建 CompositeItemTree：WindowItem (index 0) + TouchArea (index 1)
        let tree_rc =
            crate::item_tree_integration::create_composite_item_tree(window_item, touch_area_item);
        Some(tree_rc)
    }
}

pub mod helpers {
    use super::*;
    use i_slint_core::items::MouseCursor;

    pub fn hand_cursor_area() -> TouchArea {
        TouchArea::new().with_mouse_cursor(MouseCursor::Pointer)
    }

    pub fn pointer_cursor_area() -> TouchArea {
        TouchArea::new().with_mouse_cursor(MouseCursor::Pointer)
    }
}
