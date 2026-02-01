// GearUI GridLayout - 网格布局容器
//
// 对应 Slint 的 GridLayout 控件，提供子控件的网格排列功能
//
// 字段映射表：
// | GearUI 字段 | Slint 字段 | 类型 | 默认值 | 说明 |
// |-------------|------------|------|--------|------|
// | spacing     | spacing    | LogicalLength | 0px | 子控件间距 |
// | padding     | padding    | LogicalLength | 0px | 内边距 |
//
// 用法示例：
// ```rust
// let layout = GridLayout::new()
//     .with_spacing(10.0)
//     .with_padding(5.0);
// ```

use std::pin::Pin;

use crate::ViewWrapper;
use const_field_offset::FieldOffsets;
use i_slint_core::{
    ItemVTable_static, Property, declare_item_vtable,
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

/// Layer 1: Core GridLayout Item Implementation
///
/// 实现 Item trait，提供实际的网格布局逻辑。
/// 字段名和默认值完全匹配 Slint 的 GridLayout 规范。
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct GridLayoutItem {
    /// 🎯 **必需的位置和尺寸属性** - 所有 Slint Item 都需要这些
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,

    /// Spacing between children (corresponds to Slint: spacing)
    /// Default: 0px
    pub spacing: Property<LogicalLength>,
    /// Padding around the grid (corresponds to Slint: padding)
    /// Default: 0px
    pub padding: Property<LogicalLength>,
    /// Cached rendering data required by Slint
    pub cached_rendering_data: CachedRenderingData,
}

impl Item for GridLayoutItem {
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
                // 水平方向：网格布局的宽度需求
                let spacing_pin = unsafe { Pin::new_unchecked(&self.get_ref().spacing) };
                let padding_pin = unsafe { Pin::new_unchecked(&self.get_ref().padding) };
                let spacing = spacing_pin.get().get();
                let padding = padding_pin.get().get();

                LayoutInfo {
                    min: padding * 2.0,
                    preferred: padding * 2.0 + spacing * 2.0,
                    ..Default::default()
                }
            }
            Orientation::Vertical => {
                // 垂直方向：网格布局的高度需求
                let spacing_pin = unsafe { Pin::new_unchecked(&self.get_ref().spacing) };
                let padding_pin = unsafe { Pin::new_unchecked(&self.get_ref().padding) };
                let spacing = spacing_pin.get().get();
                let padding = padding_pin.get().get();

                LayoutInfo {
                    min: padding * 2.0,
                    preferred: padding * 2.0 + spacing * 2.0,
                    ..Default::default()
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

impl ItemConsts for GridLayoutItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
    fn slint_get_GearUIGridLayoutVTable() -> GearUIGridLayoutVTable for GridLayoutItem
}

/// 🎯 Phase 8: GridCell - 网格单元格，包含控件和位置信息
#[derive(Clone)]
pub struct GridCell {
    pub view: crate::ViewWrapper,
    pub row: u32,
    pub col: u32,
}

/// Layer 2: GridLayout View Control Wrapper
///
/// 用户友好的 GridLayout 控件 API，提供构建器模式。
#[derive(Clone)]
pub struct GridLayout {
    // Store properties directly for simple builder pattern
    spacing: f32,
    padding: f32,
    width: LogicalLength,
    height: LogicalLength,
    x: LogicalLength,
    y: LogicalLength,

    // 🎯 Phase 8: 子控件管理系统（网格需要位置信息）
    children: Vec<GridCell>,
}

impl GridLayout {
    /// Create a new GridLayout instance with Slint-compliant default values
    pub fn new() -> Self {
        Self {
            spacing: 0.0, // Default: 0px
            padding: 0.0, // Default: 0px
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

    /// Builder pattern: Set padding around the grid (Slint field: padding)
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
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

    /// Get current padding
    pub fn padding(&self) -> f32 {
        self.padding
    }

    /// 🎯 Phase 8: 添加子控件到网格布局中
    ///
    /// 这是核心的组合功能，允许用户添加任意控件到网格布局的指定位置。
    ///
    /// 使用示例：
    /// ```rust
    /// let layout = GridLayout::new()
    ///     .with_child(Text::new().with_text("A"), 0, 0)
    ///     .with_child(Text::new().with_text("B"), 0, 1)
    ///     .with_child(Text::new().with_text("C"), 1, 0)
    ///     .with_child(Text::new().with_text("D"), 1, 1);
    /// ```
    pub fn with_child<V: Into<crate::ViewWrapper>>(mut self, child: V, row: u32, col: u32) -> Self {
        self.children.push(GridCell { view: child.into(), row, col });
        self
    }

    /// 🎯 Phase 8: 获取子控件数量
    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    /// 🎯 **递归树展开支持** - 获取所有子控件
    pub fn get_children(&self) -> &Vec<GridCell> {
        &self.children
    }

    /// Create a GridLayoutItem with this GridLayout's properties
    fn create_item_internal(&self) -> GridLayoutItem {
        let item = GridLayoutItem::default();

        // ✅ 标准 Slint 模式：使用 unsafe Pin 访问设置属性
        unsafe {
            Pin::new_unchecked(&item.width).set(self.width);
            Pin::new_unchecked(&item.height).set(self.height);
            Pin::new_unchecked(&item.x).set(self.x);
            Pin::new_unchecked(&item.y).set(self.y);
            Pin::new_unchecked(&item.spacing).set(LogicalLength::new(self.spacing));
            Pin::new_unchecked(&item.padding).set(LogicalLength::new(self.padding));
        }

        item
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::View for GridLayout {
    type ItemType = GridLayoutItem;

    fn create_item(self) -> Self::ItemType {
        // ✅ 标准 Slint 模式：委托给内部创建方法
        self.create_item_internal()
    }

    /// ✅ **容器控件的 build() 实现** - 递归构建所有子控件
    ///
    /// 这是布局容器的核心构建逻辑：
    /// 1. 遍历所有子控件（GridCell）
    /// 2. 对每个子控件调用 build() 方法
    /// 3. 确保在构建期就完成所有子控件的静态树展开
    /// 4. 避免运行时的动态递归调用
    fn build(self) -> Self {
        let children_count = self.children.len();
        println!("🎯 GridLayout::build() - 递归构建 {} 个子控件", children_count);

        // 🎯 **关键修复**：递归构建所有子控件
        let built_children = self
            .children
            .into_iter()
            .enumerate()
            .map(|(i, cell)| {
                println!(
                    "🎯   构建子控件 {} / {} (row: {}, col: {})",
                    i + 1,
                    children_count,
                    cell.row,
                    cell.col
                );

                // ✅ 调用 ViewWrapper 的 build() 方法，实现递归构建
                // 保持 GridCell 的结构，只更新其中的 view
                GridCell { view: cell.view.build(), row: cell.row, col: cell.col }
            })
            .collect();

        GridLayout {
            children: built_children,
            spacing: self.spacing,
            padding: self.padding,
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
            "🎯 GridLayout::into_item_tree: Creating MultiItemTree with {} children",
            self.children.len()
        );

        // ✅ **关键修复**：使用能够包含多个子控件的 MultiItemTree 架构
        // 而不是只能包含2个控件的 CompositeItemTree

        if self.children.is_empty() {
            // 如果没有子控件，使用 CompositeItemTree
            println!("🎯 GridLayout has no children, using CompositeItemTree");
            let window_item = i_slint_core::items::WindowItem::default();
            let layout_item = self.create_item_internal();
            let tree_rc =
                crate::item_tree_integration::create_composite_item_tree(window_item, layout_item);
            Some(tree_rc)
        } else {
            // 如果有子控件，使用 MultiItemTree 包含所有子控件
            println!("🎯 GridLayout has {} children, using MultiItemTree", self.children.len());

            // 创建 GridLayout 控件
            let layout_item = self.create_item_internal();

            // ✅ **关键修复**：将 GridCell 转换为 ViewWrapper，然后传递给 MultiItemTree
            let child_wrappers: Vec<ViewWrapper> = self
                .children
                .into_iter()
                .map(|cell| cell.view) // 提取 GridCell 中的 ViewWrapper
                .collect();

            let multi_tree = crate::item_tree_integration::create_multi_item_tree_from_wrappers(
                layout_item,
                child_wrappers, // 传递已经构建的子控件
            );

            println!("✅ GridLayout::into_item_tree: MultiItemTree created successfully");
            Some(multi_tree)
        }
    }
}

/// Helper functions for quick GridLayout creation
pub mod helpers {
    use super::*;

    /// Create a grid layout with spacing and padding
    pub fn grid_layout_with_spacing_and_padding(spacing: f32, padding: f32) -> GridLayout {
        GridLayout::new().with_spacing(spacing).with_padding(padding)
    }

    /// Create a compact grid layout (no spacing or padding)
    pub fn compact_grid_layout() -> GridLayout {
        GridLayout::new()
    }
}
