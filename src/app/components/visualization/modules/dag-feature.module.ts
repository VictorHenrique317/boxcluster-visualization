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
        this.current_dag_level = 0;
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

    public toggleHighlightSuperpatterns(toggle: boolean){
        console.log("Toggling superpatterns");

        this.supers_highlighted = toggle;

        if(toggle){
            if(this.datapoints_with_subpatterns.size == 0){ return; }
            let gray_shade = 196;
            let gray = `rgba(${gray_shade}, ${gray_shade}, ${gray_shade}, 0.5)`;

            this.svg_feature.plot.selectAll('.datapoint')
                .filter(d => !this.datapoints_with_subpatterns.has(d.identifier))
                .raise()
                .transition('mouseover')
                .duration(300)
                .attr('fill', d => gray)
                .style('stroke', d=> gray);
            

        }else {
            this.svg_feature.drawDataPoints(this.svg_feature.getDrawnDataPoints(), true);
        }
    }

    // public activateHighlightSuperpatterns(){
    //     this.toggleHighlightSuperpatterns(true);
    // }

    // public deactivateHighlightSuperpatterns(){
    //     this.toggleHighlightSuperpatterns(false);
    // }

    public isHighlightingSuperpatterns(){
        return this.supers_highlighted;
    }

    private drawNewLevelDatapoints(datapoints: Array<DataPoint>){
        this.intersection_feature.toggleIntersections(null, true);
        this.svg_feature.deactivateHighlight();
        this.svg_feature.drawDataPoints(datapoints, true);
    }

    public ascendDag(){
        if(this.current_dag_level == 0){ return; }

        this.api_service.ascendDag().then((datapoints: Array<DataPoint>) => {
            if(datapoints.length == 0){ return; }
            
            this.drawNewLevelDatapoints(datapoints);
            
            this.current_dag_level -= 1;
            if(this.current_dag_level == 0){ this.upper_dag_arrow_active = false; }
            else{ this.upper_dag_arrow_active = true; }
        });
    }

    public descendDag(){
        let super_datapoint = this.clicked_datapoint;
        console.log("Descending from: ", super_datapoint)
        if(super_datapoint == null){ return; }

        this.api_service.descendDag(super_datapoint).then((datapoints: Array<DataPoint>) => {
            if(datapoints.length == 0){ return; }

            this.upper_dag_arrow_active = true;
            this.lower_dag_arrow_active = false;

            this.drawNewLevelDatapoints(datapoints);

            this.current_dag_level += 1;
        });
    }
}