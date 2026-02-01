// GearUI VerticalLayout - 垂直布局容器
//
// 对应 Slint 的 VerticalLayout 控件，提供子控件的垂直排列功能
//
// 字段映射表：
// | GearUI 字段 | Slint 字段 | 类型 | 默认值 | 说明 |
// |-------------|------------|------|--------|------|
// | spacing     | spacing    | LogicalLength | 0px | 子控件间距 |
// | alignment   | alignment  | LayoutAlignment | LayoutAlignment::Stretch | 对齐方式 |
//
// 用法示例：
// ```rust
// let layout = VerticalLayout::new()
//     .with_spacing(10.0)
//     .with_alignment(LayoutAlignment::Center);
// ```

use std::pin::Pin;

use const_field_offset::FieldOffsets;
use i_slint_core::{
    ItemVTable_static, Property, declare_item_vtable,
    input::{
        FocusEvent, FocusEventResult, InputEventFilterResult, InputEventResult, KeyEvent,
        KeyEventResult, MouseEvent,
    },
    item_rendering::{CachedRenderingData, ItemRenderer},
    items::{Item, ItemConsts, ItemRc, ItemVTable, LayoutAlignment, RenderingResult},
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    window::WindowAdapterRc,
};
use i_slint_core_macros::*;

// Required for VTable macro
type ItemRendererRef<'a> = &'a mut dyn i_slint_core::item_rendering::ItemRenderer;

/// Layer 1: Core VerticalLayout Item Implementation
///
/// 实现 Item trait，提供实际的垂直布局逻辑。
/// 字段名和默认值完全匹配 Slint 的 VerticalLayout 规范。
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct VerticalLayoutItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Spacing between children (corresponds to Slint: spacing)
    /// Default: 0px
    pub spacing: Property<LogicalLength>,
    /// Alignment of children (corresponds to Slint: alignment)
    /// Default: LayoutAlignment::Stretch
    pub alignment: Property<LayoutAlignment>,
    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for VerticalLayoutItem {
    fn init(self: Pin<&Self>, _self_rc: &ItemRc) {
        // No initialization required for basic layout
    }

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &WindowAdapterRc,
        _self_rc: &ItemRc,
    ) -> LayoutInfo {
        // ✅ 标准 Slint 模式：基于布局方向计算布局信息
        match orientation {
            Orientation::Horizontal => {
                // 水平方向：布局容器本身的宽度需求
                LayoutInfo { min: 0.0, preferred: 100.0, ..Default::default() }
            }
            Orientation::Vertical => {
                // 垂直方向：根据子控件和间距计算高度
                let spacing_pin = unsafe { Pin::new_unchecked(&self.get_ref().spacing) };
                let spacing = spacing_pin.get().get();

                LayoutInfo { min: spacing, preferred: spacing * 2.0, ..Default::default() }
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
        // ✅ 标准 Slint 模式：布局容器不直接渲染，只管理子控件
        println!("🎯 VerticalLayoutItem::render called - delegating to children");
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

impl ItemConsts for VerticalLayoutItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUIVerticalLayoutVTable() -> GearUIVerticalLayoutVTable for VerticalLayoutItem
}

/// Layer 2: VerticalLayout View Control Wrapper
///
/// 用户友好的 VerticalLayout 控件 API，提供构建器模式。
#[derive(Clone)]
pub struct VerticalLayout {
    // Store properties directly for simple builder pattern
    spacing: f32,
    alignment: LayoutAlignment,
    width: LogicalLength,
    height: LogicalLength,
    x: LogicalLength,
    y: LogicalLength,

    // 🎯 Phase 8: 子控件管理系统
    children: Vec<crate::ViewWrapper>,
}

impl VerticalLayout {
    /// Create a new VerticalLayout instance with Slint-compliant default values
    pub fn new() -> Self {
        Self {
            spacing: 0.0,                        // Default: 0px
            alignment: LayoutAlignment::Stretch, // Default: stretch
            width: LogicalLength::new(0.0),
            height: LogicalLength::new(0.0),
            x: LogicalLength::new(0.0),
            y: LogicalLength::new(0.0),
            children: Vec::new(), // 🎯 Phase 8: 初始化子控件列表
        }
    }

    /// Builder pattern: Set spacing between children (Slint field: spacing)
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Builder pattern: Set alignment of children (Slint field: alignment)
    pub fn with_alignment(mut self, alignment: LayoutAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Builder pattern: Set width (Slint field: width)
    pub fn with_width(mut self, width: LogicalLength) -> Self {
        self.width = width;
        self
    }

    /// Builder pattern: Set height (Slint field: height)
    pub fn with_height(mut self, height: LogicalLength) -> Self {
        self.height = height;
        self
    }

    /// Builder pattern: Set x position (Slint field: x)
    pub fn with_x(mut self, x: LogicalLength) -> Self {
        self.x = x;
        self
    }

    /// Builder pattern: Set y position (Slint field: y)
    pub fn with_y(mut self, y: LogicalLength) -> Self {
        self.y = y;
        self
    }

    /// Get current spacing
    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    /// Get current alignment
    pub fn alignment(&self) -> LayoutAlignment {
        self.alignment
    }

    /// 🎯 Phase 8: 添加子控件到布局中
    ///
    /// 这是核心的组合功能，允许用户添加任意控件到垂直布局中。
    ///
    /// 使用示例：
    /// ```rust
    /// let layout = VerticalLayout::new()
    ///     .with_child(Text::new().with_text("标题"))
    ///     .with_child(Button::new().with_text("按钮"))
    ///     .with_child(Rectangle::new().with_background(...));
    /// ```
    pub fn with_child<V: Into<crate::ViewWrapper>>(mut self, child: V) -> Self {
        self.children.push(child.into());
        self
    }

    /// 🎯 Phase 8: 添加多个子控件
    ///
    /// 便利方法，一次性添加多个子控件
    pub fn with_children<V: Into<crate::ViewWrapper>>(mut self, children: Vec<V>) -> Self {
        for child in children {
            self.children.push(child.into());
        }
        self
    }

    /// 🎯 Phase 8: 获取子控件数量
    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    /// 🎯 **递归树展开支持** - 获取所有子控件
    pub fn get_children(&self) -> &Vec<crate::ViewWrapper> {
        &self.children
    }

    /// Create a VerticalLayoutItem with this VerticalLayout's properties
    fn create_item_internal(&self) -> VerticalLayoutItem {
        let item = VerticalLayoutItem::default();

        // ✅ 标准 Slint 模式：使用 unsafe Pin 访问设置属性
        unsafe {
            Pin::new_unchecked(&item.width).set(self.width);
            Pin::new_unchecked(&item.height).set(self.height);
            Pin::new_unchecked(&item.x).set(self.x);
            Pin::new_unchecked(&item.y).set(self.y);
            Pin::new_unchecked(&item.spacing).set(LogicalLength::new(self.spacing));
            Pin::new_unchecked(&item.alignment).set(self.alignment);
        }

        item
    }
}

impl Default for VerticalLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::View for VerticalLayout {
    type ItemType = VerticalLayoutItem;

    fn create_item(self) -> Self::ItemType {
        let item = VerticalLayoutItem {
            width: Property::default(),
            height: Property::default(),
            x: Property::default(),
            y: Property::default(),
            spacing: Property::default(),
            alignment: Property::default(),
            cached_rendering_data: CachedRenderingData::default(),
        };

        // 设置属性
        unsafe {
            Pin::new_unchecked(&item.width).set(self.width);
            Pin::new_unchecked(&item.height).set(self.height);
            Pin::new_unchecked(&item.x).set(self.x);
            Pin::new_unchecked(&item.y).set(self.y);
            Pin::new_unchecked(&item.spacing).set(LogicalLength::new(self.spacing));
            Pin::new_unchecked(&item.alignment).set(self.alignment);
        }

        item
    }

    /// ✅ 递归构建所有子控件
    ///
    /// 调用每个子控件的 build() 方法，实现深度优先构建
    fn build(self) -> Self {
        let children_count = self.children.len();
        println!("🎯 VerticalLayout::build() - 递归构建 {} 个子控件", children_count);

        // 🎯 **关键修复**：递归构建所有子控件
        let built_children = self
            .children
            .into_iter()
            .enumerate()
            .map(|(i, child)| {
                println!("🎯   构建子控件 {} / {}", i + 1, children_count);

                // ✅ 调用 ViewWrapper 的 build() 方法，实现递归构建
                child.build()
            })
            .collect();

        VerticalLayout {
            children: built_children,
            spacing: self.spacing,
            alignment: self.alignment,
            width: self.width,
            height: self.height,
            x: self.x,
            y: self.y,
        }
    }

    fn transfer_properties_to_window(&self, _window_item: &i_slint_core::items::WindowItem) {
        // ✅ 标准 Slint 模式：布局容器通常不需要转移属性到窗口
        // 布局容器主要处理子控件的排列，而不是窗口级别的属性
    }

    /// ✅ **静态树展开架构** - 使用 MultiItemTree 包含所有子控件
    ///
    /// 核心理念：在构建期将所有子控件转换为静态 Item 引用，避免运行时动态转换
    /// 这解决了之前 MultiItemTree 中 visit_children_item 动态转换导致的无限递归
    fn into_item_tree(self) -> Option<i_slint_core::item_tree::ItemTreeRc> {
        println!(
            "🎯 VerticalLayout::into_item_tree: Creating MultiItemTree with {} children",
            self.children.len()
        );

        // ✅ **关键修复**：使用能够包含多个子控件的 MultiItemTree 架构
        // 而不是只能包含2个控件的 CompositeItemTree

        if self.children.is_empty() {
            // 如果没有子控件，使用 CompositeItemTree
            println!("🎯 VerticalLayout has no children, using CompositeItemTree");
            let window_item = i_slint_core::items::WindowItem::default();
            let layout_item = self.create_item_internal();
            let tree_rc =
                crate::item_tree_integration::create_composite_item_tree(window_item, layout_item);
            Some(tree_rc)
        } else {
            // 如果有子控件，使用 MultiItemTree 包含所有子控件
            println!("🎯 VerticalLayout has {} children, using MultiItemTree", self.children.len());

            // 创建 VerticalLayout 控件
            let layout_item = self.create_item_internal();

            // ✅ **关键修复**：传递已经构建的子控件，避免运行时动态转换
            let multi_tree = crate::item_tree_integration::create_multi_item_tree_from_wrappers(
                layout_item,
                self.children, // 传递已经构建的子控件
            );

            println!("✅ VerticalLayout::into_item_tree: MultiItemTree created successfully");
            Some(multi_tree)
        }
    }
}

/// Helper functions for quick VerticalLayout creation
pub mod helpers {
    use super::*;

    /// Create a vertical layout with spacing
    pub fn vertical_layout_with_spacing(spacing: f32) -> VerticalLayout {
        VerticalLayout::new().with_spacing(spacing)
    }

    /// Create a center-aligned vertical layout
    pub fn center_vertical_layout() -> VerticalLayout {
        VerticalLayout::new().with_alignment(LayoutAlignment::Center)
    }
}
