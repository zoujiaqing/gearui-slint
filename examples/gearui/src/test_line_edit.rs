// GearUI LineEdit 测试示例

use i_slint_core::SharedString;
use i_slint_core::graphics::{Brush, Color};
use i_slint_core::lengths::LogicalLength;
use i_slint_gearui::RootView;
use i_slint_gearui::input::LineEdit;
use i_slint_gearui::layout::VerticalLayout;
use i_slint_gearui::primitives::{Rectangle, Text};

fn main() {
    println!("🎯 GearUI LineEdit 测试");
    println!("===================");
    println!("功能测试：");
    println!("  • 基本文本输入");
    println!("  • 占位符显示");
    println!("  • 焦点状态");
    println!("  • 边框样式");
    println!();

    // 创建垂直布局
    let layout = VerticalLayout::new()
        .with_spacing(20.0)
        .with_child(
            Text::new()
                .with_text(SharedString::from("LineEdit 测试"))
                .with_font_size(LogicalLength::new(20.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(255, 255, 255))),
        )
        .with_child(
            Text::new()
                .with_text(SharedString::from("1. 带占位符的输入框："))
                .with_font_size(LogicalLength::new(14.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(200, 200, 200))),
        )
        .with_child(
            LineEdit::new()
                .with_placeholder(SharedString::from("请输入用户名..."))
                .with_width(300.0)
                .with_height(40.0),
        )
        .with_child(
            Text::new()
                .with_text(SharedString::from("2. 带初始文本的输入框："))
                .with_font_size(LogicalLength::new(14.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(200, 200, 200))),
        )
        .with_child(
            LineEdit::new()
                .with_text(SharedString::from("Hello GearUI"))
                .with_width(300.0)
                .with_height(40.0),
        )
        .with_child(
            Text::new()
                .with_text(SharedString::from("3. 自定义样式的输入框："))
                .with_font_size(LogicalLength::new(14.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(200, 200, 200))),
        )
        .with_child(
            LineEdit::new()
                .with_placeholder(SharedString::from("自定义颜色和字体"))
                .with_font_size(LogicalLength::new(16.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(0, 122, 255)))
                .with_background(Brush::SolidColor(Color::from_rgb_u8(245, 245, 245)))
                .with_border_color(Brush::SolidColor(Color::from_rgb_u8(0, 122, 255)))
                .with_border_width(LogicalLength::new(2.0))
                .with_width(300.0)
                .with_height(45.0),
        );

    match RootView::new(layout).run() {
        Ok(_) => println!("✅ LineEdit 测试成功"),
        Err(e) => {
            println!("❌ LineEdit 测试失败: {}", e);
            std::process::exit(1);
        }
    }
}
