#version 330 core

in ivec2 position;
in vec3 color;

// Drawing offset
uniform ivec2 offset;

out vec3 v_color;

void main() {
    ivec2 pos = position + offset;

    // Convert VRAM coordinates (0;1023, 0;511) into OpenGL coordinates
    // (-1;1, -1;1)
    float xpos = (float(pos.x) / 512) - 1.0;
    float ypos = (float(pos.y) / 256) - 1.0;

    gl_Position.xyzw = vec4(xpos, ypos, 0.0, 1.0);

    // Glium doesn't support "normalized" for now
    v_color = color/255;
}
