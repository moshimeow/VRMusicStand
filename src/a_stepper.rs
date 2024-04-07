use std::{cell::RefCell, rc::Rc};

use stereokit_rust::{
    font::Font,
    material::Material,
    maths::{Matrix, Quat, Vec3},
    mesh::Mesh,
    sk::{IStepper, MainThreadToken, SkInfo, StepperId},
    system::{Renderer, Text, TextStyle},
    util::named_colors::RED,
};

pub struct AStepper {
    id: StepperId,
    sk_info: Option<Rc<RefCell<SkInfo>>>,
    pub transform: Matrix,
    round_cube: Mesh,
    text: String,
    text_style: TextStyle,
}

unsafe impl Send for AStepper {}

impl Default for AStepper {
    fn default() -> Self {
        Self {
            id: "AStepper".to_string(),
            sk_info: None,
            transform: Matrix::tr(&((Vec3::NEG_Z * 2.5) + Vec3::Y), &Quat::from_angles(0.0, 180.0, 0.0)),
            round_cube: Mesh::generate_rounded_cube(Vec3::ONE / 5.0, 0.2, Some(16)),
            text: "Stepper A".to_owned(),
            text_style: Text::make_style(Font::default(), 0.3, RED),
        }
    }
}

impl IStepper for AStepper {
    fn initialize(&mut self, id: StepperId, sk_info: Rc<RefCell<SkInfo>>) -> bool {
        self.id = id;
        self.sk_info = Some(sk_info);
        true
    }

    fn step(&mut self, token: &MainThreadToken) {
        self.draw(token)
    }
}

impl AStepper {
    fn draw(&mut self, token: &MainThreadToken) {
        Renderer::add_mesh(token, &self.round_cube, Material::pbr(), self.transform, Some(RED.into()), None);
        Text::add_at(token, &self.text, self.transform, Some(self.text_style), None, None, None, None, None, None);
    }
}
