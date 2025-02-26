use ray_tracing::{
    camera::Camera, colour::Colour, hittable::HittableList, material::Lambertian, sphere::Sphere,
    Point3, Vec3,
};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();

    let mut world = HittableList::default();

    let r = (std::f32::consts::PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::new(&Colour::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(&Colour::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.vfov = 20.0;
    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(stdout, &mut world)?;
    Ok(())
}
