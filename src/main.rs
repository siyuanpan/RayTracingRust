mod aabb;
mod bvh;
mod camera;
mod cube;
mod hittable;
mod material;
mod medium;
mod moving_sphere;
mod perlin;
mod ray;
mod rect;
mod rotate;
mod sphere;
mod texture;
mod translate;
mod vec3;
mod world;

use bvh::BVH;
use camera::Camera;
use cube::Cube;
use hittable::Hittable;
use image;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use medium::ConstantMedium;
use moving_sphere::MovingSphere;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use rect::Rect;
use rotate::{Axis, Rotate};
use sphere::Sphere;
use std::sync::Arc;
use texture::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use translate::Translate;
use vec3::{Color, Point3, Vec3, VectorConst};
use world::{HitableList, World};

fn format_color(color: &Color, samples_per_pixel: u32) -> String {
    format!(
        "{} {} {}",
        (255.999
            * (color[0] / (samples_per_pixel as f32))
                .sqrt()
                .clamp(0.0, 0.999)) as u64,
        (255.999
            * (color[1] / (samples_per_pixel as f32))
                .sqrt()
                .clamp(0.0, 0.999)) as u64,
        (255.999
            * (color[2] / (samples_per_pixel as f32))
                .sqrt()
                .clamp(0.0, 0.999)) as u64
    )
}

fn ray_color(r: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        if depth > 0 {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                //  attenuation * ray_color(&scattered, world, depth - 1)
                return emitted + attenuation * ray_color(&scattered, world, depth - 1);
            }
        }

        emitted
    } else {
        Color::ZERO
        // Color::new(0.70, 0.80, 1.00)
        // let unit_direction = r.direction().normalized();
        // let t = 0.5 * (unit_direction.y() + 1.0);
        // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

#[allow(dead_code)]
fn random_scene() -> Box<dyn Hittable> {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    // let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_mat = Arc::new(Lambertian::new(CheckerTexture::new(
        ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
    )));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f32) + rng.gen_range(0.0..0.9),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let color = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    let albedo = ConstantTexture::new(color);
                    let sphere_mat = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    // let sphere = Sphere::new(center, 0.2, sphere_mat);
                    let sphere = MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_mat);

                    world.push(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::new(center, 0.2, sphere_mat);

                    world.push(Box::new(sphere));
                } else {
                    // Glass
                    let sphere_mat = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere::new(center, 0.2, sphere_mat);

                    world.push(Box::new(sphere));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(ConstantTexture::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    Box::new(BVH::new(world, 0.0, 1.0))
    // Box::new(world)
}

#[allow(dead_code)]
fn two_spheres() -> Box<dyn Hittable> {
    let checker = CheckerTexture::new(
        ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
    );
    let mut world = World::new();
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));
    Box::new(world)
}

#[allow(dead_code)]
fn two_perlin_spheres() -> Box<dyn Hittable> {
    let pertext = NoiseTexture::new(4.0);
    let mut world = World::new();
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));
    Box::new(world)
}

#[allow(dead_code)]
fn earth() -> Box<dyn Hittable> {
    let image = image::open("earthmap.png")
        .expect("image not found")
        .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    let earth = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(texture)),
    );
    Box::new(earth)
}

#[allow(dead_code)]
fn simple_light() -> Box<dyn Hittable> {
    let noise = NoiseTexture::new(4.0);
    let mut world = World::new();
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(noise.clone())),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(noise)),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
            4.0, 4.0, 4.0,
        )))),
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::XY,
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
            4.0, 4.0, 4.0,
        )))),
    )));

    Box::new(world)
}

#[allow(dead_code)]
fn cornell_box() -> Box<dyn Hittable> {
    let red = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
        15.0, 15.0, 15.0,
    ))));
    let mut world = World::new();
    world.push(Box::new(Rect::new(
        rect::Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red,
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        227.0,
        332.0,
        213.0,
        343.0,
        554.0,
        light,
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    // world.push(Box::new(Translate::new(
    //     Arc::new(Cube::new(
    //         Vec3::new(0.0, 0.0, 0.0),
    //         Vec3::new(165.0, 330.0, 165.0),
    //         white.clone(),
    //     )),
    //     Vec3::new(265.0, 0.0, 295.0),
    // )));
    world.push(Box::new(Translate::new(
        Arc::new(Rotate::new(
            Axis::Y,
            Arc::new(Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    )));

    world.push(Box::new(Translate::new(
        Arc::new(Rotate::new(
            Axis::Y,
            Arc::new(Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white,
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    )));

    // Box::new(world)
    Box::new(BVH::new(world, 0.0, 1.0))
}

#[allow(dead_code)]
fn cornell_smoke() -> Box<dyn Hittable> {
    let red = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
        7.0, 7.0, 7.0,
    ))));
    let mut world = World::new();
    world.push(Box::new(Rect::new(
        rect::Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red,
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        127.0,
        432.0,
        113.0,
        443.0,
        554.0,
        light,
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Box::new(Rect::new(
        rect::Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    // let box1: Arc<dyn Hittable> = Arc::new(Translate::new(
    //     Arc::new(Rotate::new(
    //         Axis::Y,
    //         Arc::new(Cube::new(
    //             Vec3::new(0.0, 0.0, 0.0),
    //             Vec3::new(165.0, 330.0, 165.0),
    //             white.clone(),
    //         )),
    //         15.0,
    //     )),
    //     Vec3::new(265.0, 0.0, 295.0),
    // ));
    // let tex1 = Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0)));
    // ConstantMedium::new(box1, 0.01, tex1);

    world.push(Box::new(ConstantMedium::new(
        Arc::new(Translate::new(
            Arc::new(Rotate::new(
                Axis::Y,
                Arc::new(Cube::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 165.0, 165.0),
                    white.clone(),
                )),
                -18.0,
            )),
            Vec3::new(130.0, 0.0, 65.0),
        )),
        0.01,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )));

    world.push(Box::new(ConstantMedium::new(
        Arc::new(Translate::new(
            Arc::new(Rotate::new(
                Axis::Y,
                Arc::new(Cube::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 330.0, 165.0),
                    white.clone(),
                )),
                15.0,
            )),
            Vec3::new(265.0, 0.0, 295.0),
        )),
        0.01,
        Arc::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
    )));

    // world.push(Box::new(Translate::new(
    //     Arc::new(Rotate::new(
    //         Axis::Y,
    //         Arc::new(Cube::new(
    //             Vec3::new(0.0, 0.0, 0.0),
    //             Vec3::new(165.0, 165.0, 165.0),
    //             white,
    //         )),
    //         -18.0,
    //     )),
    //     Vec3::new(130.0, 0.0, 65.0),
    // )));

    // Box::new(world)
    Box::new(BVH::new(world, 0.0, 1.0))
}

fn final_scene() -> Box<dyn Hittable> {
    let mut rng = rand::thread_rng();
    let ground = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.48, 0.83, 0.53,
    ))));
    let white = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    ))));
    let mut world = World::new();
    let mut box_list1 = HitableList::new();
    let nb = 20;
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rng.gen::<f32>() + 0.01);
            let z1 = z0 + w;
            box_list1.push(Box::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    world.push(Box::new(BVH::new(box_list1, 0.0, 1.0)));
    let light = Arc::new(DiffuseLight::new(ConstantTexture::new(Vec3::new(
        7.0, 7.0, 7.0,
    ))));
    world.push(Box::new(Rect::new(
        rect::Plane::ZX,
        147.0,
        412.0,
        123.0,
        423.0,
        554.0,
        light.clone(),
    )));
    let center = Vec3::new(400.0, 400.0, 200.0);
    world.push(Box::new(MovingSphere::new(
        center,
        center + Vec3::new(30.0, 0.0, 0.0),
        0.0,
        1.0,
        50.0,
        Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(
            0.7, 0.3, 0.1,
        )))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0)),
    )));
    let boundary = Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.push(Box::new(boundary.clone()));
    world.push(Box::new(ConstantMedium::new(
        Arc::new(boundary),
        0.2,
        Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    )));
    let boundary = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.push(Box::new(ConstantMedium::new(
        Arc::new(boundary),
        0.0001,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )));
    let image = image::open("earthmap.png")
        .expect("image not found")
        .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    world.push(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(Lambertian::new(texture)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(NoiseTexture::new(0.1))),
    )));
    let mut box_list2 = HitableList::new();
    let ns = 1000;
    for _ in 0..ns {
        box_list2.push(Box::new(Sphere::new(
            Vec3::new(
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
            ),
            10.0,
            white.clone(),
        )));
    }
    world.push(Box::new(Translate::new(
        Arc::new(Rotate::new(
            Axis::Y,
            Arc::new(BVH::new(box_list2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    Box::new(world)
    // Box::new(BVH::new(world, 0.0, 1.0))
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 1.0; //16.0 / 9.0;
    const WIDTH: u64 = 800; //400;
    const HEIGHT: u64 = (WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u32 = 10000;
    const MAX_DEPTH: i32 = 50;

    // World
    // let world = random_scene();
    // let aperture = 0.1;
    // let world = two_spheres();
    // let aperture = 0;
    // let world = two_perlin_spheres();
    let aperture = 0;
    // let world = earth();

    // let world = simple_light();
    // let world = cornell_box();
    // let world = cornell_smoke();
    let world = final_scene();

    // Camera
    let lookfrom = Point3::new(478.0, 278.0, -600.0); //Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(278.0, 278.0, 0.0); //Point3::new(0.0, 0.0, 0.0);
                                                 // let aperture = 0.1;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        ASPECT_RATIO as f32,
        aperture as f32,
        dist_to_focus,
        0.0,
        1.0,
    );

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    // for j in (0..HEIGHT).rev() {
    //     eprintln!("Scanlines remaining: {}", j + 1);

    let scanline: Vec<Color> = (0..(WIDTH * HEIGHT))
        .into_par_iter()
        .map(|cnt| {
            let j = HEIGHT - cnt / WIDTH - 1;
            let i = cnt % WIDTH;
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let mut rng = rand::thread_rng();
            for _ in 0..SAMPLES_PER_PIXEL {
                // let mut rng = rand::thread_rng();
                let random_u: f32 = rng.gen();
                let random_v: f32 = rng.gen();

                let u = (i as f32 + random_u) / (WIDTH - 1) as f32;
                let v = (j as f32 + random_v) / (HEIGHT - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            pixel_color

            // println!("{}", format_color(&pixel_color, SAMPLES_PER_PIXEL));
        })
        .collect();

    for pixel_color in scanline {
        println!("{}", format_color(&pixel_color, SAMPLES_PER_PIXEL));
    }
    // }
    // eprintln!("");
    eprintln!("Done.");
}
