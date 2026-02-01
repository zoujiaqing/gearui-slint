use i_slint_core::SharedString;
use i_slint_core::graphics::{Brush, Color};
use i_slint_gearui::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 GearUI Layout Tests - 验证嵌套容器架构");
    println!("🧪 VStack 包含 HStack 和 Grid 的嵌套结构");
    println!();

    // 🎯 构建复合布局：
    // VStack
    // ├── HStack
    // │   ├── Rectangle (red)
    // │   ├── Rectangle (green)
    // │   └── Rectangle (blue)
    // └── Grid
    //     ├── Text "Hello"
    //     └── Rectangle (yellow)
    let root_layout = VerticalLayout::new()
        .with_child(
            HorizontalLayout::new()
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
                        .with_width(120.0)
                        .with_height(80.0),
                )
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)))
                        .with_width(120.0)
                        .with_height(80.0),
                )
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)))
                        .with_width(120.0)
                        .with_height(80.0),
                ),
        )
        .with_child(
            GridLayout::new()
                .with_child(Text::new().with_text(SharedString::from("Hello World")), 0, 0)
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 255, 0)))
                        .with_width(120.0)
                        .with_height(60.0),
                    0,
                    1,
                ),
        );

    println!("🎯 启动复合布局测试...");
    println!();

    // 🎯 使用 RootView 统一架构运行
    RootView::new(root_layout).run()
}
