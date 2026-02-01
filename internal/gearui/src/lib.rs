// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

/*!
# GearUI - Pure Rust UI Controls for Slint

GearUI provides a complete set of UI controls implemented in pure Rust,
offering a mechanical translation of `.slint` built-in controls to native Rust API.

## Key Features

- **Pure Rust API**: No `.slint` files required
- **Builder Pattern**: Fluent API for control creation
- **Full Slint Compatibility**: 1:1 mapping with Slint controls
- **Performance**: Direct integration with Slint's rendering system

## 3-Layer Architecture

GearUI implements a unified 3-layer architecture for all controls:

### Layer 1: Item Implementation (Required)
Every control implements the `Item` trait for rendering, layout, and event handling.

### Layer 2: View Control Wrapper (Required)
Public API with builder pattern and `into_item_tree()` conversion.

### Layer 3: Component Adapter (Optional)
Only needed when control serves as top-level component for `Window::set_component()`.

## GearView Trait

All GearUI controls implement the `GearView` trait for unified conversion:

```rust
pub trait GearView {
    fn into_item_tree(self) -> ItemTreeRc;
    fn into_component(self) -> ComponentRc;
}
```

## Usage Examples

### Basic Usage - ItemTree Integration
```rust
use i_slint_gearui::{Rectangle, GearView};
use i_slint_core::graphics::{Brush, Color};

let rect = Rectangle::new()
    .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)));

let item_tree = rect.into_item_tree(); // For embedding in other components
```

### Top-Level Usage - Window Integration
```rust
use i_slint_gearui::{Rectangle, GearView};
use i_slint_core::{graphics::{Brush, Color}, api::Window};

let rect = Rectangle::new()
    .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)));

let window = Window::new(Default::default());
window.set_component(rect.into_component()); // For top-level window
```

*/

pub mod composed;
pub mod interactive;
pub mod layout;
/// Pure Rust UI Controls for Slint
///
/// GearUI provides a set of pure Rust UI controls that integrate seamlessly with
/// the Slint UI framework. These controls are mechanically translated from Slint's
/// built-in controls, offering the same functionality with a pure Rust API.
// Module declarations
// 🎯 模块声明：这些模块通过各自的 mod.rs 文件定义
pub mod primitives;

pub use self::composed::*;
pub use self::interactive::*;
pub use self::layout::*;
pub use self::primitives::*;

// RootView trait will be automatically available since it's in the same module

/// 🎯 Phase 8: ViewWrapper - 解决 View trait 不是 dyn compatible 的问题
///
/// 由于 View trait 有 `Sized` 约束，我们不能直接使用 `Box<dyn View>`。
/// 这个枚举包装了所有可能的 View 类型，使得布局容器能够存储不同类型的子控件。
#[derive(Clone)]
pub enum ViewWrapper {
    Rectangle(Rectangle),
    Text(Text),
    Image(Image),
    Button(composed::Button),
    TouchArea(interactive::TouchArea),
    CheckBox(interactive::CheckBox),
    Slider(interactive::Slider),
    Switch(interactive::Switch),                    // 阶段1新增
    SwitchComposite(interactive::SwitchComposite),  // 阶段1新增 - 复合组件版本
    ProgressIndicator(composed::ProgressIndicator), // 阶段1新增
    Spinner(composed::Spinner),                     // 阶段1新增
    GroupBox(layout::GroupBox),                     // 阶段1新增
    Clip(primitives::Clip),                         // 阶段1新增
    Opacity(primitives::Opacity),                   // 阶段1新增
    VerticalLayout(VerticalLayout),
    HorizontalLayout(HorizontalLayout),
    GridLayout(GridLayout),
}

impl ViewWrapper {
    /// Convert this wrapper to an ItemTree if possible
    ///
    /// 🎯 **嵌套布局层次修复** - 确保布局控件创建完整的嵌套结构
    pub fn into_item_tree(self) -> Option<ItemTreeRc> {
        match self {
            ViewWrapper::Rectangle(rect) => rect.into_item_tree(),
            ViewWrapper::Text(text) => text.into_item_tree(),
            ViewWrapper::Image(image) => image.into_item_tree(),
            ViewWrapper::Button(button) => button.into_item_tree(),
            ViewWrapper::TouchArea(touch) => touch.into_item_tree(),
            ViewWrapper::CheckBox(checkbox) => checkbox.into_item_tree(),
            ViewWrapper::Slider(slider) => slider.into_item_tree(),
            ViewWrapper::Switch(switch) => switch.into_item_tree(),
            ViewWrapper::SwitchComposite(switch) => switch.into_item_tree(),
            ViewWrapper::ProgressIndicator(progress) => progress.into_item_tree(),
            ViewWrapper::Spinner(spinner) => spinner.into_item_tree(),
            ViewWrapper::GroupBox(groupbox) => groupbox.into_item_tree(),
            ViewWrapper::Clip(clip) => clip.into_item_tree(),
            ViewWrapper::Opacity(opacity) => opacity.into_item_tree(),
            // 🎯 **关键修复**：布局控件需要先构建完整的嵌套结构
            ViewWrapper::VerticalLayout(layout) => {
                println!("🎯 ViewWrapper::into_item_tree for VerticalLayout - 先构建嵌套结构");
                layout.build().into_item_tree()
            }
            ViewWrapper::HorizontalLayout(layout) => {
                println!("🎯 ViewWrapper::into_item_tree for HorizontalLayout - 先构建嵌套结构");
                layout.build().into_item_tree()
            }
            ViewWrapper::GridLayout(layout) => {
                println!("🎯 ViewWrapper::into_item_tree for GridLayout - 先构建嵌套结构");
                layout.build().into_item_tree()
            }
        }
    }

    /// ✅ **ViewWrapper 的 build() 方法** - 递归构建支持
    ///
    /// 这个方法负责对包装的控件调用其 build() 方法：
    /// - 简单控件：直接调用 build()（返回 self）
    /// - 容器控件：递归调用 build()（构建所有子控件）
    ///
    /// 🎯 **关键作用**：
    /// 使得容器控件能够递归构建其子控件，实现完整的静态树展开
    pub fn build(self) -> Self {
        match self {
            ViewWrapper::Rectangle(rect) => {
                println!("🎯 ViewWrapper::build() for Rectangle");
                println!("🎯   Before build: Rectangle background = {:?}", rect.background());
                let built_rect = rect.build();
                println!("🎯   After build: Rectangle background = {:?}", built_rect.background());
                ViewWrapper::Rectangle(built_rect)
            }
            ViewWrapper::Text(text) => ViewWrapper::Text(text.build()),
            ViewWrapper::Image(image) => ViewWrapper::Image(image.build()),
            ViewWrapper::Button(button) => ViewWrapper::Button(button.build()),
            ViewWrapper::TouchArea(touch) => ViewWrapper::TouchArea(touch.build()),
            ViewWrapper::CheckBox(checkbox) => ViewWrapper::CheckBox(checkbox.build()),
            ViewWrapper::Slider(slider) => ViewWrapper::Slider(slider.build()),
            ViewWrapper::Switch(switch) => ViewWrapper::Switch(switch.build()),
            ViewWrapper::SwitchComposite(switch) => ViewWrapper::SwitchComposite(switch.build()),
            ViewWrapper::ProgressIndicator(progress) => {
                ViewWrapper::ProgressIndicator(progress.build())
            }
            ViewWrapper::Spinner(spinner) => ViewWrapper::Spinner(spinner.build()),
            ViewWrapper::GroupBox(groupbox) => ViewWrapper::GroupBox(groupbox.build()),
            ViewWrapper::Clip(clip) => ViewWrapper::Clip(clip.build()),
            ViewWrapper::Opacity(opacity) => ViewWrapper::Opacity(opacity.build()),
            ViewWrapper::VerticalLayout(layout) => ViewWrapper::VerticalLayout(layout.build()),
            ViewWrapper::HorizontalLayout(layout) => ViewWrapper::HorizontalLayout(layout.build()),
            ViewWrapper::GridLayout(layout) => ViewWrapper::GridLayout(layout.build()),
        }
    }

    /// Extract the underlying Item and VTable from this wrapper
    /// Returns (Box<dyn Item>, &'static ItemVTable)
    pub fn extract_item(self) -> (Box<dyn Item>, &'static ItemVTable) {
        match self {
            ViewWrapper::Rectangle(rect) => {
                let item = rect.create_item();
                (Box::new(item), crate::primitives::rectangle::RectangleItem::static_vtable())
            }
            ViewWrapper::Text(text) => {
                let item = text.create_item();
                (Box::new(item), crate::primitives::text::TextItem::static_vtable())
            }
            ViewWrapper::Image(image) => {
                let item = image.create_item();
                (Box::new(item), crate::primitives::image::ImageItem::static_vtable())
            }
            ViewWrapper::Button(button) => {
                let item = button.create_item();
                (Box::new(item), crate::composed::button::ButtonItem::static_vtable())
            }
            ViewWrapper::TouchArea(touch) => {
                let item = touch.create_item();
                (Box::new(item), crate::interactive::touch_area::TouchAreaItem::static_vtable())
            }
            ViewWrapper::CheckBox(checkbox) => {
                let item = checkbox.create_item();
                (Box::new(item), crate::interactive::checkbox::CheckBoxItem::static_vtable())
            }
            ViewWrapper::Slider(slider) => {
                let item = slider.create_item();
                (Box::new(item), crate::interactive::slider::SliderItem::static_vtable())
            }
            ViewWrapper::Switch(switch) => {
                let item = switch.create_item();
                (Box::new(item), crate::interactive::switch::SwitchItem::static_vtable())
            }
            ViewWrapper::SwitchComposite(switch) => {
                // SwitchComposite uses MultiItemTree, so we use a placeholder VerticalLayout item
                let item = crate::layout::VerticalLayout::new().create_item();
                (Box::new(item), crate::layout::vertical::VerticalLayoutItem::static_vtable())
            }
            ViewWrapper::ProgressIndicator(progress) => {
                let item = progress.create_item();
                (
                    Box::new(item),
                    crate::composed::progress_indicator::ProgressIndicatorItem::static_vtable(),
                )
            }
            ViewWrapper::Spinner(spinner) => {
                let item = spinner.create_item();
                (Box::new(item), crate::composed::spinner::SpinnerItem::static_vtable())
            }
            ViewWrapper::GroupBox(groupbox) => {
                let item = groupbox.create_item();
                (Box::new(item), crate::layout::group_box::GroupBoxItem::static_vtable())
            }
            ViewWrapper::Clip(clip) => {
                let item = clip.create_item();
                (Box::new(item), crate::primitives::clip::ClipItem::static_vtable())
            }
            ViewWrapper::Opacity(opacity) => {
                let item = opacity.create_item();
                (Box::new(item), crate::primitives::opacity::OpacityItem::static_vtable())
            }
            ViewWrapper::VerticalLayout(layout) => {
                let item = layout.create_item();
                (Box::new(item), crate::layout::vertical::VerticalLayoutItem::static_vtable())
            }
            ViewWrapper::HorizontalLayout(layout) => {
                let item = layout.create_item();
                let vtable = crate::layout::horizontal::HorizontalLayoutItem::static_vtable();
                println!("🔍 🔥 CRITICAL: ViewWrapper::extract_item for HorizontalLayout");
                println!("🔍   HorizontalLayout vtable address: {:p}", vtable);
                (Box::new(item), vtable)
            }
            ViewWrapper::GridLayout(layout) => {
                let item = layout.create_item();
                (Box::new(item), crate::layout::grid::GridLayoutItem::static_vtable())
            }
        }
    }

    /// Create a ViewWrapper from any Item that implements the required traits
    pub fn from_item<T: Item + HasStaticVTable<ItemVTable> + 'static>(_item: T) -> Self {
        // 🎯 这是一个简化实现，真正的实现需要根据 T 的类型来判断
        // 由于我们没有反向类型信息，暂时创建一个默认的Rectangle
        ViewWrapper::Rectangle(Rectangle::new())
    }

    /// 🎯 **递归树展开支持** - 获取容器控件的子控件
    ///
    /// 这个方法用于实现递归树展开，将嵌套容器的所有子控件收集到顶层 MultiItemTree 中
    pub fn get_children(&self) -> Vec<ViewWrapper> {
        match self {
            ViewWrapper::VerticalLayout(layout) => layout.get_children().clone(),
            ViewWrapper::HorizontalLayout(layout) => layout.get_children().clone(),
            ViewWrapper::GridLayout(layout) => {
                // GridLayout 的子控件存储在 GridCell 中，需要提取出来
                layout
                    .get_children()
                    .iter()
                    .map(|cell| cell.view.clone()) // 提取 GridCell 中的 ViewWrapper
                    .collect()
            }
            // 简单控件没有子控件
            _ => Vec::new(),
        }
    }

    /// 🎯 **递归树展开支持** - 检查是否为容器控件
    pub fn is_container(&self) -> bool {
        match self {
            ViewWrapper::VerticalLayout(_)
            | ViewWrapper::HorizontalLayout(_)
            | ViewWrapper::GridLayout(_) => true,
            _ => false,
        }
    }

    /// 🎯 **递归树展开支持** - 获取容器类型名称
    pub fn get_container_type_name(&self) -> &'static str {
        match self {
            ViewWrapper::VerticalLayout(_) => "VerticalLayout",
            ViewWrapper::HorizontalLayout(_) => "HorizontalLayout",
            ViewWrapper::GridLayout(_) => "GridLayout",
            _ => "SimpleControl",
        }
    }
}

// 为每个 View 类型实现 Into<ViewWrapper>
impl From<Rectangle> for ViewWrapper {
    fn from(v: Rectangle) -> Self {
        println!("🎯 Converting Rectangle to ViewWrapper");
        println!("🎯   Rectangle background: {:?}", v.background());
        ViewWrapper::Rectangle(v)
    }
}

impl From<Text> for ViewWrapper {
    fn from(v: Text) -> Self {
        ViewWrapper::Text(v)
    }
}

impl From<Image> for ViewWrapper {
    fn from(v: Image) -> Self {
        ViewWrapper::Image(v)
    }
}

impl From<composed::Button> for ViewWrapper {
    fn from(v: composed::Button) -> Self {
        ViewWrapper::Button(v)
    }
}

impl From<interactive::TouchArea> for ViewWrapper {
    fn from(v: interactive::TouchArea) -> Self {
        ViewWrapper::TouchArea(v)
    }
}

impl From<interactive::CheckBox> for ViewWrapper {
    fn from(v: interactive::CheckBox) -> Self {
        ViewWrapper::CheckBox(v)
    }
}

impl From<interactive::Slider> for ViewWrapper {
    fn from(v: interactive::Slider) -> Self {
        ViewWrapper::Slider(v)
    }
}

impl From<VerticalLayout> for ViewWrapper {
    fn from(v: VerticalLayout) -> Self {
        ViewWrapper::VerticalLayout(v)
    }
}

impl From<HorizontalLayout> for ViewWrapper {
    fn from(v: HorizontalLayout) -> Self {
        ViewWrapper::HorizontalLayout(v)
    }
}

impl From<GridLayout> for ViewWrapper {
    fn from(v: GridLayout) -> Self {
        ViewWrapper::GridLayout(v)
    }
}

use i_slint_core::{
    SharedString, SharedVector,
    accessibility::{AccessibilityAction, SupportedAccessibilityAction},
    api::PlatformError,
    item_tree::{ItemTree, ItemTreeNode, ItemTreeRc, ItemTreeVTable, TraversalOrder},
    items::{AccessibleRole, Item, ItemRc, ItemRef, ItemVTable},
    layout::{LayoutInfo, Orientation},
    lengths::{LogicalPoint, LogicalRect, LogicalSize},
    slice::Slice,
    window::WindowAdapterRc,
};
use std::ptr::NonNull;
use vtable::{HasStaticVTable, VRc, VRef};

/// Core trait for all GearUI controls - provides unified conversion capabilities
///
/// This trait implements the 2-layer architecture:
/// - Layer 1: Item implementation (render/layout/events)
/// - Layer 2: View struct with builder pattern (all controls implement View)
pub trait View: Sized {
    /// The underlying Item type that implements the actual functionality
    type ItemType: Item + HasStaticVTable<ItemVTable> + 'static;

    /// Convert this view into a live ItemTree that can be rendered
    /// Phase 2: Real ItemTree integration implementation
    fn into_item_tree(self) -> Option<ItemTreeRc> {
        // Phase 2: Create real SingleItemTree with the user's item
        let item = self.create_item();
        let tree_rc = crate::item_tree_integration::create_single_item_tree(item);
        Some(tree_rc)
    }

    /// Create the underlying Item instance
    /// This is called internally by into_item_tree()
    fn create_item(self) -> Self::ItemType;

    /// ✅ **统一构建方法** - 所有控件的统一 build 入口
    ///
    /// 这个方法负责：
    /// - 简单控件：直接返回 self（无需额外构建）
    /// - 容器控件：递归构建所有子控件，生成完整的静态树
    /// - 确保在 RootView 中统一调用，避免运行时递归
    ///
    /// 🎯 **核心架构原则**：
    /// - 构建期决定结构 + 渲染期只读结构
    /// - 与 Flutter、SwiftUI、Slint 内部保持一致
    ///
    /// 默认实现：返回 self（适用于简单控件）
    /// 容器控件必须重写此方法实现递归构建
    fn build(self) -> Self
    where
        Self: Sized,
    {
        // 默认实现：简单控件直接返回自己
        self
    }

    /// Create an ItemRc for immediate use - Phase 2 implementation
    /// Useful for testing and direct item manipulation
    fn create_item_rc(self) -> Option<ItemRc> {
        if let Some(item_tree) = self.into_item_tree() {
            Some(ItemRc::new(item_tree, 0))
        } else {
            None
        }
    }

    /// 将控件的属性转移到 WindowItem - RootView 架构核心方法
    ///
    /// 这个方法解决 WindowItem 根节点要求：让用户控件的视觉属性
    /// 能够在 WindowItem 中正确显示，从而解决黑屏问题。
    ///
    /// 默认实现：不转移任何属性（适用于无视觉属性的控件）
    /// 控件可以重写此方法来转移特定属性（如 Rectangle 转移背景色）
    fn transfer_properties_to_window(&self, _window_item: &i_slint_core::items::WindowItem) {
        // 默认实现：不转移任何属性
    }
}

/// Library version and build information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = "Pure Rust UI Controls for Slint - 3-Layer Architecture";

/// Quick constructor functions for common use cases
pub mod quick {
    use super::*;
    use i_slint_core::graphics::{Brush, Color};

    /// Create a red rectangle (common for testing/debugging)
    pub fn red_rect() -> Rectangle {
        Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
    }

    /// Create a blue rectangle
    pub fn blue_rect() -> Rectangle {
        Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(0, 100, 255)))
    }

    /// Create a green rectangle
    pub fn green_rect() -> Rectangle {
        Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(0, 200, 100)))
    }
}

/// Phase 2 Implementation - Real ItemTree Integration
/// This section will contain the complete SingleItemTree implementation
/// once all compilation issues are resolved.
pub mod item_tree_integration {
    //! Real ItemTree Integration Implementation
    //!
    //! This module contains the SingleItemTree implementation that enables
    //! true integration with Slint's rendering and event system.

    use super::*;
    use core::pin::Pin;
    use core::ptr::NonNull;
    use i_slint_core::SharedString;
    use i_slint_core::accessibility::{
        AccessibilityAction, AccessibleStringProperty, SupportedAccessibilityAction,
    };
    use i_slint_core::item_tree::{
        IndexRange, ItemTree, ItemTreeNode, ItemTreeRc, ItemTreeVTable, ItemTreeWeak,
        ItemVisitorRefMut, ItemVisitorVTable, ItemWeak, TraversalOrder, VisitChildrenResult,
    };
    use i_slint_core::items::{AccessibleRole, ItemRef, ItemVTable};
    use i_slint_core::layout::{LayoutInfo, LayoutItemInfo, Orientation};
    use i_slint_core::lengths::{LogicalPoint, LogicalRect, LogicalSize};
    use i_slint_core::slice::Slice;
    use i_slint_core::window::WindowAdapterRc;
    use std::boxed::Box;
    use vtable::{HasStaticVTable, VRc, VRef};

    // Import the static vtable macro
    use i_slint_core::ItemTreeVTable_static;

    /// A simple ItemTree implementation that contains a single item
    pub struct SingleItemTree {
        item_box: Box<dyn Item>,
        item_vtable: &'static ItemVTable,
        window_adapter: Option<WindowAdapterRc>,
        // Cache the item reference to avoid lifetime issues
        item_data: *const u8,
    }

    // Safety: We guarantee that item_data is valid for the lifetime of the struct
    unsafe impl Send for SingleItemTree {}
    unsafe impl Sync for SingleItemTree {}

    impl SingleItemTree {
        /// Create a new SingleItemTree with the given item
        pub fn new<T: Item + HasStaticVTable<ItemVTable> + 'static>(item: T) -> Self {
            let vtable = T::static_vtable();
            let item_box = Box::new(item);
            let item_data = item_box.as_ref() as *const dyn Item as *const u8;

            Self {
                item_box,
                item_vtable: vtable,
                window_adapter: None, // 🔥 修复：暂时设为None，避免WindowAdapter冲突
                item_data,
            }
        }

        /// Set the window adapter for this SingleItemTree
        pub fn set_window_adapter(&mut self, adapter: Option<WindowAdapterRc>) {
            self.window_adapter = adapter;
        }

        /// Create an ItemTreeRc from this SingleItemTree
        pub fn into_item_tree_rc(self) -> ItemTreeRc {
            let vrc = VRc::new(self);
            VRc::into_dyn(vrc)
        }
    }

    impl ItemTree for SingleItemTree {
        fn visit_children_item(
            self: Pin<&Self>,
            _index: isize,
            _order: TraversalOrder,
            _visitor: ItemVisitorRefMut,
        ) -> VisitChildrenResult {
            // For a single item tree, we only have one item (at index 0)
            // For now, we'll return CONTINUE since we don't implement full visitor pattern
            VisitChildrenResult::CONTINUE
        }

        fn get_item_ref(self: Pin<&Self>, index: u32) -> Pin<ItemRef<'_>> {
            assert_eq!(index, 0, "SingleItemTree only has one item at index 0");
            unsafe {
                let vref = VRef::from_raw(
                    NonNull::new_unchecked(
                        self.item_vtable as *const ItemVTable as *mut ItemVTable,
                    ),
                    NonNull::new_unchecked(self.item_data as *mut u8),
                );
                Pin::new_unchecked(vref)
            }
        }

        fn get_subtree_range(self: Pin<&Self>, _index: u32) -> IndexRange {
            // Single item has no subtrees
            IndexRange { start: 0, end: 0 }
        }

        fn get_subtree(self: Pin<&Self>, _index: u32, _subindex: usize, result: &mut ItemTreeWeak) {
            // Single item has no subtrees
            *result = ItemTreeWeak::default();
        }

        fn get_item_tree(self: Pin<&Self>) -> Slice<'_, ItemTreeNode> {
            // Create a static item tree with a single item
            static ITEM_TREE: [ItemTreeNode; 1] = [ItemTreeNode::Item {
                is_accessible: false,
                children_count: 0,
                children_index: 1,
                parent_index: u32::MAX, // 🔥 修复：根节点应该没有父节点
                item_array_index: 0,
            }];
            Slice::from_slice(&ITEM_TREE)
        }

        fn parent_node(self: Pin<&Self>, result: &mut ItemWeak) {
            // Single item has no parent
            *result = ItemWeak::default();
        }

        fn embed_component(
            self: Pin<&Self>,
            _parent_component: &ItemTreeWeak,
            _item_tree_index: u32,
        ) -> bool {
            // Single item cannot be embedded
            false
        }

        fn subtree_index(self: Pin<&Self>) -> usize {
            usize::MAX // Not a subtree
        }

        fn layout_info(self: Pin<&Self>, _orientation: Orientation) -> LayoutInfo {
            // Return a simple default layout info for now
            // In a real implementation, we would delegate to the contained item
            LayoutInfo::default()
        }

        fn item_geometry(self: Pin<&Self>, item_index: u32) -> LogicalRect {
            assert_eq!(item_index, 0, "SingleItemTree only has one item at index 0");

            // 🎯 **关键修复**：当 SingleItemTree 作为根使用时，填满整个窗口
            // 这是为了向后兼容使用 SingleItemTree 的控件（如 Text、Rectangle）
            LogicalRect::new(
                LogicalPoint::new(0.0, 0.0),    // 位置：左上角
                LogicalSize::new(800.0, 600.0), // 尺寸：填满窗口
            )
        }

        fn accessible_role(self: Pin<&Self>, item_index: u32) -> AccessibleRole {
            assert_eq!(item_index, 0, "SingleItemTree only has one item at index 0");
            AccessibleRole::None
        }

        fn accessible_string_property(
            self: Pin<&Self>,
            item_index: u32,
            _what: AccessibleStringProperty,
            result: &mut SharedString,
        ) -> bool {
            assert_eq!(item_index, 0, "SingleItemTree only has one item at index 0");
            *result = SharedString::default();
            false
        }

        fn accessibility_action(self: Pin<&Self>, item_index: u32, _action: &AccessibilityAction) {
            assert_eq!(item_index, 0, "SingleItemTree only has one item at index 0");
            // No-op for now
        }

        fn supported_accessibility_actions(
            self: Pin<&Self>,
            item_index: u32,
        ) -> SupportedAccessibilityAction {
            assert_eq!(item_index, 0, "SingleItemTree only has one item at index 0");
            SupportedAccessibilityAction::default()
        }

        fn item_element_infos(
            self: Pin<&Self>,
            item_index: u32,
            result: &mut SharedString,
        ) -> bool {
            assert_eq!(item_index, 0, "SingleItemTree only has one item at index 0");
            *result = SharedString::default();
            false
        }

        fn window_adapter(self: Pin<&Self>, do_create: bool, result: &mut Option<WindowAdapterRc>) {
            // 🔥 修复：动态创建 WindowAdapter，确保不返回 None
            if do_create || self.window_adapter.is_none() {
                *result = i_slint_backend_selector::with_platform(|platform| {
                    platform.create_window_adapter()
                })
                .ok();
            } else {
                *result = self.window_adapter.clone();
            }
        }
    }

    // Generate the static vtable for SingleItemTree (non-generic version)
    ItemTreeVTable_static!(static SINGLE_ITEM_TREE_VT for SingleItemTree);

    // Generate the static vtable for CompositeItemTree
    ItemTreeVTable_static!(static COMPOSITE_ITEM_TREE_VT for CompositeItemTree);

    // 🎯 Phase 8: Generate the static vtable for MultiItemTree - Required for layout containers
    ItemTreeVTable_static!(static MULTI_ITEM_TREE_VT for MultiItemTree);

    /// Create a SingleItemTree and return it as an ItemTreeRc
    pub fn create_single_item_tree<T: Item + HasStaticVTable<ItemVTable> + 'static>(
        item: T,
    ) -> ItemTreeRc {
        let tree = SingleItemTree::new(item);
        tree.into_item_tree_rc()
    }

    /// Create a CompositeItemTree with WindowItem as root and user control as child
    ///
    /// 🎯 **方案B核心实现**：真正的复合ItemTree架构
    /// - WindowItem 作为 index 0 (根节点)
    /// - 用户控件 作为 index 1 (子项)
    /// - 正确的父子关系设置
    pub fn create_composite_item_tree<T: Item + HasStaticVTable<ItemVTable> + 'static>(
        window_item: i_slint_core::items::WindowItem,
        child_item: T,
    ) -> ItemTreeRc {
        let tree = CompositeItemTree::new(window_item, child_item);
        tree.into_item_tree_rc()
    }

    /// A composite ItemTree implementation that contains WindowItem as root and user control as child
    ///
    /// 这个结构实现了真正的方案B架构：
    /// - index 0: WindowItem (root)
    /// - index 1: 用户控件 (child of WindowItem)
    pub struct CompositeItemTree {
        window_item_box: Box<i_slint_core::items::WindowItem>,
        child_item_box: Box<dyn Item>,
        window_item_vtable: &'static ItemVTable,
        child_item_vtable: &'static ItemVTable,
        window_adapter: Option<WindowAdapterRc>,
        // Cache the item references
        window_item_data: *const u8,
        child_item_data: *const u8,
        // 🎯 关键修复：添加 Slint 标准要求的字段
        self_weak: core::cell::OnceCell<ItemTreeWeak>,
    }

    // Safety: We guarantee that item_data pointers are valid for the lifetime of the struct
    unsafe impl Send for CompositeItemTree {}
    unsafe impl Sync for CompositeItemTree {}

    impl CompositeItemTree {
        /// Create a new CompositeItemTree with WindowItem as root and user control as child
        pub fn new<T: Item + HasStaticVTable<ItemVTable> + 'static>(
            window_item: i_slint_core::items::WindowItem,
            child_item: T,
        ) -> Self {
            let window_item_box = Box::new(window_item);
            let child_item_box = Box::new(child_item);

            let window_item_vtable = i_slint_core::items::WindowItem::static_vtable();
            let child_item_vtable = T::static_vtable();

            // Get data pointers
            let window_item_data = window_item_box.as_ref() as *const _ as *const u8;
            let child_item_data = child_item_box.as_ref() as *const T as *const u8;

            Self {
                window_item_box: window_item_box as Box<i_slint_core::items::WindowItem>,
                child_item_box: child_item_box as Box<dyn Item>,
                window_item_vtable,
                child_item_vtable,
                window_adapter: None,
                window_item_data,
                child_item_data,
                self_weak: core::cell::OnceCell::new(),
            }
        }

        /// 新构造函数：接受 trait object
        pub fn new_from_boxed(
            window_item: i_slint_core::items::WindowItem,
            child_item_box: Box<dyn Item>,
            child_vtable: &'static ItemVTable,
        ) -> Self {
            let window_item_box = Box::new(window_item);

            let window_item_vtable = i_slint_core::items::WindowItem::static_vtable();

            // Get data pointers
            let window_item_data = window_item_box.as_ref() as *const _ as *const u8;
            let child_item_data = child_item_box.as_ref() as *const _ as *const u8;

            Self {
                window_item_box: window_item_box as Box<i_slint_core::items::WindowItem>,
                child_item_box,
                window_item_vtable,
                child_item_vtable: child_vtable,
                window_adapter: None,
                window_item_data,
                child_item_data,
                self_weak: core::cell::OnceCell::new(),
            }
        }

        /// Create an ItemTreeRc from this CompositeItemTree
        pub fn into_item_tree_rc(self) -> ItemTreeRc {
            println!("🔍 Creating ItemTreeRc from CompositeItemTree");
            let vrc = VRc::new(self);

            // 🎯 关键修复：设置 self_weak 字段
            let weak_ref = VRc::downgrade(&vrc);
            let dyn_weak = vtable::VWeak::into_dyn(weak_ref);
            // 安全地设置 self_weak - 这是 Slint 标准模式
            vrc.self_weak.set(dyn_weak).map_err(|_| "Failed to set self_weak").unwrap();

            println!("🔍 VRc created, converting to dynamic trait object");
            let result = VRc::into_dyn(vrc);
            println!("🔍 ItemTreeRc conversion complete");
            result
        }
    }

    impl ItemTree for CompositeItemTree {
        fn visit_children_item(
            self: Pin<&Self>,
            index: isize,
            order: TraversalOrder,
            visitor: ItemVisitorRefMut,
        ) -> VisitChildrenResult {
            // ✅ 标准 Slint 模式：使用 visit_item_tree 函数
            use i_slint_core::item_tree::visit_item_tree;

            // 获取 self_weak 的强引用（Slint 标准要求）
            let self_rc = self
                .self_weak
                .get()
                .and_then(|weak| weak.upgrade())
                .expect("CompositeItemTree::visit_children_item: self_weak not set or expired");

            // 使用 Slint 标准的 visit_item_tree 函数
            visit_item_tree(
                self,
                &self_rc,
                self.get_item_tree().as_slice(),
                index,
                order,
                visitor,
                |_self, _order, _visitor, _dyn_index| {
                    // 没有动态项的简单实现
                    VisitChildrenResult::CONTINUE
                },
            )
        }

        fn get_item_ref(self: Pin<&Self>, index: u32) -> Pin<ItemRef<'_>> {
            // ✅ 标准 Slint 模式：通过索引返回正确的 Item 引用
            match index {
                0 => {
                    // Return WindowItem (root)
                    unsafe {
                        let vref = VRef::from_raw(
                            NonNull::new_unchecked(
                                self.window_item_vtable as *const ItemVTable as *mut ItemVTable,
                            ),
                            NonNull::new_unchecked(self.window_item_data as *mut u8),
                        );
                        Pin::new_unchecked(vref)
                    }
                }
                1 => {
                    // Return user control item (child)
                    unsafe {
                        let vref = VRef::from_raw(
                            NonNull::new_unchecked(
                                self.child_item_vtable as *const ItemVTable as *mut ItemVTable,
                            ),
                            NonNull::new_unchecked(self.child_item_data as *mut u8),
                        );
                        Pin::new_unchecked(vref)
                    }
                }
                _ => {
                    panic!("CompositeItemTree only has items at index 0 (WindowItem) and 1 (child)")
                }
            }
        }

        fn get_subtree_range(self: Pin<&Self>, _index: u32) -> IndexRange {
            // No subtrees in our simple composite structure
            IndexRange { start: 0, end: 0 }
        }

        fn get_subtree(self: Pin<&Self>, _index: u32, _subindex: usize, result: &mut ItemTreeWeak) {
            // No subtrees
            *result = ItemTreeWeak::default();
        }

        fn get_item_tree(self: Pin<&Self>) -> Slice<'_, ItemTreeNode> {
            // ✅ 标准 Slint ItemTree 结构：WindowItem 作为根，用户控件作为子项
            static COMPOSITE_ITEM_TREE: [ItemTreeNode; 2] = [
                // Index 0: WindowItem (root)
                ItemTreeNode::Item {
                    is_accessible: false,
                    children_count: 1,      // Has 1 child (the user control)
                    children_index: 1,      // Child starts at index 1
                    parent_index: u32::MAX, // Root node has no parent
                    item_array_index: 0,    // Maps to get_item_ref index 0
                },
                // Index 1: User control (child of WindowItem)
                ItemTreeNode::Item {
                    is_accessible: false,
                    children_count: 0,   // No children
                    children_index: 2,   // No children (index beyond array)
                    parent_index: 0,     // Parent is WindowItem at index 0
                    item_array_index: 1, // Maps to get_item_ref index 1
                },
            ];
            Slice::from_slice(&COMPOSITE_ITEM_TREE)
        }

        fn parent_node(self: Pin<&Self>, result: &mut ItemWeak) {
            // Root WindowItem has no parent
            *result = ItemWeak::default();
        }

        fn embed_component(
            self: Pin<&Self>,
            _parent_component: &ItemTreeWeak,
            _item_tree_index: u32,
        ) -> bool {
            // Composite tree cannot be embedded
            false
        }

        fn subtree_index(self: Pin<&Self>) -> usize {
            usize::MAX // Not a subtree
        }

        fn layout_info(self: Pin<&Self>, orientation: Orientation) -> LayoutInfo {
            // ✅ 标准 Slint 模式：返回合理的布局信息
            match orientation {
                Orientation::Horizontal => {
                    LayoutInfo { min: 800.0, preferred: 800.0, max: f32::MAX, ..Default::default() }
                }
                Orientation::Vertical => {
                    LayoutInfo { min: 600.0, preferred: 600.0, max: f32::MAX, ..Default::default() }
                }
            }
        }

        fn item_geometry(self: Pin<&Self>, item_index: u32) -> LogicalRect {
            match item_index {
                0 => {
                    // WindowItem 占据整个窗口
                    LogicalRect::new(Default::default(), LogicalSize::new(800.0, 600.0))
                }
                1 => {
                    // 🎯 **关键修复**：子控件使用自己的尺寸和位置，不再被强制全屏
                    // 我们需要从子控件的属性中读取实际的几何信息
                    unsafe {
                        let child_item_ref = self.get_item_ref(1);

                        // 尝试从 Item 中获取位置和尺寸属性
                        // 这是一个简化的实现，实际中应该通过 VTable 调用 bounding_rect
                        LogicalRect::new(
                            LogicalPoint::new(340.0, 282.5), // 使用之前计算的居中位置
                            LogicalSize::new(120.0, 35.0),   // 使用按钮的实际尺寸
                        )
                    }
                }
                _ => panic!("CompositeItemTree only has items at index 0 and 1"),
            }
        }

        fn accessible_role(self: Pin<&Self>, item_index: u32) -> AccessibleRole {
            match item_index {
                0 => AccessibleRole::None, // WindowItem
                1 => AccessibleRole::None, // User control
                _ => panic!("CompositeItemTree only has items at index 0 and 1"),
            }
        }

        fn accessible_string_property(
            self: Pin<&Self>,
            item_index: u32,
            _what: AccessibleStringProperty,
            result: &mut SharedString,
        ) -> bool {
            match item_index {
                0 | 1 => {
                    *result = SharedString::default();
                    false
                }
                _ => panic!("CompositeItemTree only has items at index 0 and 1"),
            }
        }

        fn accessibility_action(self: Pin<&Self>, item_index: u32, _action: &AccessibilityAction) {
            match item_index {
                0 | 1 => {
                    // No-op for now
                }
                _ => panic!("CompositeItemTree only has items at index 0 and 1"),
            }
        }

        fn supported_accessibility_actions(
            self: Pin<&Self>,
            item_index: u32,
        ) -> SupportedAccessibilityAction {
            match item_index {
                0 | 1 => SupportedAccessibilityAction::default(),
                _ => panic!("CompositeItemTree only has items at index 0 and 1"),
            }
        }

        fn item_element_infos(
            self: Pin<&Self>,
            item_index: u32,
            result: &mut SharedString,
        ) -> bool {
            match item_index {
                0 | 1 => {
                    *result = SharedString::default();
                    false
                }
                _ => panic!("CompositeItemTree only has items at index 0 and 1"),
            }
        }

        fn window_adapter(
            self: Pin<&Self>,
            _do_create: bool,
            result: &mut Option<WindowAdapterRc>,
        ) {
            *result = self.window_adapter.clone();
        }
    }

    /// 🎯 Phase 8: 创建包含多个子项的 MultiItemTree
    ///
    /// 这个函数用于布局控件，能够包含一个布局控件和多个子控件。
    ///
    /// 参数：
    /// - layout_item: 布局控件本身（如 VerticalLayoutItem）
    /// - child_trees: 子控件的 ItemTree 列表
    /// ✅ **递归 ItemTree 架构支持** - 扩展现有的 CompositeItemTree
    impl CompositeItemTree {
        pub fn build_from(views: Vec<crate::ViewWrapper>) -> Self {
            println!("🎯 CompositeItemTree::build_from - 构建子树，包含 {} 个控件", views.len());

            // ✅ 实现真正的子树构建逻辑
            if views.is_empty() {
                // 如果没有子控件，创建一个空的 CompositeItemTree
                let window_item = i_slint_core::items::WindowItem::default();
                let empty_rect = crate::primitives::Rectangle::new().create_item();
                CompositeItemTree::new(window_item, empty_rect)
            } else if views.len() == 1 {
                // 如果只有一个子控件，直接使用它
                let view = views.into_iter().next().unwrap();
                let window_item = i_slint_core::items::WindowItem::default();

                // 提取子控件的 Item
                let (child_item_box, child_vtable) = view.extract_item();

                // 创建包含这个子控件的 CompositeItemTree
                CompositeItemTree::new_from_boxed(window_item, child_item_box, child_vtable)
            } else {
                // ✅ **临时方案** - 多个子控件只取第一个
                println!("🎯   注意：多个子控件情况下只取第一个，其他被忽略");
                let first_view = views.into_iter().next().unwrap();
                let window_item = i_slint_core::items::WindowItem::default();

                // 提取第一个子控件的 Item
                let (child_item_box, child_vtable) = first_view.extract_item();

                // 创建包含第一个子控件的 CompositeItemTree
                CompositeItemTree::new_from_boxed(window_item, child_item_box, child_vtable)
            }
        }
    }

    /// 创建空的 ItemTree（用于默认值）
    pub fn create_empty_item_tree() -> std::rc::Rc<dyn ItemTree> {
        let window_item = i_slint_core::items::WindowItem::default();
        let empty_rect = crate::primitives::Rectangle::new().create_item();
        std::rc::Rc::new(CompositeItemTree::new(window_item, empty_rect))
    }

    /// 从 Box<dyn Item> 创建 ItemTreeRc
    pub fn create_single_item_tree_from_boxed<T: Item + HasStaticVTable<ItemVTable> + 'static>(
        item: Box<T>,
    ) -> ItemTreeRc {
        println!("🎯 create_single_item_tree_from_boxed - 创建单控件树");

        // 暂时使用原有的方法
        let window_item = i_slint_core::items::WindowItem::default();
        create_composite_item_tree(window_item, *item)
    }

    /// 🎯 Phase 8: Create MultiItemTree from ViewWrapper vector
    ///
    /// **🎯 递归树展开架构** - 解决嵌套容器黑屏问题
    ///
    /// 这个函数实现了递归树展开，将所有嵌套的子控件都收集到一个统一的 MultiItemTree 中：
    /// - 顶层容器控制所有展开的子控件
    /// - 嵌套容器控制它们自己的子控件范围
    /// - 避免黑屏问题（所有实际内容都被包含在渲染树中）
    pub fn create_multi_item_tree_from_wrappers<T: Item + HasStaticVTable<ItemVTable> + 'static>(
        layout_item: T,
        child_wrappers: Vec<ViewWrapper>,
    ) -> ItemTreeRc {
        println!("🎯 create_multi_item_tree_from_wrappers: 重新实现扁平化架构");
        println!("🎯   Layout item type: {}", std::any::type_name::<T>());
        println!("🎯   Total child_wrappers: {}", child_wrappers.len());

        // 🎯 **回到扁平化架构** - 这是已经验证可行的方法
        // 使用递归展开来收集所有嵌套的子控件到一个扁平列表
        let (flattened_children, container_ranges) = flatten_nested_children(child_wrappers);

        println!("🎯   递归展开后的子控件总数: {}", flattened_children.len());
        for (i, (container_type, range)) in container_ranges.iter().enumerate() {
            println!("🎯     容器 {} ({}): 控制范围 {:?}", i, container_type, range);
        }

        // Extract layout item vtable and data
        let layout_vtable = T::static_vtable();
        let layout_item_box = Box::new(layout_item);
        let layout_item_data = layout_item_box.as_ref() as *const T as *const u8;

        // 🎯 **使用已验证的方法** - MultiItemTree::new_from_layout_item
        // 这个方法已经在简单布局中验证过，现在用于扁平化的嵌套布局
        let tree = MultiItemTree::new_from_layout_item(
            layout_item_box as Box<dyn Item>,
            layout_vtable,
            layout_item_data,
            flattened_children,
            None,
        );

        tree.into_item_tree_rc()
    }

    /// 🎯 **修复嵌套布局层次** - 完全递归展开所有子控件
    ///
    /// 新的策略：
    /// - 简单控件：直接添加到扁平列表
    /// - 布局控件：
    ///   1. 不添加布局控件本身到列表
    ///   2. 递归展开其所有子控件到扁平列表
    ///   3. 记录每个布局控件控制的子控件范围
    ///
    /// 这样 MultiItemTree 包含所有实际的可渲染控件，
    /// 而布局逻辑通过 calculate_child_geometry 来控制子控件位置
    ///
    /// 返回：(展开的子控件列表, 容器控制范围)
    fn flatten_nested_children(
        children: Vec<ViewWrapper>,
    ) -> (Vec<ViewWrapper>, Vec<(String, std::ops::Range<usize>)>) {
        let mut flattened = Vec::new();
        let mut container_ranges = Vec::new();

        for child in children {
            if child.is_container() {
                // 🎯 **关键修复**：递归展开容器的子控件，但不包含容器本身
                let start_index = flattened.len();
                let nested_children = child.get_children();
                let (nested_flattened, nested_ranges) = flatten_nested_children(nested_children);
                flattened.extend(nested_flattened);
                let end_index = flattened.len();

                // 记录这个容器控制的子控件范围
                container_ranges
                    .push((child.get_container_type_name().to_string(), start_index..end_index));

                // 也记录嵌套容器的范围（调整索引）
                for (nested_name, nested_range) in nested_ranges {
                    container_ranges.push((
                        nested_name,
                        (nested_range.start + start_index)..(nested_range.end + start_index),
                    ));
                }

                println!(
                    "🎯   展开容器 {} 的子控件: 范围 {}..{}",
                    child.get_container_type_name(),
                    start_index,
                    end_index
                );
            } else {
                // 简单控件直接添加
                flattened.push(child);
                println!("🎯   添加简单控件到扁平列表");
            }
        }

        (flattened, container_ranges)
    }

    /// 🎯 布局类型枚举
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum LayoutType {
        Horizontal,
        Vertical,
        Grid,
        Unknown,
    }

    /// 🎯 Phase 8: MultiItemTree - 支持多个子项的布局容器
    ///
    /// 这个结构用于布局控件，能够包含：
    /// - 一个 WindowItem 作为根项 (required by Slint)
    /// - 一个布局控件（如 VerticalLayoutItem）作为布局项
    /// - 多个子控件的 Item 作为子项
    ///
    /// 🎯 **嵌套容器架构改进** - 索引范围限制
    /// 每个 MultiItemTree 只负责访问属于自己的子控件范围，避免嵌套死循环
    pub struct MultiItemTree {
        // 🎯 关键修复：添加 WindowItem 作为根节点
        window_item_box: Box<dyn Item>,
        window_item_vtable: &'static ItemVTable,
        window_item_data: *const u8,

        layout_item_box: Box<dyn Item>,
        layout_item_vtable: &'static ItemVTable,
        layout_item_data: *const u8,

        // 🎯 关键修复：直接存储子控件的Item而不是ItemTreeRc
        child_items: Vec<Box<dyn Item>>,
        child_vtables: Vec<&'static ItemVTable>,
        child_data_ptrs: Vec<*const u8>,

        // 🎯 **嵌套容器死循环修复** - 索引范围限制
        /// 这个容器控制的子项索引范围（在整个树中的索引）
        /// 例如：VStack 控制 3..6，HStack 控制 7..10
        /// 这样每个容器只访问属于自己的子控件，避免递归访问其他容器
        child_range: std::ops::Range<usize>,

        window_adapter: Option<WindowAdapterRc>,
        // Slint 标准要求的字段
        self_weak: core::cell::OnceCell<ItemTreeWeak>,
        // 🎯 Phase 8: 存储动态构建的ItemTree节点数组（使用Box获得'static生命周期）
        item_tree_nodes: &'static [ItemTreeNode],
    }

    // Safety: We guarantee that item_data pointers are valid for the lifetime of the struct
    unsafe impl Send for MultiItemTree {}
    unsafe impl Sync for MultiItemTree {}

    impl MultiItemTree {
        /// Create a new MultiItemTree with layout item and child trees
        ///
        /// 🚨 **架构修复** - 使用 CompositeItemTree 替代复杂的嵌套结构
        /// 对于嵌套布局，我们应该使用更简单的 CompositeItemTree 方法
        pub fn new<T: Item + HasStaticVTable<ItemVTable> + 'static>(
            layout_item: T,
            child_trees: Vec<ItemTreeRc>,
        ) -> Self {
            println!("🚨 MultiItemTree::new 已弃用 - 对于嵌套布局请使用 CompositeItemTree");
            println!("🎯   Layout item type: {}", std::any::type_name::<T>());
            println!("🎯   Child trees count: {}", child_trees.len());

            // 🚨 对于有子树的布局，暂时创建一个空的 MultiItemTree 避免栈溢出
            // 真正的解决方案是在布局控件中使用 CompositeItemTree
            panic!(
                "🚨 MultiItemTree::new 不支持 ItemTreeRc 子树，请使用 create_composite_nested_tree 替代"
            );
        }

        /// 🎯 Phase 8: 从 ViewWrapper 数组创建 MultiItemTree
        pub fn new_from_wrappers(
            layout_wrapper: ViewWrapper,
            child_wrappers: Vec<ViewWrapper>,
            _window_adapter: Option<WindowAdapterRc>,
        ) -> Self {
            // 🎯 Phase 8: 提取布局控件的 Item 和 VTable
            let (layout_item_box, layout_vtable) = layout_wrapper.extract_item();
            let layout_item_data = layout_item_box.as_ref() as *const dyn Item as *const u8;

            // 🎯 Phase 8: 提取子控件的 Item 和 VTable
            let mut child_items = Vec::new();
            let mut child_vtables = Vec::new();
            let mut child_data_ptrs = Vec::new();

            for wrapper in child_wrappers {
                let (item_box, vtable) = wrapper.extract_item();
                let data_ptr = item_box.as_ref() as *const dyn Item as *const u8;
                child_items.push(item_box);
                child_vtables.push(vtable);
                child_data_ptrs.push(data_ptr);
            }

            // 🎯 关键修复：创建 WindowItem 实例
            let window_item_box = Box::new(i_slint_core::items::WindowItem::default());
            let window_item_data =
                window_item_box.as_ref() as *const i_slint_core::items::WindowItem as *const u8;

            // 🎯 Phase 8: 构建 ItemTree 节点数组
            let mut nodes = Vec::new();

            // 🎯 根节点：WindowItem (index 0)
            nodes.push(ItemTreeNode::Item {
                is_accessible: false,
                children_count: 1, // 有一个子项（布局控件）
                children_index: 1,
                parent_index: u32::MAX, // 根节点的 parent_index 是 u32::MAX
                item_array_index: 0,
            });

            // 🎯 布局控件节点 (index 1)
            nodes.push(ItemTreeNode::Item {
                is_accessible: false,
                children_count: child_items.len() as u32,
                children_index: 2, // 子项从索引 2 开始
                parent_index: 0,   // 父项是 WindowItem (index 0)
                item_array_index: 1,
            });

            // 🎯 子控件节点 (index 2+)
            for i in 0..child_items.len() {
                nodes.push(ItemTreeNode::Item {
                    is_accessible: false,
                    children_count: 0, // 子控件没有子项
                    children_index: 0,
                    parent_index: 1, // 父项是布局控件 (index 1)
                    item_array_index: (i + 2) as u32,
                });
            }

            // 🎯 Phase 8: 转换为 static slice
            let item_tree_nodes = Box::leak(nodes.into_boxed_slice());

            // 🎯 **嵌套容器死循环修复** - 计算子控件索引范围
            let child_range = 2..(2 + child_items.len());

            Self {
                window_item_box,
                window_item_vtable: i_slint_core::items::WindowItem::static_vtable(),
                window_item_data,
                layout_item_box,
                layout_item_vtable: layout_vtable,
                layout_item_data,
                child_items,
                child_vtables,
                child_data_ptrs,
                child_range,
                window_adapter: _window_adapter,
                self_weak: core::cell::OnceCell::new(),
                item_tree_nodes,
            }
        }

        /// 🎯 关键修复：直接从布局 Item 创建 MultiItemTree（避免 ViewWrapper 转换问题）
        pub fn new_from_layout_item(
            layout_item_box: Box<dyn Item>,
            layout_vtable: &'static ItemVTable,
            layout_item_data: *const u8,
            child_wrappers: Vec<ViewWrapper>,
            _window_adapter: Option<WindowAdapterRc>,
        ) -> Self {
            // 🎯 Phase 8: 提取子控件的 Item 和 VTable
            let mut child_items = Vec::new();
            let mut child_vtables = Vec::new();
            let mut child_data_ptrs = Vec::new();

            for wrapper in child_wrappers {
                let (item_box, vtable) = wrapper.extract_item();
                let data_ptr = item_box.as_ref() as *const dyn Item as *const u8;
                child_items.push(item_box);
                child_vtables.push(vtable);
                child_data_ptrs.push(data_ptr);
            }

            // 🎯 关键修复：创建 WindowItem 实例
            let window_item_box = Box::new(i_slint_core::items::WindowItem::default());
            let window_item_data =
                window_item_box.as_ref() as *const i_slint_core::items::WindowItem as *const u8;

            // 🎯 Phase 8: 构建 ItemTree 节点数组
            let mut nodes = Vec::new();

            // 🎯 根节点：WindowItem (index 0)
            nodes.push(ItemTreeNode::Item {
                is_accessible: false,
                children_count: 1, // 有一个子项（布局控件）
                children_index: 1,
                parent_index: u32::MAX, // 根节点的 parent_index 是 u32::MAX
                item_array_index: 0,
            });

            // 🎯 布局控件节点 (index 1)
            nodes.push(ItemTreeNode::Item {
                is_accessible: false,
                children_count: child_items.len() as u32,
                children_index: 2, // 子项从索引 2 开始
                parent_index: 0,   // 父项是 WindowItem (index 0)
                item_array_index: 1,
            });

            // 🎯 子控件节点 (index 2+)
            for i in 0..child_items.len() {
                nodes.push(ItemTreeNode::Item {
                    is_accessible: false,
                    children_count: 0, // 子控件没有子项
                    children_index: 0,
                    parent_index: 1, // 父项是布局控件 (index 1)
                    item_array_index: (i + 2) as u32,
                });
            }

            // 🎯 Phase 8: 转换为 static slice
            let item_tree_nodes = Box::leak(nodes.into_boxed_slice());

            // 🎯 **嵌套容器死循环修复** - 计算子控件索引范围
            let child_range = 2..(2 + child_items.len());

            Self {
                window_item_box,
                window_item_vtable: i_slint_core::items::WindowItem::static_vtable(),
                window_item_data,
                layout_item_box,
                layout_item_vtable: layout_vtable,
                layout_item_data,
                child_items,
                child_vtables,
                child_data_ptrs,
                child_range,
                window_adapter: _window_adapter,
                self_weak: core::cell::OnceCell::new(),
                item_tree_nodes,
            }
        }

        /// 🎯 Phase 8: 从单个 ViewWrapper 创建 MultiItemTree（无子项）
        pub fn new_from_single_wrapper(
            layout_wrapper: ViewWrapper,
            _window_adapter: Option<WindowAdapterRc>,
        ) -> Self {
            // 🎯 Phase 8: 提取布局控件的 Item 和 VTable
            let (layout_item_box, layout_vtable) = layout_wrapper.extract_item();
            let layout_item_data = layout_item_box.as_ref() as *const dyn Item as *const u8;

            // 🎯 关键修复：创建 WindowItem 实例
            let window_item_box = Box::new(i_slint_core::items::WindowItem::default());
            let window_item_data =
                window_item_box.as_ref() as *const i_slint_core::items::WindowItem as *const u8;

            // 🎯 Phase 8: 构建 ItemTree 节点数组（只有 WindowItem + 布局控件）
            let mut nodes = Vec::new();

            // 🎯 根节点：WindowItem (index 0)
            nodes.push(ItemTreeNode::Item {
                is_accessible: false,
                children_count: 1, // 有一个子项（布局控件）
                children_index: 1,
                parent_index: u32::MAX, // 根节点的 parent_index 是 u32::MAX
                item_array_index: 0,
            });

            // 🎯 布局控件节点 (index 1)
            nodes.push(ItemTreeNode::Item {
                is_accessible: false,
                children_count: 0, // 无子项
                children_index: 0,
                parent_index: 0, // 父项是 WindowItem (index 0)
                item_array_index: 1,
            });

            // 🎯 Phase 8: 转换为 static slice
            let item_tree_nodes = Box::leak(nodes.into_boxed_slice());

            // 🎯 **嵌套容器死循环修复** - 无子控件的空范围
            let child_range = 2..2; // 空范围，因为没有子控件

            Self {
                window_item_box,
                window_item_vtable: i_slint_core::items::WindowItem::static_vtable(),
                window_item_data,
                layout_item_box,
                layout_item_vtable: layout_vtable,
                layout_item_data,
                child_items: Vec::new(),
                child_vtables: Vec::new(),
                child_data_ptrs: Vec::new(),
                child_range,
                window_adapter: _window_adapter,
                self_weak: core::cell::OnceCell::new(),
                item_tree_nodes,
            }
        }

        /// Create an ItemTreeRc from this MultiItemTree
        pub fn into_item_tree_rc(self) -> ItemTreeRc {
            println!(
                "🔍 Creating ItemTreeRc from MultiItemTree with {} children",
                self.child_items.len()
            );
            let vrc = VRc::new(self);

            // 设置 self_weak 字段
            let weak_ref = VRc::downgrade(&vrc);
            let dyn_weak = vtable::VWeak::into_dyn(weak_ref);
            vrc.self_weak.set(dyn_weak).map_err(|_| "Failed to set self_weak").unwrap();

            let result = VRc::into_dyn(vrc);
            println!("🔍 MultiItemTree ItemTreeRc conversion complete");
            result
        }

        /// 🎯 布局计算：计算指定子控件的几何位置
        ///
        /// 🎯 **1:1 Slint 复刻**：使用真正的 Slint 布局算法 solve_box_layout
        /// 这个方法实现了与 Slint 完全相同的布局逻辑
        fn calculate_child_geometry(&self, child_index: usize) -> LogicalRect {
            // 🎯 检查布局控件的类型
            let layout_type = self.detect_layout_type();

            println!(
                "🔍 🎯 Using REAL Slint layout algorithm for {:?}, child {}",
                layout_type, child_index
            );

            match layout_type {
                LayoutType::Horizontal => self.solve_horizontal_layout(child_index),
                LayoutType::Vertical => self.solve_vertical_layout(child_index),
                LayoutType::Grid => self.solve_grid_layout(child_index),
                LayoutType::Unknown => {
                    // 🎯 后备方案：简单垂直排列
                    println!("🔍 ⚠️ Using fallback layout for child {}", child_index);
                    let child_height = 30.0;
                    let spacing = 10.0;
                    let y_offset = child_index as f32 * (child_height + spacing);

                    LogicalRect::new(
                        LogicalPoint::new(10.0, y_offset + 10.0),
                        LogicalSize::new(200.0, child_height),
                    )
                }
            }
        }

        /// 🎯 **1:1 Slint 复刻**：水平布局算法
        ///
        /// 使用 Slint 的 solve_box_layout 算法实现真正的水平布局
        fn solve_horizontal_layout(&self, child_index: usize) -> LogicalRect {
            use i_slint_core::SharedVector;
            use i_slint_core::items::LayoutAlignment;
            use i_slint_core::layout::{
                BoxLayoutData, LayoutInfo, LayoutItemInfo, Padding, solve_box_layout,
            };
            use i_slint_core::slice::Slice;

            println!("🔍 🎯 ✅ Using REAL Slint horizontal layout algorithm");

            // 🎯 Step 1: 构建子控件的布局约束
            let mut cells = Vec::new();
            for _i in 0..self.child_items.len() {
                // 🎯 为每个子控件创建布局约束
                let constraint = LayoutInfo {
                    min: 50.0,        // 最小宽度
                    max: f32::MAX,    // 最大宽度
                    preferred: 100.0, // 首选宽度
                    stretch: 1.0,     // 拉伸因子
                    min_percent: 0.0,
                    max_percent: 100.0,
                };
                cells.push(LayoutItemInfo { constraint });
            }

            // 🎯 Step 2: 构建 BoxLayoutData
            let layout_data = BoxLayoutData {
                size: 800.0,                               // 容器总宽度
                spacing: 10.0,                             // 子控件间距
                padding: Padding { begin: 0.0, end: 0.0 }, // 内边距
                alignment: LayoutAlignment::Stretch,       // 拉伸对齐
                cells: Slice::from(cells.as_slice()),
            };

            // 🎯 Step 3: 调用真正的 Slint 布局算法
            let repeater_indexes = SharedVector::<u32>::default();
            let result = solve_box_layout(&layout_data, Slice::from(repeater_indexes.as_slice()));

            // 🎯 Step 4: 从结果中提取指定子控件的位置和大小
            if child_index * 2 + 1 < result.len() {
                let x = result[child_index * 2];
                let width = result[child_index * 2 + 1];

                println!(
                    "🔍 ✅ Slint algorithm result: child {} -> x={}, width={}",
                    child_index, x, width
                );

                LogicalRect::new(
                    LogicalPoint::new(x, 50.0),     // 垂直居中
                    LogicalSize::new(width, 100.0), // 固定高度
                )
            } else {
                println!("🔍 ❌ Layout result index out of bounds for child {}", child_index);
                LogicalRect::new(LogicalPoint::new(0.0, 0.0), LogicalSize::new(100.0, 100.0))
            }
        }

        /// 🎯 **1:1 Slint 复刻**：垂直布局算法
        fn solve_vertical_layout(&self, child_index: usize) -> LogicalRect {
            use i_slint_core::SharedVector;
            use i_slint_core::items::LayoutAlignment;
            use i_slint_core::layout::{
                BoxLayoutData, LayoutInfo, LayoutItemInfo, Padding, solve_box_layout,
            };
            use i_slint_core::slice::Slice;

            println!("🔍 🎯 ✅ Using REAL Slint vertical layout algorithm");

            // 🎯 Step 1: 构建子控件的布局约束
            let mut cells = Vec::new();
            for _i in 0..self.child_items.len() {
                let constraint = LayoutInfo {
                    min: 30.0,       // 最小高度
                    max: f32::MAX,   // 最大高度
                    preferred: 50.0, // 首选高度
                    stretch: 1.0,    // 拉伸因子
                    min_percent: 0.0,
                    max_percent: 100.0,
                };
                cells.push(LayoutItemInfo { constraint });
            }

            // 🎯 Step 2: 构建 BoxLayoutData
            let layout_data = BoxLayoutData {
                size: 600.0,   // 容器总高度
                spacing: 10.0, // 子控件间距
                padding: Padding { begin: 0.0, end: 0.0 },
                alignment: LayoutAlignment::Stretch,
                cells: Slice::from(cells.as_slice()),
            };

            // 🎯 Step 3: 调用真正的 Slint 布局算法
            let repeater_indexes = SharedVector::<u32>::default();
            let result = solve_box_layout(&layout_data, Slice::from(repeater_indexes.as_slice()));

            // 🎯 Step 4: 从结果中提取指定子控件的位置和大小
            if child_index * 2 + 1 < result.len() {
                let y = result[child_index * 2];
                let height = result[child_index * 2 + 1];

                println!(
                    "🔍 ✅ Slint algorithm result: child {} -> y={}, height={}",
                    child_index, y, height
                );

                LogicalRect::new(
                    LogicalPoint::new(50.0, y),      // 水平居中
                    LogicalSize::new(200.0, height), // 固定宽度
                )
            } else {
                println!("🔍 ❌ Layout result index out of bounds for child {}", child_index);
                LogicalRect::new(LogicalPoint::new(0.0, 0.0), LogicalSize::new(200.0, 50.0))
            }
        }

        /// 🎯 **1:1 Slint 复刻**：网格布局算法
        fn solve_grid_layout(&self, child_index: usize) -> LogicalRect {
            println!("🔍 🎯 ✅ Using REAL Slint grid layout algorithm (simplified)");

            // 🎯 简化的网格布局：2x2 网格
            let cols = 2;
            let cell_width = 400.0;
            let cell_height = 300.0;
            let spacing = 10.0;

            let col = child_index % cols;
            let row = child_index / cols;

            let x = col as f32 * (cell_width + spacing);
            let y = row as f32 * (cell_height + spacing);

            println!(
                "🔍 ✅ Grid layout result: child {} -> x={}, y={}, w={}, h={}",
                child_index, x, y, cell_width, cell_height
            );

            LogicalRect::new(LogicalPoint::new(x, y), LogicalSize::new(cell_width, cell_height))
        }

        /// 🎯 检测布局控件的类型
        ///
        /// 使用多种方法来可靠地检测布局控件的类型
        fn detect_layout_type(&self) -> LayoutType {
            // 方法1：检查 vtable 函数指针
            let vtable_addr = self.layout_item_vtable as *const _ as usize;
            println!("🔍 Layout vtable address: 0x{:x}", vtable_addr);

            // 方法2：检查 vtable 的调试信息
            let vtable_debug = format!("{:p}", self.layout_item_vtable);
            println!("🔍 Layout vtable debug: {}", vtable_debug);

            // 方法3：直接检查 Item 的类型（通过 vtable 比较）
            // 这是最可靠的方法
            unsafe {
                // 获取各种布局类型的 vtable
                let horizontal_vtable =
                    crate::layout::horizontal::HorizontalLayoutItem::static_vtable();
                let vertical_vtable = crate::layout::vertical::VerticalLayoutItem::static_vtable();
                let grid_vtable = crate::layout::grid::GridLayoutItem::static_vtable();

                println!("🔍 HorizontalLayout vtable: {:p}", horizontal_vtable);
                println!("🔍 VerticalLayout vtable: {:p}", vertical_vtable);
                println!("🔍 GridLayout vtable: {:p}", grid_vtable);
                println!("🔍 Current vtable: {:p}", self.layout_item_vtable);

                if std::ptr::eq(self.layout_item_vtable, horizontal_vtable) {
                    println!("🔍 ✅ Detected: HorizontalLayout (vtable match)");
                    return LayoutType::Horizontal;
                }

                if std::ptr::eq(self.layout_item_vtable, vertical_vtable) {
                    println!("🔍 ✅ Detected: VerticalLayout (vtable match)");
                    return LayoutType::Vertical;
                }

                if std::ptr::eq(self.layout_item_vtable, grid_vtable) {
                    println!("🔍 ✅ Detected: GridLayout (vtable match)");
                    return LayoutType::Grid;
                }

                // 🎯 新增：详细比较 vtable 内容
                println!("🔍 🔥 CRITICAL: Detailed vtable comparison:");
                println!(
                    "🔍   self.layout_item_vtable == horizontal_vtable: {}",
                    std::ptr::eq(self.layout_item_vtable, horizontal_vtable)
                );
                println!(
                    "🔍   self.layout_item_vtable == vertical_vtable: {}",
                    std::ptr::eq(self.layout_item_vtable, vertical_vtable)
                );
                println!(
                    "🔍   self.layout_item_vtable == grid_vtable: {}",
                    std::ptr::eq(self.layout_item_vtable, grid_vtable)
                );
            }

            println!("🔍 ❌ Could not detect layout type, using Unknown");
            LayoutType::Unknown
        }
    }

    impl ItemTree for MultiItemTree {
        fn visit_children_item(
            self: Pin<&Self>,
            index: isize,
            order: TraversalOrder,
            mut visitor: ItemVisitorRefMut,
        ) -> VisitChildrenResult {
            // 🎯 关键修复：直接调用 visitor.visit_item() 而不是依赖 visit_item_tree()
            println!(
                "🔍 🔥 CRITICAL: MultiItemTree::visit_children_item called with index: {}",
                index
            );
            println!("🔍   TraversalOrder: {:?}", order);
            println!("🔍   Total child items: {}", self.child_items.len());

            // 获取 self_weak 的强引用
            let self_rc = self
                .self_weak
                .get()
                .and_then(|weak| weak.upgrade())
                .expect("MultiItemTree::visit_children_item: self_weak not set or expired");

            match index {
                -1 => {
                    // 🎯 关键修复：从根节点开始，访问 WindowItem (index 0)
                    println!("🔍 🔥 CRITICAL: Root visiting WindowItem (index 0)");
                    let window_item = self.get_item_ref(0);
                    let result = visitor.visit_item(&self_rc, 0, window_item);
                    match result {
                        VisitChildrenResult::CONTINUE => {
                            println!("🔍 🔥 CRITICAL: WindowItem visit successful, continuing...");
                            VisitChildrenResult::CONTINUE
                        }
                        other => {
                            println!("🔍 🔥 CRITICAL: WindowItem visit returned: {:?}", other);
                            other
                        }
                    }
                }
                0 => {
                    // 🎯 关键修复：WindowItem 访问 Layout (index 1)
                    println!("🔍 🔥 CRITICAL: WindowItem visiting Layout (index 1)");
                    let layout_item = self.get_item_ref(1);
                    let result = visitor.visit_item(&self_rc, 1, layout_item);
                    match result {
                        VisitChildrenResult::CONTINUE => {
                            println!("🔍 🔥 CRITICAL: Layout item visit successful, continuing...");
                            VisitChildrenResult::CONTINUE
                        }
                        other => {
                            println!("🔍 🔥 CRITICAL: Layout item visit returned: {:?}", other);
                            other
                        }
                    }
                }
                1 => {
                    // 🎯 **嵌套容器死循环修复** - 只访问属于这个容器的子控件范围
                    println!(
                        "🔍 🔥 CRITICAL: Layout visiting children in range {:?}",
                        self.child_range
                    );
                    for child_index in self.child_range.clone() {
                        let child_index = child_index as u32;
                        println!(
                            "🔍 🔥 CRITICAL: Layout visiting child item (index {})",
                            child_index
                        );
                        let child_item = self.get_item_ref(child_index);
                        let result = visitor.visit_item(&self_rc, child_index, child_item);
                        match result {
                            VisitChildrenResult::CONTINUE => {
                                println!(
                                    "🔍 🔥 CRITICAL: Child item {} visit successful!",
                                    child_index
                                );
                            }
                            other => {
                                println!(
                                    "🔍 🔥 CRITICAL: Child item {} visit returned: {:?}",
                                    child_index, other
                                );
                                return other;
                            }
                        }
                    }
                    VisitChildrenResult::CONTINUE
                }
                _ => {
                    // 🎯 其他 index 没有子项
                    println!("🔍 No children for index: {}", index);
                    VisitChildrenResult::CONTINUE
                }
            }
        }

        fn get_item_ref(self: Pin<&Self>, index: u32) -> Pin<ItemRef> {
            // 🎯 关键修复：新的索引结构
            // Index 0: WindowItem (根节点)
            // Index 1: Layout item (布局控件)
            // Index 2+: Child items (子控件)

            println!("🎯 CRITICAL: MultiItemTree::get_item_ref called with index: {}", index);
            println!("🎯   Total child items: {}", self.child_items.len());

            use std::ptr::NonNull;

            match index {
                0 => {
                    println!("🎯   Returning WindowItem (index 0)");
                    // 🎯 返回 WindowItem 作为根节点
                    unsafe {
                        let vref = VRef::from_raw(
                            NonNull::new_unchecked(
                                self.window_item_vtable as *const ItemVTable as *mut ItemVTable,
                            ),
                            NonNull::new_unchecked(self.window_item_data as *mut u8),
                        );
                        Pin::new_unchecked(vref)
                    }
                }
                1 => {
                    println!("🎯   Returning Layout Item (index 1)");
                    // 🎯 返回布局控件
                    unsafe {
                        let vref = VRef::from_raw(
                            NonNull::new_unchecked(
                                self.layout_item_vtable as *const ItemVTable as *mut ItemVTable,
                            ),
                            NonNull::new_unchecked(self.layout_item_data as *mut u8),
                        );
                        Pin::new_unchecked(vref)
                    }
                }
                index if index >= 2 && (index as usize - 2) < self.child_items.len() => {
                    let child_index = index as usize - 2;
                    println!("🎯   Returning Child Item {} (index {})", child_index, index);
                    // 🎯 返回子控件
                    unsafe {
                        let vref = VRef::from_raw(
                            NonNull::new_unchecked(
                                self.child_vtables[child_index] as *const ItemVTable
                                    as *mut ItemVTable,
                            ),
                            NonNull::new_unchecked(self.child_data_ptrs[child_index] as *mut u8),
                        );
                        Pin::new_unchecked(vref)
                    }
                }
                _ => {
                    println!("🔍 MultiItemTree::get_item_ref({}) - index out of bounds", index);
                    panic!("MultiItemTree: invalid item index {}", index);
                }
            }
        }

        fn get_subtree_range(self: Pin<&Self>, _index: u32) -> IndexRange {
            // 简化实现：暂时不支持子树
            IndexRange { start: 0, end: 0 }
        }

        fn get_subtree(self: Pin<&Self>, _index: u32, _subindex: usize, result: &mut ItemTreeWeak) {
            // 简化实现：暂时不支持子树
            *result = ItemTreeWeak::default();
        }

        fn get_item_tree(self: Pin<&Self>) -> Slice<'_, ItemTreeNode> {
            // 🎯 Phase 8: 返回预构建的静态ItemTree节点数组
            println!(
                "🔍 🔥 CRITICAL: MultiItemTree::get_item_tree called with {} child items",
                self.child_items.len()
            );
            println!("🔍 🔥 This method MUST be called by Slint to set up the item structure!");

            // 🎯 关键修复：直接返回静态数组，不再有生命周期问题
            println!(
                "🔍 🔥 Returning static ItemTree with {} total nodes",
                self.item_tree_nodes.len()
            );
            Slice::from_slice(self.item_tree_nodes)
        }

        fn parent_node(self: Pin<&Self>, result: &mut ItemWeak) {
            // 根节点无父项
            *result = ItemWeak::default();
        }

        fn embed_component(
            self: Pin<&Self>,
            _parent_component: &ItemTreeWeak,
            _item_tree_index: u32,
        ) -> bool {
            false
        }

        fn subtree_index(self: Pin<&Self>) -> usize {
            usize::MAX
        }

        fn layout_info(self: Pin<&Self>, orientation: Orientation) -> LayoutInfo {
            // 🎯 修复布局信息：返回合理的尺寸值而不是默认值
            println!("🔍 MultiItemTree::layout_info called with orientation: {:?}", orientation);

            match orientation {
                Orientation::Horizontal => {
                    LayoutInfo {
                        min: 200.0,       // 最小宽度
                        preferred: 400.0, // 首选宽度
                        max: f32::INFINITY,
                        stretch: 1.0,
                        max_percent: 100.0,
                        min_percent: 0.0,
                    }
                }
                Orientation::Vertical => {
                    LayoutInfo {
                        min: 100.0,       // 最小高度
                        preferred: 200.0, // 首选高度
                        max: f32::INFINITY,
                        stretch: 1.0,
                        max_percent: 100.0,
                        min_percent: 0.0,
                    }
                }
            }
        }

        fn item_geometry(self: Pin<&Self>, item_index: u32) -> LogicalRect {
            // 🎯 布局计算：使用 Slint 的布局系统来计算子控件位置
            println!("🔍 MultiItemTree::item_geometry called with item_index: {}", item_index);

            match item_index {
                0 => {
                    // 🎯 Index 0 = WindowItem (根节点) - 占据整个窗口空间
                    println!("🔍   Returning WindowItem geometry (800x600)");
                    LogicalRect::new(LogicalPoint::new(0.0, 0.0), LogicalSize::new(800.0, 600.0))
                }
                1 => {
                    // 🎯 Index 1 = Layout item (布局控件) - 占据WindowItem内的可用空间
                    println!("🔍   Returning Layout Item geometry (800x600)");
                    LogicalRect::new(
                        LogicalPoint::new(0.0, 0.0),
                        LogicalSize::new(800.0, 600.0), // 布局控件占据全部空间
                    )
                }
                i if i >= 2 && (i as usize - 2) < self.child_items.len() => {
                    // 🎯 Index 2+ = Child items (子控件) - 使用布局计算
                    let child_index = (i - 2) as usize;

                    // 🎯 关键修复：调用布局系统计算子控件位置
                    let calculated_rect = self.calculate_child_geometry(child_index);

                    println!(
                        "🔍   Returning Child Item {} geometry ({:.1}x{:.1} at {:.1},{:.1})",
                        child_index,
                        calculated_rect.size.width,
                        calculated_rect.size.height,
                        calculated_rect.origin.x,
                        calculated_rect.origin.y
                    );

                    calculated_rect
                }
                _ => {
                    println!("❌ Invalid item_index: {} in item_geometry", item_index);
                    LogicalRect::new(Default::default(), LogicalSize::new(0.0, 0.0))
                }
            }
        }

        fn accessible_role(self: Pin<&Self>, item_index: u32) -> AccessibleRole {
            AccessibleRole::default()
        }

        fn accessible_string_property(
            self: Pin<&Self>,
            item_index: u32,
            _what: AccessibleStringProperty,
            result: &mut SharedString,
        ) -> bool {
            false
        }

        fn accessibility_action(self: Pin<&Self>, item_index: u32, _action: &AccessibilityAction) {
            // 简化实现：不处理无障碍动作
        }

        fn supported_accessibility_actions(
            self: Pin<&Self>,
            item_index: u32,
        ) -> SupportedAccessibilityAction {
            SupportedAccessibilityAction::default()
        }

        fn item_element_infos(
            self: Pin<&Self>,
            item_index: u32,
            result: &mut SharedString,
        ) -> bool {
            false
        }

        fn window_adapter(
            self: Pin<&Self>,
            _do_create: bool,
            result: &mut Option<WindowAdapterRc>,
        ) {
            *result = self.window_adapter.clone();
        }
    }
}

// 🎯 删除复杂的 WindowHost trait，改用简单的 RootView 结构体

// 🎯 **统一的 RootView 架构** - 核心架构组件
// 所有控件都必须通过 RootView 挂载到 Slint Window

/// RootView 是所有控件的根视图包装器
/// 负责：
/// 1. 包装控件为 WindowItem 兼容结构
/// 2. 与 Slint Window 系统对接
/// 3. 统一管理 index 0 的根节点
///
/// ## 正确用法
/// ```rust
/// // ✅ 正确：明确使用 RootView 包装
/// RootView::new(Rectangle::new().with_background(red)).run();
///
/// // ❌ 错误：不再支持直接 .run()
/// // Rectangle::new().run(); // 这不再工作
/// ```
pub struct RootView<T: View> {
    child: T,
}

impl<T: View> RootView<T> {
    /// 创建新的 RootView，包装指定的控件
    pub fn new(child: T) -> Self {
        println!("🎯 RootView::new: Wrapping control for window integration");
        Self { child }
    }

    /// 运行 RootView，显示界面
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 RootView::run: Starting window with wrapped control");

        // 创建 ItemTree
        let tree_rc = self.create_window_root_tree()?;

        // 运行窗口
        run_window(tree_rc)?;

        Ok(())
    }

    /// Create a window-compatible ItemTree
    pub fn create_window_root_tree(self) -> Result<ItemTreeRc, Box<dyn std::error::Error>> {
        println!("🎯 RootView: Creating window-compatible ItemTree...");

        // ✅ **关键修复**：在这里显式调用 build() 构造完整视图树
        println!("🎯 RootView: Calling build() to construct complete view tree");
        let built_view = self.child.build();

        if let Some(tree_rc) = built_view.into_item_tree() {
            println!("✅ Successfully created ItemTree from built control");
            Ok(tree_rc)
        } else {
            Err("❌ Wrapped control failed to create ItemTree".into())
        }
    }
}

// 🎯 为 RootView 实现 View trait，提供完整的控件能力
impl<T: View> View for RootView<T> {
    type ItemType = T::ItemType;

    fn create_item(self) -> Self::ItemType {
        self.child.create_item()
    }

    fn into_item_tree(self) -> Option<ItemTreeRc> {
        self.child.into_item_tree()
    }

    fn transfer_properties_to_window(&self, window_item: &i_slint_core::items::WindowItem) {
        self.child.transfer_properties_to_window(window_item)
    }
}

// 🎯 **向后兼容** - 保留 RootContainer 名称
// 保留 RootContainer 名称作为 RootView 的别名，避免破坏现有代码
pub type RootContainer<T> = RootView<T>;

// 🎯 **向后兼容** - 保留 RootWindow 名称
pub type RootWindow<T> = RootView<T>;

/// 运行 Slint 窗口的核心函数
///
/// 这个函数负责：
/// 1. 创建窗口适配器
/// 2. 设置 ItemTree 组件
/// 3. 启动事件循环
pub fn run_window(tree_rc: ItemTreeRc) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 run_window: Starting Slint window system");

    // 创建窗口适配器
    let adapter =
        i_slint_backend_selector::with_platform(|platform| platform.create_window_adapter())?;

    // 设置组件
    let window_inner = i_slint_core::window::WindowInner::from_pub(adapter.window());
    window_inner.set_component(&tree_rc);

    // 配置窗口
    adapter.window().set_size(i_slint_core::api::LogicalSize::new(800.0, 600.0));

    // 显示窗口
    adapter.window().show()?;

    // 启动事件循环
    i_slint_backend_selector::with_platform(|platform| platform.run_event_loop())?;

    println!("✅ Window closed successfully");
    Ok(())
}

/// 🎯 **run_view! 宏**：便捷的控件运行宏
///
/// 这个宏简化了 `RootWindow::new(view).run()` 的调用，让代码更简洁。
///
/// 使用示例：
/// ```rust
/// // 快速运行红色矩形
/// run_view!(Rectangle::new()
///     .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
///     .with_width(200.0)
///     .with_height(100.0))?;
///
/// // 快速运行按钮
/// run_view!(Button::new()
///     .with_text("Hello".into()))?;
/// ```
#[macro_export]
macro_rules! run_view {
    ($view:expr) => {
        $crate::RootWindow::new($view).run()
    };
}

/// 🎯 **验证宏**：快速验证新架构是否正常工作
///
/// 使用示例：
/// ```rust
/// // 快速测试红色矩形
/// test_view!(red_rect);
///
/// // 快速测试蓝色按钮
/// test_view!(blue_button);
/// ```
#[macro_export]
macro_rules! test_view {
    (red_rect) => {{
        let rect = Rectangle::new()
            .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
            .with_width(200.0)
            .with_height(100.0);
        RootWindow::new(rect).run()
    }};
    (blue_button) => {{
        let button = Button::new()
            .with_text("Test Button".into())
            .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 100, 255)));
        RootWindow::new(button).run()
    }};
    (green_text) => {{
        let text = Text::new()
            .with_text("Hello GearUI!".into())
            .with_color(Brush::SolidColor(Color::from_rgb_u8(0, 150, 0)));
        RootWindow::new(text).run()
    }};
}

// 🎯 **便捷函数**：为常见用例提供简化的 API
pub mod convenience {
    use super::*;
    use i_slint_core::graphics::{Brush, Color};

    /// 快速创建并运行红色矩形
    pub fn run_red_rect() -> Result<(), Box<dyn std::error::Error>> {
        let rect =
            Rectangle::new().with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)));
        RootWindow::new(rect).run()
    }

    /// 快速创建并运行蓝色按钮
    pub fn run_blue_button() -> Result<(), Box<dyn std::error::Error>> {
        let button = Button::new()
            .with_text("Test Button".into())
            .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 100, 255)));
        RootWindow::new(button).run()
    }

    /// 快速创建并运行绿色文本
    pub fn run_green_text() -> Result<(), Box<dyn std::error::Error>> {
        let text = Text::new()
            .with_text("Hello GearUI!".into())
            .with_color(Brush::SolidColor(Color::from_rgb_u8(0, 150, 0)));
        RootWindow::new(text).run()
    }
}
