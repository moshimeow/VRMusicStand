use std::{sync::Mutex, vec};

use stereokit_rust::{
    // Lines, LinePoint,
    event_loop::{SkClosures, StepperAction},
    maths::{units::*, Matrix, Pose, Quat, Vec2, Vec3},
    sk::Sk,
    sprite::{Sprite, SpriteType},
    system::{Backend, BackendOpenXR, BackendXRType, BtnState, Input, Key, LinePoint, Lines, Log, LogLevel, Projection, Renderer, Text},
    tex::SHCubemap,
    tools::{
        log_window::{LogItem, LogWindow}, 
        passthrough_fb_ext::{PassthroughFbExt, PASSTHROUGH_FLIP},
    },
    ui::{Ui, UiBtnLayout},
    util::{
        named_colors::{BLUE, LIGHT_BLUE, LIGHT_CYAN, WHITE}, Color128, Color32, Gradient
    },
};

use stereokit_rust::sk::MainThreadToken;
use stereokit_rust::system::TextAlign;

use winit::event_loop::EventLoop;

use std::array;

// use make_pdf::export_pdf_to_jpegs;


struct AppState {
    vec_sprites: Vec<Sprite>,

    root_window_pose:  Pose,
    prev_button_window_pose: Pose,
    next_button_window_pose: Pose,
    piano_window_pose: Pose,

    frame_idx_to_start_passthrough: u32,
    piano_sheet_idx: u32
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            vec_sprites: Vec::new(),
            root_window_pose: Pose::new(Vec3::new(-0.7, 1.5, -0.3), Some(Quat::look_dir(Vec3::new(1.0, 0.0, 1.0)))),
            prev_button_window_pose: Pose::new(Vec3::new(0.5, 0.0, 0.0), None),
            next_button_window_pose: Pose::new(Vec3::new(-0.5, 0.0, 0.0), None),
            piano_window_pose: Pose::new(Vec3::new(0.0, 0.3, -0.1), None),
            frame_idx_to_start_passthrough: 0,
            piano_sheet_idx: 0
        }
    }
}

/// Somewhere to copy the log
static LOG_LOG: Mutex<Vec<LogItem>> = Mutex::new(vec![]);

//use crate::launch;
#[cfg(target_os = "android")]
//use android_activity::AndroidApp;
use winit::platform::android::activity::AndroidApp;

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
/// The main function for android app
fn android_main(app: AndroidApp) {
    use stereokit_rust::sk::{DepthMode, OriginMode, SkSettings};
    let mut settings = SkSettings::default();
    settings
        .app_name("stereokit-rust")
        .assets_folder("assets")
        .origin(OriginMode::Floor)
        .render_multisample(4)
        .render_scaling(2.0)
        .depth_mode(DepthMode::Stencil)
        .log_filter(LogLevel::Diagnostic);

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Debug));

    BackendOpenXR::request_ext("XR_FB_passthrough");

    let (sk, event_loop) = settings.init_with_event_loop(app).unwrap();

    _main(sk, event_loop);
}

pub fn draw_spiral(sk: &Sk, _token: &MainThreadToken) {
    let mut line_points = [
        LinePoint {
        pt: Vec3::new(3.0, 2.0, -3.0),
        thickness: 0.1,
        color: Color32::new(0, 0, 0, 255),
    }, LinePoint {
        pt: Vec3::new(-3.0, 2.0, -3.0),
        thickness: 0.1,
        color: Color32::new(255, 0, 255, 255),
    }, LinePoint {
        pt: Vec3::new(-4.0, 2.0, -2.0),
        thickness: 0.1,
        color: Color32::new(255, 255, 255, 255),
    }
    ];


    Lines::add_list(_token, &line_points);
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
    Renderer::scaling(1.5);
    Renderer::multisample(4);

    // We want to be able to view the log using the LogWindow tool
    let fn_mut = |level: LogLevel, log_text: &str| {
        let mut items = LOG_LOG.lock().unwrap();
        for line_text in log_text.lines() {
            let subs = line_text.as_bytes().chunks(120);
            for (pos, sub_line) in subs.enumerate() {
                if let Ok(mut sub_string) = String::from_utf8(sub_line.to_vec()) {
                    if pos > 0 {
                        sub_string.insert_str(0, "‣‣‣‣");
                    }
                    if let Some(item) = items.last_mut() {
                        if item.text == sub_string {
                            item.count += 1;
                            continue;
                        }
                    }

                    items.push(LogItem { level, text: sub_string.to_owned(), count: 1 });
                };
            }
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
    let event_loop_proxy = sk.get_event_loop_proxy().clone().unwrap();
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
    // let cube_default = SHCubemap::get_rendered_sky();
    // cube0.render_as_sky();
    let mut sky = 1;

    let mut passthrough = false;
    let passthrough_enabled = BackendOpenXR::ext_enabled("XR_FB_passthrough");
    // Box<PassthroughFbExt> guy = Box::new()
    if passthrough_enabled {
        // sk.push_action(StepperAction::add("PassthroughFbExt", PassthroughFbExt::new(true)));
        sk.push_action(StepperAction::add_default::<PassthroughFbExt>("PassthroughFbExt"));
        Log::diag("Passthrough Disabled !!");
    } else {
        Log::diag("No Passthrough !!");
    }


    Log::diag(
        "======================================================================================================== !!",
    );
    let radio_on = Sprite::radio_on();
    let radio_off = Sprite::radio_off();



    let mut app_state: AppState = AppState::default();

    // let mut vec_sprites: Vec<Sprite> = Vec::new();

    for i in 0..10 {
        let name = format!("{}.png", i);
        app_state.vec_sprites.push(Sprite::from_file(name, None, None).unwrap());
        // let mut sprite: Sprite = Sprite::from_file(name);
    }


    let mut did: i32 = 0;

    let mut our_state_guy: AppState = AppState::default(); 


    
    // AppState {
    //     sprites : std::array::from_fn(|i| {Sprite::from_file(format!("{}.png", i), Some(SpriteType::Single), Some("tagada")).unwrap() }),
    //     meow : false
    // };

    let mut piano_sheet_idx: usize = 0;




    SkClosures::run_app(
        sk,
        event_loop,
        |sk, _token| {

            if app_state.frame_idx_to_start_passthrough < 500 {
                // Don't increment it forever (ie. don't eventually overflow)
                app_state.frame_idx_to_start_passthrough = app_state.frame_idx_to_start_passthrough+1;
            }
            if app_state.frame_idx_to_start_passthrough == 100 {
                sk.push_action(StepperAction::event("main".into(), PASSTHROUGH_FLIP, "1"));
            } else {

            }

            Ui::window_begin("Piano!", &mut window_demo_pose, None, None, None);

            for i in piano_sheet_idx..piano_sheet_idx+2 {

                println!("{}, {}", i, app_state.vec_sprites.get(i).is_none());

                if let Some(sprite) = app_state.vec_sprites.get(i) {
                    let j = i - piano_sheet_idx;
                    let sf: f32 = 0.7;
                    let sf2 = 0.8;
                    let trans = Vec3::new( (j as f32) * (-sf as f32) + (sf * 0.5), sf * 0.6, 0.5);
                    let rot = Quat::from_angles(-20.0, 0.0, 0.0);
                    let s = Vec3::new(sf2, sf2, sf2);
                    let transform: Matrix = Matrix::trs(&trans, &rot, &s);
                    sprite.draw(_token, transform, TextAlign::Center, None);
                }
            }

            if Ui::button("Prev", Some(Vec2::new(0.8, 0.1))) {
                if piano_sheet_idx > 0 {
                    piano_sheet_idx-=1; 
                }
                
            }

            Ui::same_line();
            Ui::hspace(0.3);



            if Ui::button("Next", Some(Vec2::new(0.8, 0.1))) {
                if piano_sheet_idx < 8 {
                    piano_sheet_idx+=1;
                }
            }

            // Ui::window_begin("Subwindow!", None, None, None);


            Ui::window_end();

            draw_spiral(sk, _token);




            // vec_sprites.iter().for_each(
            //     || {

            //     }

            // )

            
        },
        |sk| Log::info(format!("QuitReason is {:?}", sk.get_quit_reason())),
    );
}
