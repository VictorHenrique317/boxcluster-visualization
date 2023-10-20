export class DataPoint{
    identifier: number;
    size: number;
    stroke_width: number;

    x: number;
    y: number;

    r: number;
    g: number;
    b: number;

    constructor(identifier: number, size: number, stroke_width: number, x: number, y: number, r: number, g: number, b: number){
        this.identifier = identifier;
        this.size = size;
        this.stroke_width = stroke_width;
        this.x = x;
        this.y = y;
        this.r = r;
        this.g = g;
        this.b = b;
    }
}