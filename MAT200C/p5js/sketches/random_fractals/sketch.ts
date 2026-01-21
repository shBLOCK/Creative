
// @formatter:off
// language=Glsl
const VERT_SOURCE = `
#version 300 es
precision highp float;

uniform mat4 uModelViewMatrix;
uniform mat4 uProjectionMatrix;

in vec3 aPosition;
in vec2 aTexCoord;

out vec2 vPos;

void main() {
    vPos = aTexCoord * 1.7;
    gl_Position = uProjectionMatrix * uModelViewMatrix * vec4(aPosition, 1.0);
}
`.substring(1);

// language=Glsl
const FRAG_SOURCE = `
#version 300 es
precision highp float;

#define PI 3.1415927

vec2 cx_mul(vec2 a, vec2 b) {
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

vec2 cx_div(vec2 a, vec2 b) {
    float denom = 1.0 / (b.x*b.x + b.y*b.y);
    return vec2(a.x*b.x + a.y*b.y, a.y*b.x - a.x*b.y) * denom;
}

vec2 cx_sqr(vec2 a) {
    return vec2(a.x * a.x - a.y * a.y, 2.0 * a.x * a.y);
}

vec2 cx_cube(vec2 a) {
    float x2 = a.x * a.x;
    float y2 = a.y * a.y;
    float d = x2 - y2;
    return vec2(a.x * (d - y2 * 2.0), a.y * (x2 * 2.0 + d));
}

vec2 cx_exp(vec2 a) {
    return exp(a.x) * vec2(cos(a.y), sin(a.y));
}

vec2 cx_sin(vec2 a) {
    return vec2(sin(a.x) * cosh(a.y), cos(a.x) * sinh(a.y));
}
vec2 cx_cos(vec2 a) {
    return vec2(cos(a.x) * cosh(a.y), -sin(a.x) * sinh(a.y));
}

vec2 mandelbrot(vec2 z, vec2 c) {
    return cx_mul(z, z) + c;
}

vec2 burning_ship(vec2 z, vec2 c) {
    return mandelbrot(abs(z), c);
}

#define MAX_ITER 100
#define ESCAPE_THRESHOLD 10.0

uniform float uFractalABMix;

in vec2 vPos;

out vec4 oFragColor;

void main() {
    vec2 c = vPos;
    vec2 z = vec2(0.0);
    
    int iter;
    for (iter = 0; iter < MAX_ITER; iter++) {
        if (dot(z, z) > (ESCAPE_THRESHOLD * ESCAPE_THRESHOLD))
            break;
        vec2 za = HOOK_fractalA(z, c);
        vec2 zb = HOOK_fractalB(z, c);
        z = za + (zb - za) * uFractalABMix;
//        if (any(isnan(z)) || any(isinf(z))) break;
        if (any(isnan(z))) {
            oFragColor = vec4(1.0, 0.0, 0.5, 0.5);
            return;
        }
        if (any(isinf(z))) {
            oFragColor = vec4(0.0, 1.0, 0.5, 0.5);
            return;
        }
    }
    
    vec3 color;
    
    if (iter == MAX_ITER) {
        color = vec3(0.0);
    } else {
        float d = float(iter) * 0.25 + 3.5;
//        color = cos(d + vec3(0, 2, 4)) * 0.5 + 0.5;
        color = vec3(cos(d + vec2(0.0, PI / 2.0)) * 0.5 + 0.5, 1.0);
    }
    
    oFragColor = vec4(color, 1.0);
}
`.substring(1);
// @formatter:on

enum Type {
    Float,
    Complex
}

class Op {
    public shaderTemplate: string;
    public inputTypes: Type[];
    public outputType: Type;

    constructor(shaderTemplate: string, inputTypes: Type[], outputType: Type) {
        this.shaderTemplate = shaderTemplate;
        this.inputTypes = inputTypes;
        this.outputType = outputType;
    }

    public apply(inputs: string[]): string {
        return this.shaderTemplate.replaceAll(/{(\d)}/g, (_, i) => inputs[parseInt(i)]);
    }
}

const OPS = {
    create_cx: new Op("vec2({0}, {1})", [Type.Float, Type.Float], Type.Complex),
    real: new Op("{0}.x", [Type.Complex], Type.Float),
    imag: new Op("{0}.y", [Type.Complex], Type.Float),
    mag: new Op("length({0})", [Type.Complex], Type.Float),
    abs_f: new Op("abs({0})", [Type.Float], Type.Float),
    abs_cx: new Op("abs({0})", [Type.Complex], Type.Complex),
    neg_f: new Op("(-({0}))", [Type.Float], Type.Float),
    neg_cx: new Op("(-({0}))", [Type.Complex], Type.Complex),
    add_f: new Op("({0} + {1})", [Type.Float, Type.Float], Type.Float),
    add_cx: new Op("({0} + {1})", [Type.Complex, Type.Complex], Type.Complex),
    sub_f: new Op("({0} - {1})", [Type.Float, Type.Float], Type.Float),
    sub_cx: new Op("({0} - {1})", [Type.Complex, Type.Complex], Type.Complex),
    mul_f: new Op("({0} * {1})", [Type.Float, Type.Float], Type.Float),
    mul_cx: new Op("cx_mul({0}, {1})", [Type.Complex, Type.Complex], Type.Complex),
    div_f: new Op("({0} / {1})", [Type.Float, Type.Float], Type.Float),
    div_cx: new Op("cx_div({0}, {1})", [Type.Complex, Type.Complex], Type.Complex),
    sqr_f: new Op("({0} * {0})", [Type.Float], Type.Float),
    sqr_cx: new Op("cx_sqr({0})", [Type.Complex], Type.Complex),
    cube_f: new Op("({0} * {0} * {0})", [Type.Float], Type.Float),
    cube_cx: new Op("cx_cube({0})", [Type.Complex], Type.Complex),
    exp_f: new Op("exp({0})", [Type.Float], Type.Float),
    exp_cx: new Op("cx_exp({0})", [Type.Complex], Type.Complex),
    dot: new Op("dot({0}, {1})", [Type.Complex, Type.Complex], Type.Float),
    element_mul: new Op("({0} * {1})", [Type.Complex, Type.Complex], Type.Complex),
    sin_f: new Op("sin({0})", [Type.Float], Type.Float),
    sin_cx: new Op("cx_sin({0})", [Type.Complex], Type.Complex),
    cos_f: new Op("cos({0})", [Type.Float], Type.Float),
    cos_cx: new Op("cx_cos({0})", [Type.Complex], Type.Complex),
    tan_f: new Op("tan({0})", [Type.Float], Type.Float),
};

function genRandomFractalFunc(complexity_drop_off: number = 0.9) {
    function chooseOp(outputType: Type) {
        while (true) {
            const op = random(Object.values(OPS));
            if (op.outputType == outputType) return op;
        }
    }

    function gen(outputType: Type, complexity: number): string {
        if (random() > complexity) {
            // terminate with constant or parameter
            if (outputType == Type.Float) {
                return random(-3, 3).toFixed(1);
            } else {
                return random(["z", "c"]);
            }
        } else {
            const op = chooseOp(outputType);
            const inputs = op.inputTypes.map(type => gen(type, complexity * complexity_drop_off));
            return op.apply(inputs);
        }
    }

    return gen(Type.Complex, 1.0);
}

let fractalFrameBuffer: p5.Framebuffer;

let baseFractalShader: p5.Shader;
let fractalShader: p5.Shader;

let timeAcc = 0.0;
let lastFractalFunc = "mandelbrot(z, c)";
let fractalFunc = "burning_ship(z, c)";

async function setup() {
    createCanvas(windowWidth, windowHeight, WEBGL);

    fractalFrameBuffer = createFramebuffer({
        format: "unsigned-byte",
        channels: "rgba",
        depth: false,
        antialias: false,
        density: 1,
        textureFiltering: "nearest",
    });

    baseFractalShader = createShader(VERT_SOURCE, FRAG_SOURCE, {
        fragment: {
            "vec2 fractalA": `(vec2 z, vec2 c) { return ${lastFractalFunc}; }`,
            "vec2 fractalB": `(vec2 z, vec2 c) { return ${fractalFunc}; }`
        }
    });

    fractalShader = baseFractalShader;
}

function arrEqual(a: any[], b: any[]): boolean {
    if (a.length !== b.length) return false;
    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) return false;
    }
    return true;
}

function cosSmooth(x: number): number {
    return cos((x - 1.0) * PI) * 0.5 + 0.5;
}

let isGenerationFrame = false;

function draw() {
    function drawFractal(mix: number) {
        push();
        fractalShader.setUniform("uFractalABMix", mix);
        shader(fractalShader);
        blendMode(REPLACE);
        let uv = createVector(windowWidth / windowHeight, 1.0);
        beginShape();
        vertex(-windowWidth / 2, -windowHeight / 2, 0, -uv.x, -uv.y);
        vertex(+windowWidth / 2, -windowHeight / 2, 0, +uv.x, -uv.y);
        vertex(+windowWidth / 2, +windowHeight / 2, 0, +uv.x, +uv.y);
        vertex(-windowWidth / 2, +windowHeight / 2, 0, -uv.x, +uv.y);
        endShape(CLOSE);
        pop();
    }

    let mix = cosSmooth(cosSmooth(min(timeAcc, 1.0)));
    mix = min(max(mix, 1e-7), 1.0 - 1e-7);
    drawFractal(mix);

    let wasGenerationFrame = isGenerationFrame;

    if (timeAcc > 1.0) {
        isGenerationFrame = true;
        timeAcc = 0.0;
        lastFractalFunc = fractalFunc;

        function isInfOrNanPixel(pixel: number[]): boolean {
            return pixel[3] < 200;
        }

        function isGoodFractal(): boolean {
            // pixel (0, 0) doesn't work for some reason
            let firstPixel = fractalFrameBuffer.get(1, 1);
            if (isInfOrNanPixel(firstPixel))
                return false;
            let isBoring = true;
            for (let i = 0; i < 10; i++) {
                let pixel = fractalFrameBuffer.get(
                    Math.floor(random(1, fractalFrameBuffer.width - 1)),
                    Math.floor(random(1, fractalFrameBuffer.height - 1))
                );
                if (!arrEqual(pixel, firstPixel)) {
                    isBoring = false;
                }
                if (isInfOrNanPixel(pixel))
                    return false;
            }
            return !isBoring;
        }

        while (true) {
            fractalFunc = genRandomFractalFunc(0.85);
            console.log(fractalFunc);
            fractalShader = baseFractalShader.modify({
                "vec2 fractalA": `(vec2 z, vec2 c) { return ${lastFractalFunc}; }`,
                "vec2 fractalB": `(vec2 z, vec2 c) { return ${fractalFunc}; }`
            });

            // fractalFrameBuffer.draw(() => drawFractal(0.001));
            // if (!isGoodFractal()) {
            //     console.log("Transition start not good.");
            //     continue;
            // }
            fractalFrameBuffer.draw(() => drawFractal(0.5));
            if (!isGoodFractal()) {
                console.log("Transition middle not good.");
                continue;
            }
            fractalFrameBuffer.draw(() => drawFractal(1.0));
            if (!isGoodFractal()) {
                console.log("Transition end not good.");
                continue;
            }

            break;
        }
    } else {
        isGenerationFrame = false;
    }

    if (!wasGenerationFrame) {
        timeAcc += min(deltaTime / 1e3, 1/60) * 0.5;
    }
}

function windowResized() {
    resizeCanvas(windowWidth, windowHeight);
}