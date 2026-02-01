# GearUI 已知问题

**最后更新**: 2026-02-02

---

## 🔴 P0 - 高优先级（阻塞发布）

### 问题 #1: Gallery 黑屏 - 布局组件渲染失败

**状态**: 🔍 调查中  
**发现时间**: 2026-02-02  
**影响**: Gallery 全屏画廊无法正常显示

**症状**:
- ✅ Gallery 可以编译
- ✅ 窗口可以打开
- ❌ 窗口显示黑屏，组件未渲染

**已验证**:
- ✅ 单个组件可以正常渲染
  - `cargo run --bin showcase` - 显示 CheckBox "同意用户协议" ✓
  - `cargo run --bin showcase rectangle` - 显示红色矩形 ✓
  - `cargo run --bin showcase text` - 显示文本 ✓
  - `cargo run --bin showcase button` - 显示蓝色按钮 "点击我 Click Me!" ✓
  - `cargo run --bin showcase slider` - 显示滑动条 ✓

- ❌ 布局组件渲染失败（子元素未显示）
  - `cargo run --bin gallery` - 黑屏 ✗
  - `cargo run --bin gallery_simple` - 黑屏 ✗
  - `cargo run --bin test_layouts` - 黑屏 ✗
  - `cargo run --bin showcase vstack` - 黑屏 ✗
  - `cargo run --bin showcase hstack` - 黑屏 ✗

**日志分析**:
```
🎯 VerticalLayoutItem::render called - delegating to children
✅ Window closed successfully
```

说明：
- VerticalLayout 的 render 方法被调用
- 但子元素的 render 未被触发
- 没有看到 `RectangleItem::render called!`

**根本原因（推测）**:

1. **布局组件未实现子元素遍历**
   - LayoutItem 的 render 可能只是空实现
   - 需要手动调用每个子元素的 render

2. **ItemTree 结构问题**
   - 布局组件创建的 ItemTree 可能不包含子元素
   - 需要检查 `into_item_tree()` 的实现

3. **渲染顺序问题**
   - Slint 可能需要特定的 ItemTree 结构
   - 需要参考 Slint 官方的布局实现

**复现步骤**:
```bash
cd slint/examples/gearui
cargo run --bin gallery
# 窗口打开但显示黑屏
```

**相关文件**:
- `internal/gearui/src/layout/vertical.rs`
- `internal/gearui/src/layout/horizontal.rs`
- `internal/gearui/src/lib.rs` (ItemTree 创建逻辑)
- `examples/gearui/src/gallery.rs`

**下一步行动**:
1. [ ] 检查 VerticalLayout::render 的实现
2. [ ] 对比 Slint 官方的 VerticalBox 实现
3. [ ] 确保 ItemTree 正确包含所有子元素
4. [ ] 实现正确的渲染遍历逻辑
5. [ ] 添加渲染调试日志

**临时解决方案**:
- 使用 `showcase` 测试单个组件
- 避免使用复杂嵌套布局

**参考**:
- Slint VerticalBox 实现: `slint/internal/core/layouts.rs`
- ItemTree 文档: `slint/internal/core/item_tree.rs`

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
| P0     | 1    | 0      | 1      | 0      |
| P1     | 1    | 0      | 1      | 0      |
| P2     | 1    | 0      | 0      | 1      |
| **总计** | **3** | **0** | **2** | **1** |

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

### 2026-02-02
- 添加问题 #1: Gallery 黑屏
- 添加问题 #2: 容器功能未完全实现
- 添加问题 #3: 编译警告
- 创建本文档

---

**维护者**: GearUI Team  
**联系方式**: 见 README.md
