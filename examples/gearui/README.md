# GearUI Examples

GearUI 示例程序集合，展示如何使用纯 Rust API 构建 UI 组件。

## 📦 可用示例

### 🎨 Gallery - 控件画廊（推荐）

**全屏展示所有 17 个已完成的组件**

```bash
cargo run --bin gallery
```

**特点：**
- ✅ 一屏展示所有组件
- ✅ 分类清晰（基础/交互/组合/布局）
- ✅ 美观的深色主题
- ✅ 适合演示和学习

**展示内容：**
- 📦 **基础组件** (5): Rectangle, Text, Image, Clip, Opacity
- 🖱️ **交互组件** (5): TouchArea, Button, CheckBox, Slider, Switch
- 🧩 **组合组件** (2): ProgressIndicator, Spinner
- 📐 **布局组件** (4): VerticalLayout, HorizontalLayout, GridLayout, GroupBox

---

### 🎯 Showcase - 单个组件测试

**逐个测试单个组件的功能**

```bash
# 测试所有组件（交互式）
cargo run --bin showcase all

# 测试特定组件
cargo run --bin showcase button
cargo run --bin showcase checkbox
cargo run --bin showcase slider
cargo run --bin showcase switch
cargo run --bin showcase progress
cargo run --bin showcase spinner

# 测试布局
cargo run --bin showcase layouts
cargo run --bin showcase vstack
cargo run --bin showcase hstack
cargo run --bin showcase grid
```

**可用组件：**
- `button` - 按钮
- `rectangle` - 矩形
- `text` - 文本
- `checkbox` - 复选框
- `image` - 图片
- `toucharea` - 触摸区域
- `slider` - 滑动条
- `switch` - 开关
- `progress` - 进度条
- `spinner` - 加载动画
- `layouts` - 复合布局
- `vstack` - 垂直布局
- `hstack` - 水平布局
- `grid` - 网格布局

---

### 📐 Test Layouts - 布局测试

**测试复杂的嵌套布局**

```bash
cargo run --bin test_layouts
```

---

### 🔀 Test Switch Composite - Switch 组合测试

**测试 Switch 组件的组合使用**

```bash
cargo run --bin test_switch_composite
```

---

## 🚀 快速开始

### 推荐的学习路径

1. **先运行 Gallery** - 了解所有可用组件
   ```bash
   cargo run --bin gallery
   ```

2. **使用 Showcase 交互测试** - 逐个体验组件
   ```bash
   cargo run --bin showcase all
   ```

3. **查看源代码** - 学习如何使用 API
   - `src/gallery.rs` - 完整的综合示例
   - `src/showcase.rs` - 单个组件示例

---

## 📊 组件完成度

| 类别 | 完成度 | 组件列表 |
|------|--------|---------|
| 基础组件 | 5/5 (100%) | Rectangle, Text, Image, Clip, Opacity |
| 交互组件 | 5/9 (56%) | TouchArea, Button, CheckBox, Slider, Switch |
| 组合组件 | 2/5 (40%) | ProgressIndicator, Spinner |
| 布局组件 | 4/6 (67%) | Vertical, Horizontal, Grid, GroupBox |
| **总计** | **17/24 (71%)** | |

---

## 🛠️ 开发说明

### 编译所有示例

```bash
cd slint
cargo build --package gearui-examples
```

### 运行特定示例

```bash
cargo run --bin gallery       # 控件画廊
cargo run --bin showcase      # 单组件测试
cargo run --bin test_layouts  # 布局测试
```

### 查看帮助

```bash
cargo run --bin showcase      # 显示使用帮助
```

---

## 📝 代码示例

### 创建一个简单的按钮

```rust
use i_slint_core::SharedString;
use i_slint_core::graphics::{Brush, Color};
use i_slint_gearui::composed::Button;
use i_slint_gearui::RootView;

let button = Button::new()
    .with_text(SharedString::from("点击我"))
    .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 122, 255)))
    .with_enabled(true);

RootView::new(button).run()?;
```

### 创建垂直布局

```rust
use i_slint_gearui::layout::VerticalLayout;
use i_slint_gearui::primitives::Rectangle;

let layout = VerticalLayout::new()
    .with_spacing(10.0)
    .with_child(Rectangle::new().with_height(50.0))
    .with_child(Rectangle::new().with_height(50.0));

RootView::new(layout).run()?;
```

### 创建网格布局

```rust
use i_slint_gearui::layout::GridLayout;

let mut grid = GridLayout::new().with_spacing(5.0);

for row in 0..3 {
    for col in 0..3 {
        let rect = Rectangle::new()
            .with_width(100.0)
            .with_height(100.0);
        grid = grid.with_child(rect, row, col);
    }
}

RootView::new(grid).run()?;
```

---

## 🎯 下一步计划

待实现的组件（阶段 2-4）：

### 阶段 2：输入控件
- [ ] LineEdit - 单行文本输入
- [ ] TextEdit - 多行文本编辑
- [ ] SpinBox - 数值调节器

### 阶段 3：选择和容器
- [ ] ComboBox - 下拉框
- [ ] ScrollView - 滚动视图
- [ ] ListView - 列表视图
- [ ] StandardListView - 标准列表

### 阶段 4：高级组件
- [ ] TabWidget - 标签页
- [ ] StandardTableView - 表格
- [ ] DatePickerPopup - 日期选择器
- [ ] TimePickerPopup - 时间选择器
- [ ] MenuBar - 菜单栏

---

## 📚 相关文档

- [GearUI 架构文档](../../internal/gearui/ARCHITECTURE.md)
- [迁移状态报告](../../internal/gearui/MIGRATION_STATUS.md)
- [Slint 官方文档](https://slint.dev)

---

## 🤝 贡献

欢迎贡献新的示例和组件！

**添加新示例：**

1. 在 `src/` 创建新的 `.rs` 文件
2. 在 `Cargo.toml` 添加 `[[bin]]` 配置
3. 更新本 README

---

**最后更新**: 2026-02-02  
**维护者**: GearUI Team
