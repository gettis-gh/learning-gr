use nalgebra::{Vector3, Unit};

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: nalgebra::UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn apply(&self, vertex: &Vector3<f32>) -> Vector3<f32> {
        // Escala manual
        let scaled = Vector3::new(
            vertex.x * self.scale.x,
            vertex.y * self.scale.y,
            vertex.z * self.scale.z,
        );

        // Rotación
        let rotated = self.rotation * scaled;

        // Traslación
        rotated + self.position
    }
}

pub struct AngularVelocity {
    pub axis: Unit<Vector3<f32>>,  // ya es unitario
    pub speed: f32,
}