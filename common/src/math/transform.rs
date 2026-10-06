use glam::{Vec3, Quat, Mat4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    /// Creates an identity transformation.
    pub const fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Calculates and returns the resulting 4x4 model matrix.
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale, 
            self.rotation, 
            self.position,
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}