// https://www.telerik.com/blogs/angular-14-introducing-standalone-components#:~:text=Creating%20a%20Standalone%20Component,ng%20g%20c%20login%20%2D%2Dstandalone
// https://material.angular.io/components/categories
// https://css-tricks.com/snippets/css/a-guide-to-flexbox/
// https://br.pinterest.com/pin/800022321275429738/
// import * as numeric from 'numeric';

import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, ViewChild } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import { VisualizationComponent } from "./components/visualization/visualization.component";
import { MatSlideToggleModule } from '@angular/material/slide-toggle';
import {MatTabsModule} from '@angular/material/tabs';
import {MatButtonToggleModule} from '@angular/material/button-toggle';
import {MatDividerModule} from '@angular/material/divider';
import {MatListModule} from '@angular/material/list';
import {MatSelectModule} from '@angular/material/select';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatMenuModule} from '@angular/material/menu';
import {MatButtonModule} from '@angular/material/button';
import {MatRippleModule} from '@angular/material/core';
import {MatPaginatorModule} from '@angular/material/paginator';
import {MatIconModule} from '@angular/material/icon';
import { CommonModule } from "@angular/common";
import { PatternSummaryComponent } from "./components/dynamic-paginator/pattern-summary/pattern-summary.component";
import { DynamicPaginatorComponent } from "./components/dynamic-paginator/dynamic-paginator.component";
import { open } from '@tauri-apps/api/dialog';
import { RssViewComponent } from "./components/visualization/rss-view/rss-view.component";
import { provideRouter, Router, RouterOutlet} from "@angular/router";
import { environment } from "src/environments/environment";
import {MatSidenavModule} from '@angular/material/sidenav'
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatTooltipModule } from '@angular/material/tooltip';

@Component({
    selector: "app-root",
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
    standalone: true,
    imports: [
        RouterOutlet,
        CommonModule,
        VisualizationComponent,
        RssViewComponent,
        PatternSummaryComponent,
        DynamicPaginatorComponent,
        MatSlideToggleModule,
        MatTabsModule,
        MatButtonToggleModule,
        MatDividerModule,
        MatListModule,
        MatSelectModule,
        MatSlideToggleModule,
        MatCheckboxModule,
        MatMenuModule,
        MatButtonModule,
        MatRippleModule,
        MatPaginatorModule,
        MatSidenavModule,
        MatIconModule,
        MatTooltipModule
    ],
    animations: [
      trigger('slideInOut', [
        state('void', style({
          transform: 'translateX(-100%)',
          opacity: 0
        })),
        state('in', style({
          transform: 'translateX(0)',
          opacity: 1
        })),
        state('out', style({
          transform: 'translateX(-100%)',
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
    ]
    
})

export class AppComponent implements AfterViewInit{
  @ViewChild("aside") aside: ElementRef<HTMLElement>;

  @ViewChild("header") header: ElementRef<HTMLElement>;

  @ViewChild('rss_view') rss_view: RssViewComponent;
  protected rss_view_enabled: boolean = null;
  
  protected visualization_view: VisualizationComponent;

  public selected_directory: string = "";
  public tensor_path: string = "";
  public tensor_name: string = "";
  public patterns_path: string = "";

  public upload_file_mode = "tensor";
  public model_loaded = false;
  public matList_height: number;

  length = 50;
  pageSize = 10;
  pageIndex = 0;

  constructor(private router: Router, private cdr: ChangeDetectorRef){}

  ngAfterViewInit(){
    this.matList_height = this.aside.nativeElement.clientHeight - this.header.nativeElement.clientHeight;

    if(environment.dev_mode){ this.openDagView(); }
    
  }

  public onActivate(componentInstance: any) {
    if (componentInstance instanceof VisualizationComponent) {
      this.visualization_view = componentInstance;
    }

    if (componentInstance instanceof RssViewComponent) {
      this.rss_view = componentInstance;
    }

    if(this.rss_view != undefined){
      console.log("Pattern number: " + this.rss_view.getPatternNumber());
    }
  }

  private openDagView(){
    this.router.navigate(['/visualizationView']);
  }

  public async openTensorDialog(){
    const options = {
      multiple: false
    };
    const selected = await open(options);
    
    if (selected === null) { return; } // No tensor selected
    
    this.tensor_path = selected.toString();
    this.tensor_name = this.tensor_path.split('\\').pop().split('/').pop();
    if (this.tensor_path == ""){ return; } // No tensor selected

    this.selected_directory = this.tensor_path.substring(0, this.tensor_path.lastIndexOf('/'));
    this.upload_file_mode = "patterns";
  }

  public async openPatternsDialog(){
    const options = {
      multiple: false
    };
    const selected = await open(options);

    if (selected === null) { return; } // No tensor selected
    
    this.patterns_path = selected.toString();
    if (this.patterns_path == ""){ return; } // No patterns selected
    
    if (this.tensor_path != "" && this.patterns_path != ""){
      // Both are defined
      invoke("initApplication", {tensorPath: this.tensor_path, patternsPath: this.patterns_path}).then((result: any) =>{
        this.upload_file_mode = "tensor";
        this.model_loaded = true;

        // Forcing a reload
        this.router.navigateByUrl('', {skipLocationChange: true}).then(()=>{
          this.openDagView();
        });

      }).catch((error: any) => {
        console.log(error);
      });
    }
  }

  protected openSettingsDialog(){

  }

  protected toggleRssView(){
    if(this.rss_view_enabled == null){ return; }

    this.rss_view_enabled = !this.rss_view_enabled;
    this.cdr.detectChanges();
  }

  protected disableRssView(){
    this.rss_view_enabled = false;
    this.cdr.detectChanges();
  }

  public onTruncation(event){
    this.visualization_view.onTruncation(event);
  }
}
