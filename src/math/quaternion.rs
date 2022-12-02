use super::vec3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub data: [f32; 4]
}

#[allow(dead_code)]
impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { data: [
            x,
            y,
            z,
            w
        ] }
    }

    pub fn euler(x: f32, y: f32, z: f32) -> Self {
        let cz = (z * 0.5).cos();
        let sz = (z * 0.5).sin();
        let cy = (y * 0.5).cos();
        let sy = (y * 0.5).sin();
        let cx = (x * 0.5).cos();
        let sx = (x * 0.5).sin();

        Quaternion { data: [
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        ] }
    }

    pub fn identity() -> Self {
        Self::euler(0.0, 0.0, 0.0)
    }

    pub fn euler_angles(&self) -> Vector3 {
        // TODO: figure out what is wrong with this function
        let x: usize = 0;
        let y: usize = 1;
        let z: usize = 2;
        let w: usize = 3;

        let sinr_cosp = 2.0 * (self.data[w] * self.data[x] + self.data[y] + self.data[z]);
        let cosr_cosp = 1.0 - (2.0 * ((self.data[x] * self.data[x]) + (self.data[y] * self.data[y])));
        
        let out_y: f32;
        let sinp = 2.0 * (self.data[w] * self.data[y] - self.data[z] * self.data[x]);
        
        if sinp.abs() >= 1.0 {
            out_y = std::f32::consts::FRAC_PI_2.copysign(sinp);
        } else {
            out_y = sinp.asin();
        }

        let siny_cosp= 2.0 * (self.data[w] * self.data[z] + self.data[x] * self.data[y]);
        let cosy_cosp = 1.0 - (2.0 * ((self.data[y] * self.data[y]) + (self.data[z] * self.data[z])));

        let out_x = sinr_cosp.atan2(cosr_cosp);
        let out_z = siny_cosp.atan2(cosy_cosp);

        Vector3::new(out_x, out_y, out_z)
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}