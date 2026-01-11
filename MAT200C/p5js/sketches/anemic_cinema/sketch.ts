class Linear {
    public k: number;
    public b: number;

    public constructor(k: number, b: number) {
        this.k = k;
        this.b = b;
    }

    public apply(x: number): number {
        return this.k * x + this.b;
    }

    public lerp(other: Linear, t: number): Linear {
        return new Linear(lerp(this.k, other.k, t), lerp(this.b, other.b, t));
    }
}

class WhiteBlackLinear {
    public white: Linear;
    public black: Linear;

    public constructor(white: Linear, black: Linear) {
        this.white = white;
        this.black = black;
    }

    public apply(isWhite: boolean, x: number): number {
        if (isWhite) {
            return this.white.apply(x);
        } else {
            return this.black.apply(x);
        }
    }

    public lerp(other: WhiteBlackLinear, t: number): WhiteBlackLinear {
        return new WhiteBlackLinear(this.white.lerp(other.white, t), this.black.lerp(other.black, t));
    }
}

class Params {
    public angle: WhiteBlackLinear;
    public offset: WhiteBlackLinear;
    public radius: WhiteBlackLinear;

    public constructor(angle: WhiteBlackLinear, offset: WhiteBlackLinear, radius: WhiteBlackLinear) {
        this.angle = angle;
        this.offset = offset;
        this.radius = radius;
    }

    public lerp(other: Params, t: number): Params {
        return new Params(
            this.angle.lerp(other.angle, t),
            this.offset.lerp(other.offset, t),
            this.radius.lerp(other.radius, t)
        );
    }
}


async function setup() {
    createCanvas(900, 900);
}

let time = 0.0;

class Circle {
    public pos: p5.Vector;
    public radius: number;
    public isWhite: boolean;

    public constructor(pos: p5.Vector, radius: number, isWhite: boolean) {
        this.pos = pos;
        this.radius = radius;
        this.isWhite = isWhite;
    }
}

let paramsList: Params[] = [
    new Params(
        new WhiteBlackLinear(new Linear(-70, 0), new Linear(-70, 0)),
        new WhiteBlackLinear(new Linear(-0.5, 10), new Linear(0, 4)),
        new WhiteBlackLinear(new Linear(50, 20), new Linear(50, 45)),
    ),
    new Params(
        new WhiteBlackLinear(new Linear(-50, 0), new Linear(-50, 0)),
        new WhiteBlackLinear(new Linear(0, 3), new Linear(0, 4)),
        new WhiteBlackLinear(new Linear(25, 20), new Linear(25, 24)),
    ),
    new Params(
        new WhiteBlackLinear(new Linear(-50, 0), new Linear(-50, 0)),
        new WhiteBlackLinear(new Linear(0, 3), new Linear(0, 3)),
        new WhiteBlackLinear(new Linear(28, 20), new Linear(28, 30)),
    ),
    new Params(
        new WhiteBlackLinear(new Linear(-30, 0), new Linear(-30, 0)),
        new WhiteBlackLinear(new Linear(0, 23), new Linear(0, 3)),
        new WhiteBlackLinear(new Linear(30, 50), new Linear(30, 53)),
    ),
    new Params(
        new WhiteBlackLinear(new Linear(-50, 0), new Linear(-50, 0)),
        new WhiteBlackLinear(new Linear(0, 20), new Linear(0, 10)),
        new WhiteBlackLinear(new Linear(20, 40), new Linear(20, 35)),
    )
];

function draw() {
    push();
    translate(450, 450);
    rotate(time);
    background(0);
    clip(() => circle(0, 0, 800));
    noStroke();

    // let params: Params = {
    //     angle: new WhiteBlackLinear(new Linear(-50, 0), new Linear(-50, 0)),
    //     offset: new WhiteBlackLinear(new Linear(0, 20), new Linear(0, 10)),
    //     radius: new WhiteBlackLinear(new Linear(20, 40), new Linear(20, 35)),
    // };

    let paramsI = Math.floor(time / 5);
    let paramsA = paramsList[paramsI % paramsList.length];
    let paramsB = paramsList[(paramsI + 1) % paramsList.length];
    let transition = Math.max(time % 5 - 4, 0);
    let params = paramsA.lerp(paramsB, (Math.cos((-1 + transition) * PI) + 1) / 2);

    let circles: Circle[] = [];
    let lastCircle = new Circle(createVector(0, 0), 0, false);
    for (let i = 0; i < 1000; i++) {
        let isWhite = i % 2 == 0;
        let ii = Math.floor(i / 2);

        let angle = params.angle.apply(isWhite, ii);
        let offset = params.offset.apply(isWhite, ii);
        let radius = params.radius.apply(isWhite, ii);

        let pos = p5.Vector.fromAngle(radians(angle), lastCircle.radius + offset - radius).add(lastCircle.pos);

        let posMag = pos.mag();
        if (posMag - radius > 450) break;
        if (-(posMag - radius) > 450) break;
        let circle = new Circle(pos, radius, isWhite);
        circles.push(circle);
        lastCircle = circle;
    }

    circles.reverse().forEach(c => {
        fill((c.isWhite ? [0.9, 0.9, 0.9] : [0.1, 0.1, 0.1]).map(x => x * 255));
        circle(c.pos.x, c.pos.y, c.radius * 2);
    });
    pop();

    time += deltaTime * 1e-3;
}