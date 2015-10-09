#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coord;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coord;

uniform float t;
uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 perspective_mat;

void main () {
    mat4 modelview = view_mat * model_mat;
    gl_Position = perspective_mat * modelview * vec4(position, 1.0); 
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    v_position = gl_Position.xyz / gl_Position.w;
    v_tex_coord = tex_coord;
}
