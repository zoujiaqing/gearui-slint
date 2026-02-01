// GearUI 控件画廊 - 全屏展示所有可用组件
// Gallery of all GearUI components in a full-screen layout

use i_slint_core::SharedString;
use i_slint_core::graphics::{Brush, Color};
use i_slint_core::items::LayoutAlignment;
use i_slint_core::lengths::LogicalLength;
use i_slint_gearui::RootView;
use i_slint_gearui::composed::{Button, ProgressIndicator, Spinner};
use i_slint_gearui::interactive::{CheckBox, Slider, Switch};
use i_slint_gearui::layout::{GridLayout, GroupBox, HorizontalLayout, VerticalLayout};
use i_slint_gearui::primitives::{Opacity, Rectangle, Text};

/// 创建标题文本
fn create_title(title: &str) -> Text {
    Text::new()
        .with_text(SharedString::from(title))
        .with_font_size(LogicalLength::new(16.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(255, 255, 255)))
}

/// 创建描述文本
#[allow(dead_code)]
fn create_description(text: &str) -> Text {
    Text::new()
        .with_text(SharedString::from(text))
        .with_font_size(LogicalLength::new(12.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(200, 200, 200)))
}

/// 创建分隔线
fn create_divider() -> Rectangle {
    Rectangle::new()
        .with_background(Brush::SolidColor(Color::from_rgb_u8(60, 60, 60)))
        .with_height(1.0)
}

/// 第一部分：基础组件（Primitives）
fn create_primitives_section() -> VerticalLayout {
    // 标题
    let title = create_title("基础组件 (Primitives)");

    // Rectangle 示例
    let rect_title = Text::new()
        .with_text(SharedString::from("Rectangle"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let rectangles = HorizontalLayout::new()
        .with_spacing(10.0)
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                .with_width(80.0)
                .with_height(60.0),
        )
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(52, 199, 89)))
                .with_width(80.0)
                .with_height(60.0),
        )
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 122, 255)))
                .with_width(80.0)
                .with_height(60.0),
        );

    // Text 示例
    let text_title = Text::new()
        .with_text(SharedString::from("Text"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let texts = VerticalLayout::new()
        .with_spacing(8.0)
        .with_child(
            Text::new()
                .with_text(SharedString::from("Large Text - 大号文本"))
                .with_font_size(LogicalLength::new(20.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(255, 255, 255))),
        )
        .with_child(
            Text::new()
                .with_text(SharedString::from("Medium Text - 中号文本"))
                .with_font_size(LogicalLength::new(16.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(200, 200, 200))),
        )
        .with_child(
            Text::new()
                .with_text(SharedString::from("Small Text - 小号文本"))
                .with_font_size(LogicalLength::new(12.0))
                .with_color(Brush::SolidColor(Color::from_rgb_u8(150, 150, 150))),
        );

    // Opacity 示例
    let opacity_title = Text::new()
        .with_text(SharedString::from("Opacity"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let opacity_boxes = HorizontalLayout::new()
        .with_spacing(10.0)
        .with_child(
            Opacity::new()
                .set_opacity(1.0)
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                        .with_width(60.0)
                        .with_height(60.0),
                )
                .build(),
        )
        .with_child(
            Opacity::new()
                .set_opacity(0.7)
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                        .with_width(60.0)
                        .with_height(60.0),
                )
                .build(),
        )
        .with_child(
            Opacity::new()
                .set_opacity(0.4)
                .with_child(
                    Rectangle::new()
                        .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                        .with_width(60.0)
                        .with_height(60.0),
                )
                .build(),
        );

    // 组合所有基础组件
    VerticalLayout::new()
        .with_spacing(15.0)
        .with_child(title)
        .with_child(create_divider())
        .with_child(rect_title)
        .with_child(rectangles)
        .with_child(text_title)
        .with_child(texts)
        .with_child(opacity_title)
        .with_child(opacity_boxes)
}

/// 第二部分：交互组件（Interactive）
fn create_interactive_section() -> VerticalLayout {
    let title = create_title("交互组件 (Interactive)");

    // Button
    let button_title = Text::new()
        .with_text(SharedString::from("Button"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let buttons = HorizontalLayout::new()
        .with_spacing(10.0)
        .with_child(
            Button::new()
                .with_text(SharedString::from("Primary"))
                .with_background(Brush::SolidColor(Color::from_rgb_u8(0, 122, 255)))
                .with_enabled(true),
        )
        .with_child(
            Button::new()
                .with_text(SharedString::from("Success"))
                .with_background(Brush::SolidColor(Color::from_rgb_u8(52, 199, 89)))
                .with_enabled(true),
        )
        .with_child(
            Button::new()
                .with_text(SharedString::from("Danger"))
                .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                .with_enabled(true),
        );

    // CheckBox
    let checkbox_title = Text::new()
        .with_text(SharedString::from("CheckBox"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let checkboxes = VerticalLayout::new()
        .with_spacing(8.0)
        .with_child(
            CheckBox::new()
                .set_text(SharedString::from("已选中"))
                .set_checked(true)
                .set_enabled(true),
        )
        .with_child(
            CheckBox::new()
                .set_text(SharedString::from("未选中"))
                .set_checked(false)
                .set_enabled(true),
        )
        .with_child(
            CheckBox::new()
                .set_text(SharedString::from("禁用状态"))
                .set_checked(false)
                .set_enabled(false),
        );

    // Slider
    let slider_title = Text::new()
        .with_text(SharedString::from("Slider"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let sliders = VerticalLayout::new()
        .with_spacing(8.0)
        .with_child(Slider::new().set_value(30.0).set_minimum(0.0).set_maximum(100.0).set_step(1.0))
        .with_child(
            Slider::new().set_value(70.0).set_minimum(0.0).set_maximum(100.0).set_step(1.0),
        );

    // Switch
    let switch_title = Text::new()
        .with_text(SharedString::from("Switch"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let switches = HorizontalLayout::new()
        .with_spacing(15.0)
        .with_child(Switch::new().set_checked(true).set_enabled(true))
        .with_child(Switch::new().set_checked(false).set_enabled(true))
        .with_child(Switch::new().set_checked(true).set_enabled(false));

    // 组合所有交互组件
    VerticalLayout::new()
        .with_spacing(15.0)
        .with_child(title)
        .with_child(create_divider())
        .with_child(button_title)
        .with_child(buttons)
        .with_child(checkbox_title)
        .with_child(checkboxes)
        .with_child(slider_title)
        .with_child(sliders)
        .with_child(switch_title)
        .with_child(switches)
}

/// 第三部分：组合组件（Composed）
fn create_composed_section() -> VerticalLayout {
    let title = create_title("组合组件 (Composed)");

    // ProgressIndicator
    let progress_title = Text::new()
        .with_text(SharedString::from("ProgressIndicator"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let progress_bars = VerticalLayout::new()
        .with_spacing(10.0)
        .with_child(ProgressIndicator::new().set_progress(0.25).set_indeterminate(false))
        .with_child(ProgressIndicator::new().set_progress(0.50).set_indeterminate(false))
        .with_child(ProgressIndicator::new().set_progress(0.75).set_indeterminate(false));

    // Spinner
    let spinner_title = Text::new()
        .with_text(SharedString::from("Spinner"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let spinners = HorizontalLayout::new()
        .with_spacing(20.0)
        .with_alignment(LayoutAlignment::Center)
        .with_child(Spinner::new())
        .with_child(Spinner::new())
        .with_child(Spinner::new());

    // 组合所有组合组件
    VerticalLayout::new()
        .with_spacing(15.0)
        .with_child(title)
        .with_child(create_divider())
        .with_child(progress_title)
        .with_child(progress_bars)
        .with_child(spinner_title)
        .with_child(spinners)
}

/// 第四部分：布局组件（Layout）
fn create_layout_section() -> VerticalLayout {
    let title = create_title("布局组件 (Layout)");

    // GridLayout 示例
    let grid_title = Text::new()
        .with_text(SharedString::from("GridLayout (3x3)"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let colors = [
        (255, 59, 48),
        (255, 149, 0),
        (255, 204, 0),
        (52, 199, 89),
        (0, 199, 190),
        (48, 176, 199),
        (50, 173, 230),
        (94, 92, 230),
        (175, 82, 222),
    ];

    let mut grid = GridLayout::new().with_spacing(5.0);
    for row in 0..3 {
        for col in 0..3 {
            let (r, g, b) = colors[row * 3 + col];
            let rect = Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(r, g, b)))
                .with_width(50.0)
                .with_height(50.0);
            grid = grid.with_child(rect, row as u32, col as u32);
        }
    }

    // GroupBox 示例
    let _groupbox_title = Text::new()
        .with_text(SharedString::from("GroupBox"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let _groupbox_content = HorizontalLayout::new()
        .with_spacing(10.0)
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                .with_width(60.0)
                .with_height(60.0),
        )
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(52, 199, 89)))
                .with_width(60.0)
                .with_height(60.0),
        );

    // GroupBox 示例
    let groupbox_title = Text::new()
        .with_text(SharedString::from("GroupBox"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(220, 220, 220)));

    let groupbox_content = HorizontalLayout::new()
        .with_spacing(10.0)
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(255, 59, 48)))
                .with_width(60.0)
                .with_height(60.0),
        )
        .with_child(
            Rectangle::new()
                .with_background(Brush::SolidColor(Color::from_rgb_u8(52, 199, 89)))
                .with_width(60.0)
                .with_height(60.0),
        );

    let groupbox = GroupBox::new()
        .set_title(SharedString::from("Settings Group"))
        .with_child(groupbox_content)
        .build();

    // 组合所有布局组件
    VerticalLayout::new()
        .with_spacing(15.0)
        .with_child(title)
        .with_child(create_divider())
        .with_child(grid_title)
        .with_child(grid)
        .with_child(groupbox_title)
        .with_child(groupbox)
}

/// 创建主标题栏
fn create_header() -> VerticalLayout {
    let main_title = Text::new()
        .with_text(SharedString::from("GearUI 控件画廊"))
        .with_font_size(LogicalLength::new(28.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(255, 255, 255)));

    let subtitle = Text::new()
        .with_text(SharedString::from("纯 Rust 实现的 UI 组件库 | 17 个组件已完成"))
        .with_font_size(LogicalLength::new(14.0))
        .with_color(Brush::SolidColor(Color::from_rgb_u8(150, 150, 150)));

    VerticalLayout::new()
        .with_spacing(8.0)
        .with_child(main_title)
        .with_child(subtitle)
        .with_child(create_divider())
}

/// 创建主布局 - 使用 Grid 排列四个区域
fn create_main_layout() -> VerticalLayout {
    let header = create_header();

    // 创建内容区域 - 使用 2x2 Grid
    let primitives = create_primitives_section();
    let interactive = create_interactive_section();
    let composed = create_composed_section();
    let layout = create_layout_section();

    // 左右两列布局
    let left_column =
        VerticalLayout::new().with_spacing(30.0).with_child(primitives).with_child(composed);

    let right_column =
        VerticalLayout::new().with_spacing(30.0).with_child(interactive).with_child(layout);

    let content = HorizontalLayout::new()
        .with_spacing(30.0)
        .with_alignment(LayoutAlignment::Start)
        .with_child(left_column)
        .with_child(right_column);

    // 主容器
    VerticalLayout::new()
        .with_spacing(30.0)
        .with_alignment(LayoutAlignment::Start)
        .with_child(header)
        .with_child(content)
}

fn main() {
    println!("🎨 GearUI 控件画廊");
    println!("===================");
    println!("展示所有 17 个已完成的组件：");
    println!();
    println!("📦 基础组件 (Primitives):");
    println!("  • Rectangle - 矩形/背景");
    println!("  • Text - 文本渲染");
    println!("  • Image - 图片显示");
    println!("  • Clip - 裁剪区域");
    println!("  • Opacity - 透明度");
    println!();
    println!("🖱️  交互组件 (Interactive):");
    println!("  • TouchArea - 触摸区域");
    println!("  • Button - 按钮");
    println!("  • CheckBox - 复选框");
    println!("  • Slider - 滑动条");
    println!("  • Switch - 开关");
    println!();
    println!("🧩 组合组件 (Composed):");
    println!("  • ProgressIndicator - 进度条");
    println!("  • Spinner - 加载动画");
    println!();
    println!("📐 布局组件 (Layout):");
    println!("  • VerticalLayout - 垂直布局");
    println!("  • HorizontalLayout - 水平布局");
    println!("  • GridLayout - 网格布局");
    println!("  • GroupBox - 分组框");
    println!();
    println!("🚀 启动全屏展示界面...");
    println!();

    let gallery = create_main_layout();

    match RootView::new(gallery).run() {
        Ok(_) => println!("✅ Gallery 显示成功"),
        Err(e) => {
            println!("❌ Gallery 显示失败: {}", e);
            std::process::exit(1);
        }
    }
}
