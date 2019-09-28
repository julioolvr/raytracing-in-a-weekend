#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn unit(&self) -> Vector3 {
        *self * (1.0 / self.magnitude())
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn squared_length(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(&self) -> f64 {
        self.squared_length().sqrt()
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

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Vector3) -> Self {
        // This is weird
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::new(rand::random(), rand::random(), rand::random()) * 2.0
            - Vector3::new(1.0, 1.0, 1.0);

        if p.squared_length() <= 1.0 {
            return p;
        }
    }
}

pub fn reflect(vec_in: Vector3, normal: Vector3) -> Vector3 {
    vec_in - 2.0 * vec_in.dot(normal) * normal
}
