// 测试 SwitchComposite - 真正的复合组件实现

use i_slint_gearui::RootView;
use i_slint_gearui::interactive::SwitchComposite;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 GearUI SwitchComposite 测试");
    println!("================================");
    println!();
    println!("这是一个真正的复合组件，包含：");
    println!("  • 轨道 (Rectangle) - 背景色块");
    println!("  • 手柄 (Rectangle) - 白色圆形按钮");
    println!();
    println!("状态：");
    println!("  • 选中：绿色轨道，手柄在右侧");
    println!("  • 未选中：灰色轨道，手柄在左侧");
    println!();

    // 创建一个已选中的Switch
    let switch = SwitchComposite::new().set_checked(true).set_enabled(true).on_toggled(|| {
        println!("🎯 Switch 被切换了！");
    });

    println!("🎯 正在显示Switch组件...");
    println!("   轨道: 52x32px 绿色矩形");
    println!("   手柄: 28x28px 白色矩形，位于右侧");
    println!();

    RootView::new(switch).run()?;

    Ok(())
}
