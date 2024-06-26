export class DataPoint{
    identifier: number;
    size: number;
    pattern_size: number;
    density: number;
    stroke_width: number;

    x: number;
    y: number;

    r: number;
    g: number;
    b: number;
    a: number;

    constructor(identifier: number, size: number,  pattern_size: number, density: number, stroke_width: number, x: number, y: number, r: number, g: number, b: number, a:number){
        this.identifier = identifier;
        this.pattern_size = pattern_size;
        this.size = size;
        this.density = density;
        this.stroke_width = stroke_width;
        this.x = x;
        this.y = y;
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }
}