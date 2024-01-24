export class Pattern{
    identifier:number;
    dims_values: Array<Array<any>>;
    density: number;
    size: number;

    constructor(identifier: number, dims_values: Array<Array<any>>, density: number, size: number){
        this.identifier = identifier;
        this.dims_values = dims_values;
        this.density = density;
        this.size = size;
    }
}