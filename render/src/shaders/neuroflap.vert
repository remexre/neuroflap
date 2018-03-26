#version 450

layout(location = 0) in vec2 position;

void main() {
	vec2 screenPos = vec2(2 * position.x - 1, 1 - 2 * position.y);
    gl_Position = vec4(screenPos, 0.0, 1.0);
}
