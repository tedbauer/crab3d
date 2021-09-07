pub type Matrix4x4 = [[f64; 4]; 4];

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec4> for Vec3 {
    fn from(v: Vec4) -> Self {
        (v.x, v.y, v.z).into()
    }
}

impl Vec3 {
    pub fn multiply_by_scalar(&self, scalar: f64) -> Self {
        (self.x * scalar, self.y * scalar, self.z * scalar).into()
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let l = self.length();
        (self.x / l, self.y / l, self.z / l).into()
    }

    pub fn add(&self, other: &Vec3) -> Self {
        (self.x + other.x, self.y + other.y, self.z + other.z).into()
    }

    pub fn minus(&self, other: &Vec3) -> Self {
        (self.x - other.x, self.y - other.y, self.z - other.z).into()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        (
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
            .into()
    }
}

#[derive(Clone, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<(f64, f64, f64, f64)> for Vec4 {
    fn from((x, y, z, w): (f64, f64, f64, f64)) -> Self {
        Self { x, y, z, w }
    }
}

impl Vec4 {
    pub fn multiply_by_scalar(&self, scalar: f64) -> Self {
        Vec4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn multiply_by_matrix(&self, matrix: &Matrix4x4) -> Self {
        let mult_col = |col| {
            self.x * matrix[0][col]
                + self.y * matrix[1][col]
                + self.z * matrix[2][col]
                + self.w * matrix[3][col]
        };

        Vec4 {
            x: mult_col(0),
            y: mult_col(1),
            z: mult_col(2),
            w: mult_col(3),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

impl Triangle {
    pub fn transform(&self, f: impl Fn(&Vec3) -> Vec3) -> Self {
        Self(f(&self.0), f(&self.1), f(&self.2))
    }
}
