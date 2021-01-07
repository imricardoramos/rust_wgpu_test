#version 450

layout(location=0) in vec3 a_position;

layout(set=0, binding=0) // 1.
uniform Uniforms {
    vec3 position; // 2.
};

void main() {
    gl_Position = vec4((a_position+position)*0.5, 1.0);
}
