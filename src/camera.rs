use std::io::Write;

use crate::{
    colour::{write_colour, Colour},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    random::{random_f32, random_in_unit_disk},
    util::degrees_to_radians,
    Point3, Ray, Vec3,
};

pub struct Camera {
    /// Ratio of image width over height.
    #[allow(unused)]
    aspect_ratio: f32,
    /// Rendered image width in pixels.
    image_width: u32,
    /// Count of random samples for each pixel.
    samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene.
    max_depth: u32,
    /// Vertical view angle (field of view)
    #[allow(unused)]
    vfov: f32,
    /// Point camera is looking from
    #[allow(unused)]
    lookfrom: Point3,
    /// Point camera is looking at
    #[allow(unused)]
    lookat: Point3,
    /// Camera-relative "up" direction
    #[allow(unused)]
    vup: Vec3,
    /// Variation angle of rays through each pixel
    defocus_angle: f32,
    /// Distance from camera lookfrom point to plane of perfect focus
    #[allow(unused)]
    focus_dist: f32,
    /// Rendered image height
    image_height: u32,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Color scale factor for a sum of pixel samples
    pixel_sample_scale: f32,
    /// Camera frame basis vector
    #[allow(unused)]
    u: Vec3,
    /// Camera frame basis vector
    #[allow(unused)]
    v: Vec3,
    /// Camera frame basis vector
    #[allow(unused)]
    w: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(
        &mut self,
        mut stdout: impl Write,
        world: &impl Hittable,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.max_depth, world);
                }
                write_colour(&mut stdout, &(self.pixel_sample_scale * pixel_colour))?;
            }
        }

        eprintln!("Done.");
        Ok(())
    }

    fn ray_colour(&self, r: &Ray, depth: u32, world: &impl Hittable) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Colour::default();
            if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * self.ray_colour(&mut scattered, depth - 1, world);
                }
            }
            return Colour::default();
        }
        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        // auto ray_origin = (defocus_angle <= 0) ? center : defocus_disk_sample();
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CameraBuilder {
    aspect_ratio: f32,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f32,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    defocus_angle: f32,
    focus_dist: f32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Default::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: Default::default(),
            focus_dist: Default::default(),
        }
    }
}

impl CameraBuilder {
    /// Ratio of image width over height.
    pub fn aspect_ratio(self, aspect_ratio: f32) -> Self {
        Self {
            aspect_ratio,
            ..self
        }
    }

    /// Rendered image width in pixels.
    pub fn image_width(self, image_width: u32) -> Self {
        Self {
            image_width,
            ..self
        }
    }

    /// Count of random samples for each pixel.
    pub fn samples_per_pixel(self, samples_per_pixel: u32) -> Self {
        Self {
            samples_per_pixel,
            ..self
        }
    }

    /// Maximum number of ray bounces into scene.
    pub fn max_depth(self, max_depth: u32) -> Self {
        Self { max_depth, ..self }
    }

    /// Vertical view angle (field of view)
    pub fn vfov(self, vfov: f32) -> Self {
        Self { vfov, ..self }
    }

    /// Point camera is looking from
    pub fn lookfrom(self, lookfrom: Vec3) -> Self {
        Self { lookfrom, ..self }
    }

    /// Point camera is looking at
    pub fn lookat(self, lookat: Vec3) -> Self {
        Self { lookat, ..self }
    }

    /// Camera-relative "up" direction
    pub fn vup(self, vup: Vec3) -> Self {
        Self { vup, ..self }
    }

    /// Variation angle of rays through each pixel
    pub fn defocus_angle(self, defocus_angle: f32) -> Self {
        Self {
            defocus_angle,
            ..self
        }
    }

    /// Distance from camera lookfrom point to plane of perfect focus
    pub fn focus_dist(self, focus_dist: f32) -> Self {
        Self { focus_dist, ..self }
    }

    pub fn build(self) -> Camera {
        let image_height = (self.image_width as f32 / self.aspect_ratio).floor() as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };
        let pixel_sample_scale = 1.0 / self.samples_per_pixel as f32;
        let center = self.lookfrom;

        // Determine the viewport dimensions.
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (self.image_width as f32);
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            vfov: self.vfov,
            lookfrom: self.lookfrom,
            lookat: self.lookat,
            vup: self.vup,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_sample_scale,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
