import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, ViewChild , Renderer2, OnDestroy} from "@angular/core";
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
import { PatternSummaryComponent } from "./components/pattern-summary/pattern-summary.component";
import { Pattern } from "./models/pattern";
import { fs } from "@tauri-apps/api";
import { resolveResource } from "@tauri-apps/api/path";
import { ApiService } from "./services/api/api.service";
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import { Subscription } from "rxjs";
import { SearchDialogComponent } from "./components/visualization/search-dialog/search-dialog.component";
import { DataPoint } from "./models/datapoint";
import { MatSliderModule } from "@angular/material/slider";

export enum MainOption {
  MODEL_SELECTOR,
  SETTINGS,
  TRUNCATE_MODEL,
  INTERSECTION_MODE,
  SEARCH
};

export enum ApplicationStatus {
  UNLOADED,
  LOADING,
  LOADED
};

@Component({
    selector: "app-root",
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
    standalone: true,
    animations: [
        trigger('slideInOut', [
            state('void', style({ transform: 'translateX(-100%)', opacity: 0 })),
            state('in', style({ transform: 'translateX(0)', opacity: 1 })),
            state('out', style({ transform: 'translateX(-100%)', opacity: 0 })),
            transition('void => in', [animate('0.4s ease-in-out')]),
            transition('in => out', [animate('0.4s ease-in-out')]),
            transition('out => in', [animate('0.4s ease-in-out')])
        ])
    ],
    imports: [RouterOutlet, CommonModule, VisualizationComponent, RssViewComponent,
        MatSlideToggleModule, MatTabsModule, MatButtonToggleModule, MatDividerModule,
        MatListModule, MatSelectModule, MatSlideToggleModule, MatCheckboxModule, MatMenuModule, MatButtonModule,
        MatRippleModule, MatPaginatorModule, MatSidenavModule, MatIconModule, MatTooltipModule, PatternSummaryComponent,
        MatProgressSpinnerModule,MatSliderModule]
})

export class AppComponent implements AfterViewInit, OnDestroy{
  protected all_application_status = Object.values(ApplicationStatus);
  protected application_status: ApplicationStatus = ApplicationStatus.UNLOADED;

  protected MainOption = MainOption;
  protected settings_enabled: boolean = false;
  protected truncate_model_enabled: boolean;
  protected intersection_mode_enabled: boolean = false;

  protected truncate_model_disabled: boolean = false;

  private previous_filters: string[][] = [];

  @ViewChild("aside") aside: ElementRef<HTMLElement>;
  public matList_height: number;

  @ViewChild("sidenav") sidenav: MatSidenav;
  @ViewChild("model_selector") model_selector: ElementRef<HTMLElement>;
  private last_opened_folder: string = "";
  protected tensor_path: string = "";
  protected patterns_path: string = "";
  @ViewChild('rss_view') rss_view: RssViewComponent;
  @ViewChild('pattern_summary') pattern_summary: PatternSummaryComponent;
  
  @ViewChild('visualization_view') visualization_view: VisualizationComponent;
  protected hovered_pattern: Pattern;

  private datapoint_click_subscription: Subscription;
  private datapoint_hover_in_subscription: Subscription;
  private datapoint_hover_out_subscription: Subscription;
  private dag_change_subscription: Subscription;
  
  // Added property to hold the expansion factor from the slider
  expansionFactor: number = 50;

  constructor(private cdr: ChangeDetectorRef, private dialog_service: DialogService, private api_service: ApiService){}

  async ngAfterViewInit(){

    if(environment.dev_mode){
      console.log("Entering dev mode");
      
      // await fs.readTextFile(await resolveResource('resources/'))

      // let base_path = "../../src-tauri/tests/test_data"
      let tensor_path = await resolveResource('resources/dev_tensor_light.txt'); 
      let patterns_path = await resolveResource('resources/dev_patterns_light.txt');
      
      // let patterns_path = `${base_path}/other_patterns/primary_school.txt`
      this.handleModelChange({tensor_path: tensor_path, patterns_path: patterns_path});
    }
    else{
      this.toggleMainOption(MainOption.MODEL_SELECTOR);
      this.application_status = ApplicationStatus.LOADING;
    }
  }

  ngOnDestroy(){
    this.datapoint_click_subscription.unsubscribe();
    this.dag_change_subscription.unsubscribe();
    this.datapoint_hover_in_subscription.unsubscribe();
    this.datapoint_hover_out_subscription
  }

  private async handleModelChange(event: any){
    console.log("Handling model change");
    if (event.tensor_path == null || event.patterns_path == null){ return; }
    this.application_status = ApplicationStatus.LOADING;
    this.cdr.detectChanges();
    
    this.last_opened_folder = event.last_opened_folder;

    this.tensor_path = event.tensor_path;
    this.patterns_path = event.patterns_path;
    
    try{
      await this.api_service.initApplication(this.tensor_path, this.patterns_path);
    } catch(error){
      console.error(error);
      this.application_status = ApplicationStatus.UNLOADED;
      this.cdr.detectChanges();
      this.tensor_path = "";
      this.patterns_path = "";
      this.toggleMainOption(MainOption.MODEL_SELECTOR);
      this.application_status = ApplicationStatus.LOADING;
      this.dialog_service.openErrorDialog("ERROR Could not read tensor or patterns.");
      return;
    }
    
    this.application_status = ApplicationStatus.LOADED;
    this.cdr.detectChanges();

    this.datapoint_click_subscription = this.visualization_view.datapoint_click.subscribe(identifier => this.onDatapointClick(identifier));
    this.datapoint_hover_in_subscription = this.visualization_view.datapoint_hover_in.subscribe(identifier => this.onDatapointHoverIn(identifier));
    this.datapoint_hover_out_subscription = this.visualization_view.datapoint_hover_out.subscribe(identifier => this.onDatapointHoverOut(identifier));
    this.dag_change_subscription = this.visualization_view.dag_change.subscribe(() => this.onDagChange());
    // this.reloadApplication();

    this.toggleMainOption(null);
    this.togglePatternSummary(null);
    this.updatePatternSummary(null);
  }

  protected toggleMainOption(option: MainOption | null){
    if (this.isOptionDisabled(option)) { return; }
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
      case MainOption.SEARCH:
        this.openSearch();
        break;
      case null:
        break
    }
  }

  private isOptionDisabled(option: MainOption): boolean{
    switch(option){
      case MainOption.SETTINGS:
        return false;
      case MainOption.TRUNCATE_MODEL:
        return this.truncate_model_disabled;
      default:
        return false;
    }
  }

  private deactivateMainOptionsExcept(option: MainOption){
    if(this.settings_enabled && option != MainOption.SETTINGS){ this.toggleSettings(); }
    if(this.truncate_model_enabled && option != MainOption.TRUNCATE_MODEL){ this.toggleTruncateModel(); }
  }

  private openModelSelection(): void {
    let dialog_data = {
      last_opened_folder: this.last_opened_folder,
      tensor_path: this.tensor_path,
      patterns_path: this.patterns_path,
      first_open: this.tensor_path == "" && this.patterns_path == ""
    };
    const dialogRef = this.dialog_service.open(
      FileSelectionDialogComponent,
      FileSelectionDialogComponent.WIDTH,
      FileSelectionDialogComponent.HEIGHT,
      dialog_data,
      this.handleModelChange.bind(this)
    );

    dialogRef.disableClose = true; // Prevent closing on outside click
  }

  private toggleSettings(){
    this.settings_enabled = !this.settings_enabled;
    this.sidenav.toggle();
  }
  
  protected async onExpansionChange(event) {
    let newValue = event.target.ariaValueText;
    this.visualization_view.svg_feature.setExpansionFactor(newValue);
    let datapoints = await this.api_service.getDataPoints();
    await this.visualization_view.svg_feature.drawDataPoints(datapoints);
  }

  private toggleTruncateModel(){
    if(this.truncate_model_enabled == undefined){ return; }

    this.truncate_model_enabled = !this.truncate_model_enabled;
    this.cdr.detectChanges();
  }

  private async filterDatapoints(filters: string[][]){
    this.previous_filters = filters;
    this.visualization_view.filterDatapoints(filters);
  }

  private async openSearch(){
    this.visualization_view.openSearch();
    this.pattern_summary.update(null);
    this.cdr.detectChanges();

    let dialog_data = {previous_filters: this.previous_filters};

    this.dialog_service.open(SearchDialogComponent,
      SearchDialogComponent.WIDTH, 
      SearchDialogComponent.HEIGHT,
      dialog_data, 
      this.filterDatapoints.bind(this));
  }

  private onDatapointClick(identifier){
    // this.toggleMainOption(null);
  }

  private onDatapointHoverIn(identifier){
    this.toggleMainOption(null);
  }

  private onDatapointHoverOut(identifier){
    // this.toggleMainOption(null)
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

  private async onDagChange(){
    if(this.visualization_view.isOnFirstLevel()){
      // await this.rss_view.reset();
    }

    this.truncate_model_disabled = !this.visualization_view.isOnFirstLevel();

    this.truncate_model_enabled = false;
    this.cdr.detectChanges();
  }

  protected updatePatternSummary(identifier){
    this.pattern_summary.update(identifier);
  }

  protected togglePatternSummary(identifier){
    this.pattern_summary.toggleLock(identifier);
  }

  get applicationStatusUnloaded(): ApplicationStatus {
    return ApplicationStatus.UNLOADED;
  }

  get applicationStatusLoading(): ApplicationStatus {
    return ApplicationStatus.LOADING;
  }

  get applicationStatusLoaded(): ApplicationStatus {
    return ApplicationStatus.LOADED;
  }
}
