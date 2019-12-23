#version 450

layout (location = 0) in ivec2 a_Pos;
layout (location = 1) in vec4 a_Color;
layout (location = 0) out vec3 f_color;

void main(){

    float xpos = (float(a_Pos.x) / 512) - 1.0;
    float ypos = (float(a_Pos.y) / 256) - 1.0;

    gl_Position.xyzw = vec4(xpos, ypos, 0.0, 1.0);

    // Convert the components from [0;255] to [0;1]
    f_color = vec3(a_Color.r/255, a_Color.g/255, a_Color.b/255);
}