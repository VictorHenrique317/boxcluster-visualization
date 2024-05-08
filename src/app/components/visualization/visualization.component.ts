import * as d3Tip from "d3-tip";
import { resolveResource } from '@tauri-apps/api/path'
import { ChangeDetectorRef, Component, ComponentFactoryResolver, EventEmitter, InjectionToken, Input, OnDestroy, Output, Renderer2, ViewContainerRef } from '@angular/core';
import { ComponentPortal, PortalModule } from '@angular/cdk/portal';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/app/models/datapoint';
import { event, fs, invoke } from '@tauri-apps/api';
import { BaseDirectory } from "@tauri-apps/api/fs";
import { SvgService } from 'src/app/services/svg/svg.service';
import { Subscription, take } from 'rxjs';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';
import { ActivatedRoute } from '@angular/router';
import { RssViewComponent } from 'src/app/components/main_options/rss-view/rss-view.component';
import { environment } from '../../../environments/environment';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { DataPointTooltipComponent } from "./datapoint-tooltip/datapoint-tooltip.component";
import { Pattern } from "src/app/models/pattern";
import { DialogService } from "src/app/services/dialog/dialog.service";
import { legendCircle } from 'src/js/circle_legend.js';
import { Legend } from 'src/js/color_legend.js';
import { IntersectionModeFeatureModule } from 'src/app/components/visualization/intersection-mode-feature.module';
import { SvgFeatureModule } from "./svg-feature.module";
import {MatButtonModule} from '@angular/material/button';
import { ApiService } from "src/app/services/api/api.service";

@Component({
    selector: 'app-visualization',
    standalone: true,
    templateUrl: './visualization.component.html',
    styleUrls: ['./visualization.component.scss'],
    animations: [
        trigger('slideInOut', [
            state('void', style({
                transform: 'translateX(100%)',
                opacity: 0
            })),
            state('in', style({
                transform: 'translateX(0)',
                opacity: 1
            })),
            state('out', style({
                transform: 'translateX(100%)',
                opacity: 0
            })),
            transition('void => in', [
                animate('0.5s ease-in-out')
            ]),
            transition('in => out', [
                animate('0.5s ease-in-out')
            ]),
            transition('out => in', [
                animate('0.5s ease-in-out')
            ])
        ])
    ],
    imports: [
        CommonModule,
        MatCardModule,
        PortalModule,
        RssViewComponent,
        DataPointTooltipComponent,
        MatButtonModule
    ]
})

export class VisualizationComponent implements AfterViewInit, OnDestroy{
  @Output() datapoint_hover_in = new EventEmitter<number>();
  @Output() datapoint_hover_out = new EventEmitter<number>();
  @Output() datapoint_click = new EventEmitter();

  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  @ViewChild('vizualization_div') visualization_div: ElementRef<HTMLDivElement>;

  private svg_feature: SvgFeatureModule;
  protected intersection_mode_feature: IntersectionModeFeatureModule;

  constructor(private api_service: ApiService, private dialog_service: DialogService, private cdr: ChangeDetectorRef){ }

  ngOnInit(): void {
    this.intersection_mode_feature = new IntersectionModeFeatureModule(null, null, null);
  }

  async ngAfterViewInit() {
    console.log("Initializing visualization component");

    let svg_width = this.body.nativeElement.clientWidth;
    let svg_height = this.body.nativeElement.clientHeight;
    
    this.svg_feature = new SvgFeatureModule(this.cdr);
    this.svg_feature.init(this.visualization_div, svg_width, svg_height);
    this.svg_feature.datapoint_hover_in.subscribe(identifier => this.onDatapointHoverIn(identifier));
    this.svg_feature.datapoint_hover_out.subscribe(identifier => this.onDatapointHoverOut(identifier));
    this.svg_feature.datapoint_click.subscribe(identifier => this.onDatapointClick(identifier));
    
    let datapoints = await this.api_service.getDataPoints();
    this.svg_feature.drawDataPoints(datapoints);

    this.intersection_mode_feature = new IntersectionModeFeatureModule(this.svg_feature, this.dialog_service, this.api_service);
  }

  ngOnDestroy() {
    this.svg_feature.datapoint_hover_in.unsubscribe();
    this.svg_feature.datapoint_hover_out.unsubscribe();
    this.svg_feature.datapoint_click.unsubscribe();
  }

  public onResize(event) {
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.svg_feature.resizeSvg(width, height);
  }

  public async onTruncation(event){
    let new_size = event - 1; // -1 because the first point is the null model rss
    let truncated_datapoints = await this.api_service.truncateModel(new_size);

    this.svg_feature.drawDataPoints(truncated_datapoints);
  }

  private onDatapointHoverIn(identifier: number){
    this.datapoint_hover_in.emit(identifier);
  }

  private onDatapointHoverOut(identifier: number){
    this.datapoint_hover_out.emit(identifier);
  }

  private onDatapointClick(identifier: number){
    this.datapoint_click.emit(identifier);
  }

  public toggleIntersectionMode(){
    this.intersection_mode_feature.toggleIntersectionMode();
  }
}
