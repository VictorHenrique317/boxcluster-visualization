export class IntersectionDetails{
    identifier: number;
    total_untouched_percentage: number;
    total_intersection_percentage: number;
    intersections: Map<number, [number, Array<Array<string>>]>;

    constructor (identifier: number, total_untouched_percentage: number, total_intersection_percentage: number, 
        intersections: Map<number, [number, Array<Array<string>>]>){

        this.identifier = identifier;
        this.total_untouched_percentage = total_untouched_percentage;
        this.total_intersection_percentage = total_intersection_percentage;
        this.intersections = intersections;
    }
}