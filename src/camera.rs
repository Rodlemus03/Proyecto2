
use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    pub ojo: Vec3,
    pub centro: Vec3,
    pub arriba: Vec3
}

impl Camera {
    pub fn new(ojo: Vec3, centro: Vec3, arriba: Vec3) -> Self {
        Camera {
            ojo,
            centro,
            arriba
        }
    }

    pub fn base_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.centro - self.ojo).normalize();
        let right = forward.cross(&self.arriba).normalize();
        let arriba = right.cross(&forward).normalize();

        let rotated = vector.x * right + vector.y * arriba - vector.z * forward;

        return rotated.normalize();
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.ojo - self.centro;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_ojo = self.centro + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );

        self.ojo = new_ojo;
    }



    /////////////////////////////////////////////////////////////////////////////////

    fn mover(&mut self, direccion: Vec3, distancia: f32) {
        let desplazamiento = direccion.normalize() * distancia;
        self.ojo += desplazamiento;
        self.centro += desplazamiento;
    }

    pub fn mover_enfrente(&mut self, distance: f32) {
        let forward = (self.centro - self.ojo).normalize();
        self.mover(forward, distance);
    }

    pub fn mover_atras(&mut self, distance: f32) {
        let forward = (self.centro - self.ojo).normalize();
        self.mover(-forward, distance);
    }

    pub fn mover_der(&mut self, distance: f32) {
        let forward = (self.centro - self.ojo).normalize();
        let right = forward.cross(&self.arriba).normalize();
        self.mover(right, distance);
    }

    pub fn mover_izq(&mut self, distance: f32) {
        let forward = (self.centro - self.ojo).normalize();
        let right = forward.cross(&self.arriba).normalize();
        self.mover(-right, distance);
    }

}