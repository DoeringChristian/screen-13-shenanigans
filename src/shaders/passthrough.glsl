#version 450

layout(location = 0) out vec4 o_color;

layout(location = 0) in vec2 i_uv;

layout(set = 0, binding = 0) uniform sampler2D tex_s;

void main(){
    o_color = texture(tex_s, i_uv);
    //o_color = vec4(1., 0., 0., 1.);
}
