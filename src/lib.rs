pub mod a_stepper;
use std::sync::Mutex;

use a_stepper::AStepper;
use stereokit_rust::{
    maths::{units::*, Pose, Quat, Vec2, Vec3},
    sk::{OriginMode, Sk, SkSettings, StepperAction},
    sprite::Sprite,
    system::{Log, LogLevel},
    tex::SHCubemap,
    tools::log_window::{LogItem, LogWindow},
    ui::{Ui, UiBtnLayout},
    util::{
        named_colors::{BLUE, LIGHT_BLUE, LIGHT_CYAN, WHITE},
        Color128, Gradient,
    },
};
use winit::event_loop::EventLoop;

/// Somewhere to copy the log
static LOG_LOG: Mutex<Vec<LogItem>> = Mutex::new(vec![]);

//use crate::launch;
#[cfg(target_os = "android")]
//use winit::platform::android::activity::AndroidApp;
use android_activity::AndroidApp;

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
/// The main function for android app
fn android_main(app: AndroidApp) {
    let mut settings = SkSettings::default();
    settings
        .app_name("stereokit-rust")
        .assets_folder("assets")
        .origin(OriginMode::Floor)
        .render_multisample(4)
        .render_scaling(2.0)
        .log_filter(LogLevel::Diagnostic);

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Debug));

    let (sk, event_loop) = settings.init(app).unwrap();

    _main(sk, event_loop);
}

// #[allow(dead_code)]
// #[cfg(target_os = "android")]
// pub fn main() {}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    use stereokit_rust::sk::AppMode;

    let mut settings = SkSettings::default();
    settings
        .app_name("stereokit-rust")
        .assets_folder("assets")
        .origin(OriginMode::Floor)
        .log_filter(LogLevel::Diagnostic)
        .no_flatscreen_fallback(true)
        .mode(AppMode::Simulator);

    let (sk, event_loop) = settings.init().unwrap();

    _main(sk, event_loop);
}

pub fn _main(sk: Sk, event_loop: EventLoop<StepperAction>) {
    let is_testing = false;
    Log::diag("Launch my_vr_program");
    launch(sk, event_loop, is_testing);
}

pub fn launch(mut sk: Sk, event_loop: EventLoop<StepperAction>, _is_testing: bool) {
    Log::diag(
        "======================================================================================================== !!",
    );

    // We want to be able to view the log using the LogWindow tool
    let fn_mut = |level: LogLevel, log_text: &str| {
        let mut items = LOG_LOG.lock().unwrap();
        for line_text in log_text.lines() {
            if let Some(item) = items.last_mut() {
                if item.text.eq(line_text) {
                    item.count += 1;
                    return;
                }
            }
            items.push(LogItem { level, text: line_text.to_owned(), count: 1 });
        }
    };
    Log::subscribe(fn_mut);
    // need a way to do that properly Log::unsubscribe(fn_mut);

    let mut log_window = LogWindow::new(&LOG_LOG);
    log_window.pose = Pose::new(Vec3::new(-0.7, 2.0, -0.3), Some(Quat::look_dir(Vec3::new(1.0, 0.0, 1.0))));

    let mut show_log = false;
    log_window.show(show_log);

    sk.push_action(StepperAction::add("LogWindow", log_window));
    // Open or close the log window
    let event_loop_proxy = sk.get_event_loop_proxy().clone();
    let send_event_show_log = move || {
        let _ = &event_loop_proxy.send_event(StepperAction::event("main".to_string(), "ShowLogWindow", "1"));
    };

    // we will have a window to trigger some actions
    let mut window_demo_pose = Pose::new(Vec3::new(-0.7, 1.5, -0.3), Some(Quat::look_dir(Vec3::new(1.0, 0.0, 1.0))));
    let demo_win_width = 50.0 * CM;

    // we create a sky dome to be able to switch from the default sky dome
    let mut gradient_sky = Gradient::new(None);
    gradient_sky
        .add(Color128::BLACK, 0.0)
        .add(BLUE, 0.3)
        .add(LIGHT_BLUE, 0.5)
        .add(LIGHT_CYAN, 0.8)
        .add(WHITE, 1.0);
    let cube0 = SHCubemap::gen_cubemap_gradient(gradient_sky, Vec3::Y, 1024);

    //save the default cubemap.
    let cube_default = SHCubemap::get_rendered_sky();
    cube0.render_as_sky();
    let mut sky = 1;

    // launch AStepper a basic stepper
    sk.push_action(StepperAction::add_default::<AStepper>("AStepper"));

    Log::diag(
        "======================================================================================================== !!",
    );
    let radio_on = Sprite::radio_on();
    let radio_off = Sprite::radio_off();
    sk.run(
        event_loop,
        |sk| {
            Ui::window_begin("Template", &mut window_demo_pose, Some(Vec2::new(demo_win_width, 0.0)), None, None);
            if Ui::radio_img("Blue light", sky == 1, &radio_off, &radio_on, UiBtnLayout::Left, None) {
                cube0.render_as_sky();
                sky = 1;
            }
            Ui::same_line();
            if Ui::radio_img("Default light", sky == 2, &radio_off, &radio_on, UiBtnLayout::Left, None) {
                cube_default.render_as_sky();
                sky = 2;
            }
            Ui::same_line();
            Ui::hspace(0.25);
            Ui::same_line();
            if let Some(new_value) = Ui::toggle("Show Log", show_log, None) {
                show_log = new_value;
                send_event_show_log();
            }
            Ui::next_line();
            Ui::hseparator();
            if Ui::button("Exit", Some(Vec2::new(0.10, 0.10))) {
                sk.quit(None);
            }
            //Ui::image(&power_button, Vec2::new(0.1, 0.1));

            Ui::window_end();
        },
        |sk| Log::info(format!("QuitReason is {:?}", sk.get_quit_reason())),
    );
}
