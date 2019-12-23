#version 450

layout(location = 0) in vec3 f_color;
layout(location = 0) out vec4 FragColor;

void main(){
    FragColor = vec4(f_color, 1.0f);
}