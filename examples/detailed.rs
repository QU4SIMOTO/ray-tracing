use ray_tracing::{
    camera::CameraBuilder,
    colour::Colour,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    random::{random_f32, random_f32_bounded, random_vec3, random_vec3_bounded},
    sphere::Sphere,
    Point3, Vec3,
};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();

    let ground_material = Rc::new(Lambertian::new(&Colour::new(0.5, 0.5, 0.5)));

    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point3::new(
                a as f32 + 0.9 * random_f32(),
                0.2,
                b as f32 + 0.9 * random_f32(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vec3() * random_vec3();
                    let sphere_material = Rc::new(Lambertian::new(&albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3_bounded(0.5, 1.0);
                    let fuzz = random_f32_bounded(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(&Colour::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(&Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = CameraBuilder::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Point3::new(13.0, 2.0, 3.0))
        .lookat(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    cam.render(stdout, &mut world)?;

    Ok(())
}
