const CANVAS_SIZE = [600, 1000];
// const CANVAS_SIZE = [1200, 2000];

let shaderProgram;
let music;
let fft;
// let amp;

let uGridSizeX;
let uGridSizeY;
let cellSize;

async function setup() {
    await Tone.start();

    createCanvas(CANVAS_SIZE[0], CANVAS_SIZE[1], WEBGL);

    uGridSizeX = createSlider(1, 30, 12, 1);
    uGridSizeY = createSlider(1, 30, 22, 1);
    // cellSize = createSlider(1, 500, 50);
    cellSize = createSlider(1, 500, 40);

    shaderProgram = await loadShader("shader.vert", "shader.frag");
    music = new Tone.Player("8bm.mp3").toDestination();
    music.autostart = true;
    fft = new Tone.Analyser({
        size: 64,
        type: "fft",
        smoothing: 0.8
    });
    music.connect(fft);
    // music = await loadSound("8bm.mp3");
    // music.play();
    // fft = new p5.FFT(64);
    // music.connect(fft);
    // console.log(fft);
    // amp = new p5.Amplitude(0.0);
    // music.connect(amp);
}

let musicPower = 0.0;

function draw() {
    // console.log();
    let spectrum = fft.getValue().map(db => Math.pow(10.0, db / 20.0));
    let instantMusicPower = spectrum.slice(0, 2).reduce((a, b) => {
        return a + b;
    }, 0) / 2;
    // let instantMusicPower = amp.getLevel();
    // let instantMusicPower = 0.1;
    musicPower = max(lerp(musicPower, instantMusicPower, 0.3), lerp(musicPower, instantMusicPower, 0.2));
    console.log(musicPower);
    let uBleedPower = 25.0;
    uBleedPower = 0.6 / musicPower;

    push();
    shader(shaderProgram);
    shaderProgram.setUniform("uGridSize", [uGridSizeX.value(), uGridSizeY.value()]);
    shaderProgram.setUniform("uBleedPower", uBleedPower);
    shaderProgram.setUniform("uLineEdgeSmooth", 0.2);
    shaderProgram.setUniform("uRandScale", max(0.0, musicPower - 0.025) * 25.0);
    // quad(-1, -1, 1, -1, 1, 1, -1, 1);
    // quad(-400, -400, 400, -400, 400, 400, -400, 400);
    let uvx = CANVAS_SIZE[0] / cellSize.value();
    let uvy = CANVAS_SIZE[1] / cellSize.value();
    beginShape();
    vertex(-CANVAS_SIZE[0] / 2, -CANVAS_SIZE[1] / 2, 0, -uvx, -uvy);
    vertex(+CANVAS_SIZE[0] / 2, -CANVAS_SIZE[1] / 2, 0, +uvx, -uvy);
    vertex(+CANVAS_SIZE[0] / 2, +CANVAS_SIZE[1] / 2, 0, +uvx, +uvy);
    vertex(-CANVAS_SIZE[0] / 2, +CANVAS_SIZE[1] / 2, 0, -uvx, +uvy);
    endShape(CLOSE);
    pop();

    // push();
    // translate(-CANVAS_SIZE[0] / 2, -CANVAS_SIZE[1] / 2);
    // for (let i = 0; i < spectrum.length; i++) {
    //     let w = CANVAS_SIZE[0] / spectrum.length;
    //     let h = spectrum[i] * 3e3;
    //     fill("green");
    //     rect(w * i, CANVAS_SIZE[1] - h, w, h);
    // }
    // pop();
}