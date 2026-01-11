#version 300 es
precision highp float;

uniform ivec2 uGridSize;
uniform float uBleedPower;
uniform float uLineEdgeSmooth;
uniform float uRandScale;

in vec2 vTexCoord;

out vec4 oFragColor;

#define PI 3.1415926
#define TAU (PI * 2.0)

// Hash without Sine
// MIT License...
/* Copyright (c)2014 David Hoskins.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.*/
//----------------------------------------------------------------------------------------
///  3 out, 2 in...
vec3 hash32(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * vec3(.1031, .1030, .0973));
    p3 += dot(p3, p3.yxz + 33.33);
    return fract((p3.xxy + p3.yzz) * p3.zyx);
}

struct SampleResult {
    float dist;
};

float sdBox(vec2 p, vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

mat2 rot2d(float a) {
    return mat2(cos(a), sin(a), -sin(a), cos(a));
}

SampleResult sampleCell(vec2 pos, ivec2 cell) {
    int cellI = cell.y * uGridSize.x + cell.x;
    float cellINormalized = float(cellI) / float(uGridSize.x * uGridSize.y);
    vec3 rand = hash32(vec2(cell));
    pos += (rand.xy * 2.0 - 1.0) * cellINormalized * pow(uRandScale, 2.0);
    pos *= rot2d((rand.z * 2.0 - 1.0) * cellINormalized * (PI / 4.0) * pow(uRandScale, 0.5));

    float cornerRadius = 0.08;
    float dist = abs(sdBox(pos, vec2(1.0 - cornerRadius)) - cornerRadius);

    return SampleResult(dist);
}

void main() {
    vec2 uv = vTexCoord + (vec2(uGridSize) * 2.0) / 2.0;
    ivec2 cell = ivec2(floor(uv / 2.0));
    vec2 cellPos = mod(uv, vec2(2.0)) - 1.0;

    float stroke = 0.0;
    int maxo = 3;
    for (int ox = -maxo; ox <= maxo; ox++) {
        for (int oy = -maxo; oy <= maxo; oy++) {
            ivec2 offset = ivec2(ox, oy);
            ivec2 oCell = cell + offset;
            if (oCell.x < 0 || oCell.x >= uGridSize.x || oCell.y < 0 || oCell.y >= uGridSize.y) continue;
            vec2 oCellPos = cellPos - vec2(offset * 2);
            SampleResult samp = sampleCell(oCellPos, oCell);
            //            stroke += 1.0 / samp.dist * clamp(1.0 - samp.dist / 0.1, 0.0, 1.0);
            //            stroke += pow(clamp(1.0 - samp.dist / 0.05, 0.0, 1.0), 0.5);
            float d = max(0.0, samp.dist - 0.025);
            //            stroke += exp(-d) * clamp(1.0 - d / 0.5, 0.0, 1.0);
            stroke += pow(max(0.0, 1.0 - d), uBleedPower);
        }
    }

    //    const float threshold = 1.25;
    //    const float transition = 0.07;
    //    vec3 col = vec3(1.0 - smoothstep(threshold - transition, threshold + transition, pow(stroke, 0.08)));

    vec3 bg = vec3(0.9);
    vec3 col = mix(bg, vec3(0.1), smoothstep(1.0 - uLineEdgeSmooth, 1.0, stroke));
    //    vec3 col = vec3(stroke / 5.0);

    oFragColor = vec4(col, 1.0);
}