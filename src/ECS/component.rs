use cgmath::{num_traits::Float, Euler, Quaternion, Vector3};
/**
 * Component for the ECS Entity Component System
 * Contains a property and its parameters
 */

pub struct Component {
    comp_type: ComponentType,
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

    // Scripts
    Script(ParamScript),
    Shader(ParamShader),
    ShaderScript(ParamShaderScript),
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

/*          Script Components           */
pub struct ParamScript {
    file: String
}

pub struct ParamShader {}

pub struct ParamShaderScript {
    file: String
}