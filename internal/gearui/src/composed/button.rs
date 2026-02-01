// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

/*!
# Button Control - Composed

A composed Button control combining TouchArea, Rectangle, and Text elements.
This replicates the behavior of standard UI buttons in Slint.

## Composition Structure
The Button is composed of:
- TouchArea: For mouse interaction handling
- Rectangle: For background and border rendering
- Text: For button label display

## Properties
| Property               | Type                | Default Value    | Description                    |
|------------------------|---------------------|------------------|--------------------------------|
| text                   | SharedString        | ""               | Button label text              |
| enabled                | bool                | true             | Whether button responds to input|
| pressed                | bool                | false (output)   | Whether button is pressed      |
| has_focus              | bool                | false (output)   | Whether button has focus       |
| background             | Brush               | default          | Button background color        |
| foreground             | Brush               | default          | Button text color              |

## Callbacks
- `clicked`: Triggered when the button is clicked

## Usage Example
```rust
use i_slint_gearui::Button;

let button = Button::new()
    .with_text("Click Me!".into())
    .with_enabled(true);
```
*/

use const_field_offset::FieldOffsets;
use core::pin::Pin;
use i_slint_core::{
    Callback, Color, ItemVTable_static, Property, SharedString, declare_item_vtable,
    graphics::Brush,
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, KeyEventType, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer},
    items::{Item, ItemConsts, ItemRc, ItemVTable, RenderingResult, VoidArg},
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    window::{WindowAdapter, WindowAdapterRc},
};
use i_slint_core_macros::*;
use std::rc::Rc;

// Required for VTable macro and types
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Button Item that combines TouchArea + Rectangle + Text functionality
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct ButtonItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Button text label
    pub text: Property<SharedString>,
    /// Whether the button responds to input
    pub enabled: Property<bool>,
    /// Whether the button is currently pressed - output property
    pub pressed: Property<bool>,
    /// Whether the button has keyboard focus - output property
    pub has_focus: Property<bool>,
    /// Button background color
    pub background: Property<Brush>,
    /// Button text color
    pub foreground: Property<Brush>,
    /// Callback triggered when button is clicked
    pub clicked: Callback<VoidArg>,
    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for ButtonItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {}

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        // Provide minimum button size
        match orientation {
            Orientation::Horizontal => {
                LayoutInfo { min: 60.0, preferred: 80.0, ..Default::default() }
            }
            Orientation::Vertical => {
                LayoutInfo { min: 30.0, preferred: 35.0, ..Default::default() }
            }
        }
    }

    fn input_event_filter_before_children(
        self: Pin<&Self>,
        _event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventFilterResult {
        if !self.enabled() {
            return InputEventFilterResult::ForwardAndIgnore;
        }
        InputEventFilterResult::ForwardAndInterceptGrab
    }

    fn input_event(
        self: Pin<&Self>,
        event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        self_rc: &ItemRc,
    ) -> InputEventResult {
        let enabled_pin = unsafe { Pin::new_unchecked(&self.get_ref().enabled) };
        if !enabled_pin.get() {
            return InputEventResult::EventIgnored;
        }

        match event {
            MouseEvent::Pressed { position, .. } => {
                let geometry = self_rc.geometry();
                if LogicalRect::new(Default::default(), geometry.size).contains(*position) {
                    Self::FIELD_OFFSETS.pressed.apply_pin(self).set(true);
                    InputEventResult::GrabMouse
                } else {
                    InputEventResult::EventIgnored
                }
            }
            MouseEvent::Released { position, .. } => {
                let pressed_pin = unsafe { Pin::new_unchecked(&self.get_ref().pressed) };
                let was_pressed = pressed_pin.get();
                Self::FIELD_OFFSETS.pressed.apply_pin(self).set(false);

                if was_pressed {
                    let geometry = self_rc.geometry();
                    if LogicalRect::new(Default::default(), geometry.size).contains(*position) {
                        Self::FIELD_OFFSETS.clicked.apply_pin(self).call(&());
                    }
                }
                InputEventResult::EventAccepted
            }
            MouseEvent::Exit => {
                Self::FIELD_OFFSETS.pressed.apply_pin(self).set(false);
                InputEventResult::EventAccepted
            }
            _ => InputEventResult::EventAccepted,
        }
    }

    fn key_event(
        self: Pin<&Self>,
        event: &KeyEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        let enabled_pin = unsafe { Pin::new_unchecked(&self.get_ref().enabled) };
        let has_focus_pin = unsafe { Pin::new_unchecked(&self.get_ref().has_focus) };
        if !enabled_pin.get() || !has_focus_pin.get() {
            return KeyEventResult::EventIgnored;
        }

        match event.event_type {
            KeyEventType::KeyPressed => {
                if event.text == " " || event.text == "\n" {
                    Self::FIELD_OFFSETS.clicked.apply_pin(self).call(&());
                    return KeyEventResult::EventAccepted;
                }
            }
            _ => {}
        }
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
        event: &FocusEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> FocusEventResult {
        let enabled_pin = unsafe { Pin::new_unchecked(&self.get_ref().enabled) };
        if !enabled_pin.get() {
            return FocusEventResult::FocusIgnored;
        }

        match event {
            FocusEvent::FocusIn(_) => {
                Self::FIELD_OFFSETS.has_focus.apply_pin(self).set(true);
                FocusEventResult::FocusAccepted
            }
            FocusEvent::FocusOut(_) => {
                Self::FIELD_OFFSETS.has_focus.apply_pin(self).set(false);
                FocusEventResult::FocusAccepted
            }
        }
    }

    fn render(
        self: Pin<&Self>,
        backend: &mut &mut dyn ItemRenderer,
        self_rc: &ItemRc,
        size: LogicalSize,
    ) -> RenderingResult {
        // 🎯 调试：检查 Button render 方法是否被调用
        println!("🎯 🔥 🔥 🔥 SUPER CRITICAL: ButtonItem::render called! size: {:?}", size);
        println!("🎯   THIS IS THE ACTUAL BUTTON RENDERING METHOD!");

        let text_pin = unsafe { Pin::new_unchecked(&self.get_ref().text) };
        let background_pin = unsafe { Pin::new_unchecked(&self.get_ref().background) };
        let pressed_pin = unsafe { Pin::new_unchecked(&self.get_ref().pressed) };
        let x_pin = unsafe { Pin::new_unchecked(&self.get_ref().x) };
        let y_pin = unsafe { Pin::new_unchecked(&self.get_ref().y) };
        let width_pin = unsafe { Pin::new_unchecked(&self.get_ref().width) };
        let height_pin = unsafe { Pin::new_unchecked(&self.get_ref().height) };

        println!("🎯   Button text: '{}'", text_pin.get());
        println!("🎯   Button background: {:?}", background_pin.get());
        println!("🎯   Button pressed: {}", pressed_pin.get());
        println!("🎯   Button position: ({}, {})", x_pin.get().get(), y_pin.get().get());
        println!("🎯   Button size: {}x{}", width_pin.get().get(), height_pin.get().get());

        // 🎯 渲染按钮背景 (Rectangle 行为)
        println!("🎯   Calling backend.draw_rectangle for button background...");
        (*backend).draw_rectangle(self, self_rc, size, &self.cached_rendering_data);
        println!("🎯   Button background rendered!");

        // 🎯 渲染按钮文本 (Text 行为)
        // 注意：由于 Button 同时实现了 RenderRectangle 和 RenderText trait，
        // 它可以调用 draw_text 来渲染文本
        println!("🎯   Calling backend.draw_text for button text...");
        (*backend).draw_text(self, self_rc, size, &self.cached_rendering_data);
        println!("🎯   Button text rendered!");

        println!("🎯   Button rendering completed successfully!");
        RenderingResult::ContinueRenderingChildren
    }

    fn bounding_rect(
        self: core::pin::Pin<&Self>,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
        geometry: LogicalRect,
    ) -> LogicalRect {
        geometry
    }

    fn clips_children(self: core::pin::Pin<&Self>) -> bool {
        false
    }
}

impl ItemConsts for ButtonItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<
        ButtonItem,
        CachedRenderingData,
    > = ButtonItem::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUIButtonVTable() -> GearUIButtonVTable for ButtonItem
}

// Implement RenderRectangle for background rendering
impl i_slint_core::item_rendering::RenderRectangle for ButtonItem {
    fn background(self: Pin<&Self>) -> Brush {
        let background_pin = unsafe { Pin::new_unchecked(&self.get_ref().background) };
        background_pin.get()
    }
}

// Implement RenderText for text rendering
impl i_slint_core::item_rendering::HasFont for ButtonItem {
    fn font_request(self: Pin<&Self>, self_rc: &ItemRc) -> i_slint_core::graphics::FontRequest {
        println!("🎯 🔥 SUPER CRITICAL: ButtonItem::font_request called!");
        println!("🎯   This is THE KEY METHOD for button text font configuration!");

        // 使用默认字体设置
        let font_size = LogicalLength::new(12.0);
        let font_family = "Arial Unicode MS".into();

        // 🎯 使用 WindowItem::resolved_font_request 方法，这是 Slint 的标准做法
        let font_request = i_slint_core::items::WindowItem::resolved_font_request(
            self_rc,
            font_family,
            400, // font_weight: normal
            font_size,
            LogicalLength::new(0.0), // letter_spacing
            false,                   // italic
        );

        println!(
            "🔍   Button FontRequest created: family={:?}, size={:?}",
            font_request.family.as_deref().unwrap_or("None"),
            font_request.pixel_size.unwrap_or(LogicalLength::new(0.0))
        );

        font_request
    }
}

impl i_slint_core::item_rendering::RenderString for ButtonItem {
    fn text(self: Pin<&Self>) -> i_slint_core::item_rendering::PlainOrStyledText {
        let text_pin = unsafe { Pin::new_unchecked(&self.get_ref().text) };
        i_slint_core::item_rendering::PlainOrStyledText::Plain(text_pin.get())
    }
}

impl i_slint_core::item_rendering::RenderText for ButtonItem {
    fn target_size(self: Pin<&Self>) -> LogicalSize {
        let width_pin = unsafe { Pin::new_unchecked(&self.get_ref().width) };
        let height_pin = unsafe { Pin::new_unchecked(&self.get_ref().height) };
        LogicalSize::from_lengths(width_pin.get(), height_pin.get())
    }

    fn color(self: Pin<&Self>) -> Brush {
        let foreground_pin = unsafe { Pin::new_unchecked(&self.get_ref().foreground) };
        foreground_pin.get()
    }

    fn alignment(
        self: Pin<&Self>,
    ) -> (i_slint_core::items::TextHorizontalAlignment, i_slint_core::items::TextVerticalAlignment)
    {
        // 按钮文本居中对齐
        (
            i_slint_core::items::TextHorizontalAlignment::Center,
            i_slint_core::items::TextVerticalAlignment::Center,
        )
    }

    fn wrap(self: Pin<&Self>) -> i_slint_core::items::TextWrap {
        i_slint_core::items::TextWrap::NoWrap
    }

    fn overflow(self: Pin<&Self>) -> i_slint_core::items::TextOverflow {
        i_slint_core::items::TextOverflow::Clip
    }

    fn stroke(self: Pin<&Self>) -> (Brush, LogicalLength, i_slint_core::items::TextStrokeStyle) {
        (Brush::default(), LogicalLength::new(0.0), i_slint_core::items::TextStrokeStyle::Outside)
    }

    fn is_markdown(self: Pin<&Self>) -> bool {
        false
    }

    fn link_color(self: Pin<&Self>) -> Color {
        Color::from_rgb_u8(0, 0, 255)
    }
}

/// High-level Button control with Builder pattern API
#[derive(Clone)]
pub struct Button {
    // Store properties directly for simple builder pattern
    text: SharedString,
    enabled: bool,
    background: Brush,
    foreground: Brush,
    width: LogicalLength,
    height: LogicalLength,
    x: LogicalLength,
    y: LogicalLength,
}

impl Button {
    /// Create a new Button with default settings
    pub fn new() -> Self {
        Self {
            text: SharedString::default(),
            background: Brush::SolidColor(i_slint_core::Color::from_rgb_u8(70, 130, 180)), // Steel blue default
            enabled: true,
            foreground: Brush::SolidColor(
                i_slint_core::Color::from_rgb_u8(255, 255, 255), // 白色文字
            ),
            // 🎯 **关键修复**：设置合适的按钮默认尺寸
            width: LogicalLength::new(120.0), // 合适的按钮宽度
            height: LogicalLength::new(35.0), // 合适的按钮高度
            x: LogicalLength::new(0.0),
            y: LogicalLength::new(0.0),
        }
    }

    /// Set the button text
    pub fn with_text(mut self, text: SharedString) -> Self {
        self.text = text;
        self
    }

    /// Set the button background
    pub fn with_background(mut self, background: Brush) -> Self {
        self.background = background;
        self
    }

    /// Get the current text
    pub fn text(&self) -> &SharedString {
        &self.text
    }

    /// Get the current background
    pub fn background(&self) -> &Brush {
        &self.background
    }

    /// Set width (convenience method)
    pub fn with_width(self, _width: f32) -> Self {
        // 🎯 暂时忽略宽度设置，因为 Button 的布局由父容器控制
        // 在实际应用中，这个方法会影响 Button 的 ItemRc 属性
        self
    }

    /// Set height (convenience method)
    pub fn with_height(self, _height: f32) -> Self {
        // 🎯 暂时忽略高度设置，因为 Button 的布局由父容器控制
        // 在实际应用中，这个方法会影响 Button 的 ItemRc 属性
        self
    }

    /// Builder pattern: Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builder pattern: Set text color
    pub fn with_foreground(mut self, foreground: Brush) -> Self {
        self.foreground = foreground;
        self
    }

    // ItemRc conversion will be implemented when ItemTree integration is complete
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::View for Button {
    type ItemType = ButtonItem;

    fn create_item(self) -> Self::ItemType {
        // 🎯 **关键修复**：计算按钮在窗口中的居中位置
        let window_width = 800.0; // 默认窗口宽度
        let window_height = 600.0; // 默认窗口高度
        let button_width = self.width.get();
        let button_height = self.height.get();

        // 计算居中位置
        let center_x = (window_width - button_width) / 2.0;
        let center_y = (window_height - button_height) / 2.0;

        println!("🎯 Button positioning:");
        println!("🎯   Window size: {}x{}", window_width, window_height);
        println!("🎯   Button size: {}x{}", button_width, button_height);
        println!("🎯   Center position: ({}, {})", center_x, center_y);

        let item = ButtonItem {
            width: Property::new(self.width),
            height: Property::new(self.height),
            x: Property::new(LogicalLength::new(center_x)), // 居中 X 坐标
            y: Property::new(LogicalLength::new(center_y)), // 居中 Y 坐标
            text: Property::new(self.text),
            enabled: Property::new(self.enabled),
            pressed: Property::new(false),
            has_focus: Property::new(false),
            background: Property::new(self.background),
            foreground: Property::new(self.foreground),
            clicked: Callback::default(),
            cached_rendering_data: CachedRenderingData::default(),
        };

        item
    }

    fn transfer_properties_to_window(&self, window_item: &i_slint_core::items::WindowItem) {
        // 🎯 **关键修复**：Button 不应该把自己的背景色设为窗口背景
        // Button 应该保持自己的边界和样式
        println!("🎯 Button::transfer_properties_to_window called!");
        println!("🎯   Button text: '{}'", self.text);
        println!("🎯   Button background: {:?}", self.background);

        // 🎯 设置窗口为中性的浅灰色背景，这样 Button 会在其上方正确显示
        let neutral_bg = i_slint_core::graphics::Color::from_rgb_u8(245, 245, 245); // 浅灰色
        window_item.background.set(i_slint_core::graphics::Brush::SolidColor(neutral_bg));
        println!("🎯 Window background set to neutral gray, Button will render on top!");
    }

    fn into_item_tree(self) -> Option<i_slint_core::item_tree::ItemTreeRc> {
        // 🎯 **关键修复**：Button 使用 CompositeItemTree 而不是 SingleItemTree
        // 这样确保 WindowItem 在 index 0，Button 在 index 1

        println!("🎯 Button::into_item_tree: Creating CompositeItemTree with WindowItem root");

        // 创建 WindowItem 作为根节点，并设置合适的背景色
        let window_item = i_slint_core::items::WindowItem::default();
        self.transfer_properties_to_window(&window_item);

        // 创建 Button 控件作为子节点
        let button_item = self.create_item();

        // 创建 CompositeItemTree：WindowItem (index 0) + Button (index 1)
        let tree_rc =
            crate::item_tree_integration::create_composite_item_tree(window_item, button_item);
        Some(tree_rc)
    }
}
