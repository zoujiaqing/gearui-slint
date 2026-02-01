// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

/*!
# GearUI Layout Controls Module

布局控件模块，提供1:1对应Slint布局系统的纯Rust API。

## 布局控件列表

- **VerticalLayout**: 垂直布局容器，纵向排列子控件
- **HorizontalLayout**: 水平布局容器，横向排列子控件
- **GridLayout**: 网格布局容器，网格排列子控件

## 设计原则

- 🎯 **1:1对应**: 完全对应Slint的布局控件功能
- 🧩 **组合能力**: 支持任意嵌套和组合
- 🎨 **Builder模式**: 流畅的API设计
- ✅ **RootView支持**: 可作为窗口根运行

## 使用示例

```rust
use i_slint_gearui::layout::*;
use i_slint_gearui::primitives::*;

// 垂直布局示例
let app = VerticalLayout::new()
    .with_spacing(10.0)
    .with_child(Text::new().with_text("标题"))
    .with_child(
        HorizontalLayout::new()
            .with_spacing(5.0)
            .with_child(Button::new().with_text("取消"))
            .with_child(Button::new().with_text("确认"))
    )
    .run();

// 网格布局示例
let grid = GridLayout::new()
    .with_spacing(5.0)
    .with_child(Text::new().with_text("A"), 0, 0)
    .with_child(Text::new().with_text("B"), 0, 1)
    .with_child(Text::new().with_text("C"), 1, 0)
    .with_child(Text::new().with_text("D"), 1, 1)
    .run();
```
*/

pub mod grid;
pub mod group_box;
pub mod horizontal;
pub mod vertical;

// 导出所有布局控件
pub use grid::{GridCell, GridLayout, GridLayoutItem};
pub use group_box::{GroupBox, GroupBoxItem};
pub use horizontal::{HorizontalLayout, HorizontalLayoutItem};
pub use vertical::{VerticalLayout, VerticalLayoutItem};

// 🎯 为了方便使用，提供简化别名
/// VerticalLayout 的简化别名，对应 SwiftUI 的 VStack
pub use VerticalLayout as VStack;

/// HorizontalLayout 的简化别名，对应 SwiftUI 的 HStack
pub use HorizontalLayout as HStack;

/// VerticalLayout 的通用别名，对应 Flutter 的 Column
pub use VerticalLayout as Column;

/// HorizontalLayout 的通用别名，对应 Flutter 的 Row
pub use HorizontalLayout as Row;

/// GridLayout 的简化别名
pub use GridLayout as Grid;
