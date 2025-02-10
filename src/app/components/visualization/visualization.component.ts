import * as d3Tip from "d3-tip";
import { resolveResource } from '@tauri-apps/api/path'
import { ChangeDetectorRef, Component, ComponentFactoryResolver, EventEmitter, InjectionToken, Input, OnDestroy, OnInit, Output, Renderer2, ViewContainerRef } from '@angular/core';
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
import { IntersectionModeFeatureModule } from 'src/app/components/visualization/modules/intersection-mode-feature.module';
import { SvgFeatureModule } from "./modules/svg-feature.module";
import {MatButtonModule} from '@angular/material/button';
import { ApiService } from "src/app/services/api/api.service";
import { DagFeatureModule } from "./modules/dag-feature.module";
import { MatIconModule } from "@angular/material/icon";
import { MatTooltipModule } from "@angular/material/tooltip";

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
        MatButtonModule,
        MatIconModule,
        MatTooltipModule
    ]
})

export class VisualizationComponent implements OnInit, AfterViewInit, OnDestroy{
  @Output() datapoint_hover_in = new EventEmitter<number>();
  @Output() datapoint_hover_out = new EventEmitter<number>();
  @Output() datapoint_click = new EventEmitter();
  @Output() dag_change = new EventEmitter();

  private datapoint_hover_in_subscription: Subscription;
  private datapoint_hover_out_subscription: Subscription;
  private datapoint_click_subscription: Subscription;
  private dag_change_subscription: Subscription;

  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  @ViewChild('vizualization_div') visualization_div: ElementRef<HTMLDivElement>;

  private svg_feature: SvgFeatureModule;
  public intersection_mode_feature: IntersectionModeFeatureModule;
  protected dag_feature: DagFeatureModule;

  constructor(private api_service: ApiService, private dialog_service: DialogService, private cdr: ChangeDetectorRef){ }

  async ngOnInit() {
    this.intersection_mode_feature = new IntersectionModeFeatureModule(null, null, null);
    this.dag_feature = new DagFeatureModule(null, null, this.api_service);
    await this.dag_feature.init();
  }

  async ngAfterViewInit() {
    console.log("Initializing visualization component");

    let svg_width = this.body.nativeElement.clientWidth;
    let svg_height = this.body.nativeElement.clientHeight;
    
    this.svg_feature = new SvgFeatureModule(this.cdr);
    this.svg_feature.init(this.visualization_div, svg_width, svg_height);
    let background_density = await this.api_service.getCurrentLevelBackgroundDensity();
    this.svg_feature.setBackgroundColor(background_density);
    
    let datapoints = await this.api_service.getDataPoints();
    this.svg_feature.drawDataPoints(datapoints);

    this.intersection_mode_feature = new IntersectionModeFeatureModule(this.svg_feature, this.dialog_service, this.api_service);

    this.dag_feature = new DagFeatureModule(this.intersection_mode_feature, this.svg_feature, this.api_service);
    await this.dag_feature.init();

    this.datapoint_hover_in_subscription = this.svg_feature.datapoint_hover_in.subscribe(identifier => this.onDatapointHoverIn(identifier));
    this.datapoint_hover_out_subscription = this.svg_feature.datapoint_hover_out.subscribe(identifier => this.onDatapointHoverOut(identifier));
    this.datapoint_click_subscription = this.svg_feature.datapoint_click.subscribe(identifier => this.onDatapointClick(identifier));
    this.dag_change_subscription = this.dag_feature.dag_change.subscribe(() => this.onDagChange());

    // this.intersection_mode_feature.toggleIntersections(1); // TODO: Remove this line
    // this.intersection_mode_feature.showIntersectionDetails(); // TODO: Remove this line
  }

  ngOnDestroy() {
    this.datapoint_hover_in_subscription.unsubscribe();
    this.datapoint_hover_out_subscription.unsubscribe();
    this.datapoint_click_subscription.unsubscribe();
    this.dag_change_subscription.unsubscribe();
  }

  public async onResize(event) {
    console.log("Resizing window");
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;
    let datapoints = await this.api_service.getDataPoints();

    this.svg_feature.resizeSvg(width, height, datapoints);
    let background_density = await this.api_service.getCurrentLevelBackgroundDensity();
    this.svg_feature.setBackgroundColor(background_density);

    // Always call drawTextLabel when the circles are updated in any way
    for (const datapoint of datapoints) {
      const nb_of_subpatterns = await this.api_service.getNbOfSubpatterns(datapoint.identifier);
      this.svg_feature.drawTextLabel(datapoint.identifier, nb_of_subpatterns);
    }
  }

  private onDatapointHoverIn(identifier: number){
    this.datapoint_hover_in.emit(identifier); // To communicate with pattern summary
  }

  private onDatapointHoverOut(identifier: number){
    this.datapoint_hover_out.emit(identifier); // To communicate with pattern summary
  }

  private async onDatapointClick(identifier: number){
    this.dag_feature.setClickedDatapoint(identifier);
    this.datapoint_click.emit(identifier); // To communicate with pattern summary

    await this.intersection_mode_feature.toggleIntersections(identifier);
  }

  public async onTruncation(event){
    let new_size = event - 1; // -1 because the first point is the null model rss
    let truncated_datapoints = await this.api_service.truncateModel(new_size);

    this.svg_feature.deactivateHighlight();
    this.dag_feature.setClickedDatapoint(null);
    await this.intersection_mode_feature.toggleIntersections(null);
    this.svg_feature.drawDataPoints(truncated_datapoints);
    this.datapoint_click.emit(null);

    // Always call drawTextLabel when the circles are updated in any way
    for (const datapoint of truncated_datapoints) {
      const nb_of_subpatterns = await this.api_service.getNbOfSubpatterns(datapoint.identifier);
      this.svg_feature.drawTextLabel(datapoint.identifier, nb_of_subpatterns);
    }
  }

  private onDagChange(){
    this.dag_change.emit();
  }

  public openSearch(){
    this.svg_feature.deactivateHighlight();
    this.intersection_mode_feature.toggleIntersections(null).then(() => {
      this.dag_feature.setClickedDatapoint(null);
      this.datapoint_click.emit(null); // To communicate with pattern summary
    });
  }

  public async ascendDag(){
    let success = await this.dag_feature.ascendDag();
    if(success){
          let datapoints = await this.api_service.getDataPoints();

          this.svg_feature.drawDataPoints(datapoints, true);
          let background_density = await this.api_service.getCurrentLevelBackgroundDensity();
          this.svg_feature.setBackgroundColor(background_density);
          
          this.datapoint_click.emit(null);

          // Always call drawTextLabel when the circles are updated in any way
          for (const datapoint of datapoints) {
            const nb_of_subpatterns = await this.api_service.getNbOfSubpatterns(datapoint.identifier);
            this.svg_feature.drawTextLabel(datapoint.identifier, nb_of_subpatterns);
          }
        }
  }

  public async descendDag(){
    let success = await this.dag_feature.descendDag();
    if(success){
      let datapoints = await this.api_service.getDataPoints();

      this.svg_feature.drawDataPoints(datapoints, true);
      let background_density = await this.api_service.getCurrentLevelBackgroundDensity();
      this.svg_feature.setBackgroundColor(background_density);

      this.datapoint_click.emit(null);

      // Always call drawTextLabel when the circles are updated in any way
      for (const datapoint of datapoints) {
        const nb_of_subpatterns = await this.api_service.getNbOfSubpatterns(datapoint.identifier);
        this.svg_feature.drawTextLabel(datapoint.identifier, nb_of_subpatterns);
      }
    }
  }

  public async filterDatapoints(filters: string[][]){
    console.log("Filtering datapoints with filters: ", filters);
    let filtered_datapoints: DataPoint[] = await this.api_service.filterDatapoints(filters);
    this.svg_feature.drawDataPoints(filtered_datapoints, false);

    // Always call drawTextLabel when the circles are updated in any way
    for (const datapoint of filtered_datapoints) {
      const nb_of_subpatterns = await this.api_service.getNbOfSubpatterns(datapoint.identifier);
      this.svg_feature.drawTextLabel(datapoint.identifier, nb_of_subpatterns);
    }
  }

  public isOnFirstLevel(){
    return this.dag_feature.current_dag_level == 1;
  }

  public getNbOfDatapoints(){
    return this.svg_feature.getDrawnDataPoints().length;
  }
}
