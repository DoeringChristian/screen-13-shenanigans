#version 450

layout(location = 0) out vec4 o_color;

layout(location = 0) in vec2 i_uv;

void main(){
    o_color = vec4(i_uv.x, i_uv.y, 0., 1.);
}
