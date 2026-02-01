// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

/*!
# Text Control - 正确的两层架构实现

## 架构说明
- **Layer 1**: TextItem (impl Item) - 与 Slint 系统直接交互
- **Layer 2**: Text (impl View) - 用户友好的构建器模式接口

## Slint 属性映射
完全按照 Slint Text 元素的属性映射：
- text: 文本内容
- color: 文本颜色
- font-size: 字体大小

## 使用示例
```rust
Text::new()
    .with_text("Hello World!".into())
    .with_color(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
    .run();
```
*/

use crate::View;
use const_field_offset::FieldOffsets;
use core::pin::Pin;
use i_slint_core::{
    ItemVTable_static, SharedString, declare_item_vtable,
    graphics::{Brush, Color, FontRequest},
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer, RenderText},
    items::{
        Item, ItemConsts, ItemRc, ItemRef, ItemVTable, RenderingResult, TextHorizontalAlignment,
        TextOverflow, TextStrokeStyle, TextVerticalAlignment, TextWrap, WindowItem,
    },
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    properties::Property,
    window::WindowAdapterRc,
};
use i_slint_core_macros::*;
use std::rc::Rc;
use vtable::HasStaticVTable;

// Required for VTable macro
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Layer 1: Core Text Item Implementation
///
/// 实现 Item trait，提供实际的文本渲染和布局逻辑。
/// 字段名和默认值完全匹配 Slint 的 Text 规范。
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct TextItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Text content (corresponds to Slint: text)
    /// Default: ""
    pub text: Property<SharedString>,
    /// Text color (corresponds to Slint: color)
    /// Default: #000 (black)
    pub color: Property<Brush>,
    /// Font size (corresponds to Slint: font-size)
    /// Default: 12px
    pub font_size: Property<LogicalLength>,
    /// Font family (corresponds to Slint: font-family)
    /// Default: None (uses system default)
    pub font_family: Property<Option<SharedString>>,
    /// 🎯 **关键修复：缓存渲染数据** - Slint 文本渲染系统必需
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for TextItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {
        println!("🎯 🔥 🔥 🔥 SUPER CRITICAL: TextItem::init called!");
        println!("🎯   This proves TextItem is being initialized by Slint!");
    }

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        println!("🎯 🔥 🔥 🔥 SUPER CRITICAL: TextItem::layout_info called!");
        println!("🎯   This is THE KEY METHOD for text rendering!");
        println!("🎯   Orientation: {:?}", orientation);

        // ✅ 标准 Slint 模式：基于文本内容计算布局
        match orientation {
            Orientation::Horizontal => {
                // Simple heuristic for text width
                let text_pin = unsafe { Pin::new_unchecked(&self.get_ref().text) };
                let text = text_pin.get();
                let char_count = text.chars().count() as f32;
                let estimated_width = char_count * 8.0; // Simple approximation

                // 🎯 关键修复：确保最小宽度不为0
                let min_width = estimated_width.max(50.0);
                let preferred_width = estimated_width.max(100.0);

                println!("🎯   Text content: '{}'", text);
                println!("🎯   Character count: {}", char_count);
                println!("🎯   Estimated width: {}", estimated_width);
                println!("🎯   Final width: min={}, preferred={}", min_width, preferred_width);

                LayoutInfo {
                    min: min_width,
                    preferred: preferred_width,
                    max: f32::INFINITY,
                    stretch: 1.0,
                    max_percent: 100.0,
                    min_percent: 0.0,
                }
            }
            Orientation::Vertical => {
                let font_size_pin = unsafe { Pin::new_unchecked(&self.get_ref().font_size) };
                let font_size = font_size_pin.get().get();

                // 🎯 关键修复：基于字体大小计算高度
                let min_height = font_size.max(20.0);
                let preferred_height = font_size.max(30.0);

                println!("🎯   Font size: {}", font_size);
                println!("🎯   Final height: min={}, preferred={}", min_height, preferred_height);

                LayoutInfo {
                    min: min_height,
                    preferred: preferred_height,
                    max: f32::INFINITY,
                    stretch: 1.0,
                    max_percent: 100.0,
                    min_percent: 0.0,
                }
            }
        }
    }

    fn input_event_filter_before_children(
        self: Pin<&Self>,
        _event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventFilterResult {
        println!("🎯 TextItem::input_event_filter_before_children called!");
        InputEventFilterResult::ForwardAndIgnore
    }

    fn input_event(
        self: Pin<&Self>,
        _event: &MouseEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> InputEventResult {
        println!("🎯 TextItem::input_event called!");
        InputEventResult::EventIgnored
    }

    fn key_event(
        self: Pin<&Self>,
        _event: &KeyEvent,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        println!("🎯 TextItem::key_event called!");
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
        println!("🎯 TextItem::focus_event called!");
        FocusEventResult::FocusIgnored
    }

    fn render(
        self: Pin<&Self>,
        backend: &mut &mut dyn ItemRenderer,
        self_rc: &ItemRc,
        size: LogicalSize,
    ) -> RenderingResult {
        // 🎯 调试：检查 render 方法是否被调用
        println!("🎯 🔥 🔥 🔥 SUPER CRITICAL: TextItem::render called! size: {:?}", size);
        println!("🎯   THIS IS THE ACTUAL RENDERING METHOD!");
        let text_pin = unsafe { Pin::new_unchecked(&self.get_ref().text) };
        println!("🎯   Text content: '{}'", text_pin.get());

        // 🎯 关键修复：调用 draw_text 来渲染文本
        println!("🎯   Calling backend.draw_text...");
        (*backend).draw_text(self, self_rc, size, &self.cached_rendering_data);
        println!("🎯   backend.draw_text completed!");
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
        // 🎯 **关键修复**：Text 控件不需要剪裁子控件
        // Text 通常是叶子节点，没有子控件，因此不需要剪裁
        // 返回 false 避免无限循环调用
        false
    }
}

impl ItemConsts for TextItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<
        TextItem,
        CachedRenderingData,
    > = TextItem::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

impl i_slint_core::item_rendering::HasFont for TextItem {
    fn font_request(self: Pin<&Self>, self_rc: &ItemRc) -> FontRequest {
        println!("🎯 🔥 🔥 🔥 SUPER CRITICAL: TextItem::font_request called!");
        println!("🎯   This is THE KEY METHOD for font configuration!");

        let font_family_pin = unsafe { Pin::new_unchecked(&self.get_ref().font_family) };
        let font_size_pin = unsafe { Pin::new_unchecked(&self.get_ref().font_size) };

        // 🎯 关键修复：根据操作系统选择最佳中文字体
        let default_font_family = if cfg!(target_os = "macos") {
            "PingFang SC" // macOS 系统默认中文字体
        } else if cfg!(target_os = "windows") {
            "Microsoft YaHei" // Windows 系统默认中文字体
        } else {
            "Noto Sans CJK SC" // Linux 系统推荐中文字体
        };

        // 🎯 调试：检查字体设置状态
        let user_font_family = font_family_pin.get();
        println!("🔍   User set font_family: {:?}", user_font_family);
        println!("🔍   Target OS: {}", std::env::consts::OS);
        println!("🔍   Selected font_family: {}", default_font_family);

        // 🎯 优先使用用户指定的字体，否则使用系统最佳中文字体
        let font_family = user_font_family.unwrap_or_else(|| default_font_family.into());

        println!("🔍   Final font_family: {}", font_family);

        // 🎯 使用 WindowItem::resolved_font_request 方法，这是 Slint 的标准做法
        let font_request = WindowItem::resolved_font_request(
            self_rc,
            font_family,
            400, // font_weight: normal
            font_size_pin.get(),
            LogicalLength::new(0.0), // letter_spacing
            false,                   // italic
        );

        println!(
            "🔍   FontRequest created: family={:?}, size={:?}",
            font_request.family.as_deref().unwrap_or("None"),
            font_request.pixel_size.unwrap_or(LogicalLength::new(0.0))
        );

        // 🎯 关键调试：检查字体是否能被 fontdb 查询到
        #[cfg(feature = "software-renderer-systemfonts")]
        {
            use i_slint_core::sharedfontdb;
            let font_found = sharedfontdb::FONT_DB
                .with_borrow(|db| db.query(&font_request.to_fontdb_query()).is_some());
            println!("🔍   ⚠️ 字体查找结果: {}", font_found);

            // 🎯 打印所有可用字体（仅在调试时）
            println!("🧵 系统可用字体:");
            sharedfontdb::FONT_DB.with_borrow(|db| {
                let mut count = 0;
                for face in db.faces() {
                    for family in &face.families {
                        if count < 10 {
                            // 只显示前10个字体，避免输出过多
                            println!(" - {}", family.0);
                            count += 1;
                        }
                    }
                }
                if count >= 10 {
                    println!(" ... and {} more fonts", db.faces().len() - 10);
                }
            });
        }

        font_request
    }
}

impl i_slint_core::item_rendering::RenderString for TextItem {
    fn text(self: Pin<&Self>) -> i_slint_core::item_rendering::PlainOrStyledText {
        let text_pin = unsafe { Pin::new_unchecked(&self.get_ref().text) };
        i_slint_core::item_rendering::PlainOrStyledText::Plain(text_pin.get())
    }
}

impl RenderText for TextItem {
    fn target_size(self: Pin<&Self>) -> LogicalSize {
        let width_pin = unsafe { Pin::new_unchecked(&self.get_ref().width) };
        let height_pin = unsafe { Pin::new_unchecked(&self.get_ref().height) };
        LogicalSize::from_lengths(width_pin.get(), height_pin.get())
    }

    fn color(self: Pin<&Self>) -> Brush {
        let color_pin = unsafe { Pin::new_unchecked(&self.get_ref().color) };
        color_pin.get()
    }

    fn alignment(self: Pin<&Self>) -> (TextHorizontalAlignment, TextVerticalAlignment) {
        (TextHorizontalAlignment::Left, TextVerticalAlignment::Top)
    }

    fn wrap(self: Pin<&Self>) -> TextWrap {
        TextWrap::NoWrap
    }

    fn overflow(self: Pin<&Self>) -> TextOverflow {
        TextOverflow::Clip
    }

    fn stroke(self: Pin<&Self>) -> (Brush, LogicalLength, TextStrokeStyle) {
        (Brush::default(), LogicalLength::new(0.0), TextStrokeStyle::Outside)
    }

    fn is_markdown(self: Pin<&Self>) -> bool {
        false
    }

    fn link_color(self: Pin<&Self>) -> Color {
        Color::from_rgb_u8(0, 0, 255)
    }
}

declare_item_vtable! {
    fn slint_get_GearUITextVTable() -> GearUITextVTable for TextItem
}

/// Layer 2: Text View Control Wrapper
///
/// 用户友好的 Text 控件 API，提供构建器模式。
#[derive(Clone)]
pub struct Text {
    // Store properties directly for simple builder pattern
    text: SharedString,
    color: Brush,
    font_size: LogicalLength,
    font_family: Option<SharedString>,   // 🎯 新增：字体族设置
    font_family_list: Vec<SharedString>, // 🎯 新增：字体回退列表
}

impl Text {
    /// Create a new Text instance with Slint-compliant default values
    pub fn new() -> Self {
        Self {
            text: "".into(), // Default: empty string
            color: Brush::SolidColor(i_slint_core::Color::from_rgb_u8(0, 0, 0)), // Default: black
            font_size: LogicalLength::new(12.0), // Default: 12px
            font_family: None,
            font_family_list: Vec::new(), // 🎯 新增：空的回退列表
        }
    }

    /// 🧰 工具方法：列出所有可用的系统字体
    ///
    /// 用于调试和智能建议。
    pub fn list_available_fonts() -> Vec<String> {
        let mut fonts = Vec::new();

        #[cfg(feature = "software-renderer-systemfonts")]
        {
            use i_slint_core::sharedfontdb;

            println!("🔍 开始扫描系统字体...");

            // 尝试访问字体数据库
            let db = sharedfontdb::FONT_DB.lock().unwrap();
            if db.is_empty() {
                println!("⚠️  字体数据库为空，使用回退模式");
                fonts.extend_from_slice(&[
                    "Arial".to_string(),
                    "Helvetica".to_string(),
                    "Times New Roman".to_string(),
                    "Arial Unicode MS".to_string(),
                ]);
            } else {
                println!("✅ 字体数据库可用，发现 {} 个字体族", db.len());
                for face in db.faces() {
                    if let Some(family) = face.families.first() {
                        let family_name = family.0.clone();
                        if !fonts.contains(&family_name) {
                            fonts.push(family_name);
                        }
                    }
                }
            }
        }

        #[cfg(not(feature = "software-renderer-systemfonts"))]
        {
            println!("⚠️  systemfonts 功能未启用，使用默认字体列表");
            fonts.extend_from_slice(&[
                "Arial".to_string(),
                "Helvetica".to_string(),
                "Times New Roman".to_string(),
                "Arial Unicode MS".to_string(),
            ]);
        }

        fonts
    }

    /// 🧰 工具方法：检查字体是否受支持
    ///
    /// 用于调试和智能建议。
    pub fn font_supported(font_name: &str) -> bool {
        #[cfg(feature = "software-renderer-systemfonts")]
        {
            use i_slint_core::sharedfontdb;

            let db = sharedfontdb::FONT_DB.lock().unwrap();
            if db.is_empty() {
                // 回退模式：检查常见字体
                return matches!(
                    font_name,
                    "Arial" | "Helvetica" | "Times New Roman" | "Arial Unicode MS"
                );
            }

            // 在数据库中查找字体
            for face in db.faces() {
                if let Some(family) = face.families.first() {
                    if family.0 == font_name {
                        return true;
                    }
                }
            }
            false
        }

        #[cfg(not(feature = "software-renderer-systemfonts"))]
        {
            // 回退模式：检查常见字体
            matches!(font_name, "Arial" | "Helvetica" | "Times New Roman" | "Arial Unicode MS")
        }
    }

    /// 🎯 获取平台推荐的通用字体列表
    ///
    /// 基于操作系统返回最佳字体选择，支持多语言显示
    pub fn get_platform_fonts() -> Vec<String> {
        #[cfg(target_os = "macos")]
        {
            vec![
                "Arial Unicode MS".to_string(), // 通用 Unicode 字体
                "Helvetica".to_string(),        // macOS 默认
                "Arial".to_string(),            // 通用回退
                "Times New Roman".to_string(),  // 衬线回退
            ]
        }

        #[cfg(target_os = "windows")]
        {
            vec![
                "Arial Unicode MS".to_string(), // 通用 Unicode 字体
                "Arial".to_string(),            // Windows 默认
                "Tahoma".to_string(),           // Windows 通用
                "Times New Roman".to_string(),  // 衬线回退
            ]
        }

        #[cfg(target_os = "linux")]
        {
            vec![
                "Arial Unicode MS".to_string(), // 通用 Unicode 字体
                "DejaVu Sans".to_string(),      // Linux 默认
                "Liberation Sans".to_string(),  // Linux 通用
                "Arial".to_string(),            // 通用回退
            ]
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            vec![
                "Arial Unicode MS".to_string(), // 通用 Unicode 字体
                "Arial".to_string(),            // 通用回退
                "Helvetica".to_string(),        // 通用回退
                "Times New Roman".to_string(),  // 衬线回退
            ]
        }
    }

    /// 🧠 智能字体选择：获取最佳可用字体
    ///
    /// 自动选择当前系统中最佳的通用字体
    pub fn get_best_available_font() -> String {
        let platform_fonts = Self::get_platform_fonts();

        println!("🧠 智能字体选择开始...");
        println!("🎯 平台推荐字体: {:?}", platform_fonts);

        // 尝试每个推荐字体
        for font in &platform_fonts {
            if Self::font_supported(font) {
                println!("✅ 选择字体: {}", font);
                return font.clone();
            } else {
                println!("❌ 字体不可用: {}", font);
            }
        }

        // 如果都不可用，返回最安全的回退
        let fallback = "Arial".to_string();
        println!("⚠️  使用最终回退字体: {}", fallback);
        fallback
    }

    /// Builder pattern: Set text content (Slint field: text)
    pub fn with_text(mut self, text: SharedString) -> Self {
        self.text = text;
        self
    }

    /// Builder pattern: Set text color (Slint field: color)
    pub fn with_color(mut self, color: Brush) -> Self {
        self.color = color;
        self
    }

    /// Builder pattern: Set font size (Slint field: font-size)
    pub fn with_font_size(mut self, size: LogicalLength) -> Self {
        self.font_size = size;
        self
    }

    /// Set font size (accepts f32 for convenience)
    pub fn with_font_size_f32(mut self, size: f32) -> Self {
        self.font_size = LogicalLength::new(size);
        self
    }

    /// Builder pattern: Set font family (Slint field: font-family)
    ///
    /// This is the new generic font API that replaces the language-specific methods.
    /// Examples:
    /// - .with_font_family("PingFang SC")  // macOS Chinese
    /// - .with_font_family("Microsoft YaHei")  // Windows Chinese
    /// - .with_font_family("Arial")  // English
    pub fn with_font_family(mut self, font_family: impl Into<SharedString>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    /// Builder pattern: Set font family list with fallback support
    ///
    /// 🎯 **NEW**: Advanced font fallback mechanism
    /// This method allows you to specify multiple fonts in priority order.
    /// The system will try each font in sequence until one is available.
    ///
    /// Examples:
    /// ```rust
    /// Text::new()
    ///     .with_font_family_list(vec![
    ///         "PingFang SC",           // macOS preferred
    ///         "Microsoft YaHei",       // Windows fallback
    ///         "Arial Unicode MS",      // Universal fallback
    ///         "Noto Sans CJK SC"       // Linux fallback
    ///     ])
    /// ```
    pub fn with_font_family_list(mut self, families: Vec<&str>) -> Self {
        self.font_family_list = families.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Get current text content
    pub fn text(&self) -> &SharedString {
        &self.text
    }

    /// Get current color
    pub fn color(&self) -> &Brush {
        &self.color
    }

    /// Create a TextItem with this Text's properties
    pub fn create_item(&self) -> TextItem {
        let item = TextItem::default();

        // 🎯 设置位置和尺寸 - 适合布局容器中的子控件
        item.x.set(LogicalLength::new(0.0));
        item.y.set(LogicalLength::new(0.0));
        item.width.set(LogicalLength::new(200.0)); // 合适的默认宽度
        item.height.set(LogicalLength::new(30.0)); // 合适的默认高度

        // 设置文本属性
        item.text.set(self.text.clone());
        item.color.set(self.color.clone());
        item.font_size.set(self.font_size);

        // 🎯 新增：真正的字体回退机制
        if !self.font_family_list.is_empty() {
            // 🔥 实现完整的字体回退逻辑
            let mut selected_font = None;

            println!("🔍 字体回退机制启动，尝试 {} 个字体", self.font_family_list.len());

            #[cfg(feature = "software-renderer-systemfonts")]
            {
                use i_slint_core::sharedfontdb;

                for (index, family) in self.font_family_list.iter().enumerate() {
                    println!("🔍 尝试字体 {}: '{}'", index + 1, family);

                    // 创建字体请求进行查询
                    let font_request = i_slint_core::graphics::FontRequest {
                        family: Some(family.clone().into()),
                        weight: Some(400), // normal weight
                        pixel_size: Some(LogicalLength::new(12.0)),
                        letter_spacing: Some(LogicalLength::new(0.0)),
                        italic: Some(false),
                    };

                    // 查询字体数据库
                    let font_found = sharedfontdb::FONT_DB
                        .with_borrow(|db| db.query(&font_request.to_fontdb_query()).is_some());

                    if font_found {
                        println!("✅ 字体 '{}' 可用！选择为主字体", family);
                        selected_font = Some(family.clone());
                        break;
                    } else {
                        println!("❌ 字体 '{}' 不可用，尝试下一个", family);
                    }
                }
            }

            #[cfg(not(feature = "software-renderer-systemfonts"))]
            {
                // 如果没有字体数据库功能，使用第一个字体
                println!("⚠️ 字体数据库不可用，使用第一个字体: '{}'", self.font_family_list[0]);
                selected_font = Some(self.font_family_list[0].clone());
            }

            // 设置选中的字体或回退到默认字体
            if let Some(font) = selected_font {
                item.font_family.set(Some(font));
            } else {
                println!("⚠️ 所有字体都不可用，回退到默认字体: Arial Unicode MS");
                item.font_family.set(Some("Arial Unicode MS".into()));
            }
        } else if let Some(ref font) = self.font_family {
            item.font_family.set(Some(font.clone()));
        } else {
            // 🎯 关键改进：默认使用 Arial Unicode MS 确保中文显示
            item.font_family.set(Some("Arial Unicode MS".into()));
        }

        item
    }

    /// Create a shared Rc<TextItem> with this Text's properties
    pub fn create_item_rc(&self) -> Rc<TextItem> {
        Rc::new(self.create_item())
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

/// Layer 3: View Implementation - Unified Architecture
///
/// 提供完整的控件 API 集成。
impl View for Text {
    type ItemType = TextItem;

    /// Create the actual TextItem with configured properties
    fn create_item(self) -> Self::ItemType {
        let item = TextItem::default();

        // 🎯 Debug output (before moving)
        println!("🎯 Text::create_item: text = '{}', color = {:?}", self.text, self.color);

        // Configure text content and styling using .set() method for Property types
        item.text.set(self.text.clone());
        item.color.set(self.color);
        item.font_size.set(self.font_size);
        item.font_family.set(self.font_family);

        item
    }

    /// 🎯 **关键修复**：Text 使用 CompositeItemTree 架构
    ///
    /// 与 Button、Rectangle 控件保持一致，创建带 WindowItem 根节点的结构
    fn into_item_tree(self) -> Option<i_slint_core::item_tree::ItemTreeRc> {
        println!("🎯 Text::into_item_tree: Creating CompositeItemTree with WindowItem root");

        // 创建 WindowItem 作为根节点
        let window_item = i_slint_core::items::WindowItem::default();

        // 创建 Text 控件
        let text_item = self.create_item();

        // 创建 CompositeItemTree：WindowItem(index 0) + Text(index 1)
        let tree_rc =
            crate::item_tree_integration::create_composite_item_tree(window_item, text_item);

        println!("✅ Text::into_item_tree: CompositeItemTree created successfully");
        Some(tree_rc)
    }
}

/// Helper functions for creating Text items
pub mod helpers {
    use super::*;
    use i_slint_core::graphics::Color;

    /// Create a red text (useful for testing/debugging)
    pub fn red_text(content: &str) -> Text {
        Text::new()
            .with_text(content.into())
            .with_color(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
    }

    /// Create a blue text
    pub fn blue_text(content: &str) -> Text {
        Text::new()
            .with_text(content.into())
            .with_color(Brush::SolidColor(Color::from_rgb_u8(0, 100, 255)))
    }
}
