use ray_tracing::{
    camera::CameraBuilder,
    hittable::HittableList,
    hittable_list,
    material::{Dielectric, Lambertian, Metal},
    Colour, Point3, Sphere, Vec3,
};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();

    let material_ground = Rc::new(Lambertian::new(&Colour::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Colour::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(&Colour::new(0.8, 0.6, 0.2), 1.0));

    let mut world = hittable_list![
        Rc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground
        )),
        Rc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            material_center
        )),
        Rc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left
        )),
        Rc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.4,
            material_bubble
        )),
        Rc::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right
        )),
    ];

    let mut cam = CameraBuilder::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Point3::new(-2.0, 2.0, 1.0))
        .lookat(Point3::new(0.0, 0.0, -1.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(10.0)
        .focus_dist(3.4)
        .build();

    cam.render(stdout, &mut world)?;
    Ok(())
}
