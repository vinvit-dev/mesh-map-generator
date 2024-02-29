use glfw::Key;
use nalgebra_glm as glm;

use crate::keyboard_handler::KeyBoardHandler;

#[derive(Clone, Debug, Copy)]
pub struct Camera {
    pub fov: f32,
    yaw: f32,
    pitch: f32,
    last_x: f32,
    last_y: f32,
    pos: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    sensetivity: f32,
    speed: f32,
}

impl Camera {
    pub fn create_camera(s_width: f32, s_height: f32, sensetivity: f32, speed: f32) -> Self {
        Self {
            fov: 45.,
            yaw: -90.,
            pitch: 0.,
            last_x: s_width / 2.,
            last_y: s_height / 2.,
            pos: glm::vec3(0.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            sensetivity,
            speed,
        }
    }
    pub fn handle_cursor_pos(&mut self, event: &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::CursorPos(xpos_in, ypos_in) => {
                let xpos = xpos_in.clone() as f32;
                let ypos = ypos_in.clone() as f32;

                let mut xoffset = self.last_x - xpos;
                let mut yoffset = self.last_y - ypos;
                self.last_x = xpos;
                self.last_y = ypos;

                xoffset *= self.sensetivity;
                yoffset *= self.sensetivity;

                self.yaw += xoffset;
                self.pitch += yoffset;

                if self.pitch > 89. {
                    self.pitch = 89.;
                }
                if self.pitch < -89. {
                    self.pitch = -89.;
                }

                let mut front: glm::Vec3 = glm::vec3(0., 0., 0.);
                front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos() * -1.;
                front.y = self.pitch.to_radians().sin();
                front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

                self.front = glm::normalize(&front);
            }
            _ => {}
        }
    }
    pub fn handle_input(&mut self, keyboard_handler: &KeyBoardHandler, delta_time: f32) {
        let camera_speed = self.speed * delta_time;
        if keyboard_handler.key_pressed(Key::W) {
            self.pos += camera_speed * self.front;
        }
        if keyboard_handler.key_pressed(Key::S) {
            self.pos -= camera_speed * self.front;
        }
        if keyboard_handler.key_pressed(Key::Space) {
            self.pos += camera_speed * self.up;
        }
        if keyboard_handler.key_pressed(Key::C) {
            self.pos -= camera_speed * self.up;
        }
        if keyboard_handler.key_pressed(Key::A) {
            self.pos -= glm::normalize(&glm::cross(&self.front, &self.up)) * camera_speed;
        }
        if keyboard_handler.key_pressed(Key::D) {
            self.pos += glm::normalize(&glm::cross(&self.front, &self.up)) * camera_speed;
        }
    }
    pub fn handle_scroll(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Scroll(_, yoffset) => {
                self.fov -= yoffset as f32;
            }
            _ => {}
        }
    }

    pub fn look_at(self) -> glm::Mat4 {
        glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }
}
