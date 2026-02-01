// GearUI LineEdit - 单行文本输入
//
// 对应 Slint 的 TextInput 控件，提供单行文本输入功能
//
// 字段映射表：
// | GearUI 字段 | Slint 字段 | 类型 | 默认值 | 说明 |
// |-------------|------------|------|--------|------|
// | text        | text       | SharedString | "" | 输入文本内容 |
// | placeholder | placeholder-text | SharedString | "" | 占位符文本 |
// | font_size   | font-size  | LogicalLength | 12px | 字体大小 |
// | color       | color      | Brush | black | 文本颜色 |
// | enabled     | enabled    | bool | true | 是否启用 |
// | has_focus   | has-focus  | bool | false | 是否有焦点 |
//
// 用法示例：
// ```rust
// let input = LineEdit::new()
//     .with_text("Hello")
//     .with_placeholder("请输入...")
//     .with_font_size(LogicalLength::new(14.0));
// ```

use std::pin::Pin;

use const_field_offset::FieldOffsets;
use i_slint_core::{
    Callback, ItemVTable_static, Property, SharedString, declare_item_vtable,
    graphics::{Brush, Color},
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer},
    items::{Item, ItemConsts, ItemRc, ItemVTable, RenderingResult},
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    window::WindowAdapterRc,
};
use i_slint_core_macros::*;

// Required for VTable macro
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Layer 1: Core LineEdit Item Implementation
///
/// 实现 Item trait，提供实际的文本输入逻辑。
/// 字段名和默认值完全匹配 Slint 的 TextInput 规范。
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct LineEditItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Text content (corresponds to Slint: text)
    /// Default: ""
    pub text: Property<SharedString>,

    /// Placeholder text (corresponds to Slint: placeholder-text)
    /// Default: ""
    pub placeholder: Property<SharedString>,

    /// Font size (corresponds to Slint: font-size)
    /// Default: 12px
    pub font_size: Property<LogicalLength>,

    /// Text color (corresponds to Slint: color)
    /// Default: Black
    pub color: Property<Brush>,

    /// Enabled state (corresponds to Slint: enabled)
    /// Default: true
    pub enabled: Property<bool>,

    /// Focus state (corresponds to Slint: has-focus)
    /// Default: false
    pub has_focus: Property<bool>,

    /// Background brush
    pub background: Property<Brush>,

    /// Border color
    pub border_color: Property<Brush>,

    /// Border width
    pub border_width: Property<LogicalLength>,

    /// Text changed callback
    pub edited: Callback<(SharedString,)>,

    /// Accepted callback (Enter key pressed)
    pub accepted: Callback<()>,

    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for LineEditItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {
        // 初始化在 Default trait 和 create_item 中已经完成
        // 这里不需要额外操作
    }

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        // ✅ 标准 Slint 模式：基于方向计算布局信息
        match orientation {
            Orientation::Horizontal => {
                // 水平方向：最小宽度 100px，首选宽度 200px
                LayoutInfo { min: 100.0, preferred: 200.0, ..Default::default() }
            }
            Orientation::Vertical => {
                // 垂直方向：高度基于字体大小
                let font_size_pin = unsafe { Pin::new_unchecked(&self.get_ref().font_size) };
                let font_size = font_size_pin.get().get();
                let height = font_size + 16.0; // 字体大小 + 上下边距

                LayoutInfo { min: height, preferred: height, ..Default::default() }
            }
        }
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
        event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventResult {
        // 处理鼠标点击获取焦点
        if let MouseEvent::Pressed { .. } = event {
            let has_focus_pin = unsafe { Pin::new_unchecked(&self.get_ref().has_focus) };
            has_focus_pin.set(true);
            return InputEventResult::EventAccepted;
        }
        InputEventResult::EventIgnored
    }

    fn key_event(
        self: Pin<&Self>,
        event: &KeyEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        use i_slint_core::input::KeyEventType;

        let enabled_pin = unsafe { Pin::new_unchecked(&self.get_ref().enabled) };
        if !enabled_pin.get() {
            return KeyEventResult::EventIgnored;
        }

        let has_focus_pin = unsafe { Pin::new_unchecked(&self.get_ref().has_focus) };
        if !has_focus_pin.get() {
            return KeyEventResult::EventIgnored;
        }

        match event.event_type {
            KeyEventType::KeyPressed => {
                let text_pin = unsafe { Pin::new_unchecked(&self.get_ref().text) };
                let mut text = text_pin.get().to_string();

                // Handle Enter key
                if event.text == "\n" || event.text == "\r" {
                    let accepted_pin = unsafe { Pin::new_unchecked(&self.get_ref().accepted) };
                    accepted_pin.call(&());
                    return KeyEventResult::EventAccepted;
                }

                // Handle Backspace
                if event.text == "\u{0008}" {
                    if !text.is_empty() {
                        text.pop();
                        text_pin.set(SharedString::from(text.as_str()));

                        let edited_pin = unsafe { Pin::new_unchecked(&self.get_ref().edited) };
                        edited_pin.call(&(text_pin.get(),));
                    }
                    return KeyEventResult::EventAccepted;
                }

                // Handle regular text input
                if !event.text.is_empty() && event.text.chars().all(|c| c >= ' ') {
                    text.push_str(&event.text);
                    text_pin.set(SharedString::from(text.as_str()));

                    let edited_pin = unsafe { Pin::new_unchecked(&self.get_ref().edited) };
                    edited_pin.call(&(text_pin.get(),));
                    return KeyEventResult::EventAccepted;
                }

                KeyEventResult::EventIgnored
            }
            _ => KeyEventResult::EventIgnored,
        }
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
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> FocusEventResult {
        let has_focus_pin = unsafe { Pin::new_unchecked(&self.get_ref().has_focus) };

        match event {
            FocusEvent::FocusIn(_) => {
                has_focus_pin.set(true);
                FocusEventResult::FocusAccepted
            }
            FocusEvent::FocusOut(_) => {
                has_focus_pin.set(false);
                FocusEventResult::FocusAccepted
            }
            _ => FocusEventResult::FocusIgnored,
        }
    }

    fn render(
        self: Pin<&Self>,
        _backend: &mut &mut dyn ItemRenderer,
        _self_rc: &ItemRc,
        size: LogicalSize,
    ) -> RenderingResult {
        println!("🎯 LineEditItem::render called - size: {}x{}", size.width, size.height);

        // TODO: 实现完整的渲染逻辑
        // LineEdit 需要渲染：
        // 1. 背景矩形
        // 2. 边框（焦点时高亮）
        // 3. 文本或占位符
        // 4. 光标（如果有焦点）
        //
        // 目前先使用基本的渲染，完整实现需要：
        // - 实现 RenderRectangle trait for LineEditItem
        // - 使用组合方式渲染子元素（Rectangle + Text）

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

impl ItemConsts for LineEditItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUILineEditVTable() -> GearUILineEditVTable for LineEditItem
}

/// Layer 2: LineEdit View Control Wrapper
///
/// 用户友好的 LineEdit 控件 API，提供构建器模式。
#[derive(Clone)]
pub struct LineEdit {
    // Store properties for builder pattern
    text: SharedString,
    placeholder: SharedString,
    font_size: LogicalLength,
    color: Brush,
    enabled: bool,
    background: Brush,
    border_color: Brush,
    border_width: LogicalLength,
    width: LogicalLength,
    height: LogicalLength,
    x: LogicalLength,
    y: LogicalLength,
}

impl LineEdit {
    /// Create a new LineEdit instance with default values
    pub fn new() -> Self {
        Self {
            text: SharedString::from(""),
            placeholder: SharedString::from(""),
            font_size: LogicalLength::new(12.0),
            color: Brush::SolidColor(Color::from_rgb_u8(0, 0, 0)),
            enabled: true,
            background: Brush::SolidColor(Color::from_rgb_u8(255, 255, 255)),
            border_color: Brush::SolidColor(Color::from_rgb_u8(200, 200, 200)),
            border_width: LogicalLength::new(1.0),
            width: LogicalLength::new(0.0),
            height: LogicalLength::new(0.0),
            x: LogicalLength::new(0.0),
            y: LogicalLength::new(0.0),
        }
    }

    /// Builder pattern: Set text content
    pub fn with_text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    /// Builder pattern: Set placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Builder pattern: Set font size
    pub fn with_font_size(mut self, font_size: LogicalLength) -> Self {
        self.font_size = font_size;
        self
    }

    /// Builder pattern: Set text color
    pub fn with_color(mut self, color: Brush) -> Self {
        self.color = color;
        self
    }

    /// Builder pattern: Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builder pattern: Set background
    pub fn with_background(mut self, background: Brush) -> Self {
        self.background = background;
        self
    }

    /// Builder pattern: Set border color
    pub fn with_border_color(mut self, border_color: Brush) -> Self {
        self.border_color = border_color;
        self
    }

    /// Builder pattern: Set border width
    pub fn with_border_width(mut self, width: LogicalLength) -> Self {
        self.border_width = width;
        self
    }

    /// Builder pattern: Set width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = LogicalLength::new(width);
        self
    }

    /// Builder pattern: Set height
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = LogicalLength::new(height);
        self
    }

    /// Get current text
    pub fn text(&self) -> &SharedString {
        &self.text
    }

    /// Get current placeholder
    pub fn placeholder(&self) -> &SharedString {
        &self.placeholder
    }
}

impl Default for LineEdit {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::View for LineEdit {
    type ItemType = LineEditItem;

    fn create_item(self) -> Self::ItemType {
        println!(
            "🎯 LineEdit::create_item: text = '{}', placeholder = '{}'",
            self.text, self.placeholder
        );

        let item = LineEditItem::default();

        // ✅ 设置属性
        unsafe {
            Pin::new_unchecked(&item.text).set(self.text);
            Pin::new_unchecked(&item.placeholder).set(self.placeholder);
            Pin::new_unchecked(&item.font_size).set(self.font_size);
            Pin::new_unchecked(&item.color).set(self.color);
            Pin::new_unchecked(&item.enabled).set(self.enabled);
            Pin::new_unchecked(&item.background).set(self.background);
            Pin::new_unchecked(&item.border_color).set(self.border_color);
            Pin::new_unchecked(&item.border_width).set(self.border_width);
            Pin::new_unchecked(&item.width).set(self.width);
            Pin::new_unchecked(&item.height).set(self.height);
            Pin::new_unchecked(&item.x).set(self.x);
            Pin::new_unchecked(&item.y).set(self.y);
        }

        item
    }

    fn build(self) -> Self {
        self
    }

    fn transfer_properties_to_window(&self, _window_item: &i_slint_core::items::WindowItem) {
        // LineEdit 不需要转移属性到窗口
    }

    fn into_item_tree(self) -> Option<i_slint_core::item_tree::ItemTreeRc> {
        let item = Box::new(self.create_item());
        Some(crate::item_tree_integration::create_single_item_tree_from_boxed(item))
    }
}

/// Helper functions for quick LineEdit creation
pub mod helpers {
    use super::*;

    /// Create a LineEdit with placeholder
    pub fn line_edit_with_placeholder(placeholder: &str) -> LineEdit {
        LineEdit::new().with_placeholder(SharedString::from(placeholder))
    }

    /// Create a LineEdit with initial text
    pub fn line_edit_with_text(text: &str) -> LineEdit {
        LineEdit::new().with_text(SharedString::from(text))
    }
}
