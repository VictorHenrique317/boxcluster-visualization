export class DataPoint{
    identifier: number;
    value: number;
    size: number;
    density: number;
    stroke_width: number;

    x: number;
    y: number;

    r: number;
    g: number;
    b: number;
    a: number;

    constructor(identifier: number, value:number, size: number, density: number, stroke_width: number, x: number, y: number, r: number, g: number, b: number, a:number){
        this.identifier = identifier;
        this.value = value;
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