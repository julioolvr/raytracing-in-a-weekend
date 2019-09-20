#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn scale(&self, factor: f64) -> Vector3 {
        Vector3::new(self.x * factor, self.y * factor, self.z * factor)
    }

    pub fn unit(&self) -> Vector3 {
        self.scale(1.0 / self.magnitude())
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
