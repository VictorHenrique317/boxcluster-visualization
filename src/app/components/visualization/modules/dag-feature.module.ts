import { ChangeDetectorRef, ElementRef, EventEmitter, NgModule, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';
import { SvgFeatureModule } from './svg-feature.module';
import { ApiService } from 'src/app/services/api/api.service';
import { IntersectionModeFeatureModule } from './intersection-mode-feature.module';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class DagFeatureModule{
    public dag_change = new EventEmitter();

    public upper_dag_arrow_active: boolean = false;
    public lower_dag_arrow_active: boolean = false;
    protected supers_highlighted: boolean = false;

    private datapoints_with_subpatterns: Set<number>;
    public current_dag_level: number;
    private clicked_datapoint: number;
    
    private svg_feature: SvgFeatureModule;
    private intersection_feature: IntersectionModeFeatureModule;
    private api_service: ApiService
  
    constructor(intersection_feature: IntersectionModeFeatureModule, svg_feature: SvgFeatureModule, api_service: ApiService) {
        this.intersection_feature = intersection_feature;
        this.svg_feature = svg_feature;
        this.api_service = api_service;
    }

    public async init() {
        this.datapoints_with_subpatterns = new Set(
            (await this.api_service.getDatapointsWithSubPatterns()).map(datapoint => datapoint.identifier));
        this.current_dag_level = 1;
    }

    public setClickedDatapoint(identifier: number){
        if(identifier == this.clicked_datapoint){ // Clicked on the same pattern
            this.clicked_datapoint = undefined;
            this.lower_dag_arrow_active = false;
            return;
        }

        this.clicked_datapoint = identifier;
        this.lower_dag_arrow_active = this.datapoints_with_subpatterns.has(identifier)? true : false;
    }

    public async toggleHighlightSuperpatterns(toggle: boolean){
        // console.log("Toggling superpatterns");

        // this.supers_highlighted = toggle;

        // if(toggle){
        //     let gray_shade = 196;
        //     let gray = `rgba(${gray_shade}, ${gray_shade}, ${gray_shade}, 0.5)`;

        //     this.svg_feature.plot.selectAll('.datapoint')
        //         .filter(d => !this.datapoints_with_subpatterns.has(d.identifier))
        //         .raise()
        //         .transition('mouseover')
        //         .duration(300)
        //         .attr('fill', d => gray)
        //         .style('stroke', d=> gray);

        //     this.datapoints_with_subpatterns.forEach(async identifier => {
        //         let nb_of_identifiers = await this.api_service.getSubpatterns(identifier, []);
        //         this.svg_feature.drawTextLabel(identifier, nb_of_identifiers);
        //     });
        // }else {
        //     await this.svg_feature.drawDataPoints(this.svg_feature.getDrawnDataPoints(), true);
        //     this.svg_feature.removeTextLabels();
        // }
    }

    public isHighlightingSuperpatterns(){
        return this.supers_highlighted;
    }

    private async drawNewLevelDatapoints(datapoints: Array<DataPoint>){
        this.intersection_feature.toggleIntersections(null, true);
        this.svg_feature.deactivateHighlight();
        await this.svg_feature.drawDataPoints(datapoints, true);
    }

    public async ascendDag(): Promise<boolean>{
        if(this.current_dag_level == 1){ return false; }

        let datapoints = await this.api_service.ascendDag();
        if(datapoints.length == 0){ return false; }

        console.log("Ascending");
        console.log("New level datapoints:");
        console.log(datapoints);
            
        await this.drawNewLevelDatapoints(datapoints);
        
        this.current_dag_level -= 1;

        if(this.current_dag_level == 1){ this.upper_dag_arrow_active = false; }
        else{ this.upper_dag_arrow_active = true; }
        this.lower_dag_arrow_active = false;
        this.clicked_datapoint = undefined;

        console.log("Current level: ", this.current_dag_level);
        this.dag_change.emit();
        return true;
    }

    public async descendDag(): Promise<boolean>{
        let super_datapoint = this.clicked_datapoint;
        console.log("Descending from: ", super_datapoint)
        if(super_datapoint == null){ return false; }

        let datapoints = await this.api_service.descendDag(super_datapoint);
        console.log("New level datapoints:");
        console.log(datapoints);

        if(datapoints.length == 0){ return false; }

        await this.drawNewLevelDatapoints(datapoints);

        this.current_dag_level += 1;

        this.upper_dag_arrow_active = true;
        this.lower_dag_arrow_active = false;
        this.clicked_datapoint = undefined;

        console.log("Current level: ", this.current_dag_level);
        this.dag_change.emit();
        return true;
    }
}
