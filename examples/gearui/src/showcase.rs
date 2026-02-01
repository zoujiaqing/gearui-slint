// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::env;

/// 测试 TouchArea 控件 - 在透明交互区域上叠加可见内容
fn test_toucharea_overlay() {
    println!("🎯 TouchArea 叠加测试：在透明交互区域上显示红色方块");

    
    use i_slint_core::graphics::{Brush, Color};
    use i_slint_core::items::MouseCursor;
    use i_slint_gearui::RootView;
    use i_slint_gearui::interactive::TouchArea;
    use i_slint_gearui::primitives::Rectangle;

    // 创建一个 TouchArea 控件，设置手型光标
    let toucharea = TouchArea::new().with_enabled(true).with_mouse_cursor(MouseCursor::Pointer);

    // 创建一个红色方块作为可见内容
    let red_square = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(200.0)
        .with_height(200.0);

    println!("🎯 TouchArea 属性:");
    println!("  - enabled: {}", toucharea.enabled());
    println!("  - mouse_cursor: {:?}", toucharea.mouse_cursor());

    // 先测试红色方块是否可见
    println!("🎯 测试红色方块可见性");
    match RootView::new(red_square).run() {
        Ok(_) => println!("✅ Red square 显示成功"),
        Err(e) => println!("❌ Red square 显示失败: {}", e),
    }

    // 现在测试透明的 TouchArea - 黑屏是正常的
    println!("🎯 测试透明的 TouchArea（黑屏是正常的）");
    match RootView::new(toucharea).run() {
        Ok(_) => println!("✅ TouchArea 运行成功（透明，黑屏是正常的）"),
        Err(e) => println!("❌ TouchArea 运行失败: {}", e),
    }
}

/// 测试 Button 控件
fn test_button() {
    println!("🎯 Button 控件测试");

    use i_slint_core::SharedString;
    use i_slint_core::graphics::{Brush, Color};
    use i_slint_gearui::RootView;
    use i_slint_gearui::composed::Button;

    let button = Button::new()
        .with_text(SharedString::from("点击我 Click Me!"))
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 120, 215)))
        .with_enabled(true);

    match RootView::new(button).run() {
        Ok(_) => println!("✅ Button 显示成功"),
        Err(e) => println!("❌ Button 显示失败: {}", e),
    }
}

/// 测试 Rectangle 控件
fn test_rectangle() {
    println!("🎯 Rectangle 控件测试");

    use i_slint_core::graphics::{Brush, Color};
    use i_slint_gearui::RootView;
    use i_slint_gearui::primitives::Rectangle;

    let rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(400.0)
        .with_height(300.0);

    match RootView::new(rect).run() {
        Ok(_) => println!("✅ Rectangle 显示成功"),
        Err(e) => println!("❌ Rectangle 显示失败: {}", e),
    }
}

/// 测试 Text 控件
fn test_text() {
    println!("🎯 Text 控件测试");

    use i_slint_core::SharedString;
    use i_slint_core::graphics::{Brush, Color};
    use i_slint_core::lengths::LogicalLength;
    use i_slint_gearui::RootView;
    use i_slint_gearui::primitives::Text;

    let text = Text::new()
        .with_text(SharedString::from("Hello GearUI! 你好世界!"))
        .with_font_size(LogicalLength::new(24.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)));

    match RootView::new(text).run() {
        Ok(_) => println!("✅ Text 显示成功"),
        Err(e) => println!("❌ Text 显示失败: {}", e),
    }
}

/// 测试 CheckBox 控件
fn test_checkbox() {
    println!("🎯 CheckBox 控件测试");

    use i_slint_core::SharedString;
    use i_slint_gearui::RootView;
    use i_slint_gearui::interactive::CheckBox;

    let checkbox = CheckBox::new()
        .set_text(SharedString::from("同意用户协议"))
        .set_checked(false)
        .set_enabled(true);

    match RootView::new(checkbox).run() {
        Ok(_) => println!("✅ CheckBox 显示成功"),
        Err(e) => println!("❌ CheckBox 显示失败: {}", e),
    }
}

/// 测试 Image 控件
fn test_image() {
    println!("🎯 Image 控件测试");

    use i_slint_core::graphics::Image as SlintImage;
    use i_slint_core::items::ImageFit;
    use i_slint_core::lengths::LogicalLength;
    use i_slint_gearui::RootView;
    use i_slint_gearui::primitives::Image;
    use std::path::Path;

    // 尝试加载测试图片
    let image_path = Path::new("../imagefilter/assets/cat.jpg");

    match SlintImage::load_from_path(image_path) {
        Ok(image_data) => {
            println!("✅ 图片加载成功: {:?}", image_path);
            println!("  - 图片尺寸: {}x{}", image_data.size().width, image_data.size().height);

            let image = Image::new()
                .with_source(image_data)
                .with_width(LogicalLength::new(400.0))
                .with_height(LogicalLength::new(300.0))
                .with_image_fit(ImageFit::Contain);

            match RootView::new(image).run() {
                Ok(_) => println!("✅ Image 显示成功"),
                Err(e) => println!("❌ Image 显示失败: {}", e),
            }
        }
        Err(e) => {
            println!("❌ 图片加载失败: {}", e);
            println!("  - 图片路径: {:?}", image_path);
            println!("  - 当前工作目录: {:?}", std::env::current_dir().unwrap());

            // 创建一个空的Image控件作为备选测试
            println!("🎯 创建空的Image控件进行测试");
            let empty_image = Image::new()
                .with_width(LogicalLength::new(200.0))
                .with_height(LogicalLength::new(200.0));

            match RootView::new(empty_image).run() {
                Ok(_) => println!("✅ 空Image控件显示成功"),
                Err(e) => println!("❌ 空Image控件显示失败: {}", e),
            }
        }
    }
}

pub fn test_slider() {
    println!("🎯 🔥 Testing Slider Control!");

    use i_slint_gearui::RootView;
    use i_slint_gearui::interactive::Slider;

    // 🎯 创建 Slider 控件：水平滑块，范围 0-100，初始值 50
    let slider = Slider::new().set_value(50.0).set_minimum(0.0).set_maximum(100.0).set_step(1.0);

    println!(
        "🎯 Slider created: value={}, range=[{}, {}]",
        slider.value(),
        slider.minimum(),
        slider.maximum()
    );

    // 🎯 使用 RootView 运行 Slider
    match RootView::new(slider).run() {
        Ok(_) => println!("✅ Slider 显示成功"),
        Err(e) => println!("❌ Slider 显示失败: {}", e),
    }
}

/// 测试布局控件：按用户建议的布局方案
fn test_layouts() {
    println!("🎯 🔥 Testing Layout Controls - Advanced Layout!");

    use i_slint_core::graphics::{Brush, Color};
    use i_slint_core::items::LayoutAlignment;
    use i_slint_gearui::RootView;
    use i_slint_gearui::layout::{GridLayout, HorizontalLayout, VerticalLayout};
    use i_slint_gearui::primitives::Rectangle;

    // 🎯 第一排：3列横向排列 - 红、绿、蓝
    println!("\n🎯 创建第一排：HStack 3列 (红、绿、蓝)");
    let red_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(120.0)
        .with_height(80.0);

    let green_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)))
        .with_width(120.0)
        .with_height(80.0);

    let blue_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)))
        .with_width(120.0)
        .with_height(80.0);

    let first_row = HorizontalLayout::new()
        .with_spacing(10.0)
        .with_alignment(LayoutAlignment::Center)
        .with_child(red_rect)
        .with_child(green_rect)
        .with_child(blue_rect);

    // 🎯 第二排：4x4 Grid 16宫格，16种颜色
    println!("🎯 创建第二排：Grid 4x4 16宫格");

    // 生成16种不同颜色
    let colors = [
        (255, 0, 0),     // 红
        (0, 255, 0),     // 绿
        (0, 0, 255),     // 蓝
        (255, 255, 0),   // 黄
        (255, 0, 255),   // 品红
        (0, 255, 255),   // 青
        (255, 128, 0),   // 橙
        (128, 0, 255),   // 紫
        (255, 192, 203), // 粉
        (128, 128, 128), // 灰
        (255, 255, 255), // 白
        (0, 0, 0),       // 黑
        (139, 69, 19),   // 棕
        (255, 215, 0),   // 金
        (192, 192, 192), // 银
        (75, 0, 130),    // 靛青
    ];

    let mut grid = GridLayout::new().with_spacing(5.0).with_padding(10.0);

    // 填充4x4网格
    for row in 0..4 {
        for col in 0..4 {
            let color_index = row * 4 + col;
            let (r, g, b) = colors[color_index];

            let rect = Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(r, g, b)))
                .with_width(80.0)
                .with_height(80.0);

            grid = grid.with_child(rect, row as u32, col as u32);
        }
    }

    // 🎯 最终布局：VStack 包含 HStack 和 Grid
    println!("🎯 组合最终布局：VStack {{ HStack, Grid }}");
    let final_layout = VerticalLayout::new()
        .with_spacing(20.0)
        .with_alignment(LayoutAlignment::Center)
        .with_child(first_row)
        .with_child(grid);

    println!("  - 第一排（HStack）: 3个方块，间距 10px");
    println!("  - 第二排（Grid）: 4x4 = 16个方块，间距 5px");
    println!("  - 总布局（VStack）: 2层，间距 20px");

    match RootView::new(final_layout).run() {
        Ok(_) => println!("✅ 复合布局显示成功！"),
        Err(e) => println!("❌ 复合布局显示失败: {}", e),
    }
}

// 在 test_layouts 函数前面添加单独的 HStack 测试
fn test_hstack_only() {
    println!("🎯 🔥 Testing HStack Only - 单独测试水平布局");

    use i_slint_core::graphics::{Brush, Color};
    use i_slint_core::items::LayoutAlignment;
    use i_slint_gearui::RootView;
    use i_slint_gearui::layout::HorizontalLayout;
    use i_slint_gearui::primitives::Rectangle;

    // 创建3个不同颜色的方块
    let red_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(120.0)
        .with_height(80.0);

    let green_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)))
        .with_width(120.0)
        .with_height(80.0);

    let blue_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)))
        .with_width(120.0)
        .with_height(80.0);

    // 创建 HStack 水平布局
    let hstack = HorizontalLayout::new()
        .with_spacing(15.0)
        .with_alignment(LayoutAlignment::Center)
        .with_child(red_rect)
        .with_child(green_rect)
        .with_child(blue_rect);

    println!("  - 子控件数量: {}", hstack.children_count());
    println!("  - 间距: {}", hstack.spacing());
    println!("  - 对齐方式: {:?}", hstack.alignment());

    match RootView::new(hstack).run() {
        Ok(_) => println!("✅ HStack 显示成功"),
        Err(e) => println!("❌ HStack 显示失败: {}", e),
    }
}

fn test_vstack_only() {
    println!("🎯 🔥 Testing VStack Only - 单独测试垂直布局");

    use i_slint_core::graphics::{Brush, Color};
    use i_slint_core::items::LayoutAlignment;
    use i_slint_gearui::RootView;
    use i_slint_gearui::layout::VerticalLayout;
    use i_slint_gearui::primitives::Rectangle;

    // 创建3个不同颜色的方块
    let red_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(120.0)
        .with_height(60.0);

    let green_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)))
        .with_width(120.0)
        .with_height(60.0);

    let blue_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)))
        .with_width(120.0)
        .with_height(60.0);

    // 创建 VStack 垂直布局
    let vstack = VerticalLayout::new()
        .with_spacing(15.0)
        .with_alignment(LayoutAlignment::Center)
        .with_child(red_rect)
        .with_child(green_rect)
        .with_child(blue_rect);

    println!("  - 子控件数量: {}", vstack.children_count());
    println!("  - 间距: {}", vstack.spacing());
    println!("  - 对齐方式: {:?}", vstack.alignment());

    match RootView::new(vstack).run() {
        Ok(_) => println!("✅ VStack 显示成功"),
        Err(e) => println!("❌ VStack 显示失败: {}", e),
    }
}

fn test_grid_only() {
    println!("🎯 🔥 Testing Grid Only - 单独测试网格布局");

    use i_slint_core::graphics::{Brush, Color};
    use i_slint_gearui::RootView;
    use i_slint_gearui::layout::GridLayout;
    use i_slint_gearui::primitives::Rectangle;

    // 创建4个不同颜色的方块做简单2x2测试
    let red_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)))
        .with_width(100.0)
        .with_height(100.0);

    let green_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)))
        .with_width(100.0)
        .with_height(100.0);

    let blue_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 0, 255)))
        .with_width(100.0)
        .with_height(100.0);

    let yellow_rect = Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 255, 0)))
        .with_width(100.0)
        .with_height(100.0);

    // 创建2x2网格
    let grid = GridLayout::new()
        .with_spacing(10.0)
        .with_padding(10.0)
        .with_child(red_rect, 0, 0) // 第1行第1列
        .with_child(green_rect, 0, 1) // 第1行第2列
        .with_child(blue_rect, 1, 0) // 第2行第1列
        .with_child(yellow_rect, 1, 1); // 第2行第2列

    println!("  - 子控件数量: {}", grid.children_count());
    println!("  - 间距: {}", grid.spacing());
    println!("  - 内边距: {}", grid.padding());

    match RootView::new(grid).run() {
        Ok(_) => println!("✅ Grid 显示成功"),
        Err(e) => println!("❌ Grid 显示失败: {}", e),
    }
}

/// 测试 Switch 控件
/// 测试 Switch 控件
fn test_switch() {
    println!("🎯 Switch 控件测试");

    use i_slint_gearui::RootView;
    use i_slint_gearui::interactive::Switch;

    let switch = Switch::new().set_checked(true).set_enabled(true);

    match RootView::new(switch).run() {
        Ok(_) => println!("✅ Switch 显示成功"),
        Err(e) => println!("❌ Switch 显示失败: {}", e),
    }
}

/// 测试 ProgressIndicator 控件
fn test_progress() {
    println!("🎯 ProgressIndicator 控件测试");

    use i_slint_gearui::RootView;
    use i_slint_gearui::composed::ProgressIndicator;

    let progress = ProgressIndicator::new().set_progress(0.65).set_indeterminate(false);

    match RootView::new(progress).run() {
        Ok(_) => println!("✅ ProgressIndicator 显示成功"),
        Err(e) => println!("❌ ProgressIndicator 显示失败: {}", e),
    }
}

/// 测试 Spinner 控件
fn test_spinner() {
    println!("🎯 Spinner 控件测试");

    use i_slint_gearui::RootView;
    use i_slint_gearui::composed::Spinner;

    let spinner = Spinner::new();

    match RootView::new(spinner).run() {
        Ok(_) => println!("✅ Spinner 显示成功"),
        Err(e) => println!("❌ Spinner 显示失败: {}", e),
    }
}

fn main() {
    println!("🎯 GearUI 控件展示");
    println!("===================");

    // 解析命令行参数
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("button") => test_button(),
        Some("rectangle") => test_rectangle(),
        Some("text") => test_text(),
        Some("checkbox") => test_checkbox(),
        Some("image") => test_image(),
        Some("toucharea") => test_toucharea_overlay(),
        Some("slider") => test_slider(),
        Some("layouts") => test_layouts(),
        Some("vstack") => test_vstack_only(),
        Some("hstack") => test_hstack_only(),
        Some("grid") => test_grid_only(),
        // 阶段1新组件
        Some("switch") => test_switch(),
        Some("progress") => test_progress(),
        Some("spinner") => test_spinner(),
        Some("all") => {
            println!("🎯 依次测试所有控件（按Enter键切换到下一个控件）");
            println!("按Enter键开始测试Button...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            test_button();

            println!("按Enter键测试Rectangle...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_rectangle();

            println!("按Enter键测试Text...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_text();

            println!("按Enter键测试CheckBox...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_checkbox();

            println!("按Enter键测试Image...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_image();

            println!("按Enter键测试TouchArea...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_toucharea_overlay();

            println!("按Enter键测试Slider...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_slider();

            println!("按Enter键测试布局控件...");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            test_layouts();
        }
        _ => {
            println!("🎯 默认测试 CheckBox 控件");
            test_checkbox();

            println!("\n💡 使用方法:");
            println!("  cargo run --bin showcase [控件名称]");
            println!("  基础控件: button, rectangle, text, checkbox, image, toucharea, slider");
            println!("  阶段1新增: switch, progress, spinner, groupbox");
            println!("  布局控件: layouts, vstack, hstack, grid");
            println!("  其他: all");
        }
    }

    println!("\n🎯 控件测试完成！");
}
