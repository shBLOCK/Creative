function rotateDim2d(vec, dimX, dimY, theta) {
    let [x, y] = [vec.getValue(dimX), vec.getValue(dimY)];
    let [c, s] = [cos(theta), sin(theta)];
    vec.setValue(dimX, x * c + y * -s);
    vec.setValue(dimY, x * s + y * c);
}
function bitRotateRight(x, shift, totalBits) {
    shift %= totalBits;
    const tmp = x & ((1 << shift) - 1);
    x >>= shift;
    x |= tmp << (totalBits - shift);
    return x;
}
class Hypercube {
    pos;
    rot;
    size;
    moveSpeed;
    rotSpeed;
    color;
    constructor() {
        this.pos = createVector(0, 0);
        this.rot = createVector(0, 0, 0, 0);
        this.size = 150.0;
        this.moveSpeed = p5.Vector.random2D().mult(100.0);
        const ROT_SPEED = 1.0;
        this.rotSpeed = createVector(random(-ROT_SPEED, ROT_SPEED), random(-ROT_SPEED, ROT_SPEED), random(-ROT_SPEED, ROT_SPEED), random(-ROT_SPEED, ROT_SPEED));
        this.color = [random(0, 255), random(0, 255), random(0, 255)];
        this.color[random([0, 1, 2])] = 255;
    }
    tick() {
        this.pos.add(this.moveSpeed.copy().mult(deltaTime / 1e3));
        if (abs(this.pos.x) + this.size >= width / 2) {
            this.moveSpeed.x = abs(this.moveSpeed.x) * -Math.sign(this.pos.x);
        }
        if (abs(this.pos.y) + this.size >= height / 2) {
            this.moveSpeed.y = abs(this.moveSpeed.y) * -Math.sign(this.pos.y);
        }
        this.rot.add(this.rotSpeed.copy().mult(deltaTime / 1e3));
    }
    draw() {
        let vertices = [];
        for (let i = 0; i < 2 ** 4; i++) {
            let vec = createVector((i & 0b1000) >> 3, (i & 0b0100) >> 2, (i & 0b0010) >> 1, (i & 0b0001) >> 0).sub([0.5, 0.5, 0.5, 0.5]).mult(this.size);
            rotateDim2d(vec, 3, 0, this.rot.getValue(3));
            rotateDim2d(vec, 2, 3, this.rot.z);
            rotateDim2d(vec, 1, 2, this.rot.y);
            rotateDim2d(vec, 0, 1, this.rot.x);
            vertices.push(vec);
        }
        push();
        translate(this.pos);
        stroke(this.color);
        for (let dim = 0; dim < 4; dim++) {
            for (let i = 0b000; i < 0b111; i++) {
                let ia = bitRotateRight(i, dim, 4);
                let ib = bitRotateRight(i | 0b1000, dim, 4);
                let [va, vb] = [vertices[ia], vertices[ib]];
                line(va.x, va.y, va.z, vb.x, vb.y, vb.z);
            }
        }
        pop();
    }
}
let inkStrokeShader;
let inkFramebuffer;
let inkyShader;
let cubes = [];
async function setup() {
    createCanvas(windowWidth, windowHeight, WEBGL);
    inkStrokeShader = baseStrokeShader().modify({
        declarations: 'uniform float uInkSpreadArea;' +
            'uniform float uBleedPower;',
        "Inputs getPixelInputs": `(Inputs inputs) {
            float d = distance(inputs.position, inputs.center) / (inputs.strokeWeight / 2.0);
            d = clamp((1.0 - d) / uInkSpreadArea, 0.0, 1.0);
            d = pow(d, uBleedPower);
            inputs.color *= d;
            return inputs;
        }`
    });
    inkFramebuffer = createFramebuffer({
        format: "float",
        channels: "rgba",
        depth: false,
        stencil: false,
        antialias: false,
    });
    inkyShader = await loadShader("inky.vert", "inky.frag");
    for (let i = 0; i < 16; i++) {
        cubes.push(new Hypercube());
    }
}
function draw() {
    cubes.forEach(cube => cube.tick());
    inkFramebuffer.draw(() => {
        push();
        clear(0, 0, 0, 0);
        inkStrokeShader.setUniform("uInkSpreadArea", 0.95);
        inkStrokeShader.setUniform("uBleedPower", 2.0);
        strokeShader(inkStrokeShader);
        strokeWeight(70);
        stroke(255, 255, 255);
        linePerspective(true);
        strokeCap(ROUND);
        strokeJoin(ROUND);
        blendMode(ADD);
        cubes.forEach(cube => cube.draw());
        pop();
    });
    push();
    imageShader(inkyShader);
    imageMode(CENTER);
    image(inkFramebuffer, 0, 0);
    pop();
}
function windowResized() {
    resizeCanvas(windowWidth, windowHeight);
}
