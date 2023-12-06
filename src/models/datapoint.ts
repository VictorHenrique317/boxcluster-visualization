export class DataPoint{
    identifier: number;
    value: number;
    size: number;
    stroke_width: number;

    x: number;
    y: number;

    r: number;
    g: number;
    b: number;

    constructor(identifier: number, value:number, size: number, stroke_width: number, x: number, y: number, r: number, g: number, b: number){
        this.identifier = identifier;
        this.value = value;
        this.size = size;
        this.stroke_width = stroke_width;
        this.x = x;
        this.y = y;
        this.r = r;
        this.g = g;
        this.b = b;
    }
}