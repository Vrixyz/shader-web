#version 300 es

in vec3 Vertex_Position;
in vec2 Vertex_Uv;

out vec2 v_Uv;

layout(std140)uniform CameraViewProj{
    mat4 ViewProj;
};
layout(std140)uniform Transform{
    mat4 Model;
};

void main(){
    gl_Position=ViewProj*Model*vec4(Vertex_Position,1.);
    v_Uv=Vertex_Uv;
}