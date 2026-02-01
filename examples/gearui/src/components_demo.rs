// GearUI 组件演示
// 展示使用.slint实现的Switch、Progress、Spinner组件

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = ComponentsDemo::new()?;

    println!("🎯 GearUI Components Demo");
    println!("===================");
    println!("展示以下组件:");
    println!("  • Switch - 开关组件（3个状态）");
    println!("  • ProgressIndicator - 进度条（确定和不确定模式）");
    println!("  • Spinner - 加载动画（3种尺寸）");
    println!();
    println!("请观察组件的视觉效果和动画！");

    ui.run()
}
