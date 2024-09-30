use nalgebra_glm::{Vec3,vec3};
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

#[derive(Clone)]
pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
    pub velocidad:Vec3,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material, velocidad: Vec3) -> Self {
        Self {
            center,
            size,
            material,
            velocidad,
        }
    }

    pub fn actualizar_posicion(&mut self, delta_tiempo: f32) {
        let gravedad = vec3(0.0, -9.8, 0.0); // Simulamos la gravedad en la dirección Y
        self.velocidad += gravedad * delta_tiempo; // Actualizamos la velocidad con la gravedad
        self.center += self.velocidad * delta_tiempo; // Actualizamos la posición
    }
    fn get_uv(&self, punto_encuentro: &Vec3) -> (f32, f32) {
        let mitad = self.size / 2.0;
        let min = self.center - Vec3::new(mitad, mitad, mitad);
        let max = self.center + Vec3::new(mitad, mitad, mitad);
        let mut u = 0.0;
        let mut v = 0.0;
        if (punto_encuentro.x - min.x).abs() < 0.001 { 
            u = (punto_encuentro.z - min.z) / (max.z - min.z);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        } else if (punto_encuentro.x - max.x).abs() < 0.001 { 
            u = (punto_encuentro.z - min.z) / (max.z - min.z);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        } else if (punto_encuentro.y - min.y).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.z - min.z) / (max.z - min.z);
        } else if (punto_encuentro.y - max.y).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.z - min.z) / (max.z - min.z);
        } else if (punto_encuentro.z - min.z).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        } else if (punto_encuentro.z - max.z).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        }
        (u, v)
    }

}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mitad = self.size / 2.0;
        let min = self.center - Vec3::new(mitad, mitad, mitad);
        let max = self.center + Vec3::new(mitad, mitad, mitad);

        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);
        let t_min = (min - ray_origin).component_mul(&inv_dir);
        let t_max = (max - ray_origin).component_mul(&inv_dir);

        let t1 = t_min.x.min(t_max.x).max(t_min.y.min(t_max.y)).max(t_min.z.min(t_max.z));
        let t2 = t_min.x.max(t_max.x).min(t_min.y.max(t_max.y)).min(t_min.z.max(t_max.z));

        if t1 > t2 || t2 < 0.0 {
            return Intersect::empty();
        }

        let t_hit = if t1 < 0.0 { t2 } else { t1 };
        let punto_encuentro = ray_origin + ray_direction * t_hit;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        for i in 0..3 {
            if (punto_encuentro[i] - min[i]).abs() < 0.001 {
                normal[i] = -1.0;
            } else if (punto_encuentro[i] - max[i]).abs() < 0.001 {
                normal[i] = 1.0;
            }
        }

        let (u, v) = self.get_uv(&punto_encuentro);
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        Intersect::new(
            punto_encuentro,
            normal,
            t_hit,
            self.material.clone(),
            u,
            v
        )
    }
    fn get_uv(&self, punto_encuentro: &Vec3) -> (f32, f32) {
        let mitad = self.size / 2.0;
        let min = self.center - Vec3::new(mitad, mitad, mitad);
        let max = self.center + Vec3::new(mitad, mitad, mitad);
        let mut u = 0.0;
        let mut v = 0.0;
        if (punto_encuentro.x - min.x).abs() < 0.001 { 
            u = (punto_encuentro.z - min.z) / (max.z - min.z);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        } else if (punto_encuentro.x - max.x).abs() < 0.001 { 
            u = (punto_encuentro.z - min.z) / (max.z - min.z);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        } else if (punto_encuentro.y - min.y).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.z - min.z) / (max.z - min.z);
        } else if (punto_encuentro.y - max.y).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.z - min.z) / (max.z - min.z);
        } else if (punto_encuentro.z - min.z).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        } else if (punto_encuentro.z - max.z).abs() < 0.001 { 
            u = (punto_encuentro.x - min.x) / (max.x - min.x);
            v = (punto_encuentro.y - min.y) / (max.y - min.y);
        }
        (u, v)
    }
}