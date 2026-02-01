// 简单测试 - 只显示一个红色矩形

use i_slint_core::graphics::{Brush, Color};
use i_slint_gearui::RootView;
use i_slint_gearui::primitives::Rectangle;

fn main() {
    println!("🎯 简单测试：只显示一个红色矩形");

    let rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(400.0)
        .with_height(300.0);

    match RootView::new(rect).run() {
        Ok(_) => println!("✅ 显示成功"),
        Err(e) => println!("❌ 显示失败: {}", e),
    }
}
