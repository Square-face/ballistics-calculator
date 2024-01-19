/// A 3 dimensional Cartesian vector
///
/// Represents a vector using Cartesian coordinates.
///
/// # Examples
/// ```rust
/// use ballistics_calculator::types::Vec3D;
/// let v = Vec3D::new(1.0, 2.0, 3.0);
///
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// assert_eq!(v.z, 3.0);
/// ```
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Vec3D {
    /// The vectors size in the x direction
    pub x: f64,

    /// The vectors size in the y direction
    pub y: f64,

    /// The vectors size in the z direction
    pub z: f64,
}

/// A 2 dimensional Cartesian vector
///
/// Represents a vector using Cartesian coordinates.
///
/// # Examples
/// ```rust
/// use ballistics_calculator::types::Vec2D;
/// let v = Vec2D::new(1.0, 2.0);
///
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// ```
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Vec2D {
    /// The vectors size in the x direction
    pub x: f64,

    /// The vectors size in the y direction
    pub y: f64,
}

/// A 3 dimensional spherical vector
///
/// Represents a vector using spherical coordinates.
pub struct Vec3DSphere {
    /// Horizontal angle from the x axis
    pub azimuth: f64,

    /// Angle from the z axis
    pub polar: f64,

    /// Distance from origin
    pub radius: f64,
}

/// A 2 dimensional spherical vector
///
/// Represents a vector using spherical coordinates.
pub struct Vec2DSphere {
    /// Horizontal angle from the x axis
    pub polar: f64,

    /// Distance from origin
    pub radius: f64,
}

#[allow(dead_code)]
impl Vec3D {
    /// Creates a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D { x, y, z }
    }

    /// Returns the length of the vector
    ///
    /// The length is calculated using the Pythagorean theorem.
    /// sqrt(x^2 + y^2 + z^2)
    ///
    /// # Examples
    /// ```rust
    /// use ballistics_calculator::types::Vec3D;
    /// let v = Vec3D::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.length(), 5.0);
    /// ```
    pub fn length(&self) -> f64 {
        let sqrt_sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        (sqrt_sum).sqrt()
    }

    /// Returns the length on the xy plane only
    ///
    /// The length is calculated using the Pythagorean theorem.
    /// sqrt(x^2 + y^2)
    ///
    /// # Examples
    /// ```rust
    /// use ballistics_calculator::types::Vec3D;
    /// let v = Vec3D::new(3.0, 4.0, 3.0);
    /// assert_eq!(v.length_xy(), 5.0);
    /// ```
    pub fn length_xy(&self) -> f64 {
        let sqrt_sum = self.x.powi(2) + self.y.powi(2);
        (sqrt_sum).sqrt()
    }

    /// Updates the vector to a new lengthe
    ///
    /// Recalculates the vector to have the given length while maintaining the
    /// ratio of the components.
    ///
    /// # Arguments
    /// * `new` - The new length of the vector
    ///
    /// # Examples
    /// ```rust
    /// use ballistics_calculator::types::Vec3D;
    /// let mut v = Vec3D::new(3.0, 4.0, 3.0);
    ///
    /// v.update_length(15.0);
    /// assert_eq!(v.length().round(), 15.0);
    /// ```
    pub fn update_length(&mut self, new: f64) {
        let mut sphere = self.to_sphere();
        sphere.radius = new;
        let cartesian = sphere.to_vec();

        self.x = cartesian.x;
        self.y = cartesian.y;
        self.z = cartesian.z;
    }

    /// Converts the vector into its 2D projection
    ///
    /// The x cordinate is the square root of the sum
    /// of the squares of the x and y axis.
    ///
    /// The y cordinate is the z axis.
    ///
    /// # Examples
    /// ```rust
    /// use ballistics_calculator::types::Vec3D;
    /// let v = Vec3D::new(3.0, 4.0, 3.0);
    /// let v2d = v.to_2d();
    ///
    /// assert_eq!(v2d.x, 5.0);
    /// assert_eq!(v2d.y, 3.0);
    /// ```
    pub fn to_2d(&self) -> Vec2D {
        Vec2D {
            x: self.length_xy(),
            y: self.z,
        }
    }

    /// Converts the Cartesian vector to spherical coordinates
    ///
    /// # Examples
    /// ```rust
    /// use ballistics_calculator::types::Vec3D;
    /// let v = Vec3D::new(1.0, 1.0, 1.0);
    /// let v_sphere = v.to_sphere();
    ///
    /// assert_eq!(v_sphere.radius, (1f64 + 1f64 + 1f64).sqrt());
    /// assert_eq!(v_sphere.azimuth, 45f64.to_radians());
    /// ```
    pub fn to_sphere(&self) -> Vec3DSphere {
        Vec3DSphere {
            radius: self.length(),
            azimuth: (self.y / self.x).atan(),
            polar: (self.z / self.length_xy()).atan(),
        }
    }
}

impl Vec3DSphere {
    /// Converts the spherical vector to a Cartesian vector
    ///
    /// # Examples
    /// ```rust
    /// use ballistics_calculator::types::Vec3DSphere;
    /// let v_sphere = Vec3DSphere {
    ///     radius: 5.0,
    ///     azimuth: 45f64.to_radians(),
    ///     polar: 45f64.to_radians(),
    /// };
    ///
    /// let v = v_sphere.to_vec().to_sphere();
    ///
    /// assert_eq!(v.radius, 5.0);
    /// assert_eq!(v.azimuth.to_degrees().round(), 45.);
    /// assert_eq!(v.polar.to_degrees().round(), 45.);
    /// ```
    pub fn to_vec(&self) -> Vec3D {
        Vec3D {
            x: self.radius * self.azimuth.cos() * self.polar.sin(),
            y: self.radius * self.azimuth.sin() * self.polar.sin(),
            z: self.radius * self.polar.cos(),
        }
    }
}

#[allow(dead_code)]
impl Vec2D {
    /// Creates a new 2D vector
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f64 {
        todo!("Implement this function");
    }

    /// Updates the vector to a new lengthe
    ///
    /// Recalculates the vector to have the given length while maintaining the
    /// ratio of the components.
    pub fn update_length(&mut self, _new: f64) {
        todo!("Implement this function");
    }

    /// Converts the Cartesian vector to spherical coordinates
    pub fn to_sphere(&self) -> Vec2DSphere {
        todo!("Implement this function");
    }
}

#[cfg(test)]
mod vec_3d {
    #[test]
    fn new() {
        let v = super::Vec3D::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn length() {
        let v = super::Vec3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0, "Length of 3, 4, 0 should be 5");

        let v = super::Vec3D::new(1.0, 1.0, 1.0);
        assert_eq!(
            v.length(),
            (1f64 + 1f64 + 1f64).sqrt(),
            "Length of 1, 1, 1 should be sqrt(3)"
        );
    }
}
