// Switch 组件 - 使用MultiItemTree实现真正的复合组件
// 包含轨道(Rectangle) + 手柄(Rectangle)两个子元素

use crate::primitives::Rectangle;
use crate::{View, ViewWrapper};
use i_slint_core::graphics::{Brush, Color};
use std::rc::Rc;

/// Switch 公共API - 纯Rust UIKit风格
#[derive(Clone)]
pub struct SwitchComposite {
    pub checked: bool,
    pub enabled: bool,
    pub toggled_callback: Option<Rc<dyn Fn() + 'static>>,
}

impl Default for SwitchComposite {
    fn default() -> Self {
        Self::new()
    }
}

impl SwitchComposite {
    /// 创建新的Switch
    pub fn new() -> Self {
        Self { checked: false, enabled: true, toggled_callback: None }
    }

    /// 设置选中状态
    pub fn set_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// 设置启用状态
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置切换回调
    pub fn on_toggled<F: Fn() + 'static>(mut self, callback: F) -> Self {
        self.toggled_callback = Some(Rc::new(callback));
        self
    }

    /// 创建组成Switch的子视图
    fn create_children(&self) -> Vec<ViewWrapper> {
        let mut children = Vec::new();

        // 1. 轨道（背景矩形）
        let track_color = if self.enabled {
            if self.checked {
                Brush::SolidColor(Color::from_rgb_u8(52, 199, 89)) // 绿色
            } else {
                Brush::SolidColor(Color::from_rgb_u8(142, 142, 147)) // 灰色
            }
        } else {
            Brush::SolidColor(Color::from_rgb_u8(200, 200, 200))
        };

        let track = Rectangle::new()
            .with_background(track_color)
            .with_x(0.0)
            .with_y(0.0)
            .with_width(52.0)
            .with_height(32.0);

        children.push(ViewWrapper::Rectangle(track));

        // 2. 手柄（白色圆形矩形）
        let thumb_color = if self.enabled {
            Brush::SolidColor(Color::from_rgb_u8(255, 255, 255))
        } else {
            Brush::SolidColor(Color::from_rgb_u8(230, 230, 230))
        };

        let thumb_size = 28.0;
        let thumb_x = if self.checked {
            52.0 - thumb_size - 2.0 // 右侧
        } else {
            2.0 // 左侧
        };

        let thumb = Rectangle::new()
            .with_background(thumb_color)
            .with_x(thumb_x)
            .with_y(2.0)
            .with_width(thumb_size)
            .with_height(thumb_size);

        children.push(ViewWrapper::Rectangle(thumb));

        children
    }
}

/// 为SwitchComposite实现View trait
impl View for SwitchComposite {
    type ItemType = crate::layout::VerticalLayoutItem; // 临时使用，实际不会用到

    fn create_item(self) -> Self::ItemType {
        // 这个方法不会被调用，因为into_item_tree直接创建了MultiItemTree
        unimplemented!("SwitchComposite uses into_item_tree directly")
    }

    fn into_item_tree(self) -> Option<crate::ItemTreeRc> {
        // 创建一个容器Item（使用Rectangle作为根容器）
        let container = Rectangle::new()
            .with_width(52.0)
            .with_height(32.0)
            .with_background(Brush::SolidColor(Color::from_argb_u8(0, 0, 0, 0))); // 透明

        let container_item = container.create_item();

        // 创建子视图
        let children = self.create_children();

        // 使用MultiItemTree创建包含多个子Item的树
        Some(crate::item_tree_integration::create_multi_item_tree_from_wrappers(
            container_item,
            children,
        ))
    }

    fn build(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_composite_creation() {
        let switch = SwitchComposite::new();
        assert!(!switch.checked);
        assert!(switch.enabled);
    }

    #[test]
    fn test_switch_composite_builder() {
        let switch = SwitchComposite::new().set_checked(true).set_enabled(false);

        assert!(switch.checked);
        assert!(!switch.enabled);
    }
}
