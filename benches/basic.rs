use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing::{
    camera::CameraBuilder, hittable_list, material::Lambertian, Colour, Point3, Sphere, Vec3,
};
use std::rc::Rc;

fn basic() {
    let stdout = std::io::stdout();

    let r = (std::f32::consts::PI / 4.0).cos();

    let material_left = Lambertian::new(&Colour::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(&Colour::new(1.0, 0.0, 0.0));

    let mut world = hittable_list![
        Rc::new(Sphere::new(
            Point3::new(-r, 0.0, -1.0),
            r,
            Rc::new(material_left),
        )),
        Rc::new(Sphere::new(
            Point3::new(r, 0.0, -1.0),
            r,
            Rc::new(material_right),
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

    cam.render(stdout, &mut world).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic", |b| b.iter(|| basic()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
