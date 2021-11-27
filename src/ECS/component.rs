use cgmath::{num_traits::Float, Euler, Quaternion, Vector3};
/**
 * Component for the ECS Entity Component System
 * Contains properties and their parameters
 */

pub struct Component {
    compType: ComponentType,
}

pub enum ComponentType {
    // Status
    Position(Vector3<f32>),
    Rotation(CompRotation),

    // Physics
    Gravity(ParamGravity),
    Softbody(ParamSoftbody),

    // Rendering
    Image(ParamImage),
    Mesh(ParamMesh),
}

pub enum CompRotation {
    Quaternion(Quaternion<f32>),
    Euler(Euler<f32>),
}

/*			Physics Components		 */
pub struct ParamGravity {
    grav: Vector3<f32>,
}

pub struct ParamSoftbody {
    spring_const: f32,
}

/*			Rendering Components		 */
pub struct ParamImage {}

pub struct ParamMesh {}
