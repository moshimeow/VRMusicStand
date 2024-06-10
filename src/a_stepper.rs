use std::{cell::RefCell, rc::Rc};

use stereokit_rust::{
    event_loop::{IStepper, StepperClosures, StepperId},
    font::Font,
    material::Material,
    maths::{Matrix, Quat, Vec3},
    mesh::Mesh,
    sk::{MainThreadToken, SkInfo},
    system::{Renderer, Text},
    util::named_colors::RED,
};

pub struct AStepper {
    id: StepperId,
    sk_info: Option<Rc<RefCell<SkInfo>>>,
    pub transform: Matrix,
    closures: StepperClosures<'static>,
}

unsafe impl Send for AStepper {}

impl Default for AStepper {
    fn default() -> Self {
        Self {
            id: "AStepper".to_string(),
            sk_info: None,
            transform: Matrix::tr(&((Vec3::NEG_Z * 2.5) + Vec3::Y), &Quat::from_angles(0.0, 180.0, 0.0)),
            closures: StepperClosures::new(),
        }
    }
}

impl IStepper for AStepper {
    fn initialize(&mut self, id: StepperId, sk_info: Rc<RefCell<SkInfo>>) -> bool {
        self.id = id;
        self.sk_info = Some(sk_info);

        let round_cube = Mesh::generate_rounded_cube(Vec3::ONE / 5.0, 0.2, Some(16));
        let text = "Stepper A".to_owned();
        let text_style = Text::make_style(Font::default(), 0.3, RED);
        let transform = self.transform;

        self.closures.set(
            move |token| {
                Renderer::add_mesh(token, &round_cube, Material::pbr(), transform, Some(RED.into()), None);
                Text::add_at(token, &text, transform, Some(text_style), None, None, None, None, None, None);
            },
            || {},
        );
        true
    }

    fn step(&mut self, token: &MainThreadToken) {
        self.closures.step(token)
    }

    fn shutdown(&mut self) {
        self.closures.shutdown()
    }
}
