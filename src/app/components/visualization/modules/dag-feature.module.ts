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
export class DagFeatureModule implements OnInit{
    private supers_highlighted: boolean = false;
    private datapoints_with_subpatterns: DataPoint[] = [];

    private clicked_datapoint: number;
    private clicked_datapont_has_subpatterns: boolean;
    
    private svg_feature: SvgFeatureModule;
    private api_service: ApiService
  
    constructor(svg_feature: SvgFeatureModule, api_service: ApiService) {
      this.svg_feature = svg_feature;
      this.api_service = api_service;
    }

    async ngOnInit() {
        this.datapoints_with_subpatterns = await this.api_service.getDatapointsWithSubPatterns();
    }

    public setClickedDatapoint(identifier: number){
        this.clicked_datapoint = identifier;
    }

    public toggleHighlightSuperpatterns(toggle: boolean){
        this.supers_highlighted = toggle;

        if(toggle){
            if(this.datapoints_with_subpatterns.length == 0){ return; }
            let identifiers: number[] = this.datapoints_with_subpatterns.map(datapoint => datapoint.identifier);
            let identifiers_set = new Set(identifiers);

            let gray_shade = 196;
            let gray = `rgba(${gray_shade}, ${gray_shade}, ${gray_shade}, 0.5)`;

            this.svg_feature.plot.selectAll('.datapoint')
                .filter(d => !identifiers_set.has(d.identifier))
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