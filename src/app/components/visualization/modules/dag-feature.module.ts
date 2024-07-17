import { ChangeDetectorRef, ElementRef, EventEmitter, NgModule, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';
import { SvgFeatureModule } from './svg-feature.module';
import { ApiService } from 'src/app/services/api/api.service';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class DagFeatureModule{
    private supers_highlighted: boolean = false;
    private datapoints_with_subpatterns: Set<number>;

    private clicked_datapoint: number;
    public clicked_datapont_has_subpatterns: boolean = false;
    
    private svg_feature: SvgFeatureModule;
    private api_service: ApiService
  
    constructor(svg_feature: SvgFeatureModule, api_service: ApiService) {
      this.svg_feature = svg_feature;
      this.api_service = api_service;
    }

    public async init() {
        this.datapoints_with_subpatterns = new Set(
            (await this.api_service.getDatapointsWithSubPatterns()).map(datapoint => datapoint.identifier));
    }

    public setClickedDatapoint(identifier: number){
        this.clicked_datapoint = identifier;
        this.clicked_datapont_has_subpatterns = this.datapoints_with_subpatterns.has(identifier);
    }

    public toggleHighlightSuperpatterns(toggle: boolean){
        this.supers_highlighted = toggle;

        console.log("Toggling superpatterns");

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

    public ascendDag(){

    }

    public descendDag(){
        let super_datapoint = this.clicked_datapoint;
    }
}