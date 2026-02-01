# GearUI 已知问题

**最后更新**: 2026-02-02

---

## 🔴 P0 - 高优先级（阻塞发布）

### 问题 #1: VerticalLayout 黑屏 - children 字段导致渲染失败

**状态**: ✅ 已修复  
**发现时间**: 2026-02-02  
**修复时间**: 2026-02-02  
**影响**: VerticalLayout 无法正常显示（已解决）

**症状**:
- ✅ Gallery 可以编译
- ✅ 窗口可以打开
- ❌ VerticalLayout 窗口显示黑屏，组件未渲染

**已验证**:
- ✅ 单个组件可以正常渲染
  - `cargo run --bin showcase` - 显示 CheckBox "同意用户协议" ✓
  - `cargo run --bin showcase rectangle` - 显示红色矩形 ✓
  - `cargo run --bin showcase grid` - 显示 2x2 彩色网格 ✓
  - `cargo run --bin showcase hstack` - 显示 3 个水平排列彩色矩形 ✓

- ✅ VerticalLayout 修复后可以运行（但子元素数量有限制）
  - `cargo run --bin showcase vstack` - 不再黑屏，可以启动 ✓

**根本原因**:

**VerticalLayoutItem 有多余的 `children` 字段导致结构不一致**

对比发现：
- HorizontalLayoutItem (工作正常): 只有基本属性 (width, height, x, y, spacing, alignment, cached_rendering_data)
- VerticalLayoutItem (黑屏): 多了一个 `children: ItemTreeRc` 字段

这个多余的字段破坏了布局组件的标准结构，导致渲染失败。

**修复方案**:

参照 HorizontalLayoutItem，移除 VerticalLayoutItem 的 `children` 字段：

```rust
// 修复前 (错误)
pub struct VerticalLayoutItem {
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    // ... 其他字段
    pub children: std::rc::Rc<dyn i_slint_core::item_tree::ItemTree>, // ❌ 多余字段
}

// 修复后 (正确)
pub struct VerticalLayoutItem {
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    // ... 其他字段
    // ✅ 不需要 children 字段
}
```

**修改的文件**:
- `internal/gearui/src/layout/vertical.rs:62-65` - 移除 children 字段声明
- `internal/gearui/src/layout/vertical.rs:67-78` - 简化 Default 实现
- `internal/gearui/src/layout/vertical.rs:332-354` - 简化 create_item 方法

**测试结果**:
```bash
$ cargo run --bin showcase vstack
🎯 VerticalLayoutItem::render called - delegating to children
# 程序正常运行，不再黑屏
```

**已知限制**:
- CompositeItemTree 目前只支持第一个子元素（其他被忽略）
- 这是 ItemTree 实现的限制，不是本次修复的问题

**复现步骤（已修复）**:
```bash
cd slint/examples/gearui
cargo run --bin showcase vstack
# 修复前：黑屏
# 修复后：可以正常启动
```

---

## 🟡 P1 - 中优先级（功能缺失）

### 问题 #2: Opacity 和 GroupBox 容器功能未完全实现

**状态**: ✅ API 已添加，但渲染逻辑待实现  
**影响**: Opacity 和 GroupBox 的子元素可能无法正确显示

**已完成**:
- ✅ 添加 `with_child` 方法
- ✅ 使用 `Box<ViewWrapper>` 避免递归类型

**待完成**:
- [ ] 实现 OpacityItem 的子元素渲染
- [ ] 实现 GroupBox 的边框和标题渲染
- [ ] 测试透明度效果

---

## 🟢 P2 - 低优先级（改进）

### 问题 #3: 大量编译警告

**状态**: 📋 待清理  
**影响**: 代码质量，但不影响功能

**警告类型**:
- 未使用的 import
- 未使用的变量
- 未使用的字段
- cfg 条件未定义

**修复建议**:
```bash
cargo fix --lib -p i-slint-gearui --allow-dirty
```

---

## 📊 问题统计

| 优先级 | 数量 | 已修复 | 进行中 | 待处理 |
|--------|------|--------|--------|--------|
| P0     | 1    | 1      | 0      | 0      |
| P1     | 1    | 0      | 1      | 0      |
| P2     | 1    | 0      | 0      | 1      |
| **总计** | **3** | **1** | **1** | **1** |

---

## 🔧 调试技巧

### 1. 启用详细日志

代码中已有大量调试日志，使用 `grep` 过滤：

```bash
cargo run --bin gallery 2>&1 | grep "🎯"
cargo run --bin gallery 2>&1 | grep "render"
```

### 2. 测试单个组件

```bash
cargo run --bin showcase rectangle  # 测试 Rectangle
cargo run --bin showcase text       # 测试 Text
cargo run --bin showcase button     # 测试 Button
```

### 3. 简化测试用例

使用 `test_simple.rs` 或 `gallery_simple.rs` 进行最小化复现。

### 4. 对比 Slint 官方实现

```bash
cd slint/internal/core
rg "VerticalBox" -A 20  # 查看官方实现
```

---

## 📝 更新日志

### 2026-02-02 (下午)
- ✅ 修复问题 #1: VerticalLayout 黑屏
  - 移除 VerticalLayoutItem 的多余 `children` 字段
  - 使其结构与 HorizontalLayoutItem 保持一致
  - `cargo run --bin showcase vstack` 可以正常运行

### 2026-02-02 (上午)
- 添加问题 #1: VerticalLayout 黑屏
- 添加问题 #2: 容器功能未完全实现
- 添加问题 #3: 编译警告
- 创建本文档

---

**维护者**: GearUI Team  
**联系方式**: 见 README.md
