#![feature(int_roundings)]
use std::ops::Mul;

use color::OpaqueColor;
use image::{
    ImageBuffer,
    Rgba,
};

use crate::math::{
    Vec2,
    Vec3,
};

mod math;

include!(concat!(env!("OUT_DIR"), "/cie.rs"));

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const WAVE_STEP: u16 = 35; // Step size in nm

// Bounds in which we measure the wavelength of a ray
const WAVE_START: u16 = 360;
const WAVE_END: u16 = 830;

trait Material {
    // Given a incident and refracted ray, each with their respective wavelength,
    // return the ratio of strength as a value >0
    fn transmittance(
        &self,
        incident: Ray,
        refracted: Ray,
    ) -> f64;

    // Spectral Power Distribution?

    /// Returns the Spectral Radiance emitted from a point on the source towards
    /// a target
    // Could've done two Vec3s (src, target), but then that would require both are
    // in the same coordinate space, which isn't always possible/wanted
    fn emmitance(
        &self,
        target: Ray,
        distance: f64,
    ) -> f64;
}

#[derive(Debug, Clone, Copy)]
enum CIEIlluminant {
    TypeD {
        temp: f64, // In kelvin
    },
}

impl CIEIlluminant {
    // not exactly 6500K due to legacy reasons
    const D65: Self = Self::TypeD { temp: 6503.51 };

    pub fn spectral_power(
        self,
        wavelength: u16,
    ) -> f64 {
        match self {
            Self::TypeD { temp } => {
                // Compute chromaticity coordinates
                let t = Vec3::new(
                    10f64.powi(3) / temp,
                    10f64.powi(6) / temp.powi(2),
                    10f64.powi(9) / temp.powi(3),
                );
                let x = match temp {
                    4000f64..=7000f64 => 0.244_063 + t.dot(&Vec3::new(0.09911, 2.9678, -4.6070)),
                    7000f64..=25000f64 => 0.247_040 + t.dot(&Vec3::new(0.24748, 1.9018, -2.0064)),
                    _ => unreachable!("Type-D Illuminant not defined for specified temprature"),
                };
                let y = (-3.0 * x).mul_add(x, 2.87 * x) - 0.275;

                // In W/m2/nm
                let m = y.mul_add(-0.7341, x.mul_add(0.2562, 0.0241));
                Vec3::new(
                    1.0,
                    y.mul_add(5.9114, x.mul_add(-1.7703, -1.3515)) / m,
                    y.mul_add(30.0717, x.mul_add(-31.4424f64, 0.03)) / m,
                )
                .dot(&cie_daylight(wavelength))
            },
        }
    }
}

fn scale_intensity(
    target: f64,
    intensity: impl Fn(u16) -> f64,
) -> f64 {
    // "Integrate" radiant intensity over full spectrum
    let luma = (WAVE_START..=WAVE_END)
        .step_by(5)
        .map(|wavelength| intensity(wavelength) * cie_xyz(wavelength).y)
        .sum::<f64>()
        * 5.0
        * 683.002;
    target / luma
}

enum Primative {
    Sphere { origin: Vec3, radius: f64 },
}

struct IntersectionResult {
    along:  f64,
    normal: Vec3,
}

impl Primative {
    pub fn intersects(
        &self,
        ray: &Ray,
    ) -> Option<IntersectionResult> {
        match self {
            Self::Sphere { origin, radius } => {
                // Direction is normalized

                // Circle = x^2 + y^2 = r^2
                // General: |P - c|^2 = r^2

                // Line: o + du = P
                // (t, P) = set of points along line

                // Combined: |o + du - c|^2 = r^2  -- Quadradic
                let center = *origin - ray.origin;
                let b2 = ray.direction.dot(&center);
                let delta = (b2.mul_add(b2, -center.length_squared()) + radius * radius).sqrt();
                if delta.is_nan() {
                    return None;
                }

                // Since delta >= 0, we have an intersection
                let along = if delta > b2 { b2 + delta } else { b2 - delta };
                let normal = (ray.travel(along) - *origin).normalized();
                Some(IntersectionResult { along, normal })
            },
        }
    }
}

struct Object {
    prim:     Primative,
    material: Box<dyn Material>,
}

struct Ray {
    origin:    Vec3<f64>,
    direction: Vec3<f64>,

    wavelength: u16, // Measured in nanometre
}

impl Ray {
    pub fn travel(
        &self,
        along: f64,
    ) -> Vec3 {
        self.origin + self.direction * along
    }
}

// Visible spectrum of light is 380-780nm
// Red = 625-750 nm, Human Peak at 560nm
// Green = 500-565 nm, Human Peak at 530nm
// Blue = 450-485 nm, Human Peak at 420nm
// Violet = 380-450 nm

// https://en.wikipedia.org/wiki/CIE_1931_color_space#Computing_XYZ_from_spectral_data
// CIE XYZ color space converts visible spectrum to something more managable.
// It can be computed by integrating the product of the Spectral Radiance and
// the xyz function over all possible wavelengths, though [380, 780] is
// typically good enough.

// https://en.wikipedia.org/wiki/Spectral_radiance
// Spectral Radiance is represented in the unit W sr-1 m-3 (Watt per Steradian
// per Sq Metre per Metre), which is an absolute unit of a unit.
// This represents the rate of radiative transfer of energy at a given Point and
// Time. This is also dependent on the wavelength being emitted.
// Wikipedia has this as dE = I(x,t,r,v)cos(theta) dA dO dv dt
// where dE is the total energy over time released between two surfaces;
// indicated by x being the source and r being the unit vector towards the
// detector.

// To model surfaces, we should take in a point and incident ray, and return an
// array of reflected rays, along with a function specifying how much of
// each wavelength is absorbed along the route.
// Since we're Monte-Carlo, we should collapse this "function" into just
// returning reflected ray and function of wavelength.
// Since we're also going inverse-ray, we should also probably have it compute
// it from the reflected ray and create a distribution of possible incident
// rays.

// So surfaces should be modeled using two functions;
// - Takes in Incident and Reflected rays, returns function of wavelength
//   transmittance
// - Takes in Reflected ray, returns probability field of possible Incident rays
// Technically the 2nd function isnt required, but it makes rendering much
// nicer/faster.

// The issue now is that there's a distinction between emitters and reflectors
// now. One way to work around that is to have rays not ever stop at a
// "emitter", and instead have the BSRF defined above return a >1 value for
// the wavelength it emits. However now that has the problem of what the
// fuck value do we start with???

// Ok so I think its better if we just have surfaces pull double duties.
// So they have the BSRF, which defines how rays interact along a surface.
// They also have an "emittance" function, aka something to represent the
// Spectral Radiance at an area around a point.
// This allows us to use the BSRF to compute a path our ray can take, then work
// backwards along the ray back towards the camera, computing Spectral Radiance
// at every incident point to work out the Spectral Radiance of the directly hit
// surface, which we can then CIE the shit out of.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera_origin = Vec3::splat(0.0);
    let camera_view = Vec3::new(1.0, 0.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);

    let focus_distance = 1.0;
    let sensor_height = 2.0;
    let sensor_width = sensor_height * (f64::from(WIDTH) / f64::from(HEIGHT));

    // Implicitly normalized
    let camera_left = Vec3::cross(&camera_up, &camera_view);
    let viewport_left = camera_left * -sensor_width;
    let viewport_up = Vec3::cross(&camera_view, &camera_left) * sensor_height;

    let focal_point = camera_origin + camera_view * focus_distance;
    let top_left = focal_point + (viewport_up + viewport_left) * 0.5;

    let delta_u = -viewport_left / f64::from(WIDTH);
    let delta_v = -viewport_up / f64::from(HEIGHT);

    let sphere = Primative::Sphere {
        origin: Vec3::new(2.0, 0.0, 0.0),
        radius: 1.0,
    };

    // Cache this value, since computing it is expensive
    let d65_scale = scale_intensity(300.0, |wavelength| {
        CIEIlluminant::D65.spectral_power(wavelength)
    });

    let mut whitepoint = Vec3::splat(0.0);

    let mut pixels = Vec::with_capacity(WIDTH as usize * HEIGHT as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let origin = top_left + delta_u * f64::from(x) + delta_v * f64::from(y);
            let direction = (origin - camera_origin).normalized();

            let mut xyz = Vec3::splat(0.0);
            for wavelength in (WAVE_START..=WAVE_END).step_by(WAVE_STEP as usize) {
                // Compute spectral radiance for certain wavelength
                let ray = Ray {
                    origin: camera_origin,
                    direction,
                    wavelength,
                };

                // Send out ray to world
                let spectral =
                    if let Some(IntersectionResult { along, normal }) = sphere.intersects(&ray) {
                        // Emit as if D65 Illuminant
                        let power = CIEIlluminant::D65.spectral_power(wavelength) * d65_scale;
                        let ratio = (-direction).dot(&normal); // The more of an angle we view from, the less powerful it should be
                        power * ratio
                    } else {
                        0.0
                    };

                // Expects W/sr/m2/nm
                xyz += cie_xyz(wavelength) * spectral * f64::from(WAVE_STEP);
            }

            // TODO: Cover reflective case

            // Find whitepoint (largest luminance) in render
            if xyz.y > whitepoint.y {
                whitepoint = xyz;
            }

            pixels.push(xyz);
        }
    }

    println!("{whitepoint:?}");

    // Write to image file
    ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        // Tonemap (Reinhard Extended)
        let pixel = pixels[(y * WIDTH + x) as usize];
        let luma = (pixel.y * (1.0 + pixel.y / whitepoint.y.powi(2))) / (1.0 + pixel.y);
        let scale = luma / pixel.y;

        // Map XYZ to sRGB
        #[allow(clippy::cast_possible_truncation)]
        let xyz = pixel.as_array().map(|v| (v * scale) as f32); // Ensures scaled to y=1.0 at D65 white (probably)
        let xyz: OpaqueColor<color::XyzD65> = OpaqueColor::new(xyz);
        let srgb: OpaqueColor<color::Srgb> = xyz.convert();

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        Rgba(srgb.to_rgba8().to_u8_array())
    })
    .save("render.png")?;
    Ok(())
}
