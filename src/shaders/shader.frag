#version 450

// Changed
layout(location=0) out vec4 f_color;

void main() {
    // Changed
    f_color = vec4(0.5, 0.5, 0.5, 1.0);
}
