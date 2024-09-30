mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod sphere;
mod texturas;
use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::texturas::TextureManager;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{normalize, vec3, Vec3};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

fn reflector(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
    color_fondo: &Color,
) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return color_fondo.clone();
    }
    if let Some(ref textura) = intersect.material.textura {
        let diffuse_color = intersect
            .material
            .get_diffuse_color(intersect.u, intersect.v);
        return diffuse_color;
    } else {
        let diffuse_color = intersect.material.diffuse.clone();
        return diffuse_color;
    }

    let luz_dir = (light.position - intersect.point).normalize();
    let vista_dir = (ray_origin - intersect.point).normalize();
    let reflector_dir = reflector(&-luz_dir, &intersect.normal);

    let intensidad_difuminado = intersect.normal.dot(&luz_dir).max(0.0).min(1.0);
    let diffuse = intersect.material.diffuse
        * intersect.material.albedo[0]
        * intensidad_difuminado
        * light.intensity;

    let specular_intensidad = vista_dir
        .dot(&reflector_dir)
        .max(0.0)
        .powf(intersect.material.specular);
    let specular =
        light.color * intersect.material.albedo[1] * specular_intensidad * light.intensity;

    diffuse + specular
}



pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
    color_fondo: &Color,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    framebuffer
        .buffer
        .iter_mut()
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = i % framebuffer.width;
            let y = i / framebuffer.width;

            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.base_change(&ray_direction);

            let pixel_color =
                cast_ray(&camera.ojo, &rotated_direction, objects, light, color_fondo);

            *pixel = pixel_color.to_hex();
        });
}

fn main() {
    let window_width = 450;
    let window_height = 300;
    let framebuffer_width = 1200;
    let framebuffer_height = 1000;
    let frame_delay = Duration::from_millis(16);
    let duracion_recorrido_luz = Duration::from_secs(10);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Proyecto 2",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut manejador_textura = TextureManager::new();
    let imagen = image::open("uvg.png").unwrap().into_rgba8();
    manejador_textura.cargar_textura("uvg", imagen);
    let textura = manejador_textura.get_textura("uvg");
    let uvg: Material = Material::new(Color::new(255, 255, 255), 1.0, [0.0, 0.0], textura);

    let imagen = image::open("tierra.png").unwrap().into_rgba8();
    manejador_textura.cargar_textura("tierra", imagen);
    let textura = manejador_textura.get_textura("tierra");
    let tierra = Material::new(Color::new(255, 255, 255), 1.0, [0.0, 0.0], textura);

    let imagen = image::open("papel.png").unwrap().into_rgba8();
    manejador_textura.cargar_textura("papel", imagen);
    let textura = manejador_textura.get_textura("papel");
    let papel = Material::new(Color::new(255, 255, 255), 1.0, [0.0, 0.0], textura);

    let imagen: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::open("madera.png").unwrap().into_rgba8();
    manejador_textura.cargar_textura("madera", imagen);
    let textura = manejador_textura.get_textura("madera");
    let madera = Material::new(Color::new(255, 255, 255), 1.0, [0.0, 0.0], textura);

    let imagen: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::open("agua.png").unwrap().into_rgba8();
    manejador_textura.cargar_textura("agua", imagen);
    let textura = manejador_textura.get_textura("agua");
    let agua = Material::new(Color::new(255, 255, 255), 1.0, [0.0, 0.0], textura);

    let sol_material = Material::new(Color::new(255, 234, 100), 1.0, [0.0, 0.0], None);
    let luna_material = Material::new(Color::new(200, 200, 255), 1.0, [0.0, 0.0], None); // Color gris azulado

    let mut light = Light::new(
        Vec3::new(100.0, 100.0, 10.0),
        Color::new(255, 255, 255),
        5.0,
        3.0,
    );

    let mut esfera_amarilla = Sphere {
        center: light.position,
        radius: 3.0,
        material: sol_material.clone(),
    };

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI / 10.0;
    let velocidad_movimiento = 0.1;

    let color_inicial = Color::new(4, 12, 36);
    let color_final = Color::new(135, 206, 235);
    let mut color_actual = color_inicial.clone();
    let mut siguiente_color = color_final.clone();
    let mut tiempo_inicial = Instant::now();
    let mut progreso_transicion = 0.0;

    let mut tiempo_luz = Instant::now();

    let color_blanco = Color::new(255, 255, 255);
    let color_amarillo = Color::new(255, 234, 100);
    let mut color_esfera_actual = color_blanco.clone();

    let radio = 100.0;
    let duracion_recorrido_luz_secs = duracion_recorrido_luz.as_secs_f32();
    let velocidad_angular = PI / duracion_recorrido_luz_secs;
    let mut angulo = 0.0;

    let mut es_dia = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            camera.mover_enfrente(velocidad_movimiento);
        }
        if window.is_key_down(Key::S) {
            camera.mover_atras(velocidad_movimiento);
        }
        if window.is_key_down(Key::A) {
            camera.mover_izq(velocidad_movimiento);
        }
        if window.is_key_down(Key::D) {
            camera.mover_der(velocidad_movimiento);
        }

        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        if window.is_key_pressed(Key::L, minifb::KeyRepeat::No) {
            es_dia = !es_dia;
        }

        if es_dia {
            esfera_amarilla.material = sol_material.clone();
            light.color = color_amarillo.clone();
            color_actual = Color::new(135, 206, 235);
        } else {
            esfera_amarilla.material = luna_material.clone();
            light.color = color_blanco.clone();
            color_actual = Color::new(4, 12, 36);
        }

        let tiempo_actual_luz = tiempo_luz.elapsed().as_secs_f32();
        angulo = (tiempo_actual_luz * velocidad_angular) % (2.0 * PI);

        light.position.x = radio * angulo.cos();
        light.position.z = radio * angulo.sin();
        esfera_amarilla.center = light.position;

        let mut objetos: Vec<Box<dyn RayIntersect>> = Vec::new();
        objetos.push(Box::new(esfera_amarilla.clone()));

        //Creamos el suelo de tierra
        objetos.push(Box::new(Cube {
            center: Vec3::new(10.0, -310.0, -10.0),
            size: 600.0,
            material: tierra.clone(),
            velocidad: vec3(0.0, 0.0, 0.0),
        }));

        //Rellenamos
        let mut contadorX = -2.0;
        let mut contador2X = -1.5;
        while contadorX < 2.5 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contadorX, 0.0, 1.0),
                size: 0.5,
                material: uvg.clone(),
                velocidad: vec3(0.0, 0.0, 0.0),
            }));
            contadorX += 1.0;
            if contadorX < 2.5 {
                objetos.push(Box::new(Cube {
                    center: Vec3::new(contador2X, 0.0, 1.0),
                    size: 0.5,
                    material: papel.clone(),
                    velocidad: vec3(0.0, 0.0, 0.0),
                }));
            }

            contador2X += 1.0;
        }
        contadorX = -1.5;
        contador2X = -1.0;
        while contadorX < 2.5 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contadorX, 0.5, 1.0),
                size: 0.5,
                material: uvg.clone(),
                velocidad: vec3(0.0, 0.0, 0.0),
            }));
            contadorX += 1.0;
            if contadorX < 2.5 {
                objetos.push(Box::new(Cube {
                    center: Vec3::new(contador2X, 0.5, 1.0),
                    size: 0.5,
                    material: papel.clone(),
                    velocidad: vec3(0.0, 0.0, 0.0),
                }));
            }

            contador2X += 1.0;
        }
        contadorX = -1.0;
        contador2X = -0.5;
        while contadorX < 1.5 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contadorX, 1.0, 1.0),
                size: 0.5,
                material: uvg.clone(),
                velocidad: vec3(0.0, 0.0, 0.0),
            }));
            contadorX += 1.0;
            if contadorX < 1.5 {
                objetos.push(Box::new(Cube {
                    center: Vec3::new(contador2X, 1.0, 1.0),
                    size: 0.5,
                    material: papel.clone(),
                    velocidad: vec3(0.0, 0.0, 0.0),
                }));
            }

            contador2X += 1.0;
        }
        contadorX = -0.5;
        contador2X = 0.0;
        while contadorX < 1.5 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contadorX, 1.5, 1.0),
                size: 0.5,
                material: uvg.clone(),
                velocidad: vec3(0.0, 0.0, 0.0),
            }));
            contadorX += 1.0;
        }
        objetos.push(Box::new(Cube {
            center: Vec3::new(contador2X, 1.5, 1.0),
            size: 0.5,
            material: papel.clone(),
            velocidad: vec3(0.0, 0.0, 0.0),
        }));
        contadorX = 0.0;
        while contadorX < 1.0 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contadorX, 2.0, 1.0),
                size: 0.5,
                material: uvg.clone(),
                velocidad: vec3(0.0, 0.0, 0.0),
            }));
            contadorX += 1.0;
        }
        let mut contador1x = -2.0;
        let mut contador1y = 0.5;

        while contador1x < 0.5 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contador1x, contador1y, 1.0),
                size: 0.5,
                material: madera.clone(),
                velocidad: vec3(0.0, 0.0, 0.0),
            }));
            contador1x += 0.5;
            contador1y += 0.5;
        }

        let mut contador1x = 0.5;
        let mut contador1y = 2.0;
        while contador1x < 2.5 {
            objetos.push(Box::new(Cube {
                center: Vec3::new(contador1x, contador1y, 1.0),
                size: 0.5,
                material: madera.clone(),
                velocidad: vec3(2.0, 0.0, 0.0),
            }));

            contador1x += 0.5;
            contador1y -= 0.5;
        }

        let mut cubo_agua = Cube::new(
            vec3(-2.0, -1.0, 0.0), // Posición inicial
            1.0,                 // Tamaño
            agua.clone(),
            vec3(0.0, 0.0, 0.0), // Velocidad inicial (quieto)
        );
        let delta_tiempo = 0.016; 
        cubo_agua.actualizar_posicion(delta_tiempo);

        // Añadir el cubo de agua a la lista de objetos para que se renderice
        objetos.push(Box::new(cubo_agua.clone()));

        render(&mut framebuffer, &objetos, &camera, &light, &color_actual);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
