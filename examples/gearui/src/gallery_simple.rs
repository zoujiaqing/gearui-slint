// GearUI 简化版画廊 - 测试基本渲染

use i_slint_core::SharedString;
use i_slint_core::graphics::{Brush, Color};
use i_slint_core::lengths::LogicalLength;
use i_slint_gearui::RootView;
use i_slint_gearui::layout::{HorizontalLayout, VerticalLayout};
use i_slint_gearui::primitives::{Rectangle, Text};

fn main() {
    println!("🎨 GearUI 简化画廊");
    println!("==================");

    // 创建3个彩色方块
    let red = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
        .with_width(100.0)
        .with_height(100.0);

    let green = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(52, 199, 89)))
        .with_width(100.0)
        .with_height(100.0);

    let blue = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 122, 255)))
        .with_width(100.0)
        .with_height(100.0);

    // 水平排列
    let layout = HorizontalLayout::new()
        .with_spacing(20.0)
        .with_child(red)
        .with_child(green)
        .with_child(blue);

    println!("🚀 启动窗口...");

    match RootView::new(layout).run() {
        Ok(_) => println!("✅ Gallery 显示成功"),
        Err(e) => {
            println!("❌ Gallery 显示失败: {}", e);
            std::process::exit(1);
        }
    }
}
