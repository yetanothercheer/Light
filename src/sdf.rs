use image::Rgb;

use crate::vec3::Vec3;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

pub struct SDF;

// SDF PRIMITIVES

fn sd_sphere(i: Vec3, p: Vec3, s: f32) -> f32 {
    (i - p).length() - s
}

fn sd_plane(i: Vec3, n: Vec3, h: f32) -> f32 {
    i.dot(n) + h
}

impl SDF {

    fn sd_scene(p: Vec3) -> f32 {
        min!(
            sd_sphere(p, Vec3::new(0.0, 0.0, 5.0), 1.0),
            sd_plane(p, Vec3::new(0.0, -1.0, 0.0), 10.0),
            sd_plane(p, Vec3::new(0.0, 0.0, -1.0), 100.0)
        )
    }

    fn cal_normal(p: Vec3) -> Vec3 {
        Vec3::new(
            SDF::sd_scene(p + Vec3::new(0.0001, 0.0, 0.0))
                - SDF::sd_scene(p - Vec3::new(0.0001, 0.0, 0.0)),
            SDF::sd_scene(p + Vec3::new(0.0, 0.0001, 0.0))
                - SDF::sd_scene(p - Vec3::new(0.0, 0.0001, 0.0)),
            SDF::sd_scene(p + Vec3::new(0.0, 0.0, 0.0001))
                - SDF::sd_scene(p - Vec3::new(0.0, 0.0, 0.0001)),
        )
        .normalized()
    }

    fn ray_march(ro: Vec3, rd: Vec3) -> Vec3 {
        let mut dist = 0.0;

        for _ in 0..256 {
            let current = ro + rd * dist;
            let dist_to_closest = SDF::sd_scene(current);
            if dist_to_closest < 0.001 {
                let n = SDF::cal_normal(current).normalized();
                let diffuse = n.dot(Vec3::new(0.0, -1.0, -0.1).normalized());
                return Vec3::new(0.7, 0.3, 0.2) * diffuse + Vec3::new(0.2, 0.2, 0.2);
            }
            if dist > 1000.0 {
                break;
            }
            dist += dist_to_closest;
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn render(x: f32, y: f32) -> Rgb<u8> {
        let ro = Vec3::new(0f32, 0f32, -1f32);
        let rd = Vec3::new(x, y, 1f32);
        SDF::ray_march(ro, rd).to_rgb_color()
    }
}
