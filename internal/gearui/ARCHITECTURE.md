# GearUI 3-Layer Architecture Design Specification

**版本：1.0** | **日期：2024年**  
**状态：📐 设计规范** | **优先级：🔥 核心架构**

---

## ✅ 结论先行：核心原则

所有 GearUI 控件最终都需要具备 `ItemTreeRc` 与 `ComponentRc` 能力，这样才能：

1. ✅ **被 Slint 的 `Window::set_component()` 渲染**
2. ✅ **支持嵌套（控件中包含子控件）**
3. ✅ **支持事件传递、布局、绘制统一处理**

---

## 🧩 3-Layer Architecture Model

我们将控件拆分成 3 层能力模型：

| 能力级别 | 接口特征 | 用途 | 是否必须 |
|---------|---------|------|---------|
| **Layer 1: Item trait** | 实现 `Item` | 渲染/布局/事件分发 | ✅ **必须** |
| **Layer 2: View struct** | 包含字段 + 构造器 + `into_item_tree()` | 控件外部 API 封装 | ✅ **必须** |
| **Layer 3: Component 适配** | `fn into_component_rc(&self)` | 用于顶层 Window | 🔄 **可选**（仅顶层需要） |

---

## 💡 标准代码模板

### 🔧 Layer 1: Item 实现（必须）

```rust
#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct MyControlItem {
    pub my_property: Property<SomeType>,
    pub cached_rendering_data: CachedRenderingData,
    // 其它字段...
}

impl Item for MyControlItem {
    fn render(self: Pin<&Self>, backend: &mut &mut dyn ItemRenderer, self_rc: &ItemRc, size: LogicalSize) -> RenderingResult {
        // 渲染逻辑
        RenderingResult::ContinueRenderingChildren
    }
    
    fn layout_info(self: Pin<&Self>, orientation: Orientation, window_adapter: &WindowAdapterRc, self_rc: &ItemRc) -> LayoutInfo {
        // 布局逻辑
        LayoutInfo::default()
    }
    
    // 其他 Item trait 方法...
}

// 实现对应的 Render trait (如 RenderRectangle, RenderText 等)
impl RenderMyControl for MyControlItem {
    fn my_property(self: Pin<&Self>) -> SomeType {
        self.get_ref().my_property.get()
    }
}

// VTable 声明
i_slint_core::declare_item_vtable! {
    fn slint_get_MyControlVTable() -> MyControlVTable for MyControlItem
}
```

### 🔧 Layer 2: View 控件封装（必须）

```rust
pub struct MyControl {
    item: VRc<MyControlVTable, MyControlItem>,
}

impl MyControl {
    /// 创建新实例，默认值匹配 Slint 规范
    pub fn new() -> Self {
        let item = VRc::new(MyControlItem::default());
        Self { item }
    }

    /// Builder 模式设置属性
    pub fn with_my_property(self, value: SomeType) -> Self {
        VRcMapped::map(&self.item, |item| &item.my_property).set(value);
        self
    }

    /// 获取属性值
    pub fn my_property(&self) -> SomeType {
        VRcMapped::map(&self.item, |item| &item.my_property).get()
    }
}

impl Default for MyControl {
    fn default() -> Self {
        Self::new()
    }
}
```

### 🔧 Layer 3: GearView 统一接口（必须）

```rust
impl GearView for MyControl {
    fn into_item_tree(self) -> ItemTreeRc {
        // 创建包含单个 Item 的 ItemTree
        ItemTreeRc::new_single_item(self.item)
    }
    
    // into_component() 方法自动继承默认实现：
    // fn into_component(self) -> ComponentRc {
    //     GearComponent::new(self.into_item_tree())
    // }
}
```

---

## 🚀 统一转换接口设计

### GearView Trait 定义

```rust
/// 统一的 GearUI 控件转换接口
pub trait GearView: Sized {
    /// 转换为 ItemTree（用于嵌套）
    fn into_item_tree(self) -> ItemTreeRc;
    
    /// 转换为 Component（用于顶层窗口）
    fn into_component(self) -> ComponentRc {
        GearComponent::new(self.into_item_tree())
    }
}
```

### 优雅的用法示例

```rust
// 嵌套使用
let child_tree = Rectangle::new().with_background(RED).into_item_tree();

// 顶层窗口使用
window.set_component(Rectangle::new().with_background(RED).into_component());
```

---

## ✅ 设计优点

- 🚀 **通用性强**：每个控件只需实现一次模板
- 🔗 **无缝对接 Slint**：使用 Slint 原生的 `ItemTreeRc` + `ComponentRc`
- 🧩 **组合简单**：未来 Button、ScrollView 等组合控件直接嵌套 ItemTree
- 📐 **渐进扩展**：先实现 ItemTree，需要时再包装 Component

---

## 🧱 GearUI vs .slint DSL 对比

| 能力 | .slint DSL（宏控件） | GearUI 控件 |
|------|---------------------|-------------|
| **属性** | 自动生成 Property | 手动声明 Property |
| **渲染** | 自动实现 | 实现 Render trait |
| **布局** | 自动布局系统 | 调用 layout trait |
| **嵌套** | 自动管理 ItemTree | 自己构建 ItemTree |
| **运行** | 自动转换 ComponentRc | 手动 `into_component()` |

---

## 📋 实现检查清单

每个 GearUI 控件必须包含：

### Layer 1: Item Implementation
- [ ] `#[derive(FieldOffsets, Default, SlintElement)]` 结构体
- [ ] 完整的 `Item` trait 实现
- [ ] 对应的 Render trait 实现（如 `RenderRectangle`）
- [ ] `declare_item_vtable!` 宏声明
- [ ] `ItemConsts` trait 实现

### Layer 2: View Control
- [ ] 公开的控件结构体（包含 `VRc`）
- [ ] `new()` 构造函数（默认值匹配 Slint）
- [ ] `with_*()` builder 模式方法
- [ ] 属性 getter 方法
- [ ] `Default` trait 实现

### Layer 3: GearView Integration
- [ ] `GearView` trait 实现
- [ ] `into_item_tree()` 方法
- [ ] `into_component()` 方法（通常使用默认实现）

### Documentation & Testing
- [ ] 完整的字段映射表（与 Slint 1:1 对应）
- [ ] rustdoc 文档和使用示例
- [ ] 基础功能测试
- [ ] 集成测试（如果是复合控件）

---

## 🔄 迭代策略

1. **Phase 1**：实现 Layer 1 + Layer 2，placeholder Layer 3
2. **Phase 2**：实现完整的 `ItemTreeRc` 集成
3. **Phase 3**：实现 `ComponentRc` 和 Window 集成
4. **Phase 4**：复合控件和高级功能

---

## ⚠️ 重要约束

1. **字段名称**：必须与 `.slint` 控件字段完全一致
2. **默认值**：必须与 Slint 规范完全匹配
3. **类型安全**：编译时验证所有属性和事件
4. **性能**：直接集成 Slint 内部系统，无额外开销
5. **组合限制**：组合控件只能使用 GearUI 控件或标准结构

---

**📌 这是 GearUI 项目的核心设计规范，所有控件实现必须严格遵循此架构。** 