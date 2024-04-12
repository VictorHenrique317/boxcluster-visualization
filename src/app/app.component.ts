// https://www.telerik.com/blogs/angular-14-introducing-standalone-components#:~:text=Creating%20a%20Standalone%20Component,ng%20g%20c%20login%20%2D%2Dstandalone
// https://material.angular.io/components/categories
// https://css-tricks.com/snippets/css/a-guide-to-flexbox/
// https://br.pinterest.com/pin/800022321275429738/
// import * as numeric from 'numeric';

import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, ViewChild , Renderer2} from "@angular/core";
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
import { RssViewComponent } from "./components/main_options/rss-view/rss-view.component";
import { provideRouter, Router, RouterOutlet} from "@angular/router";
import { environment } from "src/environments/environment";
import {MatSidenav, MatSidenavModule} from '@angular/material/sidenav'
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatTooltipModule } from '@angular/material/tooltip';
import { FileSelectionDialogComponent } from './components/main_options/file-selection-dialog/file-selection-dialog.component';
import { take } from "rxjs/operators";
import { DialogService } from "./services/dialog/dialog.service";
import { ErrorDialogComponent } from "./components/error-dialog/error-dialog.component";

export enum MainOption {
  MODEL_SELECTOR,
  SETTINGS,
  TRUNCATE_MODEL,
  INTERSECTION_MODE
};

@Component({
    selector: "app-root",
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
    standalone: true,
    imports: [RouterOutlet, CommonModule, VisualizationComponent, RssViewComponent, PatternSummaryComponent,
        DynamicPaginatorComponent, MatSlideToggleModule, MatTabsModule, MatButtonToggleModule, MatDividerModule,
        MatListModule, MatSelectModule, MatSlideToggleModule, MatCheckboxModule, MatMenuModule, MatButtonModule,
        MatRippleModule, MatPaginatorModule, MatSidenavModule, MatIconModule,MatTooltipModule],
    animations: [
      trigger('slideInOut', [
        state('void', style({ transform: 'translateX(-100%)', opacity: 0})),
        state('in', style({ transform: 'translateX(0)', opacity: 1 })),
        state('out', style({ transform: 'translateX(-100%)', opacity: 0 })),
        transition('void => in', [ animate('0.4s ease-in-out') ]),
        transition('in => out', [ animate('0.4s ease-in-out') ]),
        transition('out => in', [ animate('0.4s ease-in-out') ])
      ])
    ]
    
})

export class AppComponent implements AfterViewInit{
  protected MainOption = MainOption;
  protected settings_enabled: boolean = false;
  protected truncate_model_enabled: boolean;
  protected intersection_mode_enabled: boolean = false;

  @ViewChild("aside") aside: ElementRef<HTMLElement>;
  public matList_height: number;

  @ViewChild("sidenav") sidenav: MatSidenav;
  @ViewChild("model_selector") model_selector: ElementRef<HTMLElement>;
  protected tensor_path: string = "";
  protected patterns_path: string = "";
  protected model_loaded = false;
  @ViewChild('rss_view') rss_view: RssViewComponent;
  
  @ViewChild('visualization_view') visualization_view: VisualizationComponent;
  
  constructor(private cdr: ChangeDetectorRef, private dialog_service: DialogService){}

  ngAfterViewInit(){
    this.matList_height = this.aside.nativeElement.clientHeight - this.model_selector.nativeElement.clientHeight;

    if(environment.dev_mode){ 
      this.model_loaded = true;
      this.cdr.detectChanges();
    }
  }

  private reloadApplication(){
    this.model_loaded = false;
    this.cdr.detectChanges();

    this.model_loaded = true;
    this.cdr.detectChanges();
  }

  private handleModelChange(event: any){
    console.log("Handling model change");
    if (event.tensor_path == null || event.patterns_path == null){ return; }

    this.model_loaded = false;
    this.tensor_path = event.tensor_path;
    this.patterns_path = event.patterns_path;
    
    invoke("initApplication", {tensorPath: this.tensor_path, patternsPath: this.patterns_path}).then((result: any) =>{
      this.model_loaded = true;
      this.reloadApplication();

    }).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("ERROR Could not read tensor or patterns.");
    });
  }

  protected toggleMainOption(option: MainOption){
    this.deactivateMainOptionsExcept(option);

    switch(option){
      case MainOption.MODEL_SELECTOR:
        this.openModelSelection();
        break;
      case MainOption.SETTINGS:
        this.toggleSettings();
        break;
      case MainOption.TRUNCATE_MODEL:
        this.toggleTruncateModel();
        break;
      case MainOption.INTERSECTION_MODE:
        this.toggleIntersectionMode();
        break;
    }
  }

  private deactivateMainOptionsExcept(option: MainOption){
    if(this.settings_enabled && option != MainOption.SETTINGS){ this.toggleSettings(); }
    if(this.truncate_model_enabled && option != MainOption.TRUNCATE_MODEL){ this.toggleTruncateModel(); }
    if(this.intersection_mode_enabled && option != MainOption.INTERSECTION_MODE){ this.toggleIntersectionMode(); }
  }

  private openModelSelection(): void {
    

    let dialog_data = {
      tensor_path: this.tensor_path,
      patterns_path: this.patterns_path
    };
    this.dialog_service.open(FileSelectionDialogComponent, 
      '500px', '400px', dialog_data, 
      this.handleModelChange.bind(this));
  }

  private toggleSettings(){
    this.settings_enabled = !this.settings_enabled;
    this.sidenav.toggle();
  }
  
  private toggleTruncateModel(){
    if(this.truncate_model_enabled == undefined){ return; }

    this.truncate_model_enabled = !this.truncate_model_enabled;
    this.cdr.detectChanges();
  }

  private toggleIntersectionMode(){
    this.intersection_mode_enabled = !this.intersection_mode_enabled;
    this.visualization_view.toggleIntersectionMode();
   }

  protected disableRssView(){
    this.truncate_model_enabled = false;
    this.cdr.detectChanges();
  }

  protected onTruncation(event){
    this.rss_view.disableSlider(); 
    setTimeout(() => { 
        this.rss_view.enableSlider();
    }, 1100);

    this.visualization_view.onTruncation(event);
 }
}
