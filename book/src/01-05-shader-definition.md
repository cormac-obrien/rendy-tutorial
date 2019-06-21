# Shader Definition

In order to get our fancy little triangle on the screen,
we have to define shaders to do the drawing.
You can write these shaders in either GLSL or HLSL,
or you can provide precompiled SPIR-V shaders.
Here they are in GLSL:

```glsl
// vert.glsl

#version 450

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec4 a_color;

layout(location = 0) out vec4 v_color;

void main() {
     f_color = a_color;
     gl_Position = vec4(a_pos, 1.0);
}
```

```glsl
// frag.glsl

#version 450

layout(location = 0) in vec4 v_color;

layout(location = 0) out vec4 out_color;

void main() {
     out_color = v_color;
}
```

As you can see, there's nothing out of the ordinary about these shaders.
