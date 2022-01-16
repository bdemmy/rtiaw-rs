use std::ops::Neg;
use rand;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
   pub e: [f64; 3]
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
   pub fn new_empty() -> Vec3 {
      Vec3{
         e: [0.0, 0.0, 0.0]
      }
   }

   pub fn new<T: Into<f64>, U: Into<f64>, V: Into<f64>>(e0: T, e1: U, e2: V) -> Vec3 {
      Vec3 {
         e: [e0.into(), e1.into(), e2.into()]
      }
   }

   #[inline(always)]
   pub fn x(&self) -> f64 {
      self.e[0]
   }

   #[inline(always)]
   pub fn y(&self) -> f64 {
      self.e[1]
   }

   #[inline(always)]
   pub fn z(&self) -> f64 {
      self.e[2]
   }

   pub fn cross(&self, rhs: &Vec3) -> Vec3 {
      Vec3 {
         e: [
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]
         ]
      }
   }

   #[inline(always)]
   pub fn dot(&self, rhs: &Vec3) -> f64 {
      self.e[0] * rhs.e[0]
          + self.e[1] * rhs.e[1]
          + self.e[2] * rhs.e[2]
   }

   #[inline(always)]
   pub fn length(&self) -> f64 {
      self.length_squared().sqrt()
   }

   #[inline(always)]
   pub fn length_squared(&self) -> f64 {
      self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
   }

   #[inline(always)]
   pub fn normalize(&mut self) {
      *self = *self / self.length();
   }

   #[inline(always)]
   pub fn normalized(&self) -> Vec3 {
      *self / self.length()
   }

   pub fn random() -> Vec3 {
      let mut rng = rand::thread_rng();

      Vec3::new(
         rng.gen_range(-1.0..1.0),
         rng.gen_range(-1.0..1.0),
         rng.gen_range(-1.0..1.0)
      )
   }

   pub fn random_range(min: f64, max: f64) -> Vec3 {
      let mut rng = rand::thread_rng();

      Vec3::new(
         rng.gen_range(min..max),
         rng.gen_range(min..max),
         rng.gen_range(min..max)
      )
   }

   pub fn random_in_unit_sphere() -> Vec3 {
      let mut rng = rand::thread_rng();
      loop {
         let p = Vec3::random_range(-1.0, 1.0);
         if p.length_squared() >= 1.0 {
            continue
         }
         return p;
      }
   }

   pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
      let in_sphere = Vec3::random_in_unit_sphere();
      return if in_sphere.dot(normal) > 0.0 {
         in_sphere
      } else {
         in_sphere * -1.0
      }
   }

   pub fn random_unit_vector() -> Vec3 {
      Vec3::random_in_unit_sphere().normalized()
   }

   pub fn near_zero(&self) -> bool {
      let s = 1e-8;
      return (self.e[0].abs() < s) & (self.e[1].abs() < s) & (self.e[2].abs() < s);
   }

   #[inline(always)]
   pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
      *v - *n * 2.0 * v.dot(n)
   }

   pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
      let negated = *uv * -1.0;
      let cos_theta = 1.0_f64.min(n.dot(&negated));
      let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
      let r_out_parallel = *n * (-((1.0 - r_out_perp.length_squared()).abs().sqrt()));
      return r_out_perp + r_out_parallel;
   }

   pub fn random_in_unit_disk() -> Vec3 {
      loop {
         let mut p = Vec3::random_range(-1.0, 1.0);
         p.e[2] = 0.0;
         if p.length_squared()  >= 1.0 {
            continue;
         }
         return p;
      }
   }
}

impl std::ops::Add<Vec3> for Vec3 {
   type Output = Vec3;

   fn add(self, rhs: Vec3) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]
         ]
      }
   }
}
impl std::ops::Sub<Vec3> for Vec3 {
   type Output = Vec3;

   fn sub(self, rhs: Vec3) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]
         ]
      }
   }
}
impl std::ops::Mul<Vec3> for Vec3 {
   type Output = Vec3;

   fn mul(self, rhs: Vec3) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2]
         ]
      }
   }
}
impl std::ops::Div<Vec3> for Vec3 {
   type Output = Vec3;

   fn div(self, rhs: Vec3) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2]
         ]
      }
   }
}

impl std::ops::Div<f64> for Vec3 {
   type Output = Vec3;

   fn div(self, rhs: f64) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs
         ]
      }
   }
}
impl std::ops::Mul<f64> for Vec3 {
   type Output = Vec3;

   fn mul(self, rhs: f64) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs
         ]
      }
   }
}
impl std::ops::Sub<f64> for Vec3 {
   type Output = Vec3;

   fn sub(self, rhs: f64) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] - rhs,
            self.e[1] - rhs,
            self.e[2] - rhs
         ]
      }
   }
}
impl std::ops::Add<f64> for Vec3 {
   type Output = Vec3;

   fn add(self, rhs: f64) -> Self::Output {
      Vec3 {
         e: [
            self.e[0] + rhs,
            self.e[1] + rhs,
            self.e[2] + rhs
         ]
      }
   }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
   fn add_assign(&mut self, rhs: Vec3) {
      self.e[0] += rhs.e[0];
      self.e[1] += rhs.e[1];
      self.e[2] += rhs.e[2];
   }
}
impl std::ops::SubAssign<Vec3> for Vec3 {
   fn sub_assign(&mut self, rhs: Vec3) {
      self.e[0] -= rhs.e[0];
      self.e[1] -= rhs.e[1];
      self.e[2] -= rhs.e[2];
   }
}
impl std::ops::DivAssign<Vec3> for Vec3 {
   fn div_assign(&mut self, rhs: Vec3) {
      self.e[0] /= rhs.e[0];
      self.e[1] /= rhs.e[1];
      self.e[2] /= rhs.e[2];
   }
}
impl std::ops::MulAssign<Vec3> for Vec3 {
   fn mul_assign(&mut self, rhs: Vec3) {
      self.e[0] *= rhs.e[0];
      self.e[1] *= rhs.e[1];
      self.e[2] *= rhs.e[2];
   }
}

impl Neg for Vec3 {
   type Output = Vec3;

   fn neg(self) -> Self::Output {
      return self * -1.0;
   }
}