### DIRECTORY src/app/ FOLDER STRUCTURE ###
/
    app.component.scss
    app.component.ts
    app.component.html
components/
    visualization/
        visualization.component.scss
        visualization.component.ts
        visualization.component.html
        visualization.component.spec.ts
        modules/
            dag-feature.module.ts
            svg-feature.module.ts
            intersection-mode-feature.module.ts
        intersection-details-dialog/
            intersection-details-dialog.component.spec.ts
            intersection-details-dialog.component.scss
            intersection-details-dialog.component.ts
            intersection-details-dialog.component.html
        datapoint-tooltip/
            datapoint-tooltip.component.scss
            datapoint-tooltip.component.ts
            datapoint-tooltip.component.html
            datapoint-tooltip.component.spec.ts
    main_options/
        rss-view/
            rss-view.component.html
            rss-view.component.scss
            rss-view.component.spec.ts
            rss-view.component.ts
        file-selection-dialog/
            file-selection-dialog.component.ts
            file-selection-dialog.component.html
            file-selection-dialog.component.spec.ts
            file-selection-dialog.component.scss
    pattern-summary/
        pattern-summary.component.html
        pattern-summary.component.spec.ts
        pattern-summary.component.scss
        pattern-summary.component.ts
        pattern-dim-dialog/
            pattern-dim-dialog.component.ts
            pattern-dim-dialog.component.scss
            pattern-dim-dialog.component.html
            pattern-dim-dialog.component.spec.ts
    error-dialog/
        error-dialog.component.scss
        error-dialog.component.ts
        error-dialog.component.html
        error-dialog.component.spec.ts
models/
    color.ts
    intersection_details.ts
    pattern.ts
    svg.ts
    datapoint.ts
services/
    svg/
        svg.service.spec.ts
        svg.service.ts
    dialog/
        dialog.service.ts
        dialog.service.spec.ts
    api/
        api.service.spec.ts
        api.service.ts
### DIRECTORY src/app/ FOLDER STRUCTURE ###

### DIRECTORY src/app/ FLATTENED CONTENT ###
### src/app/app.component.scss BEGIN ###
@use '@angular/material' as mat;
// @import '../style.scss';
@import '../theme.scss';

// $primary-palette: map-get($map: , $key: )

$max_nav_height: 5vh;

body{
    // $background: map-get($theme, background);
    overflow: hidden;
    background-color: $background-color;
    // position: absolute;
    z-index: 1;
    // top: 0px;
    // right: 0;
    // left: 0;
    // bottom: 0px;
    display: flex;
    flex-direction: row;

    width: 100vw;
    height: 100vh;

    aside{
        z-index: 2;
        background-color: mat.get-color-from-palette($primary-palette, 900);
        color: mat.get-color-from-palette($primary-palette, '900-contrast');
        // flex: 1;
        display: flex;
        flex-direction: column;

        width: 25vw;
        max-height: 100vh;
        min-height: 100vh;

        // background-color: green;

        #main-options{
            display: flex;
            flex-direction: row;
            justify-content: space-around;
            // background-color: brown;

            padding-top: 0.5em;
            padding-bottom: 0.5em;
            padding-left: 0.5em;
            padding-right: 0.5em;

            height: 10vh;
            
            #matlist-placeholder{
                min-height: $max_nav_height;
                max-height: $max_nav_height;
                padding-top: 1em;
                padding-bottom: 1em;
                padding-left: 1em;
                padding-right: 1em;
            }

            .main-option{
                min-height: $max_nav_height;
                max-height: $max_nav_height;
                padding-top: 1em;
                padding-bottom: 1em;
                padding-left: 1em;
                padding-right: 1em;
    
                user-select: none;
    
                display: flex;
                flex-direction: row;
                align-items: center;
    
                border-radius: 10px;

                background-color: mat.get-color-from-palette($primary-palette, 900);
    
                mat-icon{
                    font-size: 2.5em;
                    width: 1em;
                    height: 1em;
                }
    
                h1{
                    max-width: 80%;
    
                    margin: 0 0 0 0;
    
                    padding-top: 0.5em;
                    padding-bottom: 0.5em;
                    padding-left: 0.5em;
                    padding-right: 0.5em;
                    
                    word-wrap: break-word;
                    font-size: 1.25em;
                }
            }
    
            .main-option:hover{
                background-color: mat.get-color-from-palette($primary-palette, 800);
                cursor: pointer;
            }

            #truncate-model-button{
                mat-icon{
                    transform: scaleX(-1);
                }
            }
        }

        #content-cover{
            width: 25vw;
            height: 90vh;
            
            position: absolute;
            top: 10vh;

            z-index: 2;

            background-color: rgba(0, 0, 0, 1); 
        }

        .sidenav-content{
            height: 90vh;
            background-color: mat.get-color-from-palette($primary-palette, 900);

            #settings-wrapper-lower-content{
                width: 100%;
                height: 100%;
            }

            #settings-wrapper-upper-content{
                background-color: mat.get-color-from-palette($primary-palette, 900);
                width: 80%;

                nav{
                    display: flex;
                    flex-direction: column;

                    .setting{
                        color: white;
                        background-color: mat.get-color-from-palette($primary-palette, 800);

                        margin: 0.5em 1em 0.5em 1em;
                    }
                }
            }

            #pattern-summary{
                width: 100%;
                height: 100%;
                color: white;
            }

            app-dynamic-paginator{
                flex: 4;
            }
        }

        h2{
            padding-left: 1em;
            padding-top: 1em;
        }
    }

    #main-app{
        // background-color: green;
        width: 75vw;
        height: 100vh;
        flex: 4;
        display: flex;
        flex-direction: column;

        #rss_view {
            z-index: 2;

            position: absolute;
            top: 10vh;
            left: 0vw;

            height: 90vh;
            width: 25vw;
        }

        #visualization{
            width: 100%;
            height: 100%;
            // flex: 14;

            #select-model-warning-wrapper{
                width: 100%;
                height: 100%;

                display: flex;
                align-items: center;
                justify-content: center;

                #button-wrapper{
                    display: flex;
                    align-items: center;
                    justify-content: center;

                    padding-top: 1em;
                    padding-bottom: 1em;
                    padding-left: 1em;
                    padding-right: 1em;
        
                    user-select: none;
        
                    display: flex;
                    flex-direction: row;
                    align-items: center;
        
                    border-radius: 10px;

                    background-color: mat.get-color-from-palette($primary-palette, 800);

                    width: 40em;
                    height: 10em;

                    h1{
                        font-size: 2em;
                        color: white;
                    }
                }

                #button-wrapper:hover{
                    background-color: mat.get-color-from-palette($primary-palette, 600);
                    cursor: pointer;
                }
                
            }

            #progress-spinner{
                width: 100%;
                height: 100%;

                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;

                h1{
                    user-select: none;
                    padding: 2em 0 0 0;
                }
            }
        }

        footer{
            // flex: 1;
            // background-color: brown;
        }
    }
}

.floatable {
    z-index: 1;
    position: absolute;
}

.buttonToggled{
    background-color: mat.get-color-from-palette($primary-palette, 800) !important;
}
### src/app/app.component.scss END ###

### src/app/app.component.ts BEGIN ###
// https://www.telerik.com/blogs/angular-14-introducing-standalone-components#:~:text=Creating%20a%20Standalone%20Component,ng%20g%20c%20login%20%2D%2Dstandalone
// https://material.angular.io/components/categories
// https://css-tricks.com/snippets/css/a-guide-to-flexbox/
// https://br.pinterest.com/pin/800022321275429738/
// import * as numeric from 'numeric';

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

export enum MainOption {
  MODEL_SELECTOR,
  SETTINGS,
  TRUNCATE_MODEL,
  INTERSECTION_MODE,
  HIGHLIGHT_SUPERPATTERNS
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
        MatProgressSpinnerModule]
})

export class AppComponent implements AfterViewInit, OnDestroy{
  protected all_application_status = Object.values(ApplicationStatus);
  protected application_status: ApplicationStatus = ApplicationStatus.UNLOADED;

  protected MainOption = MainOption;
  protected settings_enabled: boolean = false;
  protected truncate_model_enabled: boolean;
  protected intersection_mode_enabled: boolean = false;
  protected highlight_superpatterns_enabled: boolean = false;

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
  
  constructor(private cdr: ChangeDetectorRef, private dialog_service: DialogService, private api_service: ApiService){}

  ngAfterViewInit(){
    this.datapoint_click_subscription = this.visualization_view.datapoint_click.subscribe(identifier => this.onDatapointClick(identifier));

    if(environment.dev_mode){
      console.log("Entering dev mode");
      let base_path = "../../src-tauri/tests/test_data/"
      let tensor_path = `${base_path}/tensors/primary_school.txt`
      let patterns_path = `${base_path}/other_patterns/primary_school.txt`
      this.handleModelChange({tensor_path: tensor_path, patterns_path: patterns_path});
    }
  }

  ngOnDestroy(){
    this.datapoint_click_subscription.unsubscribe();
  }

  private async handleModelChange(event: any){
    console.log("Handling model change");
    if (event.tensor_path == null || event.patterns_path == null){ return; }
    this.application_status = ApplicationStatus.LOADING;
    this.cdr.detectChanges();

    this.toggleMainOption(null);
    this.togglePatternSummary(null);
    this.updatePatternSummary(null);
    
    this.last_opened_folder = event.last_opened_folder;

    this.tensor_path = event.tensor_path;
    this.patterns_path = event.patterns_path;
    
    try{
      await this.api_service.initApplication(this.tensor_path, this.patterns_path);
    } catch(error){
      console.error(error);
      this.application_status = ApplicationStatus.UNLOADED;
      this.cdr.detectChanges();
      return;
    }
    
    this.application_status = ApplicationStatus.LOADED;
    this.cdr.detectChanges();
    // this.reloadApplication();
  }

  protected toggleMainOption(option: MainOption | null){
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
      case MainOption.HIGHLIGHT_SUPERPATTERNS:
        this.toggleHighlightSuperpatterns();
        break;
      case null:
        break
    }
  }

  private deactivateMainOptionsExcept(option: MainOption){
    if(this.settings_enabled && option != MainOption.SETTINGS){ this.toggleSettings(); }
    if(this.truncate_model_enabled && option != MainOption.TRUNCATE_MODEL){ this.toggleTruncateModel(); }
    if(this.highlight_superpatterns_enabled && option != MainOption.HIGHLIGHT_SUPERPATTERNS){ this.toggleHighlightSuperpatterns(); }
  }

  private openModelSelection(): void {
    let dialog_data = {
      last_opened_folder: this.last_opened_folder,
      tensor_path: this.tensor_path,
      patterns_path: this.patterns_path
    };
    this.dialog_service.open(FileSelectionDialogComponent, 
      FileSelectionDialogComponent.WIDTH, 
      FileSelectionDialogComponent.HEIGHT, 
      dialog_data, 
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

  private toggleHighlightSuperpatterns(){
    if(this.highlight_superpatterns_enabled == undefined){ return; }

    this.highlight_superpatterns_enabled = !this.highlight_superpatterns_enabled;

    this.visualization_view.toggleHighlightSuperpatterns(this.highlight_superpatterns_enabled);
    this.cdr.detectChanges();
  }

  private onDatapointClick(identifier){
    this.highlight_superpatterns_enabled = false;
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

### src/app/app.component.ts END ###

### src/app/app.component.html BEGIN ###
<body #app_body id="app_body">
    <aside #aside>
        <div id="main-options">
            <header class="main-option" id="model-selection-button" *ngIf="application_status == applicationStatusLoaded" #model_selector matRipple [matRippleCentered]="true"
                (click)="toggleMainOption(MainOption.MODEL_SELECTOR)"
                matTooltip="Change summary">
                <div id="matlist-placeholder" *ngIf="application_status == applicationStatusUnloaded"></div>
                <mat-icon aria-hidden="false" aria-label="open_file" fontIcon="upload_file" ></mat-icon>
            </header>

            <header class="main-option" id="truncate-model-button" 
                [ngClass]="{'buttonToggled': this.truncate_model_enabled}"
                *ngIf="application_status == applicationStatusLoaded"
                (click)="toggleMainOption(MainOption.TRUNCATE_MODEL)"
                matTooltip="Truncate summary">
                    <mat-icon aria-hidden="false" aria-label="settings" fontIcon="timeline"></mat-icon>
            </header>

            <header class="main-option" id="highlight-superpatterns"
                [ngClass]="{'buttonToggled': this.highlight_superpatterns_enabled}"
                *ngIf="application_status == applicationStatusLoaded"
                (click)="toggleMainOption(MainOption.HIGHLIGHT_SUPERPATTERNS)"
                matTooltip="Highlight super-patterns">
                    <mat-icon>all_out</mat-icon>
            </header>
<!-- 
            <header class="main-option" id="settings-button" matRipple [matRippleCentered]="true" 
                [ngClass]="{'buttonToggled': this.settings_enabled}"
                *ngIf="model_loaded"
                (click)="toggleMainOption(MainOption.SETTINGS)"
                matTooltip="Settings">
                <mat-icon aria-hidden="false" aria-label="settings" fontIcon="settings"></mat-icon>
            </header> -->
        </div>
        
        <div id="content-cover" [hidden]="!truncate_model_enabled"></div>
        <mat-sidenav-container #content class="sidenav-content" autosize [ngClass]="{'faded': truncate_model_enabled}">
            <div id="settings-wrapper-lower-content">
                <app-pattern-summary id="pattern-summary" #pattern_summary></app-pattern-summary>
            </div>
        
            <mat-sidenav #sidenav id="settings-wrapper-upper-content" mode="over" fixedInViewport="false">
                <nav>
                    <button class="setting" mat-raised-button [matMenuTriggerFor]="scaleMenu">Scale</button>
                    <mat-menu class="setting" #scaleMenu="matMenu">
                        <button mat-menu-item>Linear</button>
                        <button mat-menu-item>Logarithmic</button>
                    </mat-menu>
        
                    <button class="setting" mat-raised-button [matMenuTriggerFor]="areaMenu">Area attribute</button>
                    <mat-menu class="setting" #areaMenu="matMenu">
                        <button mat-menu-item>Pattern size</button>
                        <button mat-menu-item>Density</button>
                        <button mat-menu-item>G</button>
                    </mat-menu>
        
                    <mat-menu class="setting" #groupMenu="matMenu">
                        <button mat-menu-item>Flat</button>
                        <button mat-menu-item>Group by fonts</button>
                    </mat-menu>
                </nav>
            </mat-sidenav>
        </mat-sidenav-container>

        
    </aside>

    <div id="main-app">
        <app-rss-view #rss_view id="rss_view" class="floatable" 
                (onTruncation)="onTruncation($event)"
                (initialized)="disableRssView()"
                [@slideInOut]="truncate_model_enabled ? 'in' : 'out'"
                *ngIf="application_status == applicationStatusLoaded">
        </app-rss-view>

        <div id="visualization">
            <!-- <router-outlet 
                (activate)="onActivate($event)"
                (onTruncationFinished)="onTruncationFinished()"
                >
            </router-outlet> -->
            <div id="select-model-warning-wrapper" *ngIf="application_status == applicationStatusUnloaded">
                <div id="button-wrapper" (click)="toggleMainOption(MainOption.MODEL_SELECTOR)">
                    <h1> Select summary to visualize </h1>
                </div>
            </div>

            <div id="progress-spinner" *ngIf="application_status == applicationStatusLoading">
                <mat-spinner></mat-spinner>
                <h1>Loading visualization...</h1>
            </div>
            
            <app-visualization #visualization_view
                (onTruncation)="onTruncation($event)"
                (datapoint_hover_in)="updatePatternSummary($event)"
                (datapoint_hover_out)="updatePatternSummary(null)"
                (datapoint_click)="togglePatternSummary($event)"
                *ngIf="application_status == applicationStatusLoaded">
            </app-visualization>
        </div>
    </div>
</body>
### src/app/app.component.html END ###

### src/app/components/visualization/visualization.component.scss BEGIN ###
@use '@angular/material' as mat;
@import '../../../theme.scss';

html, body{
    display: flex;
    flex-direction: row;
    justify-content: center;
    // background-color: yellow;
    height: 100vh;
    width: 100;

    overflow: hidden;

    position: relative;

    section{
        display: flex;
        flex-grow: 4;

        justify-content: center;

        position: relative;

        #dag-controls{
            right: 1em;
            bottom: 50%;

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;

            padding: 1em 1em 1em 1em;

            border-radius: 10px;

            background-color: mat.get-color-from-palette($primary-palette, 500);

            span{
                color: white;
            }

            mat-icon{
                padding-top: 0.5em;
                padding-bottom: 0.5em;
                padding-left: 0.5em;
                padding-right: 0.5em;

                border-radius: 10px;

                background-color: rgb(75, 142, 230);
                color: white;
            }

            mat-icon:hover{
                cursor: pointer;
                background-color: rgb(103, 157, 228);
            }
        }

        #vizualization_div{
            z-index: 0;
            position: relative;
            object-fit: cover;
            width: 100%;
            height: 100%;

            // background-color: red;
            object-fit:contain;
        }

        #intersection_details{
            bottom: 5%;
            right: 5%;
        }
    }

    #datapoint_tooltip{
        position: absolute;
        top: 0;
        left: 0;

        width: 15%;
        height: 15%;
    }
}

.floatable {
    z-index: 1;
    position: absolute;
}

.dashed-outline {
    border:  2px dashed #000;
}

.button-disabled{
    color: rgb(145, 145, 145) !important;
    background-color: rgb(209, 209, 209) !important;
}

.button-disabled:hover{
    cursor: not-allowed !important;
}
### src/app/components/visualization/visualization.component.scss END ###

### src/app/components/visualization/visualization.component.ts BEGIN ###
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

  private datapoint_hover_in_subscription: Subscription;
  private datapoint_hover_out_subscription: Subscription;
  private datapoint_click_subscription: Subscription;

  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  @ViewChild('vizualization_div') visualization_div: ElementRef<HTMLDivElement>;

  private svg_feature: SvgFeatureModule;
  protected intersection_mode_feature: IntersectionModeFeatureModule;
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
    this. datapoint_hover_in_subscription = this.svg_feature.datapoint_hover_in.subscribe(identifier => this.onDatapointHoverIn(identifier));
    this.datapoint_hover_out_subscription = this.svg_feature.datapoint_hover_out.subscribe(identifier => this.onDatapointHoverOut(identifier));
    this.datapoint_click_subscription = this.svg_feature.datapoint_click.subscribe(identifier => this.onDatapointClick(identifier));
    
    let datapoints = await this.api_service.getDataPoints();
    this.svg_feature.drawDataPoints(datapoints);

    this.intersection_mode_feature = new IntersectionModeFeatureModule(this.svg_feature, this.dialog_service, this.api_service);

    this.dag_feature = new DagFeatureModule(this.intersection_mode_feature, this.svg_feature, this.api_service);
    await this.dag_feature.init();

    // this.intersection_mode_feature.toggleIntersections(1); // TODO: Remove this line
    // this.intersection_mode_feature.showIntersectionDetails(); // TODO: Remove this line
  }

  ngOnDestroy() {
    this.datapoint_hover_in_subscription.unsubscribe();
    this.datapoint_hover_out_subscription.unsubscribe();
    this.datapoint_click_subscription.unsubscribe();
  }

  public onResize(event) {
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.svg_feature.resizeSvg(width, height);
  }

  private onDatapointHoverIn(identifier: number){
    this.datapoint_hover_in.emit(identifier); // To communicate with pattern summary
  }

  private onDatapointHoverOut(identifier: number){
    this.datapoint_hover_out.emit(identifier); // To communicate with pattern summary
  }

  private async onDatapointClick(identifier: number){
    this.datapoint_click.emit(identifier); // To communicate with pattern summary
    this.toggleHighlightSuperpatterns(false);
    await this.intersection_mode_feature.toggleIntersections(identifier);
    this.dag_feature.setClickedDatapoint(identifier);
  }

  public async onTruncation(event){
    let new_size = event - 1; // -1 because the first point is the null model rss
    let truncated_datapoints = await this.api_service.truncateModel(new_size);

    this.svg_feature.deactivateHighlight();
    await this.intersection_mode_feature.toggleIntersections(null);
    this.svg_feature.drawDataPoints(truncated_datapoints);
    this.datapoint_click.emit(null);
  }

  public toggleHighlightSuperpatterns(toggle: boolean){
    if(toggle == true && this.dag_feature.isHighlightingSuperpatterns()){ return; }
    if(toggle == false && !this.dag_feature.isHighlightingSuperpatterns()){ return; }
    
    this.svg_feature.deactivateHighlight();
    this.intersection_mode_feature.toggleIntersections(null).then(() => {
      this.dag_feature.setClickedDatapoint(null);
      this.dag_feature.toggleHighlightSuperpatterns(toggle);
    });
  }

  public ascendDag(){
    this.dag_feature.ascendDag();
  }

  public descendDag(){
    this.dag_feature.descendDag();
  }
}

### src/app/components/visualization/visualization.component.ts END ###

### src/app/components/visualization/visualization.component.html BEGIN ###
<body #body (window:resize)="onResize($event)">
    <section>
        <div id="dag-controls" class="floatable">
            <span>Level: {{dag_feature.current_dag_level}}</span>
            <mat-icon id="arrow-up" [class.button-disabled]="!dag_feature.upper_dag_arrow_active" fontIcon="keyboard_arrow_up"  matTooltip="Ascend on DAG" matTooltipPosition="above"
            (click)="ascendDag()"></mat-icon>
            
            <mat-icon id="arrow-down" [class.button-disabled]="!dag_feature.lower_dag_arrow_active" fontIcon="keyboard_arrow_down" matTooltip="Descend on DAG"
            (click)="descendDag()"></mat-icon>
        </div>
        <div #vizualization_div id="vizualization_div"></div>
    </section>
</body>
### src/app/components/visualization/visualization.component.html END ###

### src/app/components/visualization/visualization.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { VisualizationComponent } from './visualization.component';

describe('VisualizationComponent', () => {
  let component: VisualizationComponent;
  let fixture: ComponentFixture<VisualizationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ VisualizationComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(VisualizationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/visualization/visualization.component.spec.ts END ###

### src/app/components/visualization/modules/dag-feature.module.ts BEGIN ###
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

            this.drawNewLevelDatapoints(datapoints);

            this.current_dag_level += 1;
        });
    }
}
### src/app/components/visualization/modules/dag-feature.module.ts END ###

### src/app/components/visualization/modules/svg-feature.module.ts BEGIN ###
import * as d3Tip from "d3-tip";
import { ChangeDetectorRef, ElementRef, EventEmitter, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import * as d3 from 'd3';
import { DataPoint } from 'src/app/models/datapoint';
import { legendCircle } from 'src/js/circle_legend.js';
import { Legend } from 'src/js/color_legend.js';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class SvgFeatureModule {
  public datapoint_hover_in = new EventEmitter<number>();
  public datapoint_hover_out = new EventEmitter<number>();
  public datapoint_click = new EventEmitter<number>();

  private locked_datapoint: DataPoint;
  private datapoints: Array<DataPoint>;
  private datapoints_mapping: Map<number, DataPoint>;
  
  private visualization_div: ElementRef;
  public plot: any;
  public svg: any;

  private zoom_level: number;
  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;

  private tooltip;
  private transition_duration = 300;

  private cdr: ChangeDetectorRef;

  constructor(cdr: ChangeDetectorRef){ 
    this.cdr = cdr;
  }

  public init(visualization_div: ElementRef, svg_width: number, svg_height: number){
    this.visualization_div = visualization_div;
    this.svg_width = svg_width;
    this.svg_height = svg_height;

    this.tooltip = d3Tip.default()
      .attr('class', 'd3-tip')
      .offset([-10, 0])
      .html(function(d) {
        return "\
          <div style='background-color:#ededed; padding: 0.5em 0.5em 0.5em 0.5em; border-radius: 10px; border: 1px dashed black;'>\
            <strong>ID:</strong> <span style='color:#BC2602'>" + d.identifier + "</span><br>\
            <strong>Size:</strong> <span style='color:#BC2602'>" + d.pattern_size + "</span><br>\
            <strong>Density:</strong> <span style='color:#BC2602'>" + Math.round(d.density * 100) / 100 + "</span>\
          </div>\
          ";
      });
    
    this.svg = this.createSvg();
    this.resizeSvg(this.svg_width, this.svg_height);
    this.cdr.detectChanges();
    
    this.zoom_level = this.initial_scale;
  }

  public createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
      .append('svg')
        .attr('width', this.svg_width)
        .attr('height',this.svg_height)
        .on('dblclick', () => {  });

    return svg;
  }

  public resizeSvg(width: number, height: number){
    this.svg
      .attr('width', width)
      .attr('height', height);

    let x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, (height - this.y_correction)/1]);

    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([(height - this.y_correction)/1, 0]);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.svg_width = width;
    this.svg_height = height;

    this.createPlot();
    this.drawDataPoints(this.datapoints);
  }

  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g")
      .attr("id", "plot")
      .on('dblclick', () => {  });
    
    let panning_zoom = d3.zoom()
      .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
      .translateExtent([[0, 0], [this.svg_height, this.svg_height]])
      .on("start", (event, d) => { this.svg.attr("cursor", "grabbing"); })
      .on("zoom", (event) => { 
        this.plot.attr("transform", event.transform); 
        if(event.sourceEvent instanceof WheelEvent){
          this.zoom_level = event.transform.k;
          this.drawCircleLegend();
        }
      })
      .on("end", (event, d) => {this.svg.attr("cursor", "default")});

    this.svg.call(panning_zoom);

    // Apply initial zoom level
    let x_translation_factor = 0.0;
    let y_translation_factor = 0.2;
    let initial_transform = d3.zoomIdentity
      .translate(-this.svg_width*(x_translation_factor), -this.svg_height*(y_translation_factor))
      .scale(this.initial_scale);
    this.svg.call(panning_zoom.transform, initial_transform);
    
    this.drawGridLines();
    this.drawUnselectionRect();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
    let grey_tonality = 220;
    let color = `rgb(${grey_tonality}, ${grey_tonality}, ${grey_tonality})`;
    
    this.plot.append("g") // Add the X gridlines
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", color)
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    this.plot.append("g") // Add the Y gridlines
      .attr("class", "grid")
      .attr("color", color)
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }

  private drawUnselectionRect(){
    this.plot.append('rect')
        .attr('id', 'overlay')
        .attr('x', 0)
        .attr('y', 0)
        .attr('width', this.svg_width)
        .attr('height', this.svg_height)
        .style('fill', 'rgba(0, 0, 0, 0)')
        .lower()
        .on('click', (event, d) => { 
          this.locked_datapoint = undefined;
          this.toggleHighlight(undefined);
          this.datapoint_click.emit(null) 
        });
  }

  private drawCircleLegend(){
    let min_pattern_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let max_pattern_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let mean_pattern_size = 0;
    for(let i = 0; i < this.datapoints.length; i++){
      mean_pattern_size += this.datapoints[i].pattern_size;
    }
    mean_pattern_size /= this.datapoints.length;
    mean_pattern_size = Math.round(mean_pattern_size);

    let min_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;
    let max_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;

    let legend = legendCircle(null)
      .scale(
        d3.scaleLinear()
            .domain([min_pattern_size, max_pattern_size])
            .range([min_size, max_size])
      )
      .tickValues([min_pattern_size, mean_pattern_size, max_pattern_size])
      .tickFormat((d, i, e) => `${d}${i === e.length - 1 ? " Cells" : ""}`)
      .tickSize(max_size); // defaults to 5
    
    const legend_x_padding = 10;
    const legend_y_padding = 10;
  
    this.svg.select("#circle_legend").remove();
    this.svg.append("g")
      .attr('id', 'circle_legend')
      .attr('transform', `translate(${legend_x_padding}, ${legend_y_padding})`)
      .call(legend);
  }

  private drawColorLegend(){
    let oldLegend = document.getElementById("color_legend");
    if(oldLegend){
        oldLegend.parentNode.removeChild(oldLegend);
    }

    let svg_width = this.svg.attr('width');
    let legend_width = 320;
    const legend_x_padding = 10;

    let legend_x = svg_width - (legend_width + legend_x_padding);

    let legend = Legend(d3.scaleLinear([0, 1], ["rgba(255,255,255,1)", "rgba(255,0,0,1)"]), {
      title: "Density",
      width: legend_width,
    })

    let legendGroup = this.svg.append('g')
      .attr('id', 'color_legend')
      .attr("transform", `translate(${legend_x}, 0)`);
      
    legendGroup.node().appendChild(legend);
  }

  private scalingFunction(datapoints: Array<DataPoint>) {
    // let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    // let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));
    // let max_module = Math.max(x_max_module, y_max_module);

    let scaled_datapoints: Array<DataPoint> = datapoints;
    let screen_coverage = 0.5;
    // let screen_coverage = 0.8;
    datapoints.forEach(datapoint => {
      //   let result_x = datapoint.x / x_max_module;
      //   let result_y = datapoint.y / y_max_module;

      // if (isNaN(result_x) || !isFinite(result_x)) { result_x = datapoint.x; }
      // if (isNaN(result_y) || !isFinite(result_y)) { result_y = datapoint.y; }

        datapoint.x = datapoint.x / ((1-screen_coverage) + 1);
        datapoint.y = datapoint.y / ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  private toggleHighlight(datapoint: DataPoint){
    if(this.locked_datapoint){ return; }

    let highlight_circle = this.plot.selectAll('.highlight');
    if(highlight_circle){ highlight_circle.remove(); }
    
    if(datapoint){ // Add a EMPTY circle with id highlight, the circle should not block mouse hover and click events
       // Draw a new blue circle on the coordinates of datapoint
      let highlight_radius = datapoint.size * 1.6;
      let highlight_color = 'rgba(114, 232, 247)';
      let highlight_opacity = 0.5;
      let stroke_width = highlight_radius/3;

      this.plot.append('circle')
        .attr('class', 'highlight')
        .attr('cx', this.x_scale(datapoint.x))
        .attr('cy', this.y_scale(datapoint.y))
        .attr('r', highlight_radius)
        .attr('fill', 'none')
        .attr('stroke', highlight_color)
        .attr('stroke-width', stroke_width)
        .attr('opacity', highlight_opacity)
        .style('pointer-events', 'none');

      this.plot.append('circle')
        .attr('class', 'highlight')
        .attr('cx', this.x_scale(datapoint.x))
        .attr('cy', this.y_scale(datapoint.y))
        .attr('r', highlight_radius*1.4)
        .attr('fill', 'none')
        .attr('stroke', highlight_color)
        .attr('stroke-width', stroke_width/2)
        .attr('opacity', highlight_opacity)
        .style('pointer-events', 'none');
      
    }
  }

  public deactivateHighlight(){
    this.locked_datapoint = undefined;
    this.toggleHighlight(undefined);
  }

  public drawDataPoints(datapoints: Array<DataPoint>, force_redraw: boolean = false) {
    if(datapoints == undefined || datapoints == null){ return; }
    if(this.plot == undefined){ return; }
    
    console.log("Drawing " + datapoints.length + " datapoints");
    console.log(datapoints);
    let transition_duration = this.transition_duration;
    if(force_redraw){ 
      this.plot.selectAll('.datapoint').remove();
      transition_duration = 0; 
    }

    this.datapoints = datapoints;
    this.datapoints_mapping = new Map<number, DataPoint>();
    this.datapoints.forEach(datapoint => this.datapoints_mapping.set(datapoint.identifier, datapoint));

    this.plot.call(this.tooltip);

    let scaled_datapoints = this.datapoints;
    if(!force_redraw){ scaled_datapoints = this.scalingFunction(datapoints); }
    
    const circles = this.plot.selectAll('.datapoint')
        .data(scaled_datapoints, d => d.identifier);

    circles.exit()
        .transition()
        .duration(transition_duration)
        .attr('r', 0)
        .remove(); 

    circles.transition()
        .duration(transition_duration) 
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)));

    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('class', 'datapoint')
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', 0)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer')
        .style('stroke', 'rgba(255, 0, 0, 1')
        .on('mouseover', (event, d) => { 
          this.toggleHighlight(d);
          this.tooltip.show(d, event.currentTarget);
          this.datapoint_hover_in.emit(d.identifier);
        })
        .on('mouseout', (event, d) => { 
          this.toggleHighlight(undefined);
          this.tooltip.hide(d, event.currentTarget); 
          this.datapoint_hover_out.emit(d.identifier);
        })
        .on('click', (event, d) => {
          if((this.locked_datapoint) && (this.locked_datapoint.identifier == d.identifier)){ 
            // Unhighlight the locked datapoint
            this.locked_datapoint = undefined;
            this.toggleHighlight(undefined);
          }else{
            // Highlight the clicked datapoint and lock
            this.locked_datapoint = undefined;
            this.toggleHighlight(d);
            this.locked_datapoint = d;
          }

          this.datapoint_click.emit(d.identifier);
         })
        .transition()
        .duration(transition_duration)
        .attr('r', d => d.size);
    
    this.drawCircleLegend();
    this.drawColorLegend();
  }

  public showTooltip(datapoint: DataPoint, circle: any){
    this.tooltip.show(datapoint, circle);
  }

  public hideTooltip(datapoint: DataPoint, circle: any){
    this.tooltip.hide(datapoint, circle);
  }

  public xScale(x: number){
    return this.x_scale(x);
  }

  public yScale(y: number){
    return this.y_scale(y);
  }

  public getDrawnDataPoints(){
    return this.datapoints;
  }

  public getSvgWidth(){
    return this.svg_width;
  }

  public getSvgHeight(){
    return this.svg_height;
  }

  public getDatapoint(identifier: number){
    return this.datapoints_mapping.get(identifier);
  }
}
### src/app/components/visualization/modules/svg-feature.module.ts END ###

### src/app/components/visualization/modules/intersection-mode-feature.module.ts BEGIN ###
import { EventEmitter, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';
import { SvgFeatureModule } from './svg-feature.module';
import * as d3 from 'd3';
import { environment } from 'src/environments/environment';
import { fs, invoke } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { IntersectionDetailsDialogComponent } from '../intersection-details-dialog/intersection-details-dialog.component';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import { ApiService } from 'src/app/services/api/api.service';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class IntersectionModeFeatureModule {
  private old_clicked_datapoint = null;
  private clicked_datapoint_data: DataPoint = null;
  private intersection_details: IntersectionDetails;
  private transition_duration: number = 300;

  private svg_feature: SvgFeatureModule;
  private dialog_service: DialogService;
  private api_service: ApiService

  constructor(svg_feature: SvgFeatureModule, dialog_service: DialogService, api_service: ApiService) {
    this.svg_feature = svg_feature;
    this.dialog_service = dialog_service;
    this.api_service = api_service;
  }

  private connectDatapoints(center: DataPoint, intersections:Map<number, number>){
    let svg_circles = this.svg_feature.plot.selectAll('.datapoint');
    let id_to_datapoint = new Map<number, DataPoint>(svg_circles.data()
      .map(d => [d.identifier, d]));

    for(let [identifier, percentage] of intersections.entries()){
      if(identifier == this.clicked_datapoint_data.identifier){ continue; } // itself
      if(identifier == 0){ continue; } // Excess intersections
      let related_datapoint = id_to_datapoint.get(identifier) || null;
      if (related_datapoint == null) { continue; } // Related circle is a subpattern

      let related_circle = svg_circles.filter(d => d.identifier == identifier);

      let greatest_density_color = `rgba(${center.r}, ${center.g}, ${center.b}, ${center.a})`;
      if(related_datapoint.density > center.density){
        greatest_density_color = `rgba(${related_datapoint.r}, ${related_datapoint.g}, ${related_datapoint.b}, ${related_datapoint.a})`;
      }

      let stroke_width = 6 * percentage + 2; // 2 to 8

      let x1 = this.svg_feature.xScale(center.x);
      let y1 = this.svg_feature.yScale(center.y);
      let line = this.svg_feature.plot.append('line')
        .datum({identifier:identifier, x1: x1, y1: y1})  // Bind the original coordinates to the line
        .raise()
        .attr('class', 'intersection_line')
        .attr('x1', this.svg_feature.xScale(center.x))  // Start position (x) of the line
        .attr('y1', this.svg_feature.yScale(center.y))  // Start position (y) of the line
        .attr('x2', this.svg_feature.xScale(center.x))  // Initially, end position (x) is the same as start position
        .attr('y2', this.svg_feature.yScale(center.y))  // Initially, end position (y) is the same as start position
        .attr('stroke', greatest_density_color)
        .attr('stroke-width', stroke_width)
        .on('mouseover', (event, l) => {
          d3.select(event.currentTarget).style('cursor', 'pointer');
          d3.select(event.currentTarget).attr('stroke-width', stroke_width * 3);
        })
        .on('mouseout', (event, l) => {
          d3.select(event.currentTarget).style('cursor', 'default');
          d3.select(event.currentTarget).attr('stroke-width', stroke_width);
        })
        .on('click', (event, l) => {
          this.showIntersectionDetails(l.identifier);
        })
        .transition('mouseover')
        .duration(this.transition_duration*2)
        .attr('x2', this.svg_feature.xScale(related_datapoint.x))  // Actual end position (x) of the line
        .attr('y2', this.svg_feature.yScale(related_datapoint.y))  // Actual end position (y) of the line
    }
  }

  private highlightDatapoints(relationed_identifiers: Array<number>){
    let identifiers_set = new Set(relationed_identifiers);
    let gray_shade = 196;
    let gray = `rgba(${gray_shade}, ${gray_shade}, ${gray_shade}, 0.5)`;

    this.svg_feature.plot.selectAll('.datapoint')
      .filter(d => !identifiers_set.has(d.identifier) && d.identifier != this.clicked_datapoint_data.identifier)
      .raise()
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('fill', d => gray)
      .style('stroke', d=> gray);
  }

  private createIntersectionChart(root_circle: any, intersections: Map<number, number>, original_radius: number, chart_radius: number){
    let root_datapoint = root_circle.node().__data__;
    let pie = d3.pie()
      .value((d: any) => d.value);

    let data: Array<any> = Array.from(intersections, ([key, value]) => ({key, value}));
    let pie_data = pie(data);
    
    let original_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => original_radius);
    let pie_chart_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(chart_radius);

    let pie_group = this.svg_feature.plot.append('g')
      .attr('class', 'pie_chart')
      .attr('transform', `translate(${root_circle.attr('cx')}, ${root_circle.attr('cy')})`);

    pie_group.selectAll('path')
      .data(pie_data)
      .enter()
      .append('path')
      .attr('pointer-events', 'none')
      .attr('d', original_arc)
      .attr('fill', 'red')
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('d', pie_chart_arc)
      .attr('fill', (d: any) => {
        let related_datapoint = this.svg_feature.getDatapoint(d.data.key);
        let r = 2;
        let g = 178;
        let b = 227;
        let a = 1;

        if(related_datapoint){ // If it isnt id 0 (which means total intersection to the clicked datapoint)
          // Dont color the percetage related to intersection with itself   
          if(related_datapoint.identifier == root_datapoint.identifier){ a = 0; }
        }

        let color = `rgba(${r}, ${g}, ${b}, ${a})`;
        return color;
      });
  }

  private createIntersectionCharts(identifiers: Array<number>, intersections: Map<number, number>){
    let clicked_datapoint = this.svg_feature.plot.selectAll('.datapoint')
      .filter(d => d.identifier == this.clicked_datapoint_data.identifier);

    let intersection_data: Map<number, number>  = new Map<number, number>();
    let parent_current_percentage = intersections.get(this.clicked_datapoint_data.identifier);
    let complement_percentage = 1 - parent_current_percentage; // Colored with current circle color
    intersection_data.set(0, parent_current_percentage);
    intersection_data.set(this.clicked_datapoint_data.identifier, complement_percentage);

    let original_radius = this.clicked_datapoint_data.size;
    let chart_radius = this.clicked_datapoint_data.size;
    this.createIntersectionChart(clicked_datapoint, intersection_data, original_radius, chart_radius);
    intersections.delete(this.clicked_datapoint_data.identifier);
  
    let identifiers_set = new Set(identifiers);
    let circles = this.svg_feature.plot.selectAll('.datapoint')
      .filter(d => identifiers_set.has(d.identifier));

    circles.each((d, i, nodes) => {
      intersection_data = new Map<number, number>();
      let parent_current_percentage = intersections.get(d.identifier); // Colored with the parent color
      let complement_percentage = 1 - parent_current_percentage; // Colored with current circle color

      intersection_data.set(this.clicked_datapoint_data.identifier, parent_current_percentage);
      intersection_data.set(d.identifier, complement_percentage);

      original_radius = d.size;
      chart_radius = d.size;
      this.createIntersectionChart(d3.select(nodes[i]), intersection_data, original_radius, chart_radius);
    });
  }

  private async showIntersections(){
    if(this.clicked_datapoint_data == null){ return };

    let intersections = await this.api_service.getIntersectionsPercentages(this.clicked_datapoint_data.identifier);

    let relationed_datapoints: Array<number> = Array.from(intersections.keys())
      .filter(d => (d != this.clicked_datapoint_data.identifier) && (d != 0));

    this.highlightDatapoints(relationed_datapoints);
    this.connectDatapoints(this.clicked_datapoint_data, intersections);
    let expansion_factor = 1;
    // this.expandCircle(clicked_circle, expansion_factor, intersections, intersections_colors);
    this.createIntersectionCharts(relationed_datapoints, intersections);
  }

  private async hideIntersections(no_transition: boolean = false){
    let transition_duration = no_transition? 0 : this.transition_duration;
    let intersection_lines = this.svg_feature.svg.selectAll('.intersection_line');
    await intersection_lines
      .transition('mouseout')
      .duration(transition_duration)
      .attr('x2', d => d.x1)  // End position (x) becomes the start position
      .attr('y2', d => d.y1)  // End position (y) becomes the start position
      .remove();

    if(this.clicked_datapoint_data != null){
      let circle_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => this.clicked_datapoint_data.size);

      let pie_chart = this.svg_feature.svg.selectAll('.pie_chart');
      await pie_chart.selectAll('path')
        .transition('mouseout')
        .duration(transition_duration)
        .attr('d', d=> d.size)
        .remove();  // Remove the paths after the transition

      this.svg_feature.drawDataPoints(this.svg_feature.getDrawnDataPoints(), true);
    }
  }

  public async toggleIntersections(identifier: number, no_transition: boolean = false){
    await this.hideIntersections(no_transition);
    await this.updateClickedDatapoint(identifier);

    if(identifier == null || identifier==undefined){return;}

    if((this.old_clicked_datapoint != null) && (identifier == this.old_clicked_datapoint.identifier)){ // Datapoint was clicked again
      await this.updateClickedDatapoint(null);
    }

    await this.showIntersections();
  }

  private async updateClickedDatapoint(identifier: number) {
    this.old_clicked_datapoint = this.clicked_datapoint_data;

    if(identifier == null){
      this.clicked_datapoint_data = null;
      this.intersection_details = null;
      return;
    }

    let clicked_circle = this.svg_feature.plot.selectAll('.datapoint')
      // .filter(d => d.identifier == 13); // Fix black color
      .filter(d => d.identifier == identifier);
    this.clicked_datapoint_data = clicked_circle.node().__data__;

    this.intersection_details = await this.api_service.getIntersectionDetails(this.clicked_datapoint_data.identifier);
  }

  public clickedPatternHasIntersections(): boolean {
    if(this.intersection_details == null){
      return false;
    }

    return this.intersection_details.intersections.size > 0;
  }

  public getClickedDataPoint(): DataPoint {
    return this.clicked_datapoint_data;
  }

  public async showIntersectionDetails(intersector_id: number){
    if(this.clicked_datapoint_data == null){
      console.warn("No clicked datapoint to show details.");
      return;
    }

    let intersection_details = await this.api_service.getIntersectionDetails(this.clicked_datapoint_data.identifier);

    let dialog_data = {
      intersector: intersector_id,
      intersection_details: intersection_details
    }

    this.dialog_service.open(IntersectionDetailsDialogComponent, 
      IntersectionDetailsDialogComponent.WIDTH, 
      IntersectionDetailsDialogComponent.HEIGHT, 
      dialog_data);
  }
}

### src/app/components/visualization/modules/intersection-mode-feature.module.ts END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IntersectionDetailsDialogComponent } from './intersection-details-dialog.component';

describe('IntersectionDetailsDialogComponent', () => {
  let component: IntersectionDetailsDialogComponent;
  let fixture: ComponentFixture<IntersectionDetailsDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ IntersectionDetailsDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IntersectionDetailsDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.spec.ts END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.scss BEGIN ###
@use '@angular/material' as mat;
@import '../../../../theme.scss';

body{
    margin: 0 0 0 0;
    padding: 0 0 0 0;


    width: 100%;
    height: 100%;
    
    display: flex;
    flex-direction: column;
    align-items: center;

    overflow-x: hidden;
    overflow-y: scroll;

    // background-color: green;

    header{
        width: 100%;
        height: fit-content;

        padding: 1em 1em 1em 4em;

        // background-color: blue;
    }

    section{
        width: 95%;
        height: 90%;

        padding: 1em 1em 1em 2em;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: flex-start;
        // background-color: yellow;

        // overflow: hidden;

        #pattern-dropdown-wrapper{
            
        }

        h2{
            font-weight: normal;
        }

        #intersector_table_wrapper{
            width: 100%;
            overflow: auto;
            
            // background-color: blue;
            table{
                max-width: 90%;
                // border-left: 1px solid #ccc;
                // border-right: 1px solid #ccc;

                .mat-column-dim_number{ 
                    text-align: center;
                    width: 5em;
                }

                .mat-column-dim_values_preview{
                    text-align: center;
                    width: 100%;
                    // background-color: blue;
                }

                .mat-column-expand{
                    // background-color: brown;
                    width: 3em;
                }

                th{ // Table header
                    user-select: none;
                    cursor: default;
                }

                tr.data-row{
                    // Selector for the rows
                }

                tr.data-row:not(.expanded-row):hover {
                    background: whitesmoke;
                }
                
                tr.data-row:not(.expanded-row):active {
                    background: #efefef;
                }

                .data-row td {
                    border-bottom-width: 0;
                }

                .mat-column-expand{
                    button{
                        // width: 2em;
                        mat-icon{
                            // font-size: 1em;
                        }
                    }
                    
                }

                tr.detail-row {
                    height: 0;
                }
                
                .detail {
                    display: flex;

                    // background-color: red;
                    width: 100%;
                    word-break: normal;
                }
                
                .detail-value {
                    padding: 16px;
                }
                
                .detail-value-attribution {
                    opacity: 0.5;
                }
            }
        }
    }
}

.expanded-row{
    
}

.selected-row{
    background: whitesmoke;
}
### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.scss END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.ts BEGIN ###
import { ChangeDetectorRef, Component, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import { MatTabsModule } from '@angular/material/tabs';
import {MatIconModule} from '@angular/material/icon';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';

export interface IntersectedTuple {
  dim_number: String;
  dim_values_preview: Array<String>;
  dim_values: Array<String>;

  needs_expand: boolean;
}

@Component({
  selector: 'app-intersection-details-dialog',
  standalone: true,
  imports: [
    CommonModule,
    MatIconModule,
    MatTabsModule,
    MatSortModule,
    MatTableModule,
    MatFormFieldModule, 
    MatSelectModule
  ],
  templateUrl: './intersection-details-dialog.component.html',
  styleUrls: ['./intersection-details-dialog.component.scss'],
  animations: [
    trigger('detailExpand', [
      state('collapsed,void', style({height: '0px', minHeight: '0'})),
      state('expanded', style({height: '*'})),
      transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
    ]),
  ],
})
export class IntersectionDetailsDialogComponent {
  public static WIDTH = '40vw';
  public static HEIGHT = '80vh';
  
  protected identifier: number;
  protected total_untouched_percentage: number;
  protected total_intersection_percentage: number;
  protected intersections: Map<number, [number, Array<Array<string>>]>;

  protected intersector_id: number;

  protected intersector_data_source: IntersectedTuple[];
  protected intersector_displayed_columns = ['dim_number', 'dim_values_preview'];
  protected intersector_displayed_columns_names: Map<String, String> = new Map([
    ['dim_number', 'Dim'],
    ['dim_values_preview', 'Dim preview']
  ]);
  protected intersector_displayed_columns_with_expand = [...this.intersector_displayed_columns, 'expand'];
  protected expanded_element: IntersectedTuple | null;
  private max_dim_values_preview_length = 44;
  
  constructor(public dialogRef: MatDialogRef<IntersectionDetailsDialogComponent>, 
      @Inject(MAT_DIALOG_DATA) public data: {intersector: number, intersection_details: IntersectionDetails}, private cdr: ChangeDetectorRef){

    this.identifier = data.intersection_details.identifier;
    this.total_untouched_percentage = data.intersection_details.total_untouched_percentage;
    this.total_intersection_percentage = data.intersection_details.total_intersection_percentage;
      
    this.intersector_id = data.intersector;
    let sorted_intersections: Map<number, [number, Array<Array<string>>]> = new Map([...data.intersection_details.intersections.entries()]
    .sort((a, b) => {
      return a[1][0] - b[1][0];
    }));
    this.intersections = sorted_intersections;

    // let data_source: Array<Array<number>> = Array.from(this.intersections.keys(), key => [key])
    // this.intersectors_data_source = new MatTableDataSource(data_source);
  }

  ngOnInit(): void { 
    console.log("Initializing intersection details dialog");
  }

  ngAfterViewInit(){
    let first_intersector = this.intersections.keys().next().value;
    this.intersector_id = first_intersector;  // Selects the first intersector
    this.changeIntersector();
    this.cdr.detectChanges();
  }

  protected trackColumn(index: number, column: string): any {
    return column;
  }

  protected getColumnName(column: String): String {
    return this.intersector_displayed_columns_names.get(column);
  }

  protected changeIntersector(){
    let intersected_dims: Array<Array<string>> = this.intersections.get(this.intersector_id)[1];

    let i = 0;
    let intersector_data_source: IntersectedTuple[] = [];
    intersected_dims.forEach(dim => {
      let values: Array<String> = [];
      let all_values_length = 0;

      dim.flat().forEach(value => {
        all_values_length += value.length;
        all_values_length += 1; // For the comma
        all_values_length += 1; // For the space
        values.push(" " + value);
      });

      let needs_expand: boolean;
      let dim_values_preview: Array<String> = [];
      if(all_values_length <= this.max_dim_values_preview_length - 2){ // -2 for the last comma and space
        dim_values_preview = values;
        needs_expand = false;
      }else{
        dim_values_preview.push("{" + values.length + " elements...}");
        needs_expand = true;
      }

      intersector_data_source.push(
        {dim_number: 'DIM' + (i+1), dim_values_preview: dim_values_preview, dim_values: values, needs_expand: needs_expand}
        );
      i++;
    });

    this.intersector_data_source = intersector_data_source;
  }

  protected expandRow(element: IntersectedTuple): void {
    if(element.needs_expand === false){
      this.expanded_element = null;
      return;
    }

    this.expanded_element = this.expanded_element === element ? null : element;
  }
  
}
### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.ts END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.html BEGIN ###
<body>
    <header>
        <h1>Intersection of patterns {{identifier}} and {{intersector_id}} </h1>
        <strong><span>Total intersected percentage of {{identifier}}: </span></strong><span>{{total_intersection_percentage*100 | number:'1.2-2'}}%</span><br>
        <strong><span>Total un-intersected percentage of {{identifier}}: </span></strong><span>{{total_untouched_percentage*100 | number:'1.2-2'}}%</span> 
    </header>

    <section>
        <h2>Intersection subtensor preview:</h2>

        <div id="intersector_table_wrapper">
            
            <!-- Declares a table. The multiTemplateDataRows attribute allows multiple <ng-container> elements per row -->
            <table mat-table [dataSource]="intersector_data_source" multiTemplateDataRows>
                <!-- Creates a column for each item in the array.  -->
                <ng-container *ngFor="let column of intersector_displayed_columns; trackBy: trackColumn" matColumnDef="{{column}}" sticky>
                    <th mat-header-cell *matHeaderCellDef> {{getColumnName(column)}} </th>
                    <td mat-cell *matCellDef="let element"> {{element[column]}} </td>
                </ng-container>

                <ng-container matColumnDef="expand">
                    <th mat-header-cell *matHeaderCellDef aria-label="row actions">&nbsp;</th>
                    <td mat-cell *matCellDef="let element">
                    <button mat-icon-button aria-label="expand row" 
                        (click)="(expanded_element = expanded_element === element ? null : element); 
                        $event.stopPropagation()"
                        [hidden]="!element.needs_expand">
                        
                        <mat-icon *ngIf="expanded_element === element">keyboard_arrow_up</mat-icon>
                        <mat-icon *ngIf="expanded_element !== element">keyboard_arrow_down</mat-icon>

                    </button>
                    </td>
                </ng-container>

                <!-- Expanded Content Column - The detail row is made up of this one column that spans across all columns -->
                <ng-container matColumnDef="expandedDetail">
                    <td mat-cell *matCellDef="let element" [attr.colspan]="intersector_displayed_columns_with_expand.length">
                    <div class="detail"

                    [@detailExpand]="element == expanded_element ? 'expanded' : 'collapsed'">
                        <div class="detail-value">
                            {{element.dim_values}}
                        </div>

                    </div>
                    </td>
                </ng-container>
                
                <!-- Creates a header row for the table. -->
                <tr mat-header-row *matHeaderRowDef="intersector_displayed_columns_with_expand"></tr>
                <!-- Defines the data rows. When a row is clicked, it toggles the expanded state of the row -->
                <tr mat-row *matRowDef="let element; columns: intersector_displayed_columns_with_expand;"
                    class="data-row"
                    [style.cursor]="element.needs_expand ? 'pointer' : 'default'"
                    [class.expanded-row]="expanded_element === element"
                    (click)="expandRow(element)">
                </tr>
                <!-- Defines the expanded detail row. -->
                <tr mat-row *matRowDef="let row; columns: ['expandedDetail']" class="detail-row"></tr>
            </table>

        </div>
    </section>
    
    <footer></footer>
</body>

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.html END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.scss BEGIN ###
html, body{
    margin: 0 0 0 0;
    padding: 0 0 0 0;
    
    width: 100%;
    height: 100%;

    background-color: red;
}
### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.scss END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.ts BEGIN ###
import { Component, ElementRef, Input, Renderer2 } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';

@Component({
  selector: 'app-datapoint-tooltip',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './datapoint-tooltip.component.html',
  styleUrls: ['./datapoint-tooltip.component.scss']
})
export class DataPointTooltipComponent {
  protected visible: boolean = false;
  private datapoint: DataPoint;

  constructor(private elementRef: ElementRef, private renderer: Renderer2) {}

  ngOnInit(): void {
    console.log(this.datapoint)
  }

  public toggleVisibility(){
    this.visible = !this.visible;
  }

  public setDatapoint(datapoint: DataPoint){
    this.datapoint = datapoint;
  }

  public setPosition(top: number, left: number) {
    let top_str = top + 'px';
    let left_str = left + 'px';
    this.renderer.setStyle(this.elementRef.nativeElement, 'top', top_str);
    this.renderer.setStyle(this.elementRef.nativeElement, 'left', left_str);
  }
}

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.ts END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.html BEGIN ###
<body [hidden]="!visible">
    
</body>
### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.html END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DataPointTooltipComponent } from './datapoint-tooltip.component';

describe('DatapointTooltipComponent', () => {
  let component: DataPointTooltipComponent;
  let fixture: ComponentFixture<DataPointTooltipComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ DataPointTooltipComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DataPointTooltipComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.spec.ts END ###

### src/app/components/main_options/rss-view/rss-view.component.html BEGIN ###
<body #rssWindow (window:resize)="onResize($event)">
    <div id="visualization_div" #visualization_div></div>
    <div id="slider-div">
        <mat-slider 
            class="example-margin"
            [disabled]="sliderDisabled"
            (input)="onSliderDrag($event)"
            (change)="onSliderChange($event)"
            [max]="rss_evolution.length"
            [min]="1"
            [step]="1"
            [discrete]="false"
            [showTickMarks]="true"
            >
        <input matSliderThumb [(ngModel)]="pattern_number" #slider>
        </mat-slider>

        <p>Number of patterns: {{pattern_number - 1}} </p>
    </div>
</body> 
### src/app/components/main_options/rss-view/rss-view.component.html END ###

### src/app/components/main_options/rss-view/rss-view.component.scss BEGIN ###
@use '@angular/material' as mat;
@import 'src/theme.scss';

html,body{
    height: 100%;
    width: 100%;
    padding: 0 0 0 0;
    margin : 0 0 0 0;

    overflow: hidden;
    
    background-color: mat.get-color-from-palette($primary-palette, 900);
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    user-select: none;

    #drawer{
        z-index: 0;
        background-color: red;
        width: 10%;
        height: 10%;
        user-select: none;
    }

    #drawer:hover{
        cursor: pointer;
    }

    #visualization_div{
        z-index: 0;
        position: relative;
        object-fit:contain;
        width: 90%;
        height: 25%; // Of available space

        margin-top: 2em;

        border: 4px solid mat.get-color-from-palette($primary-palette, 700);
        user-select: none;
        background-color: mat.get-color-from-palette($primary-palette, '900-contrast');
    }

    #slider-div{
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        width: 87%;
        height: 15%;

        padding-right: 1%;
        padding-left: 1%;
        padding-top: 0%;
        // padding-bottom: 1%;

        mat-slider{
            // color: white;
        }

        .mat-mdc-slider {
            width: 100%;
            // background-color: brown;
            
        }      

        p{
            user-select: none;
            color: mat.get-color-from-palette($primary-palette, '900-contrast');
        }
        // background-color: yellow;
    }

    #placeholder{
        height: 60%;
        background-color: red;
    }
}
### src/app/components/main_options/rss-view/rss-view.component.scss END ###

### src/app/components/main_options/rss-view/rss-view.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RssViewComponent } from './rss-view.component';

describe('RssViewComponent', () => {
  let component: RssViewComponent;
  let fixture: ComponentFixture<RssViewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ RssViewComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RssViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/main_options/rss-view/rss-view.component.spec.ts END ###

### src/app/components/main_options/rss-view/rss-view.component.ts BEGIN ###
import { resolveResource } from '@tauri-apps/api/path'
import * as d3 from 'd3';
import { Component, ElementRef, EventEmitter, Output, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSliderModule} from '@angular/material/slider';
import { SvgService } from 'src/app/services/svg/svg.service';
import { FormsModule } from '@angular/forms';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatCardModule} from '@angular/material/card';
import { DataPoint } from 'src/app/models/datapoint';
import { fs, invoke } from '@tauri-apps/api';
import { ChangeDetectorRef } from '@angular/core';
import { AfterViewInit } from '@angular/core'
import { Color } from 'src/app/models/color';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs';
import { environment } from 'src/environments/environment';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { ApiService } from 'src/app/services/api/api.service';

@Component({
  selector: 'app-rss-view',
  standalone: true,
  imports: [
    CommonModule,
    MatSliderModule,
    FormsModule
  ],
  templateUrl: './rss-view.component.html',
  styleUrls: ['./rss-view.component.scss']
})
export class RssViewComponent implements AfterViewInit{
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;

  @ViewChild('visualization_div') visualization_div: ElementRef<HTMLDivElement>;
  private svg: any;
  private plot: any;

  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;
  
  protected sliderDisabled: boolean = false;
  @Output() onTruncation: EventEmitter<any> = new EventEmitter();
  @Output() initialized: EventEmitter<any> = new EventEmitter();

  public rss_evolution: Array<number> = [];
  private datapoints: Array<DataPoint>;
  private scaled_datapoints: Array<DataPoint>;
  protected pattern_number;

  constructor(private route: ActivatedRoute, private dialog_service: DialogService, private api_service: ApiService){}
  
  async ngAfterViewInit() {
    this.rss_evolution = await this.api_service.getFullRssEvolution();
    let subpatterns_identifiers: number[] = await this.api_service.getAllSubpatternsIdentifiers();
    
    this.pattern_number = this.rss_evolution.length;
    this.datapoints = this.wrapIntoDatapoints(this.rss_evolution, subpatterns_identifiers);
    
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg = this.createSvg();
    this.resizeSvg(width, height, 0);
    this.drawDataPoints();

    this.connectDatapoints();
    this.initialized.emit();
  }

  private wrapIntoDatapoints(rss_evolution: Array<number>, subpatterns_identifiers: number[]): Array<DataPoint>{
    let datapoints: DataPoint[] = [];
    let subpatterns_identifiers_set: Set<number> = new Set(subpatterns_identifiers);

    let gray_shade = 160;
    for (let i = 0; i < rss_evolution.length; i++){
      let identifier = i; // i because the first index is the null model rss
      let x = undefined;
      let y = undefined;

      let r = 0;
      let g = 0;
      let b = 0;
      let a = 1;
      if(subpatterns_identifiers_set.has(identifier)){
        r = gray_shade;
        g = gray_shade;
        b = gray_shade;
      }

      let datapoint = new DataPoint(i, 10, 10, 0, 0, x, y, r, g, b, a);
      datapoints[i] = datapoint;
    }

    return datapoints;
  }

  private scalingFunction(datapoints: Array<DataPoint>): Array<any>{
    let min_rss = Math.min(...this.rss_evolution.map(rss => Math.abs(rss)));
    let max_rss = Math.max(...this.rss_evolution.map(rss => Math.abs(rss)));

    let max_y = max_rss;
    let y_range = max_rss - min_rss;
    let length = datapoints.length;

    let lateral_screen_coverage = 1;
    let vertical_screen_coverage = 0.9;
    let scaled_datapoints: Array<DataPoint> = datapoints;
    for (let i = 0; i < datapoints.length; i++){
      let datapoint: DataPoint = datapoints[i];
      let rss = this.rss_evolution[i];
      
      let x = ((i + 0.5)/length) * 2 - 1; // scale x to be between -1 and 1
      x /= ((1-lateral_screen_coverage) + 1)
      
      let y = (rss - min_rss) / y_range; // Scale y to be between 0 and 1
      y = y * 2 - 1; // Scale y to be between -1 and 1
      y /= ((1-vertical_screen_coverage) + 1)
      
      let radius = 3;
      let scaled_datapoint = new DataPoint(i, radius, 10, 0, 0, x, y, datapoint.r, datapoint.g, datapoint.b, datapoint.a);
      scaled_datapoints[i] = scaled_datapoint;
    }

    return scaled_datapoints;
  }

  private drawDataPoints() {
    if(this.plot == undefined){ return; }
  
    this.scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.plot.selectAll('circle')
        .data(this.scaled_datapoints, d => d.identifier); // Each datapoint has a unique identifier
  
    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', d => d.size)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer'); // Set cursor to pointer
  }

  private connectDatapoints(){
    console.log("Connecting datapoints");
    if(this.scaled_datapoints.length < 2){ return; }

    let line = d3.line<DataPoint>()
      .x(d => this.x_scale(d.x))
      .y(d => this.y_scale(d.y));

    for(let i = 0; i < this.scaled_datapoints.length - 1; i++) {
      let point1 = this.scaled_datapoints[i];
      let point2 = this.scaled_datapoints[i+1];

      this.plot.append('path')
        .attr('d', line([point1, point2]))
        .attr('stroke', 'black')
        .attr('stroke-width', 2)
        .attr('fill', 'none');
    }
  }

  public enableSlider(){ this.sliderDisabled = false; }
  public disableSlider(){ this.sliderDisabled = true; }
  
  protected onSliderChange(event: any) {
    this.onTruncation.emit(this.pattern_number);
  }

  protected onSliderDrag(event: any) {
    let x = this.datapoints[this.pattern_number - 1].x;
    this.drawVerticalLine(x);
  }

  public onResize(event) {
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.resizeSvg(width, height);
    this.drawDataPoints();
    this.connectDatapoints();
  }
  
  public getPatternNumber(): number{  
    return this.pattern_number;
  }

  public setPatternNumber(pattern_number: number){
    this.pattern_number = pattern_number;
  }

  // ========================= SVG FUNCTIONS ========================= //

  private createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
    .append('svg')
      .attr('width', this.svg_width)
      .attr('height',this.svg_height);

    return svg;
  }

  public resizeSvg(width: number, height: number, y_correction=0){
    this.svg
      .attr('width', width)
      .attr('height', height);

    let x_scale;

    x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, (width/1)]);

    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([(height - y_correction)/1, 0]);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.svg_width = width;
    this.svg_height = height;

    this.createPlot();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }

    // Add the X gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", "lightgrey")
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    // Add the Y gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("color", "lightgrey")
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }
  
  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g").attr("id", "plot");
    
    this.drawGridLines();
  }

  private drawVerticalLine(x: number) {
    this.plot.selectAll('#vertical-line').remove();

    this.plot.append('line')
        .attr('id', 'vertical-line')
        .attr('x1', this.x_scale(x))
        .attr('y1', 0)
        .attr('x2', this.x_scale(x))
        .attr('y2', this.svg_height)
        .attr('stroke', 'red')
        .attr('stroke-width', 2);
  }

  // ========================= SVG FUNCTIONS ========================= //
}



### src/app/components/main_options/rss-view/rss-view.component.ts END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.ts BEGIN ###
import { open } from '@tauri-apps/api/dialog';
import {Component, EventEmitter, Inject, Input, NgModule, Output} from '@angular/core';
import {MatDialogRef, MatDialogModule} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-file-selection-dialog',
  templateUrl: './file-selection-dialog.component.html',
  styleUrls: ['./file-selection-dialog.component.scss']
})
export class FileSelectionDialogComponent {
  public static WIDTH = '45vw';
  public static HEIGHT = '50vh';
  @Output() modelChange: EventEmitter<any> = new EventEmitter();

  private last_opened_folder: string;
  
  private tensor_path: string = "";
  protected tensor_name: string = "";

  private patterns_path: string = "";
  protected patterns_name: string = "";

  constructor(public dialogRef: MatDialogRef<FileSelectionDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {last_opened_folder: string, tensor_path: string, patterns_path: string}) {
      this.last_opened_folder = data.last_opened_folder;
      this.tensor_path = data.tensor_path;
      this.patterns_path = data.patterns_path;
      this.setNames();
  }

  private isStateValid(): boolean{
    if(this.tensor_path == undefined || this.tensor_path == null || this.tensor_path == ""){
      return false;
    }

    if(this.patterns_path == undefined || this.patterns_path == null || this.patterns_path == ""){
      return false;
    }

    return true;
  }

  private setNames(){
    this.tensor_name = this.tensor_path.split('\\').pop().split('/').pop();
    this.patterns_name = this.patterns_path.split('\\').pop().split('/').pop();
  }

  public async selectTensor(){
    const options = {
      multiple: false,
      defaultPath: this.last_opened_folder
    };
    const selected = await open(options);
    if (selected === null) { return; } // No tensor selected
  
    this.tensor_path = selected.toString();
    this.setNames();
    if (this.tensor_path == ""){ return; } // No tensor selected

    this.last_opened_folder = this.tensor_path;
  }
  
  public async selectPatterns(){
    const options = {
      multiple: false,
      defaultPath: this.last_opened_folder
    };
    const selected = await open(options);
    if (selected === null) { return; } // No patterns selected
    
    this.patterns_path = selected.toString();
    this.setNames();
    if (this.patterns_path == ""){ return; } // No patterns selected

    this.last_opened_folder = this.patterns_path;
  }

  protected submit() {
    if (this.isStateValid()){
      this.dialogRef.close({last_opened_folder: this.last_opened_folder, tensor_path: this.tensor_path, patterns_path: this.patterns_path});
    }else{
      this.dialogRef.close({last_opened_folder: "", tensor_path: null, patterns_path: null});
    }
   }
}

@NgModule({
  declarations: [FileSelectionDialogComponent],
  imports: [
    MatButtonModule, 
    MatDialogModule,
    MatIconModule],
})
export class FileSelectionDialogComponentModule {}
### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.ts END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.html BEGIN ###
<body>
    <h1 id="title" mat-dialog-title>Select summary</h1>

    <div id="dialog-content" mat-dialog-content>
        Select the source tensor file and the patterns you want to visualize.
    </div>

    <div id="file-selection-wrapper">
        <div class="file-selector" id="tensor-selection">
            <h2>Tensor</h2>
            <div class="clickable">
                <mat-icon (click)="selectTensor()"> attach_file </mat-icon>
                <p>{{this.tensor_name}}</p>
            </div>
        </div>
        <div class="file-selector" id="patterns-selection">
            <h2>Patterns</h2>
            <div class="clickable">
                <mat-icon (click)="selectPatterns()"> attach_file </mat-icon>
                <p>{{this.patterns_name}}</p>
            </div>
        </div>
    </div>

    <div id="dialog-actions" mat-dialog-actions>
        <button mat-button mat-dialog-close cdkFocusInitial>Close</button>
        <button mat-button mat-dialog-close (click)="submit()">Ok</button>
    </div>
</body>
### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.html END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FileSelectionDialogComponent } from './file-selection-dialog.component';

describe('FileSelectionDialogComponent', () => {
  let component: FileSelectionDialogComponent;
  let fixture: ComponentFixture<FileSelectionDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ FileSelectionDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FileSelectionDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.spec.ts END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.scss BEGIN ###
@use '@angular/material' as mat;
@import 'src/theme.scss';
body{
    margin: 0 0 0 0;
    padding: 0 0 0 0;
    
    display: flex;
    flex-direction: column;
    // background-color: red;

    width: 100%;
    height: 100%;

    #title{
        margin: 0 0 0 0;
        flex: 0.5;
        // background-color: blue;
    }

    #dialog-content{
        flex: 0.75;
        padding: 1em 1em 1em 1em;
        // background-color: green;
    }

    #file-selection-wrapper{
        flex: 3;
        // background-color: yellow;
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        align-items: center;

        .file-selector{
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;

            // background-color: orange;
            width: 12em;
            height: 12em;

            h2{
                margin: 0 0 0 0;

                font-size: 1.25em;
                text-align: center;
            }

            .clickable{
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                width: 100%;
                height: 100%;

                padding: 1em 0 0 0;

                // background-color: green;
                
                mat-icon{
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;

                    font-size: 3.5em;
                    width: 50%;
                    height: 50%;

                    padding: 0.15em 0.10em 0.15em 0.10em;
                    
                    color: mat.get-color-from-palette($primary-palette, '50-contrast');
                    background-color: mat.get-color-from-palette($primary-palette, 50);

                    border: 2px solid mat.get-color-from-palette($primary-palette, 100);
                    border-radius: 10px;
                }

                mat-icon:hover{
                    background-color: mat.get-color-from-palette($primary-palette, 100);
                    cursor: pointer;
                }

                p{  
                    padding: 1em 0 0 0;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                    
                    max-width: 100%;

                    // background-color: red;
                }
            }
        }
    }

    #dialog-actions{
        display: flex;
        flex-direction: row;
        justify-content: flex-end;
        align-items: center;

        flex: 0.5;
        // background-color: purple;
    }
}
### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.scss END ###

### src/app/components/pattern-summary/pattern-summary.component.html BEGIN ###
<body>
    <div id="pattern-preview" *ngIf="pattern">
        <div id="info">
            <h1>Pattern {{pattern.identifier}}</h1>
            <div id="details">
                <span>Size: {{pattern.size}}</span> <br>
                <span>Density: {{pattern.density}}</span>
            </div>
        </div>
    
        <section>
            <div id="dim-summaries">
                <div class="dim-summary" *ngFor="let dim_values of this.pattern.dims_values; let i = index" (click)="openDimDialog(i)">
                    <div class="dim-summary-header">
                        <h2>DIM {{i+1}}</h2>
                        <span>({{dim_values.length}} elements)</span> <br>
                    </div>
                    <div class="dim-summary-content">
                        <span class="element"> {{this.formatDimValues(dim_values)}} </span>
                    </div>
                </div>
            </div>
        </section>
        
        <footer></footer>
    </div>
</body>

### src/app/components/pattern-summary/pattern-summary.component.html END ###

### src/app/components/pattern-summary/pattern-summary.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PatternSummaryComponent } from './pattern-summary.component';

describe('PatternSummaryComponent', () => {
  let component: PatternSummaryComponent;
  let fixture: ComponentFixture<PatternSummaryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ PatternSummaryComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PatternSummaryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/pattern-summary/pattern-summary.component.spec.ts END ###

### src/app/components/pattern-summary/pattern-summary.component.scss BEGIN ###
@use '@angular/material' as mat;
@import '../../../theme.scss';

body{
    padding: 0 0 0 0;
    margin: 0 0 0 0;

    width: 100%;
    height: 100%;
    // background-color: red;

    overflow-x: hidden; /* Hide horizontal scrollbar */

    #info{
        padding: 0em 0 1em 2em;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: flex-start;

        // background-color: yellow;

        h1{
            margin: 0 0 0 0;
            // background-color: red;
        }

        #details{
            // background-color: green;
            display: flex;
            flex-direction: row;

            align-items: center;
            justify-content: flex-start;

            span{
                
                padding-right: 1em;
            }
        }
    }

    section{
        height: 100%;

        #dim-summaries{
            .dim-summary{
                padding: 1em 0 1em 2em;
                
                width: 100%;
                max-height: 15em;
                // background-color: red;

                overflow: hidden;
                user-select: none;
                
                .dim-summary-header{
                    display: flex;
                    flex-direction: row;

                    align-items: center;

                    h2,span{
                        margin: 0 0 0 0;
                        padding: 0 0 0 0;
                    }

                    h2{
                        padding-right: 0.5em;
                    }
                }

                .dim-summary-content{
                    display: inline-block;
                    width: 85%;

                    // white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    word-wrap: break-word;

                    .element{
                        word-wrap: break-word;
                        overflow: hidden;
                        text-overflow: ellipsis;
                    }
                }
            }

            .dim-summary:hover{
                cursor: pointer;
                background-color: mat.get-color-from-palette($primary-palette, 800);
            }
        }
    }

    
}
### src/app/components/pattern-summary/pattern-summary.component.scss END ###

### src/app/components/pattern-summary/pattern-summary.component.ts BEGIN ###
import { ChangeDetectorRef, Component, Input, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Pattern } from 'src/app/models/pattern';
import { environment } from 'src/environments/environment';
import { fs, invoke } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import {MatTabsModule} from '@angular/material/tabs';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatPaginator, MatPaginatorModule} from '@angular/material/paginator';
import { MatInputModule } from '@angular/material/input';
import {MatSelectChange, MatSelectModule} from '@angular/material/select';
import { ApiService } from 'src/app/services/api/api.service';
import { PatternDimDialogComponent } from './pattern-dim-dialog/pattern-dim-dialog.component';

const MAX_VALUE_STRING_LENGTH = 140;

@Component({
  selector: 'app-pattern-summary',
  standalone: true,
  imports: [CommonModule, MatTabsModule, MatTableModule, MatFormFieldModule, MatPaginatorModule, 
    MatInputModule, MatSelectModule],
  templateUrl: './pattern-summary.component.html',
  styleUrls: ['./pattern-summary.component.scss']
})
export class PatternSummaryComponent {
  @Input() public pattern: Pattern;
  private locked: boolean = false;
  
  private input: HTMLInputElement;

  constructor(private api_service: ApiService, private dialog_service: DialogService) {}

  async ngOnInit(): Promise<void> {
    console.log("Initializing PatternSummaryComponent");
    // await this.update(1); // TODO: Retirar
    // this.openDimDialog(1); // TODO: Retirar
  }

  protected formatDimValues(dims_values: string[]): string {
    let formated_string = dims_values.join(", ");

    if(formated_string.length > MAX_VALUE_STRING_LENGTH){
      formated_string = formated_string.slice(0, MAX_VALUE_STRING_LENGTH) + " (...)";
    }

    return formated_string;
  }

  protected openDimDialog(dim_index: number): void {
    let dialog_data = {
      dim_values: this.pattern.dims_values[dim_index]
    };

    this.dialog_service.open(PatternDimDialogComponent, 
      PatternDimDialogComponent.WIDTH, 
      PatternDimDialogComponent.HEIGHT, 
      dialog_data);
  }

  public async update(identifier){
    if (this.locked){ return; }

    if(identifier == null){
      this.pattern = undefined;
      return;
    }

    this.pattern = await this.api_service.getPattern(identifier);
  }

  public toggleLock(identifier: number){
    if(identifier == null){ // De-select current pattern
      this.locked = false;
      this.update(null);
      return;
    }

    if((!this.pattern) || (identifier != this.pattern.identifier)){ // Lock on another pattern
      this.locked = false;
      this.update(identifier);
    }

    this.locked = !this.locked;
  }
}

### src/app/components/pattern-summary/pattern-summary.component.ts END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.ts BEGIN ###
import { Component, Inject, Input, OnInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';

@Component({
  selector: 'app-pattern-dim-dialog',
  standalone: true,
  imports: [CommonModule, MatFormFieldModule, MatPaginatorModule, 
    MatInputModule, MatTableModule],
  templateUrl: './pattern-dim-dialog.component.html',
  styleUrls: ['./pattern-dim-dialog.component.scss']
})
export class PatternDimDialogComponent implements OnInit{
  public static WIDTH = '40vw';
  public static HEIGHT = '60vh';

  private dim_values: string[];
  protected displayed_columns: string[] = ['Elements'];
  // protected data_source: MatTableDataSource<Array<any>>;
  protected data_source;

  @ViewChild("input") input: HTMLInputElement;
  
  constructor(public dialogRef: MatDialogRef<PatternDimDialogComponent>, @Inject(MAT_DIALOG_DATA) public data: {dim_values: string[]}) {
    this.dim_values = data.dim_values;
    this.data_source = new MatTableDataSource(data.dim_values);
  }

  ngOnInit(): void {
    
  }

  protected applyFilter(event: Event) {
    this.data_source.data = this.dim_values;
    this.input = (event.target as HTMLInputElement);

    const filterValue = (event.target as HTMLInputElement).value.trim().toLowerCase();

    let filteredData = this.data_source.data.filter(item => {
        let itemStr = JSON.stringify(item).toLowerCase();
        return itemStr.includes(filterValue);
    });
    
    this.data_source.data = filteredData;
  }
}

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.ts END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.scss BEGIN ###
body{
    padding: 0 0 0 0;
    margin: 0 0 0 0;

    width: 100%;
    height: 100%;

    // background-color: red;

    display: flex;
    flex-direction: row;

    overflow: hidden;
    
    #table-wrapper{
        width: 70%;
        height: 90%;
        overflow-y: auto;
        overflow-x: auto;
    }

    #filter{
        overflow: hidden;
        padding-top: 1em;
        
        padding-left: 2em;
        width: 20%;
        height: 20%;
    }
}
### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.scss END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.html BEGIN ###
<body>
    <div id="table-wrapper">
        <table mat-table [dataSource]="data_source">
            <!-- Elements column -->
            <ng-container matColumnDef="Elements">
            <th mat-header-cell *matHeaderCellDef>Elements ({{ data_source.data.length}})</th>
            <td mat-cell *matCellDef="let row"> {{row}} </td>
            </ng-container>
        
            <tr mat-header-row *matHeaderRowDef="displayed_columns; sticky: true"></tr>
            <tr class="intersectors_data_row" mat-row *matRowDef="let row; columns: displayed_columns;"></tr>
        </table>
    </div>

    <mat-form-field id="filter" class="filter">
        <mat-label>Filter</mat-label>
        <input #input matInput (keyup)="applyFilter($event)" placeholder="">
    </mat-form-field>
</body>
### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.html END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PatternDimDialogComponent } from './pattern-dim-dialog.component';

describe('PatternDimDialogComponent', () => {
  let component: PatternDimDialogComponent;
  let fixture: ComponentFixture<PatternDimDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ PatternDimDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PatternDimDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.spec.ts END ###

### src/app/components/error-dialog/error-dialog.component.scss BEGIN ###
body{
    overflow: hidden;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    // background-color: red;

    width: 100%;
    height: 100%;

    header{
        width: 100%;
        display: flex;
        flex-direction: column;
        // background-color: blue;
        
        #title{
            display: flex;
            flex-direction: row;
            align-items: center;
            padding: 1em 0 0 1em;

            // background-color: green;
            
            mat-icon{
                color: red;
            }
    
            h1{
                margin: 0 0 0 0;
                padding: 0 0 0 0.5em;
                // background-color: yellow;
                // height: 20%;
            }
        }
    
        span{
            padding: 0.5em 0 0 1em;
            width: 100%;
            height: 10%;
            // background-color: blue;
        }
    }

    section{
        padding: 1em 0 0 2em;
        width: 100%;
        height: 40%;
        // background-color: green;
        
        color: red;
    }

    #dialog-actions{
        // height: 10%;
        // background-color: yellow;
    }
}
### src/app/components/error-dialog/error-dialog.component.scss END ###

### src/app/components/error-dialog/error-dialog.component.ts BEGIN ###
import { Component, Inject, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';

@Component({
  selector: 'app-error-dialog',
  templateUrl: './error-dialog.component.html',
  styleUrls: ['./error-dialog.component.scss']
})
export class ErrorDialogComponent {
  public static WIDTH = '30vw';
  public static HEIGHT = '30vh';

  protected error_message: string;
  
  constructor(public dialogRef: MatDialogRef<ErrorDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {error_message: string}) {
      this.error_message = data.error_message;
  }

  protected submit(){
    this.dialogRef.close();
  }

}

@NgModule({
  declarations: [ErrorDialogComponent],
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule
  ],
})
export class ErrorDialogComponentModule {}
### src/app/components/error-dialog/error-dialog.component.ts END ###

### src/app/components/error-dialog/error-dialog.component.html BEGIN ###
<body>
    <header>
        <div id="title">
            <mat-icon>error</mat-icon>
            <h1>ERROR</h1>
        </div>
        
        <span>An error occurred while executing the application:</span>
    </header>
    
    <section>{{this.error_message}}</section>

    <div id="dialog-actions" mat-dialog-actions>
        <button mat-button mat-dialog-close cdkFocusInitial (click)="submit()">Close</button>
    </div>
</body>
### src/app/components/error-dialog/error-dialog.component.html END ###

### src/app/components/error-dialog/error-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ErrorDialogComponent } from './error-dialog.component';

describe('ErrorDialogComponent', () => {
  let component: ErrorDialogComponent;
  let fixture: ComponentFixture<ErrorDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ ErrorDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ErrorDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/error-dialog/error-dialog.component.spec.ts END ###

### src/app/models/color.ts BEGIN ###
export class Color{
    r: number;
    g: number;
    b: number;
}
### src/app/models/color.ts END ###

### src/app/models/intersection_details.ts BEGIN ###
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
### src/app/models/intersection_details.ts END ###

### src/app/models/pattern.ts BEGIN ###
export class Pattern{
    identifier:number;
    dims_values: Array<Array<string>>;
    density: number;
    size: number;

    constructor(identifier: number, dims_values: Array<Array<string>>, density: number, size: number){
        this.identifier = identifier;
        this.dims_values = dims_values;
        this.density = density;
        this.size = size;
    }

    public static fromResponse(response: any): Pattern{
        return new Pattern(response.identifier, response.dims_values, response.density, response.size);
    }
}
### src/app/models/pattern.ts END ###

### src/app/models/svg.ts BEGIN ###
// import * as d3 from 'd3';
// import { ElementRef } from '@angular/core';
// import { DataPoint } from './datapoint';
// import { event } from '@tauri-apps/api';

// export class Svg {
//     public d3_svg: any;
//     public plot: any;

//     private width: number;
//     private height: number;

//     private x_scale: any;
//     private y_scale: any;

//     private gridlines: boolean;
//     private number_of_gridlines: number;

//     private pannable: boolean;

//     private initial_scale: number;

//     constructor(vizualization_div: ElementRef<HTMLDivElement>, width: number, height: number, 
//                 number_of_gridlines: number = 40, gridlines: boolean = true, pannable: boolean = true){

//         this.width = width;
//         this.height = height;
//         this.number_of_gridlines = number_of_gridlines;
//         this.gridlines = gridlines;
//         this.pannable = pannable;
//         this.create(vizualization_div);
//     }

//     private create(vizualization_div: ElementRef<HTMLDivElement>){
//       this.d3_svg = d3.select(vizualization_div.nativeElement)
//       .append('svg')
//         .attr('width', this.width)
//         .attr('height',this.height);
//     }

//     public resize(width: number, height: number, y_correction=0){
//       this.d3_svg
//         .attr('width', width)
//         .attr('height', height);

//       let x_scale;

//       if(this.pannable){ // Only the pannable visualization will have square aspect ratio
//         x_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([0, (height - y_correction)/1]);

//       }else if(!this.pannable){
//         x_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([0, (width/1)]);
//       }
  
//       let y_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([(height - y_correction)/1, 0]);

//       this.x_scale = x_scale;
//       this.y_scale = y_scale;
//       this.width = width;
//       this.height = height;
  
//       this.createPlot();
//     }

//     private drawGridLines() {
//       let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
//       let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
  
//       // Add the X gridlines
//       this.plot.append("g")			
//         .attr("class", "grid")
//         .attr("transform", "translate(0," + this.height + ")")
//         .attr("color", "lightgrey")
//         .call(makeXGridlines()
//             .tickSize(-this.height)
//             .tickFormat(() => "")
//         )
  
//       // Add the Y gridlines
//       this.plot.append("g")			
//         .attr("class", "grid")
//         .attr("color", "lightgrey")
//         .call(makeYGridlines()
//             .tickSize(-1 * this.width)
//             // .tickSize(-300)
//             .tickFormat(() => "")
//         )
//     }
    
//     private createPlot(){
//       if(this.plot != undefined){ this.d3_svg.select("#plot").remove(); }
//       this.plot = this.d3_svg.append("g").attr("id", "plot");
      
//       if(this.pannable){ // Only the pannable square visualization will execute this
//         let panning_zoom = d3.zoom()
//           .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
//           // .translateExtent([[0, 0], [this.height, this.height/1.2]])
//           .translateExtent([[0, 0], [this.height, this.height]])
//           .on("start", (event, d) => { this.d3_svg.attr("cursor", "grabbing"); })
//           .on("zoom", (event) => { this.plot.attr("transform", event.transform); })
//           .on("end", (event, d) => {this.d3_svg.attr("cursor", "default")});
    
//         this.d3_svg.call(panning_zoom);

//         // Apply initial zoom level
//         this.initial_scale=  1.4;
//         let x_translation_factor = 0.0;
//         // let y_translation_factor = 0.15;
//         let y_translation_factor = 0.2;
//         let initial_transform = d3.zoomIdentity
//           .translate(-this.width*(x_translation_factor), -this.height*(y_translation_factor))
//           // .translate(-this.width*(x_translation_factor), 0)
//           .scale(this.initial_scale);
//         this.d3_svg.call(panning_zoom.transform, initial_transform);
//       }
      
//       if(this.gridlines){ this.drawGridLines(); }
//       // this.drawDataPoints();
//     }

//     public drawVerticalLine(x: number) {
//       // Remove any existing line
//       this.plot.selectAll('#vertical-line').remove();
  
//       // Draw a new line
//       this.plot.append('line')
//           .attr('id', 'vertical-line')
//           .attr('x1', this.x_scale(x))
//           .attr('y1', 0)
//           .attr('x2', this.x_scale(x))
//           .attr('y2', this.height)
//           .attr('stroke', 'red')
//           .attr('stroke-width', 2);
//   }

//   public getXScale(){
//     return this.x_scale;
//   }

//   public getYScale(){
//     return this.y_scale;
//   }

//   public getInitialScale(){
//     return this.initial_scale;
//   }
// }
### src/app/models/svg.ts END ###

### src/app/models/datapoint.ts BEGIN ###
export class DataPoint{
    identifier: number;
    size: number;
    pattern_size: number;
    density: number;
    stroke_width: number;

    x: number;
    y: number;

    r: number;
    g: number;
    b: number;
    a: number;

    constructor(identifier: number, size: number,  pattern_size: number, density: number, stroke_width: number, x: number, y: number, r: number, g: number, b: number, a:number){
        this.identifier = identifier;
        this.pattern_size = pattern_size;
        this.size = size;
        this.density = density;
        this.stroke_width = stroke_width;
        this.x = x;
        this.y = y;
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }
}
### src/app/models/datapoint.ts END ###

### src/app/services/svg/svg.service.spec.ts BEGIN ###
import { TestBed } from '@angular/core/testing';

import { SvgService } from './svg.service';

describe('SvgService', () => {
  let service: SvgService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SvgService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

### src/app/services/svg/svg.service.spec.ts END ###

### src/app/services/svg/svg.service.ts BEGIN ###
import { ElementRef, Injectable } from '@angular/core';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';

@Injectable({
  providedIn: 'root'
})
export class SvgService {

  constructor() { }

  



  // private drawGridLines() {
  //   let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(40) }
  //   let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(40) }

  //   // Add the X gridlines
  //   this.plot.append("g")			
  //     .attr("class", "grid")
  //     .attr("transform", "translate(0," + this.height + ")")
  //     .attr("color", "grey")
  //     .call(makeXGridlines()
  //         .tickSize(-this.height)
  //         .tickFormat(() => "")
  //     )

  //   // Add the Y gridlines
  //   this.plot.append("g")			
  //     .attr("class", "grid")
  //     .attr("color", "grey")
  //     .call(makeYGridlines()
  //         .tickSize(-1 * this.width)
  //         .tickFormat(() => "")
  //     )
  // }

  // private createPlot(svg: any, width: number, height: number){
  //   svg.select("#plot").remove();
  //   let plot = svg.append("g").attr("id", "plot");
  
  //   let panning_zoom = d3.zoom()
  //     .scaleExtent([1, 10]) // This control how much you can unzoom (x1) and zoom (x10)
  //     .translateExtent([[0, 0], [width, height]])
  //     .on("start", (event, d) => { svg.attr("cursor", "grabbing"); })
  //     .on("zoom", (event) => { plot.attr("transform", event.transform); })
  //     .on("end", (event, d) => {svg.attr("cursor", "default")});
  
  //   svg.call(panning_zoom);
  
  //   // Apply initial zoom level
  //   let initial_scale = 1.2;
  //   let translation_factor = 0.1;
  //   let initial_transform = d3.zoomIdentity
  //     .translate(-width*(translation_factor), -height*(translation_factor))
  //     .scale(initial_scale);
  //   svg.call(panning_zoom.transform, initial_transform);
  
  //   this.drawGridLines();
  //   this.drawDataPoints();
  // }


}

### src/app/services/svg/svg.service.ts END ###

### src/app/services/dialog/dialog.service.ts BEGIN ###
import { Injectable, OnDestroy } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { take } from 'rxjs/operators';
import { ErrorDialogComponent } from 'src/app/components/error-dialog/error-dialog.component';

@Injectable({
  providedIn: 'root'
})
export class DialogService implements OnDestroy{
  private dialog_subscription: Subscription;

  constructor(public dialog: MatDialog) { }

  public open(dialog_component, width: string, height: string, dialog_data, closeFunction=null) {
    let enterAnimationDuration = '300ms';
    let exitAnimationDuration = '300ms';

    const dialogRef = this.dialog.open(dialog_component, {
      width: width,
      height: height,
      enterAnimationDuration,
      exitAnimationDuration,
      
      data: dialog_data
    });

     this.dialog_subscription = dialogRef.afterClosed().pipe(take(1)).subscribe(result => {
      // Executes when the dialog is closed
      if (result) {
        if (closeFunction){
          closeFunction(result);
        }
      }
    });
  }

  ngOnDestroy() {
    if (this.dialog_subscription) {
      this.dialog_subscription.unsubscribe();
    }
  }

  public openErrorDialog(error_message: string) {
    this.open(ErrorDialogComponent, ErrorDialogComponent.WIDTH, ErrorDialogComponent.HEIGHT, 
      {error_message: error_message});
  }
}

### src/app/services/dialog/dialog.service.ts END ###

### src/app/services/dialog/dialog.service.spec.ts BEGIN ###
import { TestBed } from '@angular/core/testing';

import { DialogService } from './dialog.service';

describe('DialogService', () => {
  let service: DialogService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DialogService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

### src/app/services/dialog/dialog.service.spec.ts END ###

### src/app/services/api/api.service.spec.ts BEGIN ###
import { TestBed } from '@angular/core/testing';

import { ApiService } from './api.service';

describe('ApiService', () => {
  let service: ApiService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ApiService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

### src/app/services/api/api.service.spec.ts END ###

### src/app/services/api/api.service.ts BEGIN ###
import { Injectable } from '@angular/core';
import { fs, invoke } from '@tauri-apps/api';
import { Pattern } from 'src/app/models/pattern';
import { environment } from 'src/environments/environment';
import { DialogService } from '../dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { DataPoint } from 'src/app/models/datapoint';
import { IntersectionDetails } from 'src/app/models/intersection_details';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  constructor(private dialog_service: DialogService) { }

  public async initApplication(tensor_path: string, patterns_path: string){
    await invoke("initApplication", {tensorPath: tensor_path, patternsPath: patterns_path}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("ERROR Could not read tensor or patterns.");
      throw error;
    });
  }

  public async getFullRssEvolution(): Promise<Array<number>> {
    console.log("Initializing rss view component");
    console.log("Invoking getFullRssEvolution");

    let rss_evolution;
    rss_evolution = await invoke("getFullRssEvolution").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Could not load rss graph.");
      throw error;
    });

    console.log("Received rss_evolution:");
    console.log(rss_evolution);

    return rss_evolution;
  }

  public async truncateModel(new_size: number): Promise<any>{
    console.log("Truncating datapoints to only: " + new_size);
    let truncated_datapoints;
    await invoke("truncateModel", {newSize: new_size}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while truncating datapoints.");
      throw error;
    });

    truncated_datapoints = await this.getDataPoints();

    return truncated_datapoints;
  }

  public async getIntersectionsPercentages(identifier: number): Promise<Map<number, number>> {
    let raw_data;
    raw_data = await invoke("getIntersectionsPercentages", {identifier: identifier})
      .catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while getting intersections.");
        throw error;
    });

    let intersections = new Map<number, number>();
    for (let key in raw_data) { intersections.set(Number(key), Number(raw_data[key])); }

    return intersections;
  }

  public async getIntersectionDetails(identifier: number): Promise<IntersectionDetails>{
    let data: any;
    data = await invoke("getIntersectionDetails", {identifier: identifier}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching intersection details.");
      throw error;
    });

    let intersections: Map<number, [number, Array<Array<string>>]> = new Map();
    for (let key in data.intersections) { 
      let value = data.intersections[key];
      let percentage = Math.round(value[0]*100)/100;
      let dims_intersections = value[1];
      intersections.set(Number(key), [percentage, dims_intersections]);
    }

    let intersection_details: IntersectionDetails = new IntersectionDetails(
      data.identifier,
      Math.round(data.total_untouched_percentage * 100)/100,
      Math.round(data.total_intersection_percentage * 100)/100,
      intersections
    );

    return intersection_details;
  }

  public async getPattern(identifier: number): Promise<Pattern> {
    let pattern;
    pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching pattern.");
      throw error;
    });
    
    return Pattern.fromResponse(pattern);
  }

  public async getDataPoints(): Promise<Array<DataPoint>> {
    console.log("Invoking getDataPoints");
    let datapoints;
    datapoints = await invoke("getDataPoints").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching data points.");
      throw error;
    });

    console.log("Received datapoints:");
    console.log(datapoints);

    return datapoints;
  }

  public async getAllSubpatternsIdentifiers(): Promise<Array<number>> {
    let subpatterns_identifiers;
    subpatterns_identifiers = await invoke("getAllSubPatternsIdentifiers").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching subpatterns identifiers.");
      throw error;
    });

    return subpatterns_identifiers;
  }

  public async getDatapointsWithSubPatterns(): Promise<Array<DataPoint>> {
    let datapoints;
    datapoints = await invoke("getDatapointsWithSubPatterns").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching datapoints with subpatterns.");
      throw error;
    });

    return datapoints;
  }

  public async descendDag(identifier: number): Promise<Array<DataPoint>> {
    let datapoints;
    datapoints = await invoke("descendDag", {nextIdentifier: identifier}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while descending DAG.");
      throw error;
    });

    return datapoints;
  }

  public async ascendDag(): Promise<Array<DataPoint>> {
    let datapoints;
    datapoints = await invoke("ascendDag").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while ascending DAG.");
      throw error;
    });

    return datapoints;
  }
}

### src/app/services/api/api.service.ts END ###

### DIRECTORY src/app/ FLATTENED CONTENT ###
### DIRECTORY src-tauri/src/ FOLDER STRUCTURE ###
/
    lib.rs
    flamegraph.svg
    main.rs
common/
    progress_bar.rs
    generic_error.rs
    mod.rs
controller/
    application_controller.rs
    mod.rs
    dynamic_paginator_controller.rs
    states/
        mod.rs
        states.rs
commands/
    mod.rs
    application_commands.rs
    paginator_commands.rs
database/
    dag_node.rs
    subtensor.rs
    dag.rs
    mod.rs
    datapoint.rs
    pattern.rs
    intersections_details.rs
    tensor.rs
    raw_pattern.rs
model/
    identifier_representation.rs
    identifier_mapper.rs
    mod.rs
    analysis/
        mod.rs
        ordered_pair.rs
        metrics/
            coordinates.rs
            intersections_predictions.rs
            full_model_rss.rs
            distances.rs
            mod.rs
            rss_evolution.rs
            metric.rs
            empty_model_rss.rs
            intersection/
                untouched_delta_rss.rs
                intersection_metrics.rs
                intersections_percentages.rs
                mod.rs
                prediction_matrix.rs
                intersections_indices.rs
    io/
        reader.rs
        translator.rs
        tensor_reader.rs
        pattern_reader.rs
        mod.rs
services/
    plot_service.rs
    mod.rs
    io_service.rs
    metrics_service.rs
    datapoint_service.rs
    dynamic_paginator_service.rs
    dag/
        dag_service.rs
        dag_arranger_service.rs
        mod.rs
        dag_creator_service.rs
    application/
        mod.rs
        application_state_service.rs
        application_service.rs
temp/
    retweets.png
    synth-100-3d-co16.png
### DIRECTORY src-tauri/src/ FOLDER STRUCTURE ###

### DIRECTORY src-tauri/src/ FLATTENED CONTENT ###
### src-tauri/src/lib.rs BEGIN ###
// https://www.sheshbabu.com/posts/rust-module-system/
#![allow(non_snake_case)]
pub mod common;
pub mod controller;
pub mod services;
pub mod model;
pub mod database;
pub mod commands;

use services::application::application_service::ApplicationService;

pub fn main() {
// let path = "../tests/test_data/real1.txt".to_owned(); 
    // let path = "../tests/test_data/4k-big-patterns.txt".to_owned(); 
    // let path = "../tests/test_data/9k-small-patterns.txt".to_owned();
    // let path = "../tests/test_data/simple-msuper.txt".to_owned();
    // let path = "../tests/test_data/simple-msub-2.txt".to_owned();
    // // let path = "../tests/test_data/synth-2.txt".to_owned();
    // let path = "../tests/test_data/paf-1.txt".to_owned();
    // let path = "../tests/test_data/paf-1.processed".to_owned();
    // let path = "../tests/test_data/real-1.txt".to_owned();
    // let path = "../tests/test_data/dataset-co16.fuzzy_tensor".to_owned();
    
    // let tensor_path = "../tests/test_data/tensors/4k-big-patterns-fuzzytensor.txt".to_owned();
    // let patterns_path = "../tests/test_data/4k-big-patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/dissimilarity_matrix/2.tensor".to_owned();
    // let patterns_path = "tests/test_data/dissimilarity_matrix/2.patterns".to_owned();
    
    // let tensor_path = "../tests/test_data/distance_test/b.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/b_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/c.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/c_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/dataset-co16.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/synth-100-3d-co16.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/retweets-sparser.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test_patterns/158-retweets-sparser.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/primary-school.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/paf-1.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets3d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets3d_patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets2d_patterns.txt".to_owned();
    
    // let tensor_path = "tests/test_data/dissimilarity_matrix/8.tensor".to_owned();
    // let patterns_path = "tests/test_data/dissimilarity_matrix/8.patterns".to_owned();

    // let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    // // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_patterns.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_300_patterns.txt".to_owned();
    
    let tensor_path = "tests/test_data/tensors/B_bio.txt".to_owned();
    let patterns_path = "tests/test_data/other_patterns/B_bio.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets2d_patterns.txt".to_owned();

    let mut application_manager = ApplicationService::default();
    application_manager.init(&tensor_path, &patterns_path).unwrap();

    // let rss_evolution = application_manager.getFullRssEvolution();
    // dbg!(rss_evolution);
}
### src-tauri/src/lib.rs END ###

### src-tauri/src/flamegraph.svg BEGIN ###
<?xml version="1.0" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg version="1.1" width="1200" height="60" onload="init(evt)" viewBox="0 0 1200 60" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:fg="http://github.com/jonhoo/inferno"><!--Flame graph stack visualization. See https://github.com/brendangregg/FlameGraph for latest version, and http://www.brendangregg.com/flamegraphs.html for examples.--><!--NOTES: --><text x="50.0000%" y="24.00">ERROR: No valid input provided to flamegraph</text></svg>
### src-tauri/src/flamegraph.svg END ###

### src-tauri/src/main.rs BEGIN ###
#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization::controller::states::states::*;
use boxcluster_visualization::commands::paginator_commands::*;
use boxcluster_visualization::commands::application_commands::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .manage(ApplicationServiceState(Default::default()))
        .manage(PaginatorServiceState(Default::default()))
        .invoke_handler(tauri::generate_handler![ 
            getSoundingPattern,
            refreshPageSize,
            goToPage,
            goToFirstPage,
            goToLastPage,
            nextPage,
            previousPage,

            initApplication,
            changePatterns,
            ascendDag,
            descendDag,
            truncateModel,
            getFullRssEvolution,
            getTruncatedRssEvolution,
            getDataPoints,
            getAllSubPatternsIdentifiers,
            getDatapointsWithSubPatterns,
            getPattern,
            getIntersectionsPercentages,
            getIntersectionDetails,
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // boxcluster_visualization::main()
}

### src-tauri/src/main.rs END ###

### src-tauri/src/common/progress_bar.rs BEGIN ###
use indicatif::{ProgressBar, ProgressStyle};
use std::env;

pub fn new(total: u64, message: &str) -> ProgressBar {
    let hide_progress = env::var("HIDE_PROGRESS").is_ok();
    let bar = if hide_progress {
        ProgressBar::hidden()
    } else {
        let bar = ProgressBar::new(total);
        bar.set_message(message.to_owned());
        bar.set_style(ProgressStyle::default_bar()
            .template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7} | Elapsed time: {elapsed} | Estimated time:{eta}").unwrap()
            .progress_chars("=>-"));
        bar
    };

    return bar;
}

### src-tauri/src/common/progress_bar.rs END ###

### src-tauri/src/common/generic_error.rs BEGIN ###
use std::sync::{PoisonError, MutexGuard};
use colored::*;

#[derive(Debug, thiserror::Error)]
pub enum GenericError {
    #[error(transparent)]
    Tauri(tauri::Error),

    #[error("Failed to acquire lock due to a poisoned mutex.")]
    MutexPoisonError,

    // Add a new variant for a custom error message
    #[error("ERROR in file {file} at line {line}: {message}")]
    Custom {
        message: String,
        file: String,
        line: u32,
    },
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for GenericError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        GenericError::MutexPoisonError
    }
}

impl serde::Serialize for GenericError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {   
        serializer.serialize_str(self.getErrorMessage().as_str())
    }
}

impl GenericError {
    pub fn new(message: &str, file: &str, line: &u32) -> GenericError {
        GenericError::Custom { message: message.to_string(), file: file.to_string(), line: *line }
    }

    pub fn getErrorMessage(&self) -> String {
        match self {
            GenericError::MutexPoisonError => {
                format!("Failed to acquire lock due to a poisoned mutex.")
            },
            GenericError::Custom { message, file, line } => {
                format!("ERROR in file {} at line {}: {}", file, line, message)
            },
            GenericError::Tauri(err) => {
              format!("Tauri error: {}", err)
            },
        }
    }

    pub fn print(&self) {
        println!("{}", self.getErrorMessage().red());
    }

    pub fn from<T, E: std::fmt::Debug>(result: Result<T, E>, message: &str, file: &str, line: &u32) -> Result<T, GenericError> {
        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                let error = GenericError::new(message, file, line);
                Err(error)
            }
        }
    }
}

### src-tauri/src/common/generic_error.rs END ###

### src-tauri/src/common/mod.rs BEGIN ###
pub mod progress_bar;
pub mod generic_error;
### src-tauri/src/common/mod.rs END ###

### src-tauri/src/controller/application_controller.rs BEGIN ###

### src-tauri/src/controller/application_controller.rs END ###

### src-tauri/src/controller/mod.rs BEGIN ###
pub mod states;
// pub mod application_controller;
// pub mod dynamic_paginator_controller;
### src-tauri/src/controller/mod.rs END ###

### src-tauri/src/controller/dynamic_paginator_controller.rs BEGIN ###
// #![allow(non_snake_case)]
// use crate::pattern::pattern::Pattern;
// use crate::states::states::{DagState, PaginatorState};
// use tauri::State;

// #[tauri::command]
// pub fn getSoundingPattern(dag_state:State<DagState>, paginator_state:State<PaginatorState>) -> Pattern{
//     let dag = dag_state.0.lock().unwrap();
//     let paginator = paginator_state.0.lock().unwrap();

//     return paginator.getSoundingPattern();
// }

// #[tauri::command]
// pub fn refreshPageSize(pageSize: u32, dag_state:State<DagState>, paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let dag = dag_state.0.lock().unwrap();
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.refreshPageSize(pageSize);
// }

// #[tauri::command]
// pub fn goToPage(page_index: u32, paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.goToPage(&page_index);
// }

// #[tauri::command]
// pub fn goToFirstPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();
//     let first_page = paginator.first_page.clone();

//     return paginator.goToPage(&first_page);
// }

// #[tauri::command]
// pub fn goToLastPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();
//     let last_page = paginator.last_page.clone();

//     return paginator.goToPage(&last_page);
// }

// #[tauri::command]
// pub fn nextPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.nextPage();
// }

// #[tauri::command]
// pub fn previousPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.previousPage();
// }
### src-tauri/src/controller/dynamic_paginator_controller.rs END ###

### src-tauri/src/controller/states/mod.rs BEGIN ###
pub mod states;
### src-tauri/src/controller/states/mod.rs END ###

### src-tauri/src/controller/states/states.rs BEGIN ###
use std::sync::Mutex;
use crate::services::{dynamic_paginator_service::DynamicPaginatorService, application::application_service::ApplicationService};

pub struct ApplicationServiceState(pub Mutex<ApplicationService>);
pub struct PaginatorServiceState(pub Mutex<DynamicPaginatorService>);
### src-tauri/src/controller/states/states.rs END ###

### src-tauri/src/commands/mod.rs BEGIN ###
pub mod paginator_commands;
pub mod application_commands;
### src-tauri/src/commands/mod.rs END ###

### src-tauri/src/commands/application_commands.rs BEGIN ###
use std::collections::HashMap;

use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::ApplicationServiceState, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, raw_pattern::RawPattern}};

#[tauri::command]
pub fn initApplication(application_service: State<ApplicationServiceState>, tensor_path: String, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changeTensor...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.init(&tensor_path, &patterns_path);
}

#[tauri::command]
pub fn changePatterns(application_service: State<ApplicationServiceState>, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changePatterns...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.changePatterns(&patterns_path);
}

#[tauri::command]
pub fn truncateModel(application_service: State<ApplicationServiceState>, new_size: u32) 
        -> Result<Vec<(f32, f32)>, GenericError>{
    println!("Calling truncateModel...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.truncateModel(&new_size);
}

#[tauri::command]
pub fn getFullRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getFullRssEvolution...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getFullRssEvolution();
}

#[tauri::command]
pub fn getTruncatedRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getTruncatedRssEvolution...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getTruncatedRssEvolution();
}

#[tauri::command]
pub fn getAllSubPatternsIdentifiers(application_service: State<ApplicationServiceState>) -> Result<Vec<u32>, GenericError> {
    println!("Calling getAllSubPatternsIdentifiers...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getAllSubPatternsIdentifiers();
}

#[tauri::command]
pub fn getDatapointsWithSubPatterns(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling getDatapointsWithSubPatterns...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getDatapointsWithSubPatterns();
}

#[tauri::command]
pub fn descendDag(application_service: State<ApplicationServiceState>, next_identifier: u32) -> Result<Vec<DataPoint>, GenericError>{
    println!("Calling descendDag...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.descendDag(&next_identifier);
}

#[tauri::command]
pub fn ascendDag(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError>{
    println!("Calling ascendDag...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.ascendDag();
}

#[tauri::command]
pub fn getDataPoints(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling getDataPoints...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getDataPoints();
}

#[tauri::command]
pub fn getPattern(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<RawPattern, GenericError> {
    println!("Calling getPattern...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getRawPattern(&identifier);
}

#[tauri::command]
pub fn getIntersectionsPercentages(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<HashMap<u32, f64>, GenericError> {
    println!("Calling getIntersectionsPercentages...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getIntersectionsPercentages(&identifier);
}

#[tauri::command]
pub fn getIntersectionDetails(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<IntersectionsDetails, GenericError> {
    println!("Calling getIntersectionDetails...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getIntersectionDetails(&identifier);
}
### src-tauri/src/commands/application_commands.rs END ###

### src-tauri/src/commands/paginator_commands.rs BEGIN ###
use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::{ApplicationServiceState, PaginatorServiceState}, database::raw_pattern::RawPattern};

#[tauri::command]
pub fn getSoundingPattern(paginator_service: State<PaginatorServiceState>, application_service: State<ApplicationServiceState>) 
        -> Result<RawPattern, GenericError> {

    println!("Calling getSoundingPattern...");

    let paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.getSoundingPattern(identifier_mapper, application_service.getTranslator());
}

#[tauri::command]
pub fn refreshPageSize(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_size: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {
    
    println!("Calling refreshPageSize...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.refreshPageSize(identifier_mapper, application_service.getTranslator(), page_size);
}

#[tauri::command]
pub fn goToPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_index: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &page_index);
}

#[tauri::command]
pub fn goToFirstPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToFirstPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    let first_page = paginator_service.first_page.clone();
    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &first_page);
}

#[tauri::command]
pub fn goToLastPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToLastPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let last_page = paginator_service.last_page.clone();
    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &last_page);
}

#[tauri::command]
pub fn nextPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling nextPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.nextPage(identifier_mapper, application_service.getTranslator());
}

#[tauri::command]
pub fn previousPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling previousPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.previousPage(identifier_mapper, application_service.getTranslator());
}
### src-tauri/src/commands/paginator_commands.rs END ###

### src-tauri/src/database/dag_node.rs BEGIN ###
use std::collections::HashSet;


pub struct DagNode{
    pub identifier: u32,
    pub supers: Vec<u32>,
    pub subs: Vec<u32>,

    // This pattern is overlapped by these ones, here only appear the patterns that overlaps AND 
    // have greater density.
    pub overlappings: HashSet<u32>, 
}

impl DagNode{
    pub fn new(identifier: &u32) -> DagNode{
        return DagNode { 
            identifier: *identifier,
            supers: Vec::new(), 
            subs: Vec::new(), 
            overlappings: HashSet::new() };
    }
}
### src-tauri/src/database/dag_node.rs END ###

### src-tauri/src/database/subtensor.rs BEGIN ###
#![allow(non_snake_case)]


use itertools::Itertools;


use crate::common::generic_error::GenericError;

use super::tensor::Tensor;

pub struct Subtensor{
    pub dims_values: Vec<Vec<usize>>,
    pub density: f64,
    pub size: u32,
    pub indices: Vec<Vec<usize>>,
}

impl Subtensor {
    pub fn new(tensor: &Tensor,  dims_values: &Vec<Vec<usize>>) -> Result<Subtensor, GenericError> {
        let (indices, density, size) = Subtensor::iterateOver(tensor, &dims_values)?;

        return Ok(
            Subtensor {
                dims_values: dims_values.clone(),
                density: density,
                size: size,
                indices: indices,
            }
        );
    }

    fn calculateSize(dims_values: &Vec<Vec<usize>>) -> u32{
        let mut size: u32 = 1;

        for dims_value in dims_values{
            size *= dims_value.len() as u32;
        }
        return size;
    }

    fn iterateOver(tensor: &Tensor, dims_values: &Vec<Vec<usize>>) -> Result<(Vec<Vec<usize>>, f64, u32), GenericError> {
        let mut sum = 0.0;
        let subtensor_size = Subtensor::calculateSize(&dims_values);
        let mut indices: Vec<Vec<usize>> = Vec::with_capacity(subtensor_size.clone() as usize);
        
        for index in dims_values.iter().cloned().multi_cartesian_product(){
            sum += *tensor.dims_values.get(index.as_slice())
                .ok_or(GenericError::new(&format!("Tensor index {:?} not found", index), file!(), &line!()))? 
                as f64;

            indices.push(index);
        }

        let density = sum / subtensor_size as f64;
        return Ok((indices, density, subtensor_size as u32));
    }
}

### src-tauri/src/database/subtensor.rs END ###

### src-tauri/src/database/dag.rs BEGIN ###
#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use crate::common::generic_error::GenericError;

use super::{dag_node::DagNode, pattern::Pattern};

#[derive(Default)]
pub struct Dag {
    nodes: HashMap<u32, DagNode>,
}

impl Dag {
    pub fn new(patterns: &Vec<&Pattern>) -> Self {
        return Dag { 
            nodes: Dag::createNodes(patterns),
        };
    }

    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        return self.nodes.keys().map(|i| *i).collect();
    }


    pub fn getNumberOfNodes(&self) -> u32 {
        return self.nodes.len() as u32;
    }

    pub fn isEdge(&self, node: &u32) -> Result<bool, GenericError> {
        let node_p = self.nodes.get(node)
            .ok_or(GenericError::new(&format!("Node {} not found", node), file!(), &line!()))?;
        
        return Ok(node_p.subs.len() == 0);
    }

    pub fn isFont(&self, node: &u32) -> Result<bool, GenericError> {
        return Ok(
            self.nodes.get(node)
            .ok_or(GenericError::new(&format!("Node {} not found", node), file!(), &line!()))?
            .supers.len() == 0
        );
    }

    pub fn hasSubs(&self, node: &u32) -> Result<bool, GenericError> {
        return Ok(
            self.nodes.get(node)
            .ok_or(GenericError::new(&format!("Node {} not found", node), file!(), &line!()))?
            .subs.len() != 0
        );
    }

    pub fn getOverllapings(&self) -> HashMap<u32, HashSet<u32>>{
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (id, node) in self.nodes.iter(){
            overlappings.insert(*id, node.overlappings.clone());
        }
        
        return overlappings;
    }
    
    pub fn extractNodes(self) -> HashMap<u32, DagNode>{
        return self.nodes;
    }

    fn createNodes(_patterns: &[&Pattern]) -> HashMap<u32, DagNode> {
        todo!()
    }

}

### src-tauri/src/database/dag.rs END ###

### src-tauri/src/database/mod.rs BEGIN ###
pub mod dag_node;
pub mod dag;
pub mod datapoint;
pub mod pattern;
pub mod raw_pattern;
pub mod subtensor;
pub mod tensor;
pub mod intersections_details;
### src-tauri/src/database/mod.rs END ###

### src-tauri/src/database/datapoint.rs BEGIN ###
use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub identifier: u32,
    pub size: f32,
    pub pattern_size: u32,
    pub density: f32,
    pub stroke_width: u32,

    pub x: f32,
    pub y: f32,

    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: f32,
}

impl Serialize for DataPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("DataPoint", 8)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("pattern_size", &self.pattern_size)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("stroke_width", &self.stroke_width)?;
    
 
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
 
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("a", &self.a)?;

       state.end()
    }
 }

impl DataPoint {
    pub fn new(identifier: &u32, size: &f32, pattern_size: &u32, density: &f32, stroke_width: &u32, x: &f32, y: &f32, r: &u32, g: &u32, b: &u32, a: &f32) -> DataPoint { 
        return DataPoint { identifier: *identifier, 
            x: *x, 
            y: *y , 
            size: *size, 
            pattern_size: *pattern_size, 
            density: *density, 
            stroke_width:*stroke_width, 
            r: *r,  
            g: *g,
            b: *b,
            a: *a,
            };
    }
}
### src-tauri/src/database/datapoint.rs END ###

### src-tauri/src/database/pattern.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashSet;
use debug_print::{debug_println, debug_print};
use itertools::Itertools;
use ndarray::{IxDynImpl, Dim};
use serde::{Serialize, ser::SerializeStruct};
use std::hash::{Hash, Hasher};

use crate::common::generic_error::GenericError;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Relation {
    NotRelatable,
    Overlaps,
    SuperPattern,
    SubPattern,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub identifier: u32, // Starts at 1
    pub dims_values: Vec<Vec<usize>>,
    pub unordered_dims_values: Vec<Vec<usize>>,
    pub density: f64,
    pub size: u32,
    pub indices_as_dims: Vec<Dim<IxDynImpl>>,
    pub indices: Vec<Vec<usize>>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        if self.dims_values == other.dims_values {
            return true;
        }
        return false;
    }
}

impl Eq for Pattern {}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Maybe can cause problems? Ideally we should hash dims_values (and not identifier) 
        self.identifier.hash(state);
    }
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("Pattern", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("dims_values", &self.unordered_dims_values)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}

impl Pattern {
    pub fn new<'a>(identifier: u32, dims_values: Vec<Vec<usize>>, density: f64) -> Pattern {
        let size = Pattern::calculateSize(&dims_values);
        let indices = Pattern::getIndices(&dims_values);

        return Pattern {
            identifier: identifier,
            dims_values: Pattern::sortDimsValues(&dims_values),
            unordered_dims_values: dims_values,
            density: density,
            size: size,
            indices_as_dims: Pattern::getIndicesAsDims(&indices),
            indices: indices
        };
    }

    fn calculateSize(dims_values: &Vec<Vec<usize>>) -> u32{
        let mut size: u32 = 1;

        for dims_value in dims_values{
            size *= dims_value.len() as u32;
        }
        return size;
    }

    fn getIndices(dims_values: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        return dims_values.iter()
            .cloned()
            .multi_cartesian_product()
            .collect();
    }

    fn getIndicesAsDims(indices: &Vec<Vec<usize>>) -> Vec<Dim<IxDynImpl>> {
        let mut indices_as_dims: Vec<Dim<IxDynImpl>> = Vec::new();

        for index in indices{
            indices_as_dims.push(Dim(index.clone()));
        }

        return indices_as_dims;
    }

    fn sortDimsValues(dims_values: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        let mut dims_values: Vec<Vec<usize>> = dims_values.clone();

        for dim_values in dims_values.iter_mut(){
            dim_values.sort();
        }
        return dims_values;
    }

    fn intersectionPercentage(vector: &Vec<usize>, base: &Vec<usize>) -> f64 { // Only works for sorted vectors
        let reference_area = vector.len() as f64;
        let mut used_vector = vector;
        let mut used_base = base;

        if vector.len() > base.len(){
            // One dimension of possible sub 'vector' is larger than the corresponding dim on base, so its not contained in base
            used_vector = base;
            used_base = vector;
            // Switches the vectors of place so that vector is always smaller than base
            // panic!("Wrong use of intersection method");
        }

        let mut current_index = 0;
        let mut contained_values_sum = 0;
        let mut stop = false;

        for element in used_vector{
            loop{
                let base_element = used_base.get(current_index);

                let base_element = match base_element {
                    None => {
                        stop = true;
                        break;
                    }

                    Some(base_element) => { base_element },
                };

                if base_element > element { // If the vector is sorted the value will not be found anymore
                    break;
                }

                current_index += 1; // Element is lesser or equal than base element, can change index

                if element == base_element{
                    contained_values_sum += 1;
                    break;
                }
            }

            if stop{
                break;
            }

        }

        return contained_values_sum as f64 / reference_area; // Percetange of intersection on VECTOR
    }

    pub fn selfRelationTo(&self, pattern: &Pattern) -> Result<Relation, GenericError> {
        debug_print!("    Comparing patterns {} to {}: ", &self.identifier, &pattern.identifier);
        if self.identifier == pattern.identifier{
            debug_println!("{:?} (Identical patterns)", Relation::NotRelatable);
            return Ok(Relation::NotRelatable);
        }  
        
        // Relation of the actual pattern
        let self_dims_values = self.dims_values.iter();
        let mut other_dims_values = pattern.dims_values.iter();
        let mut full_intersection = true;

        for self_dims_value in self_dims_values{
            let other_dims_value = other_dims_values.next()
                .ok_or(GenericError::new(
                    &format!("Pattern {} has less dimensions than pattern {}", &self.identifier, &pattern.identifier),
                    file!(), &line!()))?; 

            let intersection_percentage: f64;

            if self.size > pattern.size{ // Self is possible super
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value);
            }
            else if pattern.size > self.size{ // Pattern is possible super
                intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);
            }
            else{ // No one is super but there may be an overlap
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value); // Doesn't matter the order
            }

            // intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);

            if intersection_percentage == 0.0{
                debug_println!("{:?}", Relation::NotRelatable);
                return Ok(Relation::NotRelatable);
            }

            if intersection_percentage < 1.0{
                full_intersection = false;
            }
        }

        if full_intersection == false {
            debug_println!("{:?}", Relation::Overlaps);
            return Ok(Relation::Overlaps);
        }

        // Here all dimensions have 100% intersection

        if self.size > pattern.size{
            debug_println!("{:?}", Relation::SuperPattern);
            return Ok(Relation::SuperPattern);
        }

        if self.size < pattern.size{
            debug_println!("{:?}", Relation::SubPattern);
            return Ok(Relation::SubPattern);
        }

        // Its the same pattern if the execution reaches here, duplicated patterns exist in the input file
        return Err(GenericError::new(
            &format!("Duplicated patterns detected in input file: {} and {}", &self.identifier, &pattern.identifier),
            file!(), &line!()));
    }

    pub fn intersection(&self, pattern: &Pattern) -> Vec<Vec<usize>> {
        let indices: HashSet<Vec<usize>> = self.indices.iter().cloned().collect();
        let intersections = indices
            .intersection(&pattern.indices.iter().cloned().collect())
            .map(|i| i.clone())
            .collect();
    
        return intersections;
    }

    pub fn dimIntersection(&self, other: &Pattern) -> Result<Vec<Vec<usize>>, GenericError> {
        let mut intersections: Vec<Vec<usize>> = Vec::new();

        for (dim, self_dim) in self.dims_values.iter().enumerate(){
            let other_dim = other.dims_values.get(dim)
                .ok_or(GenericError::new(&format!("Pattern {} has less dimensions than pattern {}", self.identifier, other.identifier), file!(), &line!()))?;

            let mut intersection: Vec<usize> = Vec::new();

            for self_value in self_dim.iter(){
                if other_dim.contains(self_value){
                    intersection.push(*self_value);
                }
            }

            if intersection.is_empty(){ return Ok(Vec::new()); } // Intersection has to occur in every dim

            intersections.push(intersection);
        }

        return Ok(intersections);
    }

    pub fn union(&self, pattern: &Pattern) -> Vec<Vec<usize>> {
        let indices: HashSet<Vec<usize>> = self.indices.iter().cloned().collect();
        let unions = indices
            .union(&pattern.indices.iter().cloned().collect())
            .map(|i| i.clone())
            .collect();
        
        return unions;
    }
}
### src-tauri/src/database/pattern.rs END ###

### src-tauri/src/database/intersections_details.rs BEGIN ###
use serde::{ser::SerializeStruct, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct IntersectionsDetails{
    pub identifier: u32,
    pub total_untouched_percentage: f64,
    pub total_intersection_percentage: f64,

    pub intersections: HashMap<u32, (f64, Vec<Vec<String>>)>, // Identifier, (percentage, raw_dims)
}

impl IntersectionsDetails{
    pub fn new(identifier: u32, total_untouched_percentage: f64, total_intersection_percentage: f64, 
                intersections: HashMap<u32, (f64, Vec<Vec<String>>)>) -> IntersectionsDetails{
        
        return IntersectionsDetails{
            identifier: identifier,
            total_untouched_percentage: total_untouched_percentage,
            total_intersection_percentage: total_intersection_percentage,
            intersections: intersections,
        };
    }
}

impl Serialize for IntersectionsDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("IntersectionsDetails", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("total_untouched_percentage", &self.total_untouched_percentage)?;
        state.serialize_field("total_intersection_percentage", &self.total_intersection_percentage)?;
        state.serialize_field("intersections", &self.intersections)?;
        state.end()
    }
}
### src-tauri/src/database/intersections_details.rs END ###

### src-tauri/src/database/tensor.rs BEGIN ###
#![allow(non_snake_case)]
use ndarray::ArrayD;

#[derive(Debug)]
pub enum TensorType {
    FullFuzzy,
    FullBoolean, // Most time expensive
    PartialExplicit, // Most time expensive
    PartialImplicit,
}

impl TensorType{
    pub fn hasDensity(&self) -> bool {
        match self {
            TensorType::FullFuzzy => true,
            TensorType::FullBoolean => true,
            TensorType::PartialExplicit => true,
            TensorType::PartialImplicit => false,
        }
    }
}
pub struct Tensor{
    pub path: String,
    pub dims_values: ArrayD<f64>,
    pub size: Vec<usize>,
    pub dimension: u32,
    pub density: f64,
    pub tensor_type: TensorType 
}

impl Tensor{
    pub fn new(path: &String, dims_values: ArrayD<f64>, size: &Vec<usize>, dimension: &u32, density: &f64, tensor_type: TensorType) -> Self{
        return Tensor{
            path: path.to_owned(),
            density: *density,
            dimension: *dimension,
            size: size.clone(),
            dims_values: dims_values, 
            tensor_type: tensor_type
        };
    }
}
### src-tauri/src/database/tensor.rs END ###

### src-tauri/src/database/raw_pattern.rs BEGIN ###
use serde::{ser::SerializeStruct, Serialize};

#[derive(Clone, Debug)]
pub struct RawPattern {
    pub identifier: u32,
    pub dims_values: Vec<Vec<String>>,
    pub density: f64,
    pub size: u32,
}

impl RawPattern {
    pub fn new<'a>(identifier: &u32, dims_values: &Vec<Vec<String>>, density: &f64, size: &u32) -> RawPattern {

        return RawPattern {
            identifier: *identifier,
            dims_values: dims_values.clone(),
            density: *density,
            size: *size,
        };
    }
}

impl Serialize for RawPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("RawPattern", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("dims_values", &self.dims_values)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}
### src-tauri/src/database/raw_pattern.rs END ###

### src-tauri/src/model/identifier_representation.rs BEGIN ###
use crate::{database::{dag_node::DagNode, datapoint::DataPoint, pattern::Pattern, raw_pattern::RawPattern}, common::generic_error::GenericError};

use super::io::translator::Translator;


pub struct IdentifierRepresentation {
    pattern_representation: Option<Pattern>,
    dag_node_representation: Option<DagNode>,
    data_point_representation: Option<DataPoint>,
}

impl IdentifierRepresentation {
    pub fn new(pattern_representation: Pattern) -> IdentifierRepresentation {
        return IdentifierRepresentation { 
            pattern_representation: Some(pattern_representation), 
            dag_node_representation: None, 
            data_point_representation: None 
        };
    }

    pub fn insertDagNodeRepresentation(&mut self, dag_node_representation: DagNode){
        self.dag_node_representation = Some(dag_node_representation);
    }

    pub fn insertDataPointRepresentation(&mut self, data_point_representation: DataPoint){
        self.data_point_representation = Some(data_point_representation);
    }

    pub fn removeDagNodeRepresentation(&mut self){
        self.dag_node_representation = None;
    }

    pub fn removeDatapointRepresentation(&mut self){
        self.data_point_representation = None;
    }

    pub fn asPattern(&self) -> Result<&Pattern, GenericError> {
        return self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()));
    }

    pub fn asRawPattern(&self, translator: &Translator) -> Result<RawPattern, GenericError> {
        let pattern = self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()))?;

        let raw_dims_values = translator.untranslateLineDims(&pattern.unordered_dims_values)?;
        let raw_dims_values: Vec<Vec<String>> = raw_dims_values.iter()
            .map(|raw_dim_values| raw_dim_values.split(",").map(|s| s.to_string()).collect())
            .collect();

        let raw_pattern = RawPattern::new(&pattern.identifier, &raw_dims_values, &pattern.density, &pattern.size);

        return Ok(raw_pattern);
    }

    pub fn asDagNode(&self) -> Result<&DagNode, GenericError> {
        return self.dag_node_representation.as_ref()
            .ok_or(GenericError::new("Could not get dag node representation", file!(), &line!()));
    }

    pub fn asDataPoint(&self) -> Result<&DataPoint, GenericError> {
        return self.data_point_representation.as_ref()
            .ok_or(GenericError::new("Could not get data point representation", file!(), &line!()));
    }
}
### src-tauri/src/model/identifier_representation.rs END ###

### src-tauri/src/model/identifier_mapper.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::{database::{pattern::Pattern, dag_node::DagNode, datapoint::DataPoint}, common::generic_error::GenericError};

use super::identifier_representation::IdentifierRepresentation;

pub struct IdentifierMapper{
    mapping: HashMap<u32, IdentifierRepresentation>, // WARNING: ID's start at 1
}

impl IdentifierMapper{
    pub fn new(pattern_representations: Vec<Pattern>) -> IdentifierMapper{
        return IdentifierMapper { 
            mapping: IdentifierMapper::createInitialMapping(pattern_representations),
        };
    }

    fn createInitialMapping(pattern_representations: Vec<Pattern>) -> HashMap<u32, IdentifierRepresentation>{
        let mut mapping: HashMap<u32, IdentifierRepresentation> = HashMap::new();

        for pattern_representation in pattern_representations {
            mapping.insert(pattern_representation.identifier, IdentifierRepresentation::new(pattern_representation));
        }

        return mapping;
    }

    fn removeAllDagNodeRepresentations(&mut self){
        for identifier_representation in self.mapping.values_mut() {
            identifier_representation.removeDagNodeRepresentation();
        }
    }

    fn removeAllDatapointRepresentations(&mut self){
        for identifier_representation in self.mapping.values_mut() {
            identifier_representation.removeDatapointRepresentation();
        }
    }

    pub fn insertDagNodeRepresentations(&mut self, dag_nodes_representations: Vec<DagNode>) -> Result<(), GenericError>{
        self.removeAllDagNodeRepresentations();

        let dag_nodes_representations: HashMap<u32, DagNode> = dag_nodes_representations.into_iter()
            .map(|dag_node| (dag_node.identifier, dag_node))
            .collect();

        for (identifier, dag_nodes_representation) in dag_nodes_representations {
            let identifier_representation = self.mapping.get_mut(&identifier)
                .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()))?;

            identifier_representation.insertDagNodeRepresentation(dag_nodes_representation);
        }

        return Ok(());
    }

    pub fn insertDataPointRepresentations(&mut self, data_point_representations: Vec<DataPoint>) -> Result<(), GenericError>{
        self.removeAllDatapointRepresentations();

        let data_point_representations: HashMap<u32, DataPoint> = data_point_representations.into_iter()
            .map(|data_point| (data_point.identifier, data_point))
            .collect();
        
        for (identifier, data_point_representation) in data_point_representations {
            let identifier_representation = self.mapping.get_mut(&identifier)
                .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()))?;

            identifier_representation.insertDataPointRepresentation(data_point_representation);
        }

        return Ok(());
    }

    pub fn getRepresentation(&self, identifier: &u32) -> Result<&IdentifierRepresentation, GenericError>{
        return self.mapping.get(identifier)
            .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()));
    }

    pub fn getRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        return self.mapping.values().collect();
    }

    pub fn getRepresentationsFrom(&self, identifiers: &Vec<u32>) -> Vec<&IdentifierRepresentation>{
        return identifiers.iter()
            .filter_map(|identifier| self.getRepresentation(identifier).ok())
            .collect();
    }

    pub fn getOrderedRepresentationsFrom(&self, identifiers: &Vec<u32>) -> Vec<&IdentifierRepresentation>{
        let mut identifiers = identifiers.clone();
        identifiers.sort();

        let representations = self.getRepresentationsFrom(&identifiers);
        // Representations will be naturally ordered
        return representations;
    }

    pub fn getIdentifier(&self, identifier: &u32) -> Result<&IdentifierRepresentation, GenericError>{
        return self.mapping.get(identifier)
            .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()));
    }

    pub fn getIdentifiers(&self) -> Vec<u32>{
        let mut keys: Vec<u32> = self.mapping.keys().cloned().collect();
        keys.sort();
        return keys;
    }

    pub fn getMapping(&self) -> &HashMap<u32, IdentifierRepresentation>{
        return &self.mapping;
    }

    pub fn getOrderedRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        let keys: Vec<u32> = self.getIdentifiers();

        let values: Vec<&IdentifierRepresentation> = keys.iter()
            .map(|k| self.mapping.get(k)
                .expect("Should have gotten identifier representation"))
            .collect();
        return values;
    }

    pub fn getOrderedPatterns(&self) -> Vec<&Pattern> {
        return self.getOrderedRepresentations().iter()
            .map(|representation| representation.asPattern()
                .expect("Should have gotten pattern representation"))
            .collect();
    }

    pub fn getOrderedPatternsFrom(&self, identifiers: &Vec<u32>) -> Vec<&Pattern> {
        return self.getOrderedRepresentationsFrom(identifiers).iter()
            .map(|representation| representation.asPattern()
                .expect("Should have gotten pattern representation"))
            .collect();
    }

    pub fn getOrderedDataPoints(&self) -> Vec<&DataPoint> {
        return self.getOrderedRepresentations().iter()
            .map(|representation| representation.asDataPoint()
                .expect("Should have gotten data point representation"))
            .collect();
    }

    pub fn getOrderedDataPointsFrom(&self, identifiers: &Vec<u32>) -> Vec<&DataPoint> {
        return self.getOrderedRepresentationsFrom(identifiers).iter()
            .map(|representation| representation.asDataPoint()
                .expect("Should have gotten data point representation"))
            .collect();
    }

    pub fn length(&self) -> u32{
        return self.mapping.len() as u32;
    }
}
### src-tauri/src/model/identifier_mapper.rs END ###

### src-tauri/src/model/mod.rs BEGIN ###
pub mod analysis;
pub mod io;
pub mod identifier_mapper;
pub mod identifier_representation;
### src-tauri/src/model/mod.rs END ###

### src-tauri/src/model/analysis/mod.rs BEGIN ###
pub mod metrics;
pub mod ordered_pair;
### src-tauri/src/model/analysis/mod.rs END ###

### src-tauri/src/model/analysis/ordered_pair.rs BEGIN ###
use std::hash::Hash;
use crate::database::pattern::Pattern;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OrderedPair <'a>{
    pub x: &'a Pattern,
    pub y: &'a Pattern,
}

impl OrderedPair <'_> {
    pub fn new<'a>(x: &'a Pattern, y: &'a Pattern) -> OrderedPair<'a> {
        let mut pair = vec![x, y];
        pair.sort_by_key(|obj| obj.identifier);

        return OrderedPair {
            x: *pair.get(0).expect("Should have gotten first element of ordered pair"),
            y: *pair.get(1).expect("Should have gotten second element of ordered pair"),
        };
    }

    pub fn getOther(&self, current: &Pattern) -> &Pattern {
        if current == self.x { 
            return self.y;
        }
        return self.x;
    }

    pub fn get(&self) -> Vec<&Pattern> {
        return vec![self.x, self.y];
    }
}
### src-tauri/src/model/analysis/ordered_pair.rs END ###

### src-tauri/src/model/analysis/metrics/coordinates.rs BEGIN ###
#![allow(non_snake_case)]

use core::panic;
use std::{collections::HashMap, sync::{Mutex, Arc}};
use nalgebra::DMatrix;
use ndarray::{Array, ArrayD, Dim, IxDynImpl};
use rayon::{iter::IndexedParallelIterator, prelude::{IntoParallelRefIterator, ParallelIterator}};
use crate::common::{generic_error::GenericError, progress_bar};
use super::{metric::Metric, distances::DistancesTrait};
use ndarray::{Array2, Axis};
use rand::prelude::*;
use rand::SeedableRng;
use ndarray_rand::rand_distr::Uniform;

pub struct Coordinates {
    value: HashMap<u32, (f64, f64)>,
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, (f64, f64)>> for Coordinates{
    fn get(&self) -> &HashMap<u32, (f64, f64)> {
        return &self.value;
    }
}

impl Coordinates {
    pub fn new<T: DistancesTrait>(distances: &T) -> Result<Coordinates, GenericError>{
        println!("  Coordinates...");
        return Ok(
            Coordinates { 
                value: Coordinates::calculate(distances)?,
            }
        );
    }

    // fn printMatrix(matrix: &DMatrix<f64>){
    //     println!("Printing matrix:");
    //     for i in 0..matrix.nrows(){
    //         for j in 0..matrix.ncols(){
    //             print!("{:.2} ", matrix[(i, j)]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    fn buildDissimilarityMatrix<T: DistancesTrait>(distances: &T, n: usize) -> Result<DMatrix<f64>, GenericError> {
        let size: Vec<usize> = vec![n, n];
        let distance_matrix: Arc<Mutex<ArrayD<f64>>> = Arc::new(Mutex::new(Array::zeros(Dim(size.clone())).into_dyn()));

        let mut visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        visible_identifiers.sort();
        let visible_identifiers2: Vec<u32> = visible_identifiers.clone();
        
        let distances: Arc<Mutex<HashMap<u32, HashMap<u32, f64>>>> = Arc::new(Mutex::new(distances.get().clone()));
        visible_identifiers.par_iter().enumerate().try_for_each(|(i, &identifier_1)| -> Result<(), GenericError> {
            
            for (j, identifier_2) in visible_identifiers2.iter().enumerate(){
                let distances_lock = distances.lock()
                    .map_err(|_| GenericError::new("Error while getting distances thread lock", file!(), &line!()))?;
                
                let mut distance: f64 = 0.0;
                if identifier_1 != *identifier_2 {
                    distance = distances_lock.get(&identifier_1)
                    .ok_or(GenericError::new(&format!("Identifier {} not found", identifier_1), file!(), &line!()))?
                    .get(identifier_2)
                    .ok_or(GenericError::new(&format!("Identifier {} not found", identifier_2), file!(), &line!()))?.clone();
                }
                
                let index: Dim<IxDynImpl> = Dim(vec![i, j]);
                
                let mut distance_matrix_lock = distance_matrix.lock()
                    .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                let matrix_value = distance_matrix_lock.get_mut(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;
                
                *matrix_value = distance;
            }

            return Ok(());
        })?;

        let distance_matrix = distance_matrix.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?
            .clone();
        
        let mut dissimilarity_matrix = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                let index: Dim<IxDynImpl> = Dim(vec![i, j]);
                let matrix_value = distance_matrix.get(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;

                dissimilarity_matrix[(i, j)] = *matrix_value;
            }
        }

        return Ok(dissimilarity_matrix);
    }

    fn SMACOF(d: &Array2<f64>, p: usize, max_iter: usize, tol: f64, random_state: Option<u64>) -> HashMap<u32, (f64, f64)> {
        println!("      Initializing SMACOF...");
        let n = d.shape()[0];
        let mut rng = match random_state {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
    
        // Initialize points randomly in p-dimensional space
        let mut x: Array2<f64> = Array::zeros((n, p));
        let step = Uniform::new(0.0, 1.0);
        for i in 0..n {
            for j in 0..p {
                x[[i, j]] = step.sample(&mut rng);
            }
        }
    
        // Add a small epsilon to avoid division by zero
        let epsilon = 1e-10;
        let d = d + epsilon;
        let w = 1.0 / (d.mapv(|v| v.powi(2)));
    
        // Compute stress function
        let compute_stress = |x: &Array2<f64>| -> f64 {
            let mut stress = 0.0;
            for i in 0..n {
                for j in (i + 1)..n {
                    let dist_ij = (&x.row(i) - &x.row(j)).mapv(f64::abs).sum();
                    stress += w[[i, j]] * (dist_ij - d[[i, j]]).powi(2);
                }
            }
            stress
        };
    
        // Majorization loop
        let initial_stress = compute_stress(&x);
        println!("\t\tInitial stress: {}", initial_stress);
        let bar = progress_bar::new(max_iter as u64, "\t\tIterations");
        for _ in 0..max_iter {
            let stress_prev = compute_stress(&x);
    
            // Update each point using the majorization step
            for i in 0..n {
                let mut numerator = Array2::zeros((1, p));
                let mut denominator = 0.0;
                for j in 0..n {
                    if i != j {
                        let dist_ij = (&x.row(i) - &x.row(j)).mapv(f64::abs).sum();
                        let dist_ij = if dist_ij == 0.0 { epsilon } else { dist_ij };
                        let term1 = x.row(j).to_owned() + ((d[[i, j]] / dist_ij) * (&x.row(j) - &x.row(i)).to_owned());
                        let contrib = w[[i, j]] * d[[i, j]] * term1;
                        numerator = numerator + contrib.insert_axis(Axis(0));
                        denominator += w[[i, j]] * d[[i, j]];
                    }
                }
                x.row_mut(i).assign(&(numerator.sum_axis(Axis(0)) / denominator));
            }
    
            bar.inc(1);
    
            // Compute new stress
            let stress_new = compute_stress(&x);
    
            // Check for convergence
            if (stress_prev - stress_new).abs() < tol {
                break;
            }
        }
        bar.finish();
        let final_stress = compute_stress(&x);
        println!();
        println!("\t\tFinal stress: {}", final_stress);
    
        // Convert result to hashmap
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        for i in 0..n {
            let n_x: f64 = x[[i, 0]];
            let n_y: f64 = x[[i, 1]];
            xys.insert(i as u32, (n_x, n_y));
        }
    
        return xys;
    }

    fn indexToIdentifier(index: &u32, ordered_visible_identifiers: &Vec<u32>) -> Result<u32, GenericError> {
        return Ok(*ordered_visible_identifiers.get(*index as usize)
            .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?);
    }


    fn scaleCoordinates(xys: &HashMap<u32, (f64, f64)>) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if xys.len() == 0 { return Ok(HashMap::new()); }

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        
        for (_, (x, y)) in xys.iter() {
            if x > &x_max { x_max = *x; }
            if x < &x_min { x_min = *x; }

            if y > &y_max { y_max = *y; }
            if y < &y_min { y_min = *y; }
        }

        let x_abs_max = x_max.abs().max(x_min.abs());
        let y_abs_max = y_max.abs().max(y_min.abs());
        x_min /= x_abs_max;
        x_max /= x_abs_max;
        y_min /= y_abs_max;
        y_max /= y_abs_max;

        let xys: HashMap<u32, (f64, f64)> = xys.iter()
            .map(|(i, coords)| (*i, (coords.0.clone() / x_abs_max, coords.1.clone() / y_abs_max)))
            .collect(); // Scale to [-1, 1]

        let left_space = 1.0 + x_min; // This one or bellow is going to be zero
        let right_space = 1.0 - x_max; // This one or bellow is going to be zero

        let top_space = 1.0 - y_max; // This one or bellow is going to be zero
        let bottom_space = 1.0 + y_min; // This one or bellow is going to be zero

        let x_delta = match right_space > left_space {
            true => (left_space + right_space) / 2.0, // Shift to the right
            false => -(left_space + right_space) / 2.0, // Shift to the left
        };

        let y_delta = match top_space > bottom_space {
            true => (top_space + bottom_space) / 2.0, // Shift to the top
            false => -(top_space + bottom_space) / 2.0, // Shift to the bottom
        };

        let x_max = x_max + x_delta; // x_max and x_min should be equal
        // let x_min = x_min + x_delta; // x_max and x_min should be equal
        let y_max = y_max + y_delta; // y_max and y_min should be equal
        // let y_min = y_min + y_delta; // y_max and y_min should be equal
        let x_scaling_factor = 1.0/x_max; // x_max will be 1 and x_min will be -1
        let y_scaling_factor = 1.0/y_max; // y_max will be 1 and y_min will be -1
    
        let mut scaled_coordinates: HashMap<u32, (f64, f64)> = HashMap::new();
        for (i, (x, y)) in xys.iter(){
            let scaled_x = x_scaling_factor * (*x + x_delta);
            let scaled_y = y_scaling_factor * (*y + y_delta);

            if scaled_x > 1.001 || scaled_y > 1.001 {
                panic!("Scaled coordinates are out of bounds: x: {}, y: {}", scaled_x, scaled_y);
            }

            scaled_coordinates.insert(*i, (scaled_x, scaled_y));
        }

        return Ok(scaled_coordinates);
    }

    fn calculate<T: DistancesTrait>(distances: &T) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if distances.get().len() == 0{ // Only one datapoint, no need to calculate MDS
            let mut xys = HashMap::new();
            xys.insert(1, (0.0, 0.0));
            return Ok(xys);
        }

        println!("  Applying Multi Dimensional Scaling...");
        let n: usize = distances.get().len();
        let dissimilarity_matrix: DMatrix<f64> = Coordinates::buildDissimilarityMatrix(distances, n)?;
        let dissimilarity_matrix: Array2<f64> = Array2::from_shape_vec((n, n), dissimilarity_matrix.data.as_vec().clone())
            .map_err(|_| GenericError::new("Error converting dissimilarity matrix to ndarray", file!(), &line!()))?;
        
        let xys: HashMap<u32, (f64, f64)> = Coordinates::SMACOF(&dissimilarity_matrix, 2, 300, 1e-6, Some(42));

        let mut ordered_visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        ordered_visible_identifiers.sort();
        let scaled_xys: HashMap<u32, (f64, f64)> = Coordinates::scaleCoordinates(&xys)?;

        let mut result: HashMap<u32, (f64, f64)> = HashMap::new();
        for entry in scaled_xys.iter(){
            let i = entry.0;
            let identifier = Coordinates::indexToIdentifier(i, &ordered_visible_identifiers)?;

            result.insert(identifier, *entry.1);
        }

        return Ok(result);
    }
}


### src-tauri/src/model/analysis/metrics/coordinates.rs END ###

### src-tauri/src/model/analysis/metrics/intersections_predictions.rs BEGIN ###
#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Mutex, Arc}};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::{database::pattern::Pattern, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper}, common::generic_error::GenericError};

pub struct IntersectionsPredictions<'a>{
    value: HashMap<Vec<usize>, &'a Pattern>,
}

#[allow(non_camel_case_types)]
impl<'a> Metric<HashMap<Vec<usize>, &'a Pattern>> for IntersectionsPredictions<'a> {
    fn get(&self) -> &HashMap<Vec<usize>, &'a Pattern> {
        return &self.value;
    }
}

impl IntersectionsPredictions<'_>{
    pub fn new<'a>(identifier_mapper: &'a IdentifierMapper) -> Result<IntersectionsPredictions<'a>, GenericError>{
        println!("  Intersections predictions...");
        return Ok(
            IntersectionsPredictions { 
                value: IntersectionsPredictions::calculate(identifier_mapper)?,
            }
        );
    }
    
    fn calculate<'a>(identifier_mapper: &'a IdentifierMapper) -> Result<HashMap<Vec<usize>, &'a Pattern>, GenericError> {

        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();
        for identifier_representation in identifier_mapper.getRepresentations(){
            let node = identifier_representation.asDagNode()?;
            overlappings.insert(node.identifier, node.overlappings.clone());
        }

        let intersections_predictions: Arc<Mutex<HashMap<Vec<usize>, &Pattern>>> = Arc::new(Mutex::new(HashMap::new()));

        overlappings.par_iter().try_for_each(|(overlapped, overlappers)| -> Result<(), GenericError>{
            let overlapped: &Pattern = identifier_mapper.getRepresentation(overlapped)?.asPattern()?;

            for overlapper in overlappers{
                let overlapper: &Pattern = identifier_mapper.getRepresentation(overlapper)?.asPattern()?;
                let intersection_indices: Vec<Vec<usize>> = overlapped.intersection(overlapper);

                for intersection_index in intersection_indices {
                    let mut intersections_predictions_lock = intersections_predictions
                        .lock()
                        .map_err(|_| GenericError::new("Could not lock intersections predictions", file!(), &line!()))?;

                    let possible_previous_prediction = intersections_predictions_lock.get_mut(&intersection_index); // EXPENSIVE
                    match possible_previous_prediction{
                        None => {
                            intersections_predictions_lock.insert(intersection_index.clone(), overlapper);
                        }
                        Some(previous_prediction) => { // Multiple overlapping in one index

                            if overlapper.density > previous_prediction.density{ // Switch to current overlapper
                                *previous_prediction = overlapper;
                            }
                        }
                    };

                    
                }
            }

            return Ok(());
        })?;
            
        return Ok(
            intersections_predictions.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections predictions", file!(), &line!()))?
            .clone()
        );
    }
}
### src-tauri/src/model/analysis/metrics/intersections_predictions.rs END ###

### src-tauri/src/model/analysis/metrics/full_model_rss.rs BEGIN ###
// #![allow(non_snake_case)]
// use std::{collections::{HashMap, HashSet, LinkedList}, time::Instant};
// use indicatif::{ProgressBar, ProgressStyle};
// use ndarray::{Dim, IxDynImpl, indices, Dimension, ShapeBuilder};
// use crate::{tensor::tensor::Tensor, pattern::{pattern_mapper::PatternMapper, pattern::Pattern}, utils::{ordered_pair::OrderedPair}};
// use super::{metric::Metric, intersections_predictions::IntersectionsPredictions};

// pub struct FullModelRss{
//     pub untouched_rss_s: HashMap<u32, f64>,
//     pub intersection_rss_s: HashMap<OrderedPair, f64>, // {Overlapped, {overlapper, intersection_rss}}
//     pub model_edges_rss: f64,

//     value: f64,
// }

// #[allow(non_camel_case_types)]
// impl Metric<f64> for FullModelRss{
//     fn get(&self) -> &f64{
//         return &self.value;
//     }
// }

// impl FullModelRss{
//     pub fn new(pattern_mapper: &PatternMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions) ->  FullModelRss{
//         let all_rss = FullModelRss::calculateAll(pattern_mapper, tensor, intersections_predictions);
        
//         return FullModelRss { 
//             untouched_rss_s: all_rss.0,
//             intersection_rss_s: all_rss.1,
//             model_edges_rss: all_rss.2,
//             value: all_rss.3,
//         }
//     }

//     fn getEdgeIndices(pattern_mapper: &PatternMapper, tensor: &Tensor) ->  Vec<Dim<IxDynImpl>>{
//         let mut non_edge_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();
//         let mut edge_indices: Vec<Dim<IxDynImpl>> = Vec::new();
        
//         for pattern in pattern_mapper.getPatterns(){
//             for index in pattern.indices.iter(){
//                 non_edge_indices.insert(index.clone());
//             }
//         }

//         for index in tensor.dims_values.indexed_iter(){
//             let index = index.0;
//             if non_edge_indices.contains(&index) { continue ;}
//             edge_indices.push(index);
//         }

//         return edge_indices;
//     }

//     fn calculatePatternRss(pattern_mapper: &PatternMapper, tensor: &Tensor, 
//         intersections_predictions: &IntersectionsPredictions) 
//         -> (HashMap<u32, f64>, HashMap<OrderedPair, f64>){

//         let intersections_predictions: &HashMap<Dim<IxDynImpl>, &Pattern> = intersections_predictions.get();                                                                                                          
//         let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
//         let mut intersection_rss_s: HashMap<OrderedPair, f64> = HashMap::new();

//         for pattern in pattern_mapper.getPatterns(){
//             let mut pattern_untouched_rss = 0.0;

//             for index in pattern.indices.iter(){
//                 let actual_value = tensor.dims_values.get(index).unwrap();

//                 let possible_overlapper = intersections_predictions.get(&index);
//                 if possible_overlapper.is_some() { // Index IS touched by another pattern
//                     let overlapper = *possible_overlapper.unwrap();
//                     if overlapper.density == pattern.density{ // Current pattern was determined to be the overlapper previously
//                         continue;
//                     }

                    
//                     let overlapper_contribution = (actual_value - overlapper.density).powi(2);

//                     let pair = OrderedPair::new(&pattern.identifier, &overlapper.identifier);
//                     let intersection_rss = intersection_rss_s.get_mut(&pair);
                    
//                     if intersection_rss.is_some(){
//                         *intersection_rss.unwrap() += overlapper_contribution; // This pair has a previous RSS value, sum new
//                     }else{
//                         intersection_rss_s.insert(pair, overlapper_contribution); // This pair hasnt a previous RSS value
//                     }
                    
//                     continue;
//                 }
                
//                 // Index IS NOT touched by another pattern
//                 pattern_untouched_rss += (actual_value - pattern.density).powi(2);
//             }

//             untouched_rss_s.insert(pattern.identifier, pattern_untouched_rss);
//         }

//         return (untouched_rss_s, intersection_rss_s);
//     }

//     fn calculateModelEdgesRss(tensor: &Tensor, edge_indices: Vec<Dim<IxDynImpl>>) -> f64{
//         let mut model_edges_rss = 0.0;
//         for edge_index in edge_indices{
//             let actual_value = tensor.dims_values.get(edge_index).unwrap();
//             model_edges_rss += (actual_value - tensor.density).powi(2);
//         }
//         return model_edges_rss;
//     }

//     fn calculateFullModelRss(untouched_rss_s: &HashMap<u32, f64>,
//                             intersection_rss_s: &HashMap<OrderedPair, f64>,
//                             model_edges_rss: &f64) -> f64 {

//         let mut full_model_rss = *model_edges_rss;

//         for (_, untouched_rss) in untouched_rss_s {
//             full_model_rss += *untouched_rss;
//         }

//         for (_, intersection_rss) in intersection_rss_s {
//             full_model_rss += *intersection_rss;
//         }

//         return full_model_rss;
//     }

//     fn calculateAll(pattern_mapper: &PatternMapper, tensor: &Tensor, 
//         intersections_predictions: &IntersectionsPredictions) 
//         -> (HashMap<u32, f64>, HashMap<OrderedPair, f64>, f64, f64){
        
//         let (untouched_rss_s, intersection_rss_s) = 
//         FullModelRss::calculatePatternRss(
//             pattern_mapper, 
//             tensor,
//             intersections_predictions);
        
//         println!("  Model edges RSS...");
//         let model_edges_rss: f64 = FullModelRss::calculateModelEdgesRss(
//             tensor, 
//             FullModelRss::getEdgeIndices(pattern_mapper, tensor));

//         let full_model_rss: f64 = FullModelRss::calculateFullModelRss(
//             &untouched_rss_s, 
//             &intersection_rss_s, 
//             &model_edges_rss);

//         return (untouched_rss_s, intersection_rss_s, model_edges_rss, full_model_rss);
//     }
// }
    
    
### src-tauri/src/model/analysis/metrics/full_model_rss.rs END ###

### src-tauri/src/model/analysis/metrics/distances.rs BEGIN ###
#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};
use crate::{common::{generic_error::GenericError, progress_bar}, database::{pattern::Pattern, subtensor::Subtensor, tensor::Tensor}, model::{analysis::ordered_pair::OrderedPair, identifier_mapper::IdentifierMapper}};
use super::{intersections_predictions::IntersectionsPredictions, metric::Metric};

pub trait DistancesTrait {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>>;
}

pub struct DistancesView {
    view: HashMap<u32, HashMap<u32, f64>>,
    // mapping: HashMap<u32, u32>,

}

impl Metric<HashMap<u32, HashMap<u32, f64>>> for DistancesView{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.view;
    }
}

impl DistancesTrait for DistancesView {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.view;
    }
}

#[allow(non_camel_case_types)]
impl DistancesView {
    fn new(view: &HashMap<u32, HashMap<u32, f64>> , mapping: HashMap<u32, u32>) -> DistancesView{
        mapping.len();
        return DistancesView { 
            view: view.clone(),
            // mapping: mapping,
        };
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Distances{
    value: HashMap<u32, HashMap<u32, f64>>, 
}

impl DistancesTrait for Distances {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, HashMap<u32, f64>>> for Distances{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

impl Distances{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, visible_identifiers: &Vec<u32>) 
            -> Result<Distances, GenericError>{
                
        println!("  Distances...");
        return Ok(
            Distances { 
                value: Distances::calculate(identifier_mapper, tensor, intersections_predictions, visible_identifiers)?,
            }
        );
    }

    fn calculatePairRss(tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, pair: &OrderedPair) 
            -> Result<(HashMap<u32, f64>, f64), GenericError> {

        let intersections_predictions = intersections_predictions.get();
        let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
        let mut intersection_rss = 0.0;
        let mut seen_intersection_indices: HashSet<&Vec<usize>> = HashSet::new();

        for pattern in pair.get(){
            let mut untouched_rss = 0.0;

            for index in pattern.indices.iter(){
                let actual_value = *tensor.dims_values.get(index.as_slice())
                    .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;
    
                let possible_overlapper = intersections_predictions.get(index);

                match possible_overlapper {
                None => { }, // No overlapper
                Some(possible_overlapper) => {
                    if *possible_overlapper == pair.getOther(pattern) { // Here there is intersection with the pair
                        if seen_intersection_indices.contains(index){ continue; } // Avoid double counting

                        let overlapper = possible_overlapper;
                        let overlapper_contribution = (actual_value - overlapper.density).powi(2);
                        
                        intersection_rss += overlapper_contribution;
                        seen_intersection_indices.insert(index);
                        continue;
                    }}
                }

                untouched_rss += (actual_value - pattern.density).powi(2);
            }

            untouched_rss_s.insert(pattern.identifier, untouched_rss);
        }

        return Ok((untouched_rss_s, intersection_rss));
    }

    fn getXUYDimsValues(x: &Pattern, y: &Pattern) -> Result<Vec<Vec<usize>>, GenericError> {
        let mut xuy_dims_values: Vec<Vec<usize>> = vec![Vec::new(); x.dims_values.len()];

        for (i, dim_values) in x.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i)
                .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?;

            for value in dim_values{
                // if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.push(*value);
            }
        }

        for (i, dim_values) in y.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i)
                .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?;

            for value in dim_values{
                if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.push(*value);
            }
        }

        return Ok(xuy_dims_values);
    }
    
    fn getXUY(tensor:&Tensor, x: &Pattern, y: &Pattern) -> Result<Subtensor, GenericError>{
        let xuy_dims_values = Distances::getXUYDimsValues(x, y)?;
        let xuy = Subtensor::new(tensor, &xuy_dims_values); // Expensive
        return xuy;
    }

    fn getCoveredXUYRss(tensor:&Tensor, xuy: &Subtensor, x: &Pattern, y: &Pattern) -> Result<f64, GenericError>{
        let mut xuy_rss = 0.0;

        let interested_indices: Vec<Vec<usize>> = x.union(y);
        for index in interested_indices.iter(){
            let actual_value = *tensor.dims_values.get(index.as_slice())
                .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;

            xuy_rss += (actual_value - xuy.density).powi(2);
        }   

        return Ok(xuy_rss);
    }

    fn normalize(x: &Pattern, y: &Pattern, raw_distance: &f64) -> Result<f64, GenericError>{
        let mut dimensions_sum_area = 1.0;
        for i in 0..x.dims_values.len() {
            let ith_x_dimension_size = x.dims_values.get(i)
            .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?
            .len() as f64;

            let ith_y_dimension_size = y.dims_values.get(i)
            .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?
            .len() as f64;

            dimensions_sum_area *= ith_x_dimension_size + ith_y_dimension_size;
        }
        let mut xuy_reference_density = (x.size as f64 * x.density) + (y.size as f64 * y.density);
        xuy_reference_density /= dimensions_sum_area;

        let mut denominator = x.size as f64 * (x.density - xuy_reference_density).powi(2);
        denominator += y.size as f64 * (y.density - xuy_reference_density).powi(2);  

        let normalized_distance = raw_distance / denominator;
        return Ok((10000.0 * normalized_distance).round() / 10000.0);
    }

    fn insertIntoDistancesMatrix(distances: &mut HashMap<u32, HashMap<u32, f64>>, x: &Pattern, y: &Pattern, distance: &f64)
            -> Result<(), GenericError>{

        if !distances.contains_key(&x.identifier){
            distances.insert(x.identifier, HashMap::new());
        }

        let distances_from_x = distances.get_mut(&x.identifier)
            .ok_or(GenericError::new(&format!("Distances from {} not found", &x.identifier), file!(), &line!()))?;

        distances_from_x.insert(y.identifier, *distance);

        return Ok(());
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, intersections_predictions: &IntersectionsPredictions, visible_identifiers: &Vec<u32>) 
            -> Result<HashMap<u32, HashMap<u32, f64>>, GenericError>{
        // 58s, 30s, 46s, 39s, 37s, 19s, 16s, 5s, 3s
        let distances = Arc::new(Mutex::new(HashMap::new()));
        let visible_patterns: Result<Vec<&Pattern>, GenericError> = visible_identifiers.iter()
                .map(|identifier| identifier_mapper.getRepresentation(identifier)?.asPattern())
                .collect();

        let visible_patterns = visible_patterns?;

        let mut total_distances = 0;
        if visible_identifiers.len() > 1 {
            total_distances = (visible_identifiers.len().pow(2) as u32 / 2) - visible_identifiers.len() as u32
        }

        let total_distances = total_distances as u64;
        let bar = progress_bar::new(total_distances, "  Calculated distances");

        visible_patterns.par_iter().enumerate().try_for_each(|(row, x)| 
                -> Result<(), GenericError>{

            if row != 0 {
                for (col, y) in visible_patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        let xuy = Distances::getXUY(tensor, x, y)?;
                        let covered_xuy_rss = Distances::getCoveredXUYRss(tensor, &xuy, x, y)?;
                        
                        let pair = OrderedPair::new(x, y);
                        let (untouched_rss, x_y_intersection_rss) = Distances::
                            calculatePairRss(tensor, intersections_predictions, &pair)?;
                        
                        let untouched_rss_x = *untouched_rss.get(&x.identifier)
                            .ok_or(GenericError::new(&format!("Untouched RSS for pattern {} not found", &x.identifier), file!(), &line!()))?;

                        let untouched_rss_y = *untouched_rss.get(&y.identifier)
                            .ok_or(GenericError::new(&format!("Untouched RSS for pattern {} not found", &y.identifier), file!(), &line!()))?;
                        
                        let raw_distance = covered_xuy_rss - untouched_rss_x - untouched_rss_y - x_y_intersection_rss;
                        
                        let normalized_distance = Distances::normalize(x, y, &raw_distance)?;
                        
                        let mut distances = distances.lock()
                            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                        Distances::insertIntoDistancesMatrix(&mut distances, &x, &y, &normalized_distance)?;
                        Distances::insertIntoDistancesMatrix(&mut distances, &y, &x, &normalized_distance)?;
                        bar.inc(1);
                    }
                }
            }

            return Ok(());
        })?;
    
        bar.finish();
        let distances = distances.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?
            .clone();

        return Ok(distances);
    }

    pub fn getView(&self, identifier_mapper: &IdentifierMapper, identifiers: &Vec<u32>) -> Result<DistancesView, GenericError>{
        let mut patterns: Vec<&Pattern> = Vec::new();
        // Maps the identifier of the pattern INSIDE the view to the REAL identifier
        let mut mapping: HashMap<u32, u32> = HashMap::new();

        for (i, real_identifier) in identifiers.iter().enumerate(){
            let view_identifier = (i + 1) as u32; // Because i starts at zero
            let representation = identifier_mapper.getRepresentation(real_identifier)?;
            let pattern = representation.asPattern()?;
            
            patterns.push(pattern);
            mapping.insert(view_identifier, *real_identifier);
        }

        let mut distances_view: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        for (row, x) in patterns.iter().enumerate(){
            if row != 0 {
                for (col, y) in patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        let distance = self.value.get(&x.identifier)
                            .ok_or(GenericError::new(&format!("Distance from {} not found", &x.identifier), file!(), &line!()))?
                            .get(&y.identifier)
                            .ok_or(GenericError::new(&format!("Distance from {} to {} not found", &x.identifier, &y.identifier), file!(), &line!()))?;

                        Distances::insertIntoDistancesMatrix(&mut distances_view, &x, &y, distance)?;    
                        Distances::insertIntoDistancesMatrix(&mut distances_view, &y, &x, distance)?;    
                    }
                }
            }
        }

        return Ok(DistancesView::new(&distances_view, mapping));
    }
}
### src-tauri/src/model/analysis/metrics/distances.rs END ###

### src-tauri/src/model/analysis/metrics/mod.rs BEGIN ###
pub mod metric;
pub mod rss_evolution;
pub mod empty_model_rss;
// pub mod full_model_rss;
pub mod intersections_predictions;
pub mod distances;
pub mod coordinates;
pub mod intersection;
### src-tauri/src/model/analysis/metrics/mod.rs END ###

### src-tauri/src/model/analysis/metrics/rss_evolution.rs BEGIN ###
use ndarray::{ArrayD, Dim, IxDynImpl};
use std::iter::Iterator;
use crate::common::generic_error::GenericError;
use crate::common::progress_bar;
use crate::database::pattern::Pattern;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::intersection::intersections_indices::IntersectionsIndices;
use super::intersection::prediction_matrix::PredictionMatrix;
use super::intersection::untouched_delta_rss::UntouchedDeltaRss;
use super::metric::Metric;

pub struct RssEvolution{
    value: Vec<(u32, f64)>,
    truncated_value: Vec<(u32, f64)>,
}

#[allow(non_camel_case_types)]
impl Metric<Vec<(u32, f64)>> for RssEvolution{
    fn get(&self) -> &Vec<(u32, f64)> {
        return &self.value;
    }
}

impl RssEvolution{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, empty_model_rss: &EmptyModelRss, 
        patterns: &Vec<&Pattern>, prediction_matrix: &mut PredictionMatrix, untouched_delta_rss: &UntouchedDeltaRss, 
        intersections_indices: &IntersectionsIndices) -> Result<RssEvolution, GenericError>{

        println!("  RssEvolution...");
        
        let rss_evolution = RssEvolution::calculate(identifier_mapper, tensor, 
            empty_model_rss, patterns, prediction_matrix, untouched_delta_rss, intersections_indices)?;
        return Ok(
            RssEvolution{
                value: rss_evolution.clone(),
                truncated_value: rss_evolution,
            }
        );
    }

    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    fn updateRssAtIndex(tensor_matrix: &ArrayD<f64>, total_rss: &f64, index: &Dim<IxDynImpl>, old_prediction: &f64, 
                        new_prediction: &f64, prediction_matrix: &mut PredictionMatrix) -> Result<f64, GenericError>{
        
        prediction_matrix.insert(index.clone(), *new_prediction);
        // drop(prediction_matrix);

        let actual_value = tensor_matrix.get(index)
            .ok_or(GenericError::new(&format!("Index {:?} not found", index), file!(), &line!()))?;

        let new_prediction_rss = RssEvolution::calculateRss(actual_value, new_prediction);
        let old_prediction_rss = RssEvolution::calculateRss(actual_value, old_prediction);

        let total_rss = total_rss - old_prediction_rss + new_prediction_rss;
        
        return Ok(total_rss);
    }

    fn updatePredictionMatrix(prediction_matrix: &mut PredictionMatrix, intersections_indices: &IntersectionsIndices,
                            candidate_pattern: &Pattern) -> Result<(), GenericError>{
        
        let all_intersections_indices = intersections_indices.getValue(&candidate_pattern.identifier);
        let all_intersections_indices = match all_intersections_indices{
            None => { return Ok(()); } // No intersection
            Some(all_intersections_indices) => { all_intersections_indices },
        };
            
        for (_, intersection_indices) in all_intersections_indices {
            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.getMutValue(intersection_index)
                    .ok_or(GenericError::new(&format!("Index {:?} not found", intersection_index), file!(), &line!()))?;

                let max_prediction = f64::max(candidate_pattern.density, *previous_prediction);

                if max_prediction > *previous_prediction { // Then change to the new prediction
                    *previous_prediction = max_prediction;
                }
            }
        }

        return Ok(());
    }

    fn calculateCandidateModelRss(current_model_rss: &f64, candidate_pattern: &Pattern,
            tensor: &Tensor,
            identifier_mapper: &IdentifierMapper,
            untouched_delta_rss: &UntouchedDeltaRss,
            prediction_matrix: &mut PredictionMatrix,
            intersections_indices: &IntersectionsIndices,
            seen_candidates: &Vec<u32>) -> Result<f64, GenericError>{

        // let prediction_matrix = prediction_matrix.get();
        // let untouched_delta_rss = untouched_delta_rss.get();
        // let intersections_indices = intersections_indices.get();

        let mut candidate_model_rss = *current_model_rss + untouched_delta_rss.getValue(&candidate_pattern.identifier)
            .ok_or(GenericError::new(
                &format!("Untouched delta rss for pattern {} not found", candidate_pattern.identifier), file!(), &line!()))?
            .1;
            
        let candidate_intersections = intersections_indices.getValue(&candidate_pattern.identifier);
        let candidate_intersections = match candidate_intersections {
            None => { return Ok(candidate_model_rss); } // No intersection
            Some(candidate_intersections) => { candidate_intersections },
        };
        
        // Here we can also have indices with no intersection
        let candidate_prediction = candidate_pattern.density;

        for (intersector, intersection_indices) in candidate_intersections {
            // First deal with intersection indices
            let ignore_intersector = !seen_candidates.contains(intersector);
            
            let intersector_prediction = identifier_mapper
                .getRepresentation(intersector)?
                .asPattern()?.density;

            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.getValue(intersection_index)
                    .ok_or(GenericError::new(&format!("Index {:?} not found", intersection_index), file!(), &line!()))?;

                let previous_prediction_copy = previous_prediction.clone();

                if ignore_intersector == true { // Intersector is not in the submodel, act as if the candidate is not intersected
                    candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                        &candidate_model_rss, 
                        intersection_index, 
                        &previous_prediction_copy, 
                        &candidate_prediction,
                        prediction_matrix)?;
                    
                    continue;
                }

                let mut max_prediction = f64::max(intersector_prediction, candidate_prediction);
                max_prediction = f64::max(max_prediction, *previous_prediction);

                candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                    &candidate_model_rss, 
                    intersection_index, 
                    &previous_prediction_copy, 
                    &max_prediction,
                    prediction_matrix)?;
            }
        }
        return Ok(candidate_model_rss);
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>,
        prediction_matrix: &mut PredictionMatrix, untouched_delta_rss: &UntouchedDeltaRss, 
        intersections_indices: &IntersectionsIndices) 
        -> Result<Vec<(u32, f64)>, GenericError>{
        
        let pattern_nb = patterns.len();

        let mut current_model_rss = *empty_model_rss.get();
        let mut rss_evolution: Vec<(u32, f64)> = vec![(0, current_model_rss)];
        let mut seen_candidates: Vec<u32> = vec![];
        
        let bar = progress_bar::new(pattern_nb as u64, "    Submodels calculated");
        for (_, pattern) in patterns.iter().enumerate(){

            let candidate_model_rss = RssEvolution::calculateCandidateModelRss(
                &current_model_rss, 
                pattern, 
                tensor, 
                identifier_mapper, 
                &untouched_delta_rss, 
                prediction_matrix,
                &intersections_indices, 
                &seen_candidates)?;

            current_model_rss = candidate_model_rss;
            rss_evolution.push((pattern.identifier, current_model_rss));
            seen_candidates.push(pattern.identifier);
            RssEvolution::updatePredictionMatrix(prediction_matrix, &intersections_indices, pattern)?;
            bar.inc(1);
        }

        bar.finish();
        return Ok(rss_evolution);
    }

    pub fn truncate(&mut self, new_size: &u32){
        let full_rss_evolution: Vec<(u32, f64)> = self.value.clone();
        
        // retain the first k + 1 elements, where k is the new size
        let truncated_rss_evolution: Vec<(u32, f64)> = full_rss_evolution.into_iter()
            .take(*new_size as usize + 1)
            .map(|(pattern_identifier, rss)| (pattern_identifier, rss))
            .collect();

        self.truncated_value = truncated_rss_evolution;
    }

    pub fn getTruncated(&self) -> &Vec<(u32, f64)>{
        return &self.truncated_value;
    }
}
### src-tauri/src/model/analysis/metrics/rss_evolution.rs END ###

### src-tauri/src/model/analysis/metrics/metric.rs BEGIN ###
pub trait Metric<T>{
    fn get(&self) -> &T;
}
### src-tauri/src/model/analysis/metrics/metric.rs END ###

### src-tauri/src/model/analysis/metrics/empty_model_rss.rs BEGIN ###
use crate::database::tensor::Tensor;

use super::metric::Metric;

#[derive(Default)]
pub struct EmptyModelRss{
    value: f64, 
}

#[allow(non_camel_case_types)]
impl Metric<f64> for EmptyModelRss{
    fn get(&self) -> &f64 {
        return &self.value;
    }
}

impl EmptyModelRss{
    pub fn new(tensor: &Tensor) -> EmptyModelRss {
        println!("  Empty model RSS...");
        return EmptyModelRss { value: EmptyModelRss::calculate(tensor) }
    }

    fn calculate(tensor: &Tensor) -> f64{
        let mut rss = 0.0;
        for actual_value in tensor.dims_values.iter(){
            rss += (actual_value - tensor.density).powi(2);
        }
        return rss;
    }
}
    
    
### src-tauri/src/model/analysis/metrics/empty_model_rss.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/untouched_delta_rss.rs BEGIN ###
use std::collections::HashMap;

use crate::model::analysis::metrics::metric::Metric;

pub struct UntouchedDeltaRss{
    value: HashMap<u32, (u32, f64)>,
}

impl Metric<HashMap<u32, (u32, f64)>> for UntouchedDeltaRss{
    fn get(&self) -> &HashMap<u32, (u32, f64)>{
        return &self.value;
    }
}

impl UntouchedDeltaRss{
    pub fn new(value: HashMap<u32, (u32, f64)>) -> UntouchedDeltaRss{
        return UntouchedDeltaRss{
            value: value,
        };
    }

    pub fn getValue(&self, value: &u32) -> Option<&(u32, f64)>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &u32) -> Option<&mut (u32, f64)>{
        return self.value.get_mut(value);
    }
}
### src-tauri/src/model/analysis/metrics/intersection/untouched_delta_rss.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/intersection_metrics.rs BEGIN ###
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use ndarray::{Dim, IxDynImpl};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{common::generic_error::GenericError, database::{pattern::Pattern, tensor::Tensor}, model::identifier_mapper::IdentifierMapper};

use super::{intersections_percentages::IntersectionsPercentages, intersections_indices::IntersectionsIndices, prediction_matrix::PredictionMatrix, untouched_delta_rss::UntouchedDeltaRss};

pub struct IntersectionMetrics {}

impl IntersectionMetrics{
    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    pub fn calculate(tensor: &Tensor, patterns: &Vec<&Pattern>, identifier_mapper: &IdentifierMapper) 
            -> Result<(PredictionMatrix, UntouchedDeltaRss, IntersectionsIndices, IntersectionsPercentages), GenericError>{

        let prediction_matrix: HashMap<Dim<IxDynImpl>, f64> = HashMap::new();
        let untouched_rss_s: HashMap<u32, (u32, f64)>= HashMap::new();
        let intersections_indices: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> = HashMap::new();
        let intersections_percentages: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new(); // This is a symmetric relation

        for pattern in patterns {
            let node = identifier_mapper.getRepresentation(&pattern.identifier)?.asDagNode()?;

            for other_pattern in patterns {
                let other_node = identifier_mapper.getRepresentation(&other_pattern.identifier)?.asDagNode()?;
                
                if node.overlappings.contains(&other_node.identifier) || other_node.overlappings.contains(&node.identifier) {
                    overlappings.entry(pattern.identifier)
                        .or_insert(HashSet::new())
                        .insert(other_pattern.identifier);

                    overlappings.entry(other_pattern.identifier)
                        .or_insert(HashSet::new())
                        .insert(pattern.identifier);
                }
            }
        }

        let prediction_matrix: Arc<Mutex<HashMap<Dim<IxDynImpl>, f64>>> = Arc::new(Mutex::new(prediction_matrix));
        let untouched_rss_s: Arc<Mutex<HashMap<u32, (u32, f64)>>> = Arc::new(Mutex::new(untouched_rss_s));
        let intersections_indices: Arc<Mutex<HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>>> = Arc::new(Mutex::new(intersections_indices));
        let intersections_percentages: Arc<Mutex<HashMap<u32, HashMap<u32, f64>>>> = Arc::new(Mutex::new(intersections_percentages));

        patterns.par_iter().try_for_each(|pattern| -> Result<(), GenericError> {

            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();
            // let MAX_PATTERN_INTERSECTIONS = 6;
            let mut pattern_intersections_percentages: HashMap<u32, f64> = HashMap::new();
            let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

            let self_overlappings = overlappings.get(&pattern.identifier);

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; } // Itself
                
                match self_overlappings {
                    None => continue, // This pattern doesnt overlap any other pattern
                    Some(self_overlappings) => {
                        if !self_overlappings.contains(&other_pattern.identifier) { continue; } // These two do not overlap
                    },
                };

                // Here we know that pattern and other_pattern overlap

                let intersection_indices: Vec<Dim<IxDynImpl>> = pattern.intersection(other_pattern)
                    .into_iter()
                    .map(|index| Dim(index))
                    .collect();

                for index in intersection_indices.iter() {
                    all_intersection_indices.insert(index.clone());
                    prediction_matrix.lock()
                        .as_mut()
                        .map_err(|_| GenericError::new("Could not lock prediction matrix", file!(), &line!()))?
                        .insert(index.clone(), tensor.density);
                }

                if !intersection_indices.is_empty() { // There are intersections between pattern and other_pattern
                    let intersection_percentage = intersection_indices.len() as f64 / other_pattern.size as f64;
                    
                    pattern_intersections.insert(other_pattern.identifier, intersection_indices);
                    pattern_intersections_percentages.insert(other_pattern.identifier, intersection_percentage);
                }else{
                    unreachable!("There should be at least one intersection");
                }
            }

            if !pattern_intersections.is_empty(){ // This pattern has intersections with other patterns
                intersections_indices.lock()
                    .as_mut()
                    .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
                    .insert(pattern.identifier, pattern_intersections);
            }

            // if pattern_intersections_percentages.len() > MAX_PATTERN_INTERSECTIONS{
            //     // This truncates pattern_intersections_percentages up to (MAX_PATTERN_INTERSECTIONS - 1) elements
            //     // and inserts the sum of the excess elements on it after
            //     let mut sorted_pattern_intersections_indices: Vec<u32> = pattern_intersections_percentages.keys().cloned().collect();
            //     sorted_pattern_intersections_indices.sort_by(|a, b| { // Decreasing order, based on the intersection percentage
            //         pattern_intersections_percentages.get(b).partial_cmp(&pattern_intersections_percentages.get(a)).unwrap()});

            //     let excess_indices = sorted_pattern_intersections_indices.split_off(MAX_PATTERN_INTERSECTIONS - 1);
            //     let excess_percentages_sum = excess_indices.iter()
            //         .map(|index| pattern_intersections_percentages.get(index).unwrap())
            //         .sum::<f64>();

            //     // Retain in pattern_intersections_percentages only the entries in which the key is in sorted_pattern_intersections_indices
            //     pattern_intersections_percentages.retain(|key, _| sorted_pattern_intersections_indices.contains(key));
            //     pattern_intersections_percentages.insert(0, excess_percentages_sum);
            // }

            let total_intersection_percentage = all_intersection_indices.len() as f64 / pattern.size as f64;
            let untouched_percentage = 1.0 - total_intersection_percentage;
            if untouched_percentage < 0.0 || untouched_percentage > 1.0 {
                unreachable!("Untouched percentage should be between 0 and 1 but it is: {}", untouched_percentage);
            }
            pattern_intersections_percentages.insert(pattern.identifier, total_intersection_percentage);
            
            intersections_percentages.lock()
                .as_mut()
                .map_err(|_| GenericError::new("Could not lock intersections percentages", file!(), &line!()))?
                .insert(pattern.identifier, pattern_intersections_percentages);

            let prediction = &pattern.density;
            let mut untouched_size: u32 = 0;
            let untouched_rss: f64 = pattern.indices_as_dims.clone().into_iter()
                .filter(|index| !all_intersection_indices.contains(index))
                .map(|index| -> Result<f64, GenericError> {
                    let actual_value = tensor.dims_values.get(&index)
                        .ok_or(GenericError::new(&format!("Index {:?} not found", index), file!(), &line!()))?;

                    let prediction_rss = IntersectionMetrics::calculateRss(actual_value, prediction);
                    let lambda_0_rss = IntersectionMetrics::calculateRss(actual_value, &tensor.density);
                    let delta_rss = prediction_rss - lambda_0_rss;

                    untouched_size += 1;
                    Ok(delta_rss)
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum();

            untouched_rss_s.lock()
                .as_mut()
                .map_err(|_| GenericError::new("Could not lock untouched rss", file!(), &line!()))?
                .insert(pattern.identifier, (untouched_size, untouched_rss));

            return Ok(());
        })?;

        let prediction_matrix = PredictionMatrix::new(prediction_matrix.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock prediction matrix", file!(), &line!()))?
            .clone());

        let untouched_rss_s = UntouchedDeltaRss::new(untouched_rss_s.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock untouched rss", file!(), &line!()))?
            .clone());

        let intersections_indices = IntersectionsIndices::new(intersections_indices.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
            .clone());

        let pattern_intersections_percentages = IntersectionsPercentages::new(intersections_percentages.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections percentages", file!(), &line!()))?
            .clone());

        return Ok((prediction_matrix, untouched_rss_s, intersections_indices, pattern_intersections_percentages));
    }
}
### src-tauri/src/model/analysis/metrics/intersection/intersection_metrics.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/intersections_percentages.rs BEGIN ###
use std::collections::HashMap;

use crate::model::analysis::metrics::metric::Metric;

pub struct IntersectionsPercentages{
    value: HashMap<u32, HashMap<u32, f64>>,
}

impl Metric<HashMap<u32, HashMap<u32, f64>>> for IntersectionsPercentages{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>>{
        return &self.value;
    }
}

impl IntersectionsPercentages {
    pub fn new(value: HashMap<u32, HashMap<u32, f64>>) -> IntersectionsPercentages{
        return IntersectionsPercentages{
            value: value,
        };
    }
}
### src-tauri/src/model/analysis/metrics/intersection/intersections_percentages.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/mod.rs BEGIN ###
pub mod intersection_metrics;
pub mod intersections_indices;
pub mod prediction_matrix;
pub mod untouched_delta_rss;
pub mod intersections_percentages;
### src-tauri/src/model/analysis/metrics/intersection/mod.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/prediction_matrix.rs BEGIN ###
use std::collections::HashMap;

use ndarray::{Dim, IxDynImpl};

use crate::model::analysis::metrics::metric::Metric;

pub struct PredictionMatrix{
    value: HashMap<Dim<IxDynImpl>, f64>,
}

impl Metric<HashMap<Dim<IxDynImpl>, f64>> for PredictionMatrix{
    fn get(&self) -> &HashMap<Dim<IxDynImpl>, f64>{
        return &self.value;
    }
}

impl PredictionMatrix{
    pub fn new(value: HashMap<Dim<IxDynImpl>, f64>) -> PredictionMatrix{
        return PredictionMatrix{
            value: value,
        };
    }

    pub fn insert(&mut self, index: Dim<IxDynImpl>, value: f64){
        self.value.insert(index, value);
    }

    pub fn getValue(&self, value: &Dim<IxDynImpl>) -> Option<&f64>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &Dim<IxDynImpl>) -> Option<&mut f64>{
        return self.value.get_mut(value);
    }
}
### src-tauri/src/model/analysis/metrics/intersection/prediction_matrix.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/intersections_indices.rs BEGIN ###
use std::collections::HashMap;

use ndarray::{Dim, IxDynImpl};

use crate::model::analysis::metrics::metric::Metric;

pub struct IntersectionsIndices{
    value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>,
}

impl Metric<HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>> for IntersectionsIndices{
    fn get(&self) -> &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>
    {
        return &self.value;
    }
}

impl IntersectionsIndices{
    pub fn new(value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>) -> IntersectionsIndices{
        return IntersectionsIndices{
            value: value,
        };
    }

    pub fn getValue(&self, value: &u32) -> Option<&HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &u32) -> Option<&mut HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get_mut(value);
    }
}
### src-tauri/src/model/analysis/metrics/intersection/intersections_indices.rs END ###

### src-tauri/src/model/io/reader.rs BEGIN ###
#![allow(non_snake_case)]
use std::{fs::{self, File}, num::{ParseFloatError, ParseIntError}};
use std::io::{prelude::*, BufReader};

use crate::common::generic_error::GenericError;

pub struct Reader{}

impl Reader{
    pub fn readRawLines(file_path:&String) -> Result<Vec<String>, GenericError>{
        let lines: Vec<String> = fs::read_to_string(file_path)
            .map_err(|_| GenericError::new(format!("Could not open file ({})", file_path).as_str(), file!(), &line!()))?
            .split("\n")
            .map(|i| i.to_owned().replace("\r", ""))
            .filter(|i| !i.is_empty())
            .collect();
            
        match lines.get(0){ // Checks if file is empty
            Some(i) => i,
            None => return Err(GenericError::new(format!("File {} is empty", file_path).as_str(), file!(), &line!())),
        };
        
        return Ok(lines);
    }

    pub fn readRawFirstLine(file_path:&String) -> Result<String, GenericError>{
        let file = File::open(file_path)
            .map_err(|_| GenericError::new(format!("Could not open file ({})", file_path).as_str(), file!(), &line!()))?;

        let reader = BufReader::new(file);
        let mut first_line = "".to_owned();

        for line in reader.lines() {
            first_line = line
                .map_err(|_| GenericError::new(format!("Could not read first line of file ({})", file_path).as_str(), file!(), &line!()))?;
            break;
        }
        
        return Ok(first_line);
    }

    pub fn preProcessLine(line: &String) -> Vec<String> {
        let line_columns: Vec<String> = line.split(" ")
                .map(|s| s.to_owned().replace("\r", ""))
                .collect();

        let mut processed_line_columns: Vec<String> = Vec::new();
        for column in line_columns {
            if column == ":"{ break; }

            processed_line_columns.push(column);
        }

        return processed_line_columns;
    }

    fn tryGetDensity(vector: &Vec<String>) -> Result<Option<f64>, GenericError>{
        let mut density: Option<f64> = None;
        let vector_length = vector.len();
        for (i, dim_values_str) in vector.iter().enumerate() {
    
            if i == vector_length - 1 { // Only tests on the last element
                let dim_values_str = dim_values_str.replace("\r", "");
                let density_test_1: Result<f64, ParseFloatError> = dim_values_str.parse::<f64>(); // Tries to parse to float
                if density_test_1.is_ok(){ // Can be density or a single dimension
                    
                    let density_test_2: Result<u32, ParseIntError> = dim_values_str.parse(); // Tries to parse to int
                    if density_test_2.is_err(){ // Then its the true density
                        density = Some(density_test_1.expect("Density test 1 should be ok"));
                    }
                }
            }
        }
        return Ok(density);
    }

    pub fn fileHasDensity(file_path: &String) -> Result<bool, GenericError> {
        let file = File::open(file_path)
            .map_err(|_| GenericError::new(format!("Could not open file ({})", file_path).as_str(), file!(), &line!()))?;

        let reader = BufReader::new(file);

        for line in reader.lines() { // Line per line
            let current_line = line
                .map_err(|_| GenericError::new(format!("Could not read line of file ({})", file_path).as_str(), file!(), &line!()))?;
            
            let dimensions: Vec<String> = Reader::preProcessLine(&current_line);

            let density = Reader::tryGetDensity(&dimensions)?;
            if density.is_some(){ return Ok(true); }
        }

        return Ok(false);
    }
}
### src-tauri/src/model/io/reader.rs END ###

### src-tauri/src/model/io/translator.rs BEGIN ###

#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::common::generic_error::GenericError;

use super::reader::Reader;

#[derive(Default)]
pub struct Translator{
    translator: Vec<HashMap<String, u32>>,
    reversed_translator: Vec<HashMap<u32, String>>,
}

// Translation source path HAS to be a tensor

impl Translator {    

    pub fn new(translation_source_path: &String) -> Result<Translator, GenericError>{
        println!("Creating translator...");
        let translator = Translator::createTranslator(&translation_source_path)?;
        let reversed_translator = Translator::reverseTranslator(&translator);

        return Ok(
            Translator { 
                translator: translator,
                reversed_translator: reversed_translator,
            }
        );
    }
    
    fn createEmptyTranslator(sample_line:&String) -> Vec<HashMap<String, u32>>{
        let mut empty_translator: Vec<HashMap<String, u32>> = Vec::new();
    
        let sample_line: Vec<String> = sample_line.split(" ").map(|i| i.to_owned()).collect();

        let dimensions_nb = sample_line.len() - 1;
        
        for _ in 0..dimensions_nb{
            empty_translator.push(HashMap::new());
        }
    
        return empty_translator;
    }

    fn createTranslator(translation_source_path: &String) -> Result<Vec<HashMap<String, u32>>, GenericError> {
        let lines = Reader::readRawLines(&translation_source_path)?;
        let sample_line = lines.get(0)
            .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;

        let mut translator: Vec<HashMap<String, u32>> = Translator::createEmptyTranslator(sample_line);
        // let file_has_density: bool = AmbientReader::fileHasDensity(&lines);

        for line in lines{
            let dimensions: Vec<String> = line.split(" ")
                .map(|i| i.to_owned())
                .collect();
            
            for (i, dimension) in dimensions.iter().enumerate(){
                if i == dimensions.len() - 1 { // On the last 'dimension' of this line
                    break;
                }
                
                let dim_translator: &mut HashMap<String, u32> = translator.get_mut(i)
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;
    
                let values: Vec<String> = dimension.split(",")
                    .map(|i| i.to_owned())
                    .collect();
    
                // dbg!(&values);
                
                for value in values{
                    let translated_value = dim_translator.get(&value);
    
                    if translated_value.is_none(){ // Key does not exist
                        dim_translator.insert(value, dim_translator.len() as u32); // Starts from 0
                    }
                }
            }
        }
    
        return Ok(translator);
    }

    fn reverseTranslator(translator: &Vec<HashMap<String, u32>>) -> Vec<HashMap<u32, String>> {
        let mut reversed_translator: Vec<HashMap<u32, String>> = Vec::new();
    
        for dim_translator in translator{
            let mut reversed_dim_translator: HashMap<u32, String> = HashMap::new();
    
            for (key, value) in dim_translator{
                reversed_dim_translator.insert(*value, key.to_owned());
            }
            reversed_translator.push(reversed_dim_translator);
        }
    
        return reversed_translator;
    }

    pub fn translateLineDims(&self, line_dims: &Vec<String>) -> Result<Vec<Vec<usize>>, GenericError>{
        let mut translated_lines: Vec<Vec<usize>> = Vec::new();
        // dbg!(&line_dims);
    
        for (i, dim) in line_dims.iter().enumerate(){
            // dbg!(&i);
            
            // dbg!(self.translator.len());
            
            let dim_translator = self.translator.get(i)
                .ok_or(GenericError::new(format!("Could not get translator for dimension: {:?}", dim).as_str(), file!(), &line!()))?;

            let values: Vec<String> = dim.split(",").map(|i| i.to_owned()).collect();
            let mut translated_dim: Vec<usize> = Vec::new();
    
            for value in values{
                let translated_value = dim_translator.get(&value)
                    .ok_or(GenericError::new(format!("Could not translate value: {}", value).as_str(), file!(), &line!()))?;

                let translated_value = usize::try_from(*translated_value)
                    .map_err(|_| GenericError::new(format!("Could not cast {} to usize", translated_value).as_str(), file!(), &line!()))?;

                translated_dim.push(translated_value);
            }
            
            translated_lines.push(translated_dim);
        }
    
        return Ok(translated_lines);
    }
    
    pub fn untranslateLineDims(&self, dims_values: &Vec<Vec<usize>>) -> Result<Vec<String>, GenericError>{
        let mut original_dims: Vec<String> = Vec::new();
        for (i, dim) in dims_values.iter().enumerate(){
            let mut original_dim: Vec<String> = Vec::new();
    
            for value in dim{
                let value = *value as u32;
                let original_value =  self.reversed_translator.get(i)
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?
                    .get(&value)
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?
                    .to_owned();
    
                original_dim.push(original_value);
            }
    
            original_dims.push(original_dim.join(","));
        }

        return Ok(original_dims);
    }

    pub fn getSize(&self) -> Vec<usize>{
        let mut translator_size: Vec<usize> = Vec::new();

        for dim_translator in self.translator.iter(){
            translator_size.push(dim_translator.len());
        }

        return translator_size;
    }
}
### src-tauri/src/model/io/translator.rs END ###

### src-tauri/src/model/io/tensor_reader.rs BEGIN ###
#![allow(non_snake_case)]
use std::{num::{ParseFloatError, ParseIntError}, collections::HashSet};

use debug_print::debug_println;
use ndarray::{ArrayD, Array, Dim, IxDynImpl};


use crate::{database::tensor::{Tensor, TensorType}, common::generic_error::GenericError};

use super::{translator::Translator, reader::Reader};

pub struct TensorReader<'a> {
    pub file_path: String,
    pub translator: &'a Translator,
}

impl TensorReader<'_>{
    pub fn new<'a>(file_path: &String, translator: &'a Translator) -> TensorReader<'a> {
        return TensorReader {
            file_path: file_path.clone(),
            translator: translator,
        };
    }

    fn calculateDimension(&self) -> Result<u32, GenericError>{
        let first_line: String = Reader::readRawFirstLine(&self.file_path)?;
        let first_line: Vec<&str> = first_line
            .split(" ")
            .collect();

        return Ok(first_line.len() as u32 - 1);
    }

    fn getTensorSize(&self) -> Vec<usize>{
        return self.translator.getSize();
    }

    fn createEmptySizedMatrix(&self, size: &Vec<usize>) -> ArrayD<f64>{
        let matrix: ArrayD<f64> = Array::zeros(Dim(size.clone())).into_dyn();
        return matrix;
    }

    fn defineTensorType(&self, lines_dims: &Vec<Vec<String>>) -> Result<TensorType, GenericError> {
        let mut last_values: HashSet<u32> = HashSet::new();
        for line_dims in lines_dims {
            let last_value = line_dims.last()
                .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;
            
            let float_parse_test: Result<f64, ParseFloatError> = last_value.parse::<f64>(); // Tries to parse to float
            if float_parse_test.is_ok() { // Can be int or float
                
                let int_parse_test: Result<u32, ParseIntError> = last_value.parse::<u32>(); // Tries to parse to int
                if int_parse_test.is_err(){ // Then its number with floating points
                    // 100% of full fuzzy tensors will be identified here, but the others will be identified later (exaustively)
                    return Ok(TensorType::FullFuzzy);
                }
            }

            if float_parse_test.is_err() { // Then its a string (dimension)
                return Ok(TensorType::PartialImplicit); // There can be partial implicits where the last dimension IS NOT a string
            }

            // Here the tensor can be PartialImplicit, PartialExplicit or FullBoolean
            // Last value is for sure an integer

            let last_value = last_value.parse::<u32>()
                .map_err(|_| GenericError::new("Error parsing tensor file", file!(), &line!()))?;
            if last_value != 0 && last_value != 1 {
                // 100% of partial implicits will be identified here, even if they pass the previous test
                return Ok(TensorType::PartialImplicit);
            }

            // Here the tensor can be PartialExplicit or FullBoolean
            // To determine which one it is, we need to iterate through all lines
            last_values.insert(last_value);
        }

        // Here the tensor can be PartialExplicit or FullBoolean
        if last_values.contains(&0) {
            // Then its full boolean
            return Ok(TensorType::FullBoolean);
        }

        return Ok(TensorType::PartialExplicit);
    }

    fn processFile(&self, tensor_size: &Vec<usize>) -> Result<(ArrayD<f64>, TensorType), GenericError>{
        debug_println!("    Processing tensor file ...");
        let lines = Reader::readRawLines(&self.file_path)?;
        let lines_dims: Vec<Vec<String>> = lines.into_iter()
            .map(|line| line.split(" ").map(|i| i.to_owned()).collect())
            .collect();

        // lines_dims[0] = {"a", "d", "g", "density"}

        let mut dims_values_matrix: ArrayD<f64> = self.createEmptySizedMatrix(tensor_size);
        let tensor_type = self.defineTensorType(&lines_dims)?;

        for line_dims in lines_dims{
            let mut line_dims =  line_dims;
            let mut density = 1.0;
            if tensor_type.hasDensity() {
                density = line_dims.pop()
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?
                    .parse::<f64>()
                    .map_err(|_| GenericError::new("Error parsing tensor file", file!(), &line!()))?;
            }
            
            let translated_line = self.translator.translateLineDims(&line_dims)?;
            let dims_values: Result<Vec<usize>, _> = translated_line
                .iter()
                .map(|v| v.get(0)
                    .ok_or_else(||
                        GenericError::new("Error parsing tensor file", file!(), &line!())))
                    .map(|res| res.map(|&v| v as usize))
                .collect();
                        
            let index: Dim<IxDynImpl> = Dim(dims_values?);
            let matrix_value = dims_values_matrix.get_mut(index)
                .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;

            *matrix_value = density;
        }
        debug_println!("    Done");
        return Ok((dims_values_matrix, tensor_type));
    }

    fn calculateDensity(&self, dims_values: &ArrayD<f64>, size: &Vec<usize>) -> f64{
        let mut area = 1.0;

        for dim_size in size{
            area *= *dim_size as f64;
        }

        return dims_values.sum() as f64 / area;
    }

    pub fn read(self) -> Result<Tensor, GenericError>{
        let dimension = self.calculateDimension()?;
        let tensor_size = self.getTensorSize();
        let (dims_values, tensor_type) = self.processFile(&tensor_size)?;
        let density = self.calculateDensity(&dims_values, &tensor_size);
        return Ok(
            Tensor::new(&self.file_path, dims_values, &tensor_size, &dimension, &density, tensor_type)
        );
    }
    
}
### src-tauri/src/model/io/tensor_reader.rs END ###

### src-tauri/src/model/io/pattern_reader.rs BEGIN ###
#![allow(non_snake_case)]

use crate::{database::pattern::Pattern, common::generic_error::GenericError};
use super::{translator::Translator, reader::Reader};

pub struct PatternReader<'a> {
    pub file_path: String,
    pub translator: &'a Translator,
    file_has_densities: bool,
}

impl PatternReader<'_>{
    pub fn new<'a>(file_path: &String, translator: &'a Translator) -> Result<PatternReader<'a>, GenericError> {
        let file_has_densities = Reader::fileHasDensity(&file_path)?;

        let instance = PatternReader {
            file_path: file_path.clone(),
            translator: translator,
            file_has_densities: file_has_densities,
        };

        return Ok(instance);
    }

    pub fn read<'a>(self) -> Result<Vec<Pattern>, GenericError>{
        let mut patterns: Vec<Pattern> = Vec::new();        
        let lines: Vec<String> = Reader::readRawLines(&self.file_path)?;
        
        for (i, line) in lines.iter().enumerate() {
            let mut density: f64 = 1.0;
            let mut line_dims: Vec<String> = Reader::preProcessLine(&line);

            if self.file_has_densities{
                density = line_dims.pop()
                    .ok_or(GenericError::new(format!("Could not get density, line: {:?}, line_dims: {:?}", line, line_dims).as_str(), file!(), &line!()))?
                    .parse::<f64>()
                    .map_err(|_| GenericError::new(format!("Could not parse density to f64, line: {:?}, line_dims: {:?}", line, line_dims).as_str(), file!(), &line!()))?
            }

            patterns.push(Pattern::new(
                i as u32 + 1, 
                self.translator.translateLineDims(&line_dims)?,
                density
            ));
        }

        return Ok(patterns);
    }
}


### src-tauri/src/model/io/pattern_reader.rs END ###

### src-tauri/src/model/io/mod.rs BEGIN ###
pub mod translator;
pub mod reader;
pub mod pattern_reader;
pub mod tensor_reader;
### src-tauri/src/model/io/mod.rs END ###

### src-tauri/src/services/plot_service.rs BEGIN ###
#![allow(non_snake_case)]
// use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, Circle}, style::{WHITE, Color, IntoFont, RGBAColor, RGBColor, TextStyle}};

// use crate::{model::{identifier_representation::IdentifierRepresentation}};

use super::application::application_state_service::ApplicationStateService;

pub struct PlotService{}

impl PlotService{
    pub fn plot(application_state: &ApplicationStateService){
        let _ = application_state;
        // let root = BitMapBackend::new("scatter.png", (1600, 900)).into_drawing_area();
        // root.fill(&WHITE).unwrap();

        // let identifier_mapper = application_state.identifierMapper().unwrap();

        // let visible_identifiers = application_state.getVisibleIdentifiers();
        // let visible_representations: Vec<&IdentifierRepresentation> = visible_identifiers.iter()
        //     .map(|identifier| identifier_mapper.getRepresentation(identifier).unwrap())
        //     .collect();
    
        // let mut x_range = 0.0;
        // let mut y_range = 0.0;
        // for identifier_representation in visible_representations.iter(){
        //     let datapoint = identifier_representation.asDataPoint().unwrap();
            
        //     let positive_x_range = x_range.clone();
        //     let negative_x_range = x_range.clone() * -1.0;
    
        //     let positive_y_range = y_range.clone();
        //     let negative_y_range = y_range.clone() * -1.0;
    
        //     if datapoint.x > positive_x_range{ x_range = datapoint.x.clone().abs(); }
        //     else if datapoint.x < negative_x_range{ x_range = datapoint.x.clone().abs(); }
    
        //     if datapoint.y > positive_y_range{ y_range = datapoint.y.clone().abs(); }
        //     else if datapoint.y < negative_y_range{ y_range = datapoint.y.clone().abs(); }
        // }
    
        // x_range *= 1.1;
        // y_range *= 1.1;
    
        // let mut chart = ChartBuilder::on(&root)
        //     .caption("Scatter Plot", ("sans-serif", 50).into_font())
        //     .margin(5)
        //     .x_label_area_size(30)
        //     .y_label_area_size(30)
        //     .build_cartesian_2d(-1.0 * x_range..x_range, -1.0 * y_range..y_range).unwrap();
    
        // chart.configure_mesh().draw().unwrap();
    
        // // Enforcing that overlapping points are drawn in the correct order
        // let mut representations = visible_representations;
        // representations.sort_by(|a, b| 
        //     b.asDataPoint().unwrap().size.partial_cmp(&a.asDataPoint().unwrap().size).unwrap()); 
            
        // for identifier_representation in representations{
        //     let datapoint = identifier_representation.asDataPoint().unwrap();
        //     let mut color = RGBColor(datapoint.r as u8, datapoint.g as u8, datapoint.b as u8).filled();
            

        //     // 2 e 7
        //     // if datapoint.identifier == 2 {color = RGBColor(0, 255, 0).filled();}
        //     // if datapoint.identifier == 7 {color = RGBColor(0, 0, 255).filled();}

        //     // let pattern = identifier_representation.asPattern();
        //     // dbg!(datapoint.color);
        //     // dbg!(pattern.identifier);
            
        //     chart.draw_series(
        //         std::iter::once(Circle::new((
        //             datapoint.x, datapoint.y), 
        //             2 * datapoint.size as i32,
        //             color.filled()
        //             // ShapeStyle {
        //             //     color: datapoint.color.to_rgba(),
        //             //     filled: false,
        //             //     stroke_width: datapoint.stroke_width,
        //             // }
        //         ))
        //     ).unwrap();
        // }
    
        // println!("PLOTTED TEST GRAPH");
    }
}
### src-tauri/src/services/plot_service.rs END ###

### src-tauri/src/services/mod.rs BEGIN ###
pub mod application;
pub mod dynamic_paginator_service;
pub mod dag;
pub mod io_service;
pub mod datapoint_service;
pub mod plot_service;
pub mod metrics_service;
### src-tauri/src/services/mod.rs END ###

### src-tauri/src/services/io_service.rs BEGIN ###
use crate::{model::io::{translator::Translator, tensor_reader::TensorReader, pattern_reader::PatternReader}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct IoService {
    tensor_path: String,
    patterns_path: String,
    translator: Translator,
}

impl Default for IoService{
    fn default() -> Self {
        return IoService{
            tensor_path: String::from(""),
            patterns_path: String::from(""),
            translator: Translator::default(),
        };
    }
}

impl IoService {
    pub fn new(tensor_path: &String, patterns_path: &String) -> Result<IoService, GenericError> {
        let translator = Translator::new(&tensor_path)?;

        return Ok(
            IoService {
                tensor_path: tensor_path.to_owned(),
                patterns_path: patterns_path.to_owned(),
                translator: translator,
            }
        );
    }

    pub fn setPatternsPath(&mut self, patterns_path: &String) {
        self.patterns_path = patterns_path.to_owned();
    }

    pub fn readTensor(&self) -> Result<Tensor, GenericError> {
        println!("Reading tensor ...");
        let tensor_reader = TensorReader::new(
            &self.tensor_path,
            &self.translator);
        return tensor_reader.read();
    }

    pub fn readPatterns(&self) -> Result<Vec<Pattern>, GenericError> {
        println!("Reading patterns ...");
        let pattern_reader = PatternReader::new(
                &self.patterns_path,
                &self.translator)?;

        return pattern_reader.read();
    }

    pub fn getTranslator(&self) -> &Translator {
        return &self.translator;
    }
}
### src-tauri/src/services/io_service.rs END ###

### src-tauri/src/services/metrics_service.rs BEGIN ###
#![allow(non_snake_case)]

use crate::model::analysis::metrics::intersection::intersection_metrics::IntersectionMetrics;
use crate::model::analysis::metrics::intersection::intersections_percentages::IntersectionsPercentages;
use crate::model::analysis::metrics::intersections_predictions::IntersectionsPredictions;
use crate::{model::{analysis::metrics::{empty_model_rss::EmptyModelRss, distances::Distances, coordinates::Coordinates, rss_evolution::RssEvolution}, identifier_mapper::IdentifierMapper}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct MetricsService{
    pub empty_model_rss: EmptyModelRss,
    pub rss_evolution: RssEvolution,
    pub all_initial_visible_distances: Distances,
    pub coordinates: Coordinates,
    pub intersections_percentages: IntersectionsPercentages,
}

impl MetricsService{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, visible_identifiers: &Vec<u32>) -> Result<MetricsService, GenericError>{
        println!("Calculating metrics...");

        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper)?;

        let (prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices,
            intersections_percentages) = IntersectionMetrics::calculate(
                tensor,
                &identifier_mapper.getOrderedPatterns(),
                identifier_mapper)?;
        let mut prediction_matrix = prediction_matrix;

        let empty_model_rss = EmptyModelRss::new(tensor);
        let patterns: Vec<&Pattern> = identifier_mapper.getOrderedPatterns();

        let rss_evolution = RssEvolution::new(
            identifier_mapper,
            tensor,
            &empty_model_rss,
            &patterns,
            &mut prediction_matrix,
            &untouched_delta_rss,
            &intersections_indices
        )?;

        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions,
            &visible_identifiers,
        )?;

        let coordinates = Coordinates::new(
            &distances,
        )?;

        println!("All metrics done!");
        return Ok(
            MetricsService {
                empty_model_rss: empty_model_rss,
                rss_evolution: rss_evolution,
                all_initial_visible_distances: distances,
                coordinates: coordinates,
                intersections_percentages: intersections_percentages,
             }
        );
    }

    pub fn update(&mut self, tensor: &Tensor, identifier_mapper: &IdentifierMapper, visible_identifiers: &Vec<u32>, lazy: &bool)
            -> Result<(), GenericError>{

        let visible_patterns = identifier_mapper.getOrderedPatternsFrom(visible_identifiers);
        
        let coordinates = Coordinates::new(
            &self.all_initial_visible_distances.getView(identifier_mapper, visible_identifiers)?,
        )?;
        self.coordinates = coordinates;

        let (prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices,
            intersections_percentages) = IntersectionMetrics::calculate(
                tensor,
                &visible_patterns,
                identifier_mapper)?;
        let mut prediction_matrix = prediction_matrix;
        self.intersections_percentages = intersections_percentages;

        if !lazy{ // Re-calculate rss_evolution
            let rss_evolution = RssEvolution::new(
                identifier_mapper,
                tensor,
                &self.empty_model_rss,
                &visible_patterns,
                &mut prediction_matrix,
                &untouched_delta_rss,
                &intersections_indices
            )?;

            self.rss_evolution = rss_evolution;
        
        }else if *lazy{ // Just truncate
            let new_size = visible_identifiers.len() as u32;
            self.rss_evolution.truncate(&new_size);
        }

        return Ok(());
    }
}
### src-tauri/src/services/metrics_service.rs END ###

### src-tauri/src/services/datapoint_service.rs BEGIN ###
use crate::{model::{identifier_mapper::IdentifierMapper, analysis::metrics::{coordinates::Coordinates, metric::Metric}}, database::{datapoint::DataPoint, pattern::Pattern}, common::generic_error::GenericError};

pub struct DataPointService {}

impl DataPointService {
    fn normalizeSize(size: &u32, dimension: &u32) -> f32 {
        // let size_multiplier = 1.0;
        // let normalized_size = size_multiplier * (*size as f32 / *min_size as f32).ln();

        // if normalized_size == 0.0 {
        //     return size_multiplier;
        // }
        // return normalized_size;
        return (*size as f32).powf(1.0 / *dimension as f32);
        
    }

    fn densityToColor(density: &f64) -> (u32, u32, u32, f32) {
        let r = 255 as u32;

        let g = (255.0 * (1.0 - density)) as u32;
        let b = (255.0 * (1.0 - density)) as u32;
        
        let a = 1.0;

        return (r, g, b, a);
    }

    pub fn createDataPoints(identifier_mapper: &IdentifierMapper, coordinates: &Coordinates) -> Result<Vec<DataPoint>, GenericError> {
        let coordinates = coordinates.get();

        let mut pattern_representations: Vec<&Pattern> = Vec::new();
        for (identifier, _) in coordinates{
            let pattern = identifier_mapper.getRepresentation(identifier)?.asPattern()?;
            pattern_representations.push(pattern);
        }

        let mut datapoints: Vec<DataPoint> = Vec::new();
        let dimension = pattern_representations.get(0)
            .ok_or(GenericError::new("Could not get dimension", file!(), &line!()))?
            .dims_values.len() as u32;

        for pattern in pattern_representations {
            let coord = coordinates.get(&pattern.identifier)
                .ok_or(GenericError::new(format!("Could not get coordinate: {}", &pattern.identifier).as_str(), file!(), &line!()))?;
            
            let size = DataPointService::normalizeSize(&pattern.size, &dimension);
            let density = pattern.density as f32;
            // let stroke_width = DataPointService::calculateStrokeWidth(&max_size, &size);
            let stroke_width = 2;
            let color = DataPointService::densityToColor(&pattern.density);
            
            let x = coord.0 as f32;
            // let x = f32::round(100.0 * x) / 100.0;
            
            let y = coord.1 as f32;
            // let y = f32::round(100.0 * y) / 100.0;
            
            let datapoint = DataPoint::new(
                &pattern.identifier,
                &size,
                &pattern.size,
                &density,
                &stroke_width,
                &x,
                &y,
                &color.0,
                &color.1,
                &color.2,
                &color.3
                );

            datapoints.push(datapoint);
        }

        return Ok(datapoints);
    }
}
### src-tauri/src/services/datapoint_service.rs END ###

### src-tauri/src/services/dynamic_paginator_service.rs BEGIN ###
#![allow(non_snake_case)]

use crate::{model::{identifier_mapper::IdentifierMapper, io::translator::Translator}, database::raw_pattern::RawPattern, common::generic_error::GenericError};

pub struct DynamicPaginatorService{
    current_page: u32,
    page_size: u32,
  
    pub first_page: u32,
    pub last_page: u32,
}

impl<'a> Default for DynamicPaginatorService{
    fn default() -> Self { 
        return DynamicPaginatorService {
            current_page: 0,
            page_size: 1, 
            first_page: 0, 
            last_page: 0 };
    }
}

impl DynamicPaginatorService{
    pub fn getSoundingPattern(&self, identifier_mapper: &IdentifierMapper, translator: &Translator) -> Result<RawPattern, GenericError>{
        return Ok(identifier_mapper.getRepresentation(&1)?.asRawPattern(translator)?); // ID's start at 1
    }

    pub fn refreshPageSize(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator, page_size: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        self.page_size = page_size;
        self.refreshLastPage(identifier_mapper);

        let first_page = self.first_page.clone();
        return self.goToPage(identifier_mapper, translator, &first_page);
    }

    fn refreshLastPage(&mut self, identifier_mapper: &IdentifierMapper){
        self.last_page = (identifier_mapper.length() as f64 / self.page_size as f64).ceil() as u32 - 1;
    }

    pub fn goToPage(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator, page_index: &u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        if *page_index > self.last_page {
            return self.goToPage(identifier_mapper, translator, &self.last_page.clone());
        }

        if *page_index < self.first_page {
            return self.goToPage(identifier_mapper, translator, &self.first_page.clone());
        }

        let mut page_patterns: Vec<RawPattern> = Vec::new();
        self.current_page = *page_index;

        let first_index = self.current_page * self.page_size;
        let last_index = first_index + self.page_size - 1;
        let last_pattern_index = identifier_mapper.length() - 1;

        for i in first_index..last_index + 1{
            if i > last_pattern_index {
                break;
            }
            page_patterns.push(identifier_mapper.getRepresentation(&(i + 1))?.asRawPattern(translator)?);
        }

        return Ok((page_patterns, self.current_page.clone(), self.last_page.clone()));
    }
    
    pub fn nextPage(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        return self.goToPage(identifier_mapper, translator, &(self.current_page + 1));
    }

    pub fn previousPage(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        if self.current_page == self.first_page { // Prevents u32 overflow when trying to go to page -1
            return self.goToPage(identifier_mapper, translator, &(self.first_page).clone());
        }
        return self.goToPage(identifier_mapper, translator, &(self.current_page - 1).clone());
    }
}
### src-tauri/src/services/dynamic_paginator_service.rs END ###

### src-tauri/src/services/dag/dag_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::{database::dag_node::DagNode, common::generic_error::GenericError};
use crate::model::identifier_mapper::IdentifierMapper;
use super::dag_creator_service::DagCreatorService;

pub struct DagService{
    font_nodes: Vec<u32>,
    sub_nodes: Vec<u32>,
}

impl DagService{
    fn createFlatDagNodes(identifier_mapper: &IdentifierMapper) -> Vec<DagNode> {
        let mut nodes: Vec<DagNode> = Vec::new();
        for id in identifier_mapper.getIdentifiers(){
            nodes.push(DagNode::new(&id));
        }
        return nodes;
    }

    pub fn createAndArrange(identifier_mapper: &IdentifierMapper) -> Result<Vec<DagNode>, GenericError> {
        let flat_dag_nodes = DagService::createFlatDagNodes(identifier_mapper);
        let dag_creator_service = DagCreatorService::new(identifier_mapper);
        return dag_creator_service.create(flat_dag_nodes);
    }

    fn identifyNodes(identifier_mapper: &IdentifierMapper) -> Result<(Vec<u32>, Vec<u32>), GenericError>{
        let mut font_nodes: Vec<u32> = Vec::new();
        let mut sub_nodes: Vec<u32> = Vec::new();

        for representation in identifier_mapper.getRepresentations(){
            let dag_node = representation.asDagNode()?;

            if dag_node.supers.len() == 0{
                font_nodes.push(dag_node.identifier);
            }else{
                sub_nodes.push(dag_node.identifier);
            }
        
        }

        return Ok((font_nodes, sub_nodes));
    }

    pub fn new(identifier_mapper: &IdentifierMapper) -> Result<DagService, GenericError>{
        let (font_nodes, sub_nodes) = DagService::identifyNodes(identifier_mapper)?;
        return Ok(
            DagService{
                font_nodes: font_nodes,
                sub_nodes: sub_nodes,
            }
        );
    }

    pub fn getFontNodes(&self) -> Vec<u32> {
        return self.font_nodes.clone();
    }

    pub fn getSubNodes(&self) -> Vec<u32> {
        return self.sub_nodes.clone();
    }

    pub fn ascendDag(&self, identifier_mapper: &IdentifierMapper, current_identifier: &u32) -> Result<Vec<u32>, GenericError> {
        let supers = &identifier_mapper.getRepresentation(current_identifier)?.asDagNode()?.supers;
        if supers.len() == 0{
            return Ok(self.getFontNodes());
        }

        return Ok(supers.clone());
    }

    pub fn descendDag(&self, identifier_mapper: &IdentifierMapper, next_identifier: &u32) -> Result<Vec<u32> , GenericError> {
        let dag_node = identifier_mapper.getRepresentation(next_identifier)?.asDagNode()?;
        return Ok(dag_node.subs.clone());
    }

    pub fn getFlattenedSubs(&self, identifier_mapper: &IdentifierMapper) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let dag_nodes: Result<Vec<&DagNode>, GenericError> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes?{
            flattened_subs.insert(dag_node.identifier, dag_node.subs.clone());
        }

        return Ok(flattened_subs);
    }

    pub fn getFlattenedSupers(&self, identifier_mapper: &IdentifierMapper) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let dag_nodes: Result<Vec<&DagNode>, GenericError> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes?{
            flattened_supers.insert(dag_node.identifier, dag_node.supers.clone());
        }

        return Ok(flattened_supers);
    }
    
}
### src-tauri/src/services/dag/dag_service.rs END ###

### src-tauri/src/services/dag/dag_arranger_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::{database::dag_node::DagNode, common::generic_error::GenericError};
use debug_print::debug_println;

pub (in crate::services::dag) struct DagArrangerService{
    fonts: Vec<u32>,
    nodes: HashMap<u32, DagNode>,
}

impl DagArrangerService{
    pub fn new() -> DagArrangerService{
        return DagArrangerService {
            fonts: Vec::new(),
            nodes: HashMap::new(),
        };
    }

    pub fn init(&mut self, nodes: Vec<DagNode>){
        let nodes: HashMap<u32, DagNode> = nodes.into_iter()
            .map(|node| (node.identifier, node))
            .collect();

        self.nodes = nodes;
    }

    pub fn addFont(&mut self, new_font: &u32){
        debug_println!("    {} is now a font", new_font);
        if !self.fonts.contains(new_font){
            self.fonts.push(*new_font);
        }
    }
    
    pub fn removeFont(&mut self, old_font: &u32){
        debug_println!("    {} is not a font anymore", old_font);
        self.fonts.retain(|f| f != old_font);
    }

    pub fn addOverlappingNode(&mut self, overlapped_node: &u32, overlapping_node: &u32) -> Result<(), GenericError>{
        let overlapped_node = self.nodes.get_mut(overlapped_node)
            .ok_or(GenericError::new("Error adding overlapping node", file!(), &line!()))?;

        overlapped_node.overlappings.insert(*overlapping_node);

        return Ok(());
    }

    pub fn addBellow(&mut self, adding_node: &u32, parent: &u32) -> Result<(), GenericError>{
        let adding_node_p = self.nodes.get_mut(&adding_node)
            .ok_or(GenericError::new("Error adding node", file!(), &line!()))?;

        adding_node_p.supers.push(*parent);

        let parent_p = self.nodes.get_mut(&parent)
            .ok_or(GenericError::new("Error adding node", file!(), &line!()))?;
        
        parent_p.subs.push(*adding_node);

        return Ok(());
    }

    pub fn moveSubtreeBellow(&mut self, moving_node: &u32, new_parent: &u32) -> Result<(), GenericError>{
        let moving_node_p = self.nodes.get_mut(&moving_node)
            .ok_or(GenericError::new("Error moving node", file!(), &line!()))?;

        let old_parents: Vec<u32> = moving_node_p.supers.clone();
        moving_node_p.supers = vec![*new_parent]; // Removes old parents and adds new super of moving node

        for old_parent in old_parents{ // Deletes moving node from its old parents
            let old_parent_p = self.nodes.get_mut(&old_parent)
                .ok_or(GenericError::new("Error moving node", file!(), &line!()))?;

            old_parent_p.subs.retain(|p| p != moving_node);
        }

        let new_parent = self.nodes.get_mut(&new_parent)
            .ok_or(GenericError::new("Error moving node", file!(), &line!()))?;

        new_parent.subs.push(*moving_node); // Adds moving node to its new super

        return Ok(());
    }

    pub fn traverse(&self, to_node: &u32) -> Result<&Vec<u32>, GenericError>{
        return Ok(
            &self.nodes.get(to_node)
                .ok_or(GenericError::new("Error while traversing dag", file!(), &line!()))?
                .subs
        );
    }

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, node) in self.nodes.iter(){
            flattened_subs.insert(*id, node.subs.clone());
        }        

        return flattened_subs;
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, node) in self.nodes.iter(){
            flattened_supers.insert(*id, node.supers.clone());
        }        

        return flattened_supers;
    }

    pub fn getNode(&self, identifier: &u32) -> Result<&DagNode, GenericError>{
        return Ok(
            self.nodes.get(identifier)
                .ok_or(GenericError::new("Error getting node", file!(), &line!()))?
        );
    }

    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        let mut nodes: Vec<u32> = Vec::new();
        for node in self.nodes.values(){
            nodes.push(node.identifier);
        }
        return nodes;
    }

    pub fn getFontNodes(&self) -> Vec<&DagNode> {
        let mut font_nodes: Vec<&DagNode> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(self.getNode(font).expect("Should have found font node"));
        }   

        return font_nodes;
    }

    pub fn getFontNodesIdentifiers(&self) -> Vec<u32>{
        let mut font_nodes: Vec<u32> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(*font);
        }   

        return font_nodes;
    }

    pub fn finish(self) -> Vec<DagNode> {
        return self.nodes.into_iter()
            .map(|(_, node)| node)
            .collect();
    }
}
### src-tauri/src/services/dag/dag_arranger_service.rs END ###

### src-tauri/src/services/dag/mod.rs BEGIN ###
pub mod dag_service;
mod dag_creator_service;
mod dag_arranger_service;
### src-tauri/src/services/dag/mod.rs END ###

### src-tauri/src/services/dag/dag_creator_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::common::generic_error::GenericError;
use crate::database::dag_node::DagNode;
use crate::database::pattern::{Pattern, Relation};
use crate::model::identifier_mapper::IdentifierMapper;
use crate::common::progress_bar;
use colored::Colorize;
use debug_print::{debug_println, debug_print};

use super::dag_arranger_service::DagArrangerService;


#[derive(PartialEq, Debug, Clone, Copy)]
enum InsertionPlace{
    Bellow,
    Above,
}

pub (in crate::services::dag) struct DagCreatorService<'a>{
    dag_arranger_service: DagArrangerService,
    identifier_mapper: &'a IdentifierMapper,

    actual_node: u32,
    insertion_points: HashMap<u32, InsertionPlace>,

    assigned_belonging_level: bool,
    belonging_level: u32,
    belonging_branch: u32,
}

impl DagCreatorService<'_>{
    pub fn new<'a>(identifier_mapper: &'a IdentifierMapper) -> DagCreatorService<'a>{
        return DagCreatorService {
            dag_arranger_service: DagArrangerService::new(),
            identifier_mapper: identifier_mapper,
            actual_node: 0,
            insertion_points: HashMap::new(),
            assigned_belonging_level: false,
            belonging_level:0, 
            belonging_branch:0, 
        };
    }

    fn changePosition(&mut self, new_position: u32) -> Result<&Vec<u32>, GenericError> {
        self.actual_node = new_position;
        return Ok(
            self.dag_arranger_service.traverse(&self.actual_node)?
        );
    }

    fn firstRelationToSecond(&self, first_node: &u32, second_node: &u32) -> Result<Relation, GenericError> {
        let first_patern: &Pattern = self.identifier_mapper.getRepresentation(first_node)?.asPattern()?;
        let second_patern: &Pattern = self.identifier_mapper.getRepresentation(second_node)?.asPattern()?;
        return first_patern.selfRelationTo(second_patern);
    }

    fn refreshOverlappingRelations(&mut self, first_node: &u32, second_node: &u32, relation: Relation) -> Result<(), GenericError>{
        if relation == Relation::NotRelatable{ return Ok(()); }

        let first_pattern_density = self.identifier_mapper.getRepresentation(first_node)?.asPattern()?.density;
        let second_pattern_density = self.identifier_mapper.getRepresentation(second_node)?.asPattern()?.density;

        if first_pattern_density >= second_pattern_density{
            self.dag_arranger_service.addOverlappingNode(second_node, first_node)?;
        }

        if first_pattern_density <= second_pattern_density {
            self.dag_arranger_service.addOverlappingNode(first_node, second_node)?;
        } 

        return Ok(());
    }

    fn traverseTree(&mut self, subtree_font: &u32, node_to_compare: &u32, current_branch: u32, current_level: u32) -> Result<(), GenericError>{
        debug_print!("\n    => Traversing subtree of {}, ", subtree_font);
        let current_level_nodes: Vec<u32> = self.changePosition(*subtree_font)?.clone();
        let mut next_level_fonts: Vec<u32> = Vec::new();
        debug_println!("level: {}, level size: {}, branch: {}, belonging_branch: {}, belonging_level: {}", &current_level, current_level_nodes.len(), &current_branch, &self.belonging_branch, &self.belonging_level);

        let mut belongs_to_this_level: bool = false;
        let relation = self.firstRelationToSecond(node_to_compare, &subtree_font)?;
        if relation == Relation::SubPattern{ belongs_to_this_level = true; }
        self.refreshOverlappingRelations(node_to_compare, subtree_font, relation)?;
        
        for current_level_node in current_level_nodes.iter(){
            if relation == Relation::SuperPattern{ // Node to compare is super of subtree_font, does not need to traverse this branch
                continue;
            }

            if relation == Relation::NotRelatable{ // Node to compare does not have 'physical' contact with subtree_font, does not need to traverse this branch
                continue;
            }
            next_level_fonts.push(*current_level_node);
        }

        for (i, next_level_font) in next_level_fonts.iter().enumerate(){ // RECURSIVE
            let next_branch = current_branch + i as u32;
            self.traverseTree(&next_level_font, node_to_compare, next_branch, current_level + 1)?;
        }
        // Recursion returnal bellow

        // Insertion operation
        if belongs_to_this_level && !self.assigned_belonging_level{ // Makes sure to insert on the deepest possible
            debug_println!("    Setting insertion point to bellow {}", subtree_font);
            
            self.insertion_points.insert(*subtree_font, InsertionPlace::Bellow);
            self.assigned_belonging_level = true; // Previous levels cannot change insertion point now
            self.belonging_level = current_level;
            self.belonging_branch = current_branch;
            debug_println!("    Belonging branch is now {}", &self.belonging_branch);
        }

        // Connects node_to_compare as super of this subtree font
        if relation == Relation::SuperPattern{
            // A pattern (node_to_compare) from an upper branch is super of the font of this branch
            // Sets the super relation on the recursion returnal
            debug_println!("    {} {} {}{}", format!("{}", &node_to_compare).yellow(), "located in a different upper branch is super of".yellow(), format!("{}", &subtree_font).yellow(), ", CONNECTING them".yellow());
            self.dag_arranger_service.addBellow(subtree_font, node_to_compare)?;
        }
    
        // Connects node_to_compare as sub of different branches
        if relation == Relation::SubPattern && current_branch != self.belonging_branch{ // Makes sure to connect ONLY to different branches
            // A pattern (node_to_compare) from a DIFFERENT branch is sub of the font of this branch
            // Sets the sub relation on the recursion returnal

            if current_level < self.belonging_level{ // Avoids the connection of patterns that are above the insertion point
                return Ok(());
            }

            debug_println!("    {} {} {}{}", format!("{}", node_to_compare).yellow(), "located in a different below branch is sub of".yellow(), format!("{}", &subtree_font).yellow(), ", CONNECTING them".yellow());
            self.dag_arranger_service.addBellow(node_to_compare, subtree_font)?;
        }

        return Ok(());

    }

    fn getRelationedFonts(&mut self,node: &u32) -> Result<HashMap<u32, Relation>, GenericError> {
        let fonts: Vec<u32> = self.dag_arranger_service.getFontNodesIdentifiers();
        debug_println!("    Current fonts {:?}", fonts);
        let mut relationed_fonts: HashMap<u32, Relation> = HashMap::new();

        for font in fonts{
            let relation = self.firstRelationToSecond(node, &font)?;
            if relation == Relation::NotRelatable{ continue; }
            self.refreshOverlappingRelations(node, &font, relation)?;

            relationed_fonts.insert(font, relation);
        }
        return Ok(relationed_fonts);
    }

    fn setInsertionPoints(&mut self, node: &u32) -> Result<(), GenericError>{
        debug_println!("{} {}", "\n=> SETTING insertion points for node".green(), format!("{}", node).green());
        self.insertion_points.clear();
        let relationed_fonts: HashMap<u32, Relation> = self.getRelationedFonts(node)?;

        if relationed_fonts.len() == 0{
            // This node is a new font
            debug_println!("    Node does not have any relationed font, setting it to be a new font");
            return Ok(());
        }

        debug_println!("    Found relationed fonts: {:?}", &relationed_fonts);

        for relationed_font in relationed_fonts {
            // Finds the insertion points on each font subtree
            if relationed_font.1 == Relation::SuperPattern{
                // Node is super of relationed_font, consequently node is the new font
                debug_println!("    {} is super of {}, setting insertion point to be above {}", node, &relationed_font.0, &relationed_font.0);
                self.insertion_points.insert(relationed_font.0, InsertionPlace::Above);
                continue;
            }

            self.assigned_belonging_level = false;
            self.belonging_branch = 0;
            self.belonging_level = 0;
            self.traverseTree(&relationed_font.0, node, 1, 2)?;
        }

        return Ok(());
    }

    fn insertNodeAbove(&mut self, node: &u32, insertion_point: &u32) -> Result<(), GenericError>{
        debug_println!("    Inserting {} above {}", node, insertion_point);
        self.dag_arranger_service.moveSubtreeBellow(insertion_point, node)?;

        return Ok(());
    }

    fn insertNodeBellow(&mut self, node: &u32, insertion_point: &u32) -> Result<(), GenericError>{
        let subs = self.dag_arranger_service.traverse(insertion_point)?.clone();

        debug_println!("    Inserting {} bellow {}", node, insertion_point);
        self.dag_arranger_service.addBellow(node, insertion_point)?;

        if subs.is_empty(){
            return Ok(());
        }

        debug_println!("    Insertion point has subs {:?}", &subs);
        for sub in subs{
            let relation = self.firstRelationToSecond(node, &sub)?;
            self.refreshOverlappingRelations(node, &sub, relation)?;

            if relation == Relation::SuperPattern{
                // If the node is super of someone rearrange dag
                debug_println!("    {} is super of {}, putting {} subtree bellow {}", node, &sub, &sub, node);
                self.dag_arranger_service.moveSubtreeBellow(&sub, node)?;
            }
        }

        return Ok(());
    }

    fn insertNodeOnDag(&mut self, node: &u32) -> Result<(), GenericError>{
        debug_println!("{} {} {}", "\n==> INSERTING node".yellow(), format!("{}", node).yellow(), "on DAG".yellow());
        debug_println!("    Insertion points: {:?} (empty if is new font)", &self.insertion_points);

        if self.insertion_points.is_empty(){
            self.dag_arranger_service.addFont(node);
        }

        for insertion_point in self.insertion_points.clone().iter(){
            debug_println!();
            let insertion_place = insertion_point.1;
            let insertion_point = insertion_point.0;

            if *insertion_place == InsertionPlace::Above{
                // This should only trigger if the dag has draw a pseudo-font
                self.dag_arranger_service.removeFont(insertion_point);
                self.dag_arranger_service.addFont(node);
                
                self.insertNodeAbove(node, insertion_point)?;
                continue;
            }

            if *insertion_place == InsertionPlace::Bellow{
                self.insertNodeBellow(node, insertion_point)?;
                continue;
            }
        }

        return Ok(());
    }

    pub fn create(mut self, flat_dag_nodes: Vec<DagNode>) -> Result<Vec<DagNode>, GenericError>{
        debug_println!("Nodes: {:?}", &flat_dag_nodes.iter().map(|node| node.identifier).collect::<Vec<u32>>());
        let bar = progress_bar::new(flat_dag_nodes.len() as u64,"Patterns inserted on DAG");

        self.dag_arranger_service.init(flat_dag_nodes);
        
        for id in self.dag_arranger_service.getNodesIdentifiers(){
            self.setInsertionPoints(&id)?;
            self.insertNodeOnDag(&id)?;
            bar.inc(1);
        }
        bar.finish();

        debug_println!("Subs: {:?}", self.dag_arranger_service.getFlattenedSubs());
        debug_println!("Supers: {:?}", self.dag_arranger_service.getFlattenedSupers());

        println!("\n  Nb of fonts found: {}", self.dag_arranger_service.getFontNodes().len());

        return Ok(self.dag_arranger_service.finish());
    }
}
### src-tauri/src/services/dag/dag_creator_service.rs END ###

### src-tauri/src/services/application/mod.rs BEGIN ###
pub mod application_service;
pub mod application_state_service;
### src-tauri/src/services/application/mod.rs END ###

### src-tauri/src/services/application/application_state_service.rs BEGIN ###
#![allow(non_snake_case)]
use crate::common::generic_error::GenericError;
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;

use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::DagService;
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::MetricsService;

#[derive(Default)]
pub struct ApplicationStateService{
    tensor: Option<Tensor>,
    identifier_mapper: Option<IdentifierMapper>,

    metrics_service: Option<MetricsService>,
    dag_service: Option<DagService>,

    current_identifier: u32,
    current_level: u32,
    current_level_identifiers: Vec<u32>,
    visible_identifiers: Vec<u32>,
}

impl ApplicationStateService{
    pub fn init(&mut self, tensor: Tensor, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        self.tensor = Some(tensor);
        self.changePatterns(patterns)?;

        return Ok(());
    }

    pub fn changePatterns(&mut self, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        // Inserts the pattern representations
        let mut identifier_mapper = IdentifierMapper::new(patterns);

        // Inserts the dag node representations
        identifier_mapper.insertDagNodeRepresentations(
            DagService::createAndArrange(&identifier_mapper)?,
        )?;
        
        let dag_service = DagService::new(&identifier_mapper)?;
        self.dag_service = Some(dag_service);

        self.current_level = 0;
        self.current_level_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getFontNodes();

        self.visible_identifiers = self.current_level_identifiers.clone();

        // Inserts the data point representations
        let metrics_service = MetricsService::new(
            &identifier_mapper,
            self.tensor.as_ref()
                .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?,
            &self.visible_identifiers
        )?;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, &metrics_service.coordinates)?
        )?;

        self.metrics_service = Some(metrics_service);
        self.identifier_mapper = Some(identifier_mapper);

        return Ok(());
    }

    fn update(&mut self, new_current_level_identifiers: &Option<Vec<u32>>) -> Result<(), GenericError>{
        let tensor = self.tensor.as_ref()
            .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?;

        let identifier_mapper = self.identifier_mapper.as_mut()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()))?;

        let lazy = match new_current_level_identifiers {
            Some(_) => false, // Changing the current level identifiers has to be done eagerly
            None => true, // Here we do not need to re-calculate rss_evolution
        };

        let identifiers_used_to_update = match new_current_level_identifiers {
            Some(new_current_level_identifiers) => new_current_level_identifiers, // Updates all identifiers and reset visible identifiers (dag movement)
            None => &self.visible_identifiers, // Updates only the visible identifiers, (truncation)
        };

        self.metrics_service.as_mut().ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .update(tensor, identifier_mapper, identifiers_used_to_update, &lazy)?;

        let coordinates = &self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .coordinates;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, coordinates)?
        )?;

        // Should also insert dagNode representations
        
        if !lazy{ // Reset everything because current_level_identifiers is gonna be changed
            self.current_level_identifiers = identifiers_used_to_update.clone();
            self.visible_identifiers = identifiers_used_to_update.clone();
        }

        return Ok(());
    }

    pub fn ascendDag(&mut self) -> Result<bool, GenericError>{
        if self.current_identifier == 0 || self.current_level == 0 { return Ok(false); }

        let previous_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .ascendDag(self.identifierMapper()?, &self.current_identifier)?;

        if self.current_level > 0 { self.current_level -= 1; }

        self.update(&Some(previous_identifiers))?;

        return Ok(true);
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<bool, GenericError>{
        let next_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .descendDag(self.identifierMapper()?, next_identifier)?;

        if next_identifiers.len() == 0{ return Ok(false); }
        
        self.current_level += 1;
        self.update(&Some(next_identifiers))?;
        return Ok(true);
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<(), GenericError>{
        let mut all_identifiers = self.identifierMapper()?
            .getIdentifiers().clone();
            
        all_identifiers.sort();
        all_identifiers.truncate(*new_size as usize);

        let mut current_level_identifiers = self.current_level_identifiers.clone();
        current_level_identifiers.sort();

        self.visible_identifiers = current_level_identifiers.iter()
            .filter(|identifier| all_identifiers.contains(&identifier))
            .map(|identifier| identifier.clone())
            .collect();

        self.update(&None)?;

        return Ok(());
    }

    pub fn identifierMapper(&self) -> Result<&IdentifierMapper, GenericError>{
        return self.identifier_mapper.as_ref()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()));
    }

    pub fn getAllIdentifiers(&self) -> Result<&Vec<u32>, GenericError>{
        return Ok(
            &self.current_level_identifiers
        );
    }

    pub fn getVisibleIdentifiers(&self) -> &Vec<u32>{
        return &self.visible_identifiers;
    }

    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return Ok(
            self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getSubNodes()
        );
    }

    pub fn getMetricsService(&self) -> Result<&MetricsService, GenericError>{
        return Ok(
            self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
        );
    }

    pub fn getDagService(&self) -> Result<&DagService, GenericError>{
        return self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()));
    }

    pub fn getCurrentDagLevel(&self) -> u32{
        return self.current_level;
    }
}
### src-tauri/src/services/application/application_state_service.rs END ###

### src-tauri/src/services/application/application_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use itertools::Itertools;

use crate::{common::generic_error::GenericError, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, raw_pattern::RawPattern}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper, io::translator::Translator}, services::{io_service::IoService, plot_service::PlotService}};
use super::application_state_service::ApplicationStateService;

pub struct ApplicationService{
    io_service: IoService,
    application_state_service: ApplicationStateService,
}

impl Default for ApplicationService{
    fn default() -> Self {
        return ApplicationService{
            io_service: IoService::default(),
            application_state_service: ApplicationStateService::default(),
        };
    }
}

impl ApplicationService{
    pub fn init(&mut self, tensor_path: &String, patterns_path: &String) -> Result<(), GenericError>{
        let start_time = Instant::now();
        println!("Initializing model...");

        self.io_service = IoService::new(tensor_path, patterns_path)?;
        let tensor = self.io_service.readTensor()?;
        let patterns = self.io_service.readPatterns()?;

        self.application_state_service = ApplicationStateService::default();
        self.application_state_service.init(tensor, patterns)?;

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(&self.application_state_service);
        return Ok(());
    }

    pub fn getFlattenedSupers(&self) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let identifier_mapper = self.application_state_service.identifierMapper()?;
        return Ok(
            self.application_state_service.getDagService()?.getFlattenedSupers(identifier_mapper)?
        );
    }

    pub fn getFlattenedSubs(&self) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let identifier_mapper = self.application_state_service.identifierMapper()?;
        return Ok(
            self.application_state_service.getDagService()?.getFlattenedSubs(identifier_mapper)?
        );
    }

    pub fn getDistances(&self) -> Result<&HashMap<u32, HashMap<u32, f64>>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.all_initial_visible_distances.get()
        );
    }

    pub fn getIdentifierMapper(&self) -> Result<&IdentifierMapper, GenericError> {
        return self.application_state_service.identifierMapper();
    }

    pub fn getTranslator(&self) -> &Translator {
        return self.io_service.getTranslator();
    }

    // ================ External API ================

    pub fn changePatterns(&mut self, patterns_path: &String) -> Result<(), GenericError>{
        println!("\nChanging patterns to: {}", patterns_path);
        self.io_service.setPatternsPath(patterns_path);
        let patterns = self.io_service.readPatterns()?;

        self.application_state_service.changePatterns(patterns)?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn getDatapointsWithSubPatterns(&self) -> Result<Vec<DataPoint>, GenericError>{
        let datapoints = self.getDataPoints()?;
        let mut datapoints_with_subpatterns: Vec<DataPoint> = vec![];

        for datapoint in datapoints.iter(){
            let identifier = datapoint.identifier;

            let subpatterns = &self.getIdentifierMapper()?.getRepresentation(&identifier)?.asDagNode()?.subs;
            if subpatterns.len() > 0{ datapoints_with_subpatterns.push(datapoint.clone()); }
        }
        
        return Ok(datapoints_with_subpatterns);
    }

    pub fn ascendDag(&mut self) -> Result<Vec<DataPoint>, GenericError>{
        println!("\nAscending dag");
        let result = self.application_state_service.ascendDag()?;
        if !result { return Ok(Vec::new());}

        PlotService::plot(&self.application_state_service);
        return self.getDataPoints();
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<Vec<DataPoint>, GenericError> {
        println!("\nDescending dag to: {}", next_identifier);
        let result = self.application_state_service.descendDag(next_identifier)?;
        if !result { return Ok(Vec::new());}

        PlotService::plot(&self.application_state_service);
        return self.getDataPoints();
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<Vec<(f32, f32)>, GenericError> {
        println!("\nTruncating model to {} patterns", new_size);
        self.application_state_service.truncateModel(&new_size)?;
        PlotService::plot(&self.application_state_service);

        let mut datapoints = self.getDataPoints()?;
        datapoints.truncate(*new_size as usize);

        let datapoints_changes: Vec<(f32, f32)> = datapoints.into_iter()
            .map(|datapoint| (datapoint.x, datapoint.y))
            .collect();

        return Ok(datapoints_changes);
    }

    pub fn getDataPoints(&self) -> Result<Vec<DataPoint>, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();
        // let identifiers = self.application_state_service.getAllIdentifiers()?;
        let datapoints: Vec<DataPoint> = self.application_state_service.identifierMapper()?
            .getOrderedDataPointsFrom(visible_identifiers).into_iter()
            .map(|datapoint| datapoint.clone())
            .collect();

        return Ok(datapoints);
    }
    
    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return self.application_state_service.getAllSubPatternsIdentifiers();
    }

    pub fn getRawPattern(&self, identifier: &u32) -> Result<RawPattern, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();

        if !visible_identifiers.contains(identifier){
            return Err(GenericError::new("Identifier not visible", file!(), &line!()));
        }

        return self.getIdentifierMapper()?.getIdentifier(identifier)?
            .asRawPattern(self.io_service.getTranslator());
    }

    pub fn getFullRssEvolution(&self) -> Result<Vec<f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.rss_evolution.get().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect()
        );
    }

    pub fn getTruncatedRssEvolution(&self) -> Result<Vec<f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.rss_evolution.getTruncated().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect()
        );
    }

    pub fn getIntersectionsPercentages(&self, identifier: &u32) -> Result<HashMap<u32, f64>, GenericError> {
        let intesections_percentages = self.application_state_service.getMetricsService()?.intersections_percentages.get();
        return Ok(
            intesections_percentages.get(identifier)
                .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?
                .clone()
        );
    }

    pub fn getIntersectionDetails(&self, identifier: &u32) -> Result<IntersectionsDetails, GenericError>{
        let intersection_percentages: HashMap<u32, f64> = match self.application_state_service.getMetricsService()?
            .intersections_percentages.get().get(identifier){

            Some(intersection_percentages) => intersection_percentages.clone(),
            None => HashMap::new(),
        };

        let total_intersection_percentage = intersection_percentages.get(identifier)
            .expect("Should have a total untouched percentage, even if its 0").clone();
        let total_untouched_percentage = 1.0 - total_intersection_percentage;
        
        let current_pattern = self.getIdentifierMapper()?.getIdentifier(identifier)?.asPattern()?;
        let all_dims_intersections: Result<HashMap<u32, (f64, Vec<Vec<String>>)>, GenericError> = intersection_percentages.into_iter()
            .filter(|(other_identifier, _)| *other_identifier != *identifier)
            .map(|(other_identifier, percentage)| {

                let other_pattern = self.getIdentifierMapper()?.getIdentifier(&other_identifier)?.asPattern()?;
                
                let dims_intersections = current_pattern.dimIntersection(&other_pattern)?;
                let dims_intersections = self.getTranslator()
                    .untranslateLineDims(&dims_intersections)?.iter()
                    .map(|line| {
                        let values: Vec<String> = line.split(",").map(|dim| dim.to_string()).collect_vec();
                        return values;
                    })
                    .collect();

                return Ok((other_identifier, (percentage, dims_intersections)));
            })
            .collect();
        let all_dims_intersections = all_dims_intersections?;
        
        let intersections_details = IntersectionsDetails::new(*identifier, 
            total_untouched_percentage, total_intersection_percentage, all_dims_intersections);
        
        return Ok(intersections_details);
    }

    pub fn getCurrentDagLevel(&self) -> u32{
        return self.application_state_service.getCurrentDagLevel();
    }

}
### src-tauri/src/services/application/application_service.rs END ###

### DIRECTORY src-tauri/src/ FLATTENED CONTENT ###
