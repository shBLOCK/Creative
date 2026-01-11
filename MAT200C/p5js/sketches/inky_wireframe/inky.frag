#version 300 es
precision highp float;

uniform sampler2D uSampler;

in vec2 vTexCoord;

out vec4 oFragColor;

void main() {
    vec3 stroke = texture(uSampler, vTexCoord).rgb;
    vec3 col = mix(vec3(0.9), vec3(0.1), smoothstep(1.0 - 0.1, 1.0, stroke));
    oFragColor = vec4(col, 1.0);
}