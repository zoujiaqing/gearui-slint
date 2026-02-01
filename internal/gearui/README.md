# GearUI - Pure Rust UI Component Library

[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Slint Version](https://img.shields.io/badge/slint-1.15.0-green.svg)](https://slint.dev)

**GearUI** 是一个基于 Slint 引擎构建的纯 Rust UI 组件库，提供完整的控件集和优雅的 Builder 模式 API。

## 🎯 核心特性

- **纯 Rust API** - 无需 `.slint` 文件，完全使用 Rust 编写 UI
- **Builder 模式** - 流畅的链式 API，符合 Rust 惯用法
- **完整组件覆盖** - 目标覆盖所有 Slint 标准组件（当前已实现 10 个）
- **深度集成** - 直接使用 Slint 的 ItemTree 和渲染系统，零性能损耗
- **类型安全** - 编译时验证，无运行时类型转换

## 📦 已实现组件

### 基础组件 (Primitives)
- ✅ **Rectangle** - 矩形绘制，支持背景色
- ✅ **Text** - 文本渲染，支持多语言（中英文）
- ✅ **Image** - 图像显示，支持缩放模式

### 交互组件 (Interactive)
- ✅ **TouchArea** - 触摸响应区域
- ✅ **CheckBox** - 复选框，支持状态切换
- ✅ **Slider** - 滑块控件，支持值范围

### 组合组件 (Composed)
- ✅ **Button** - 按钮，包含文本和点击事件

### 布局组件 (Layout)
- ✅ **VerticalLayout** - 垂直布局容器
- ✅ **HorizontalLayout** - 水平布局容器
- ✅ **GridLayout** - 网格布局容器

## 🚀 快速开始

### 基础用法

```rust
use i_slint_gearui::*;
use i_slint_core::graphics::{Brush, Color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建红色矩形
    let rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(200.0)
        .with_height(100.0);
    
    // 运行
    RootView::new(rect).run()
}
```

### 组合控件

```rust
use i_slint_gearui::*;
use i_slint_core::graphics::{Brush, Color};
use i_slint_core::SharedString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建按钮
    let button = Button::new()
        .with_text(SharedString::from("Click Me!"))
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 120, 215)))
        .with_width(120.0)
        .with_height(35.0);
    
    RootView::new(button).run()
}
```

### 布局嵌套

```rust
use i_slint_gearui::*;
use i_slint_core::graphics::{Brush, Color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 垂直布局包含三个水平排列的矩形
    let layout = VerticalLayout::new()
        .with_child(
            HorizontalLayout::new()
                .with_child(Rectangle::new()
                    .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
                    .with_width(120.0)
                    .with_height(80.0))
                .with_child(Rectangle::new()
                    .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)))
                    .with_width(120.0)
                    .with_height(80.0))
                .with_child(Rectangle::new()
                    .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)))
                    .with_width(120.0)
                    .with_height(80.0))
        );
    
    RootView::new(layout).run()
}
```

## 🏗️ 架构设计

GearUI 采用清晰的 3 层架构：

```
┌─────────────────────────────────────────┐
│  Layer 3: GearView 转换接口             │
│  (into_item_tree, into_component)       │
├─────────────────────────────────────────┤
│  Layer 2: View 控件包装 (View Wrapper)  │
│  • 构建器模式 (Builder Pattern)         │
│  • 用户友好的 API                        │
│  • 属性 getter/setter                    │
├─────────────────────────────────────────┤
│  Layer 1: Item 实现 (ItemXxx)           │
│  • 实现 Item trait                       │
│  • 渲染 (render)                         │
│  • 布局 (layout_info)                    │
│  • 事件处理 (input_event, key_event)     │
│  • VTable 声明                           │
└─────────────────────────────────────────┘
```

详见 [ARCHITECTURE.md](ARCHITECTURE.md)

## 📚 示例程序

查看 `examples/gearui/` 目录：

```bash
# 运行所有控件展示
cargo run --bin showcase

# 运行布局测试
cargo run --bin test_layouts
```

## 🛠️ 开发路线图

### 阶段 1：基础扩展组件（计划中）
- [ ] Switch - 切换开关
- [ ] ProgressIndicator - 进度指示器
- [ ] Spinner - 加载动画
- [ ] GroupBox - 分组框容器
- [ ] Clip - 裁剪容器
- [ ] Opacity - 透明度控制

### 阶段 2：输入控件（计划中）
- [ ] LineEdit - 单行文本输入
- [ ] TextEdit - 多行文本编辑
- [ ] SpinBox - 数值输入框

### 阶段 3：选择和容器（计划中）
- [ ] ComboBox - 下拉选择框
- [ ] ScrollView - 滚动视图
- [ ] ListView - 列表视图
- [ ] StandardListView - 标准列表视图

### 阶段 4：高级组件（计划中）
- [ ] TabWidget - 标签页控件
- [ ] StandardTableView - 表格视图
- [ ] DatePickerPopup - 日期选择器
- [ ] TimePickerPopup - 时间选择器
- [ ] MenuBar - 菜单栏

## 🤝 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用以下许可证之一：

- GPL-3.0-only
- LicenseRef-Slint-Royalty-free-2.0
- LicenseRef-Slint-Software-3.0

详见 [LICENSE](../../LICENSE.md)

## 🙏 致谢

- [Slint UI](https://slint.dev) - 提供优秀的 UI 引擎
- 所有贡献者

## 📞 联系方式

- 项目主页：https://github.com/slint-ui/slint
- 文档：https://slint.dev/docs
- 问题反馈：https://github.com/slint-ui/slint/issues

---

**Made with ❤️ by GearUI Contributors**
