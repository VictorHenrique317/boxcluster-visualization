export class Pattern{
    identifier:number;
    dims_values: Array<Array<string>>;
    density: number;
    size: number;

    constructor(identifier: number, dims_values: Array<Array<string>>, density: number, size: number){
        this.identifier = identifier;
        this.dims_values = dims_values;
        this.density = density;
        this.size = size;
    }

    public static fromResponse(response: any): Pattern{
        return new Pattern(response.identifier, response.dims_values, response.density, response.size);
    }
}