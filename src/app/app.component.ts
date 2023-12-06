// https://www.telerik.com/blogs/angular-14-introducing-standalone-components#:~:text=Creating%20a%20Standalone%20Component,ng%20g%20c%20login%20%2D%2Dstandalone
// https://material.angular.io/components/categories
// https://css-tricks.com/snippets/css/a-guide-to-flexbox/
// https://br.pinterest.com/pin/800022321275429738/
// import * as numeric from 'numeric';

import { AfterViewInit, Component, ElementRef, ViewChild } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import { DagComponent } from "./dag/dag.component";
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
import { PatternSummaryComponent } from "./pattern-summary/pattern-summary.component";
import { DynamicPaginatorComponent } from "./dynamic-paginator/dynamic-paginator.component";
import { open } from '@tauri-apps/api/dialog';
import { RssViewComponent } from "./rss-view/rss-view.component";
import { provideRouter, Router, RouterOutlet} from "@angular/router";



@Component({
  selector: "app-root",
  templateUrl:'./app.component.html',
  styleUrls:['./app.component.scss'],
  standalone: true,
  imports: [
    RouterOutlet,
    CommonModule,
    DagComponent,
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
    MatIconModule]
})

export class AppComponent implements AfterViewInit{
  private DEV_MODE: boolean = true;

  @ViewChild("aside") aside: ElementRef<HTMLElement>;
  @ViewChild("header") header: ElementRef<HTMLElement>;
  
  protected dag_view: DagComponent;
  private rss_view: RssViewComponent;

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

  constructor(private router: Router){}

  ngAfterViewInit(){
    this.matList_height = this.aside.nativeElement.clientHeight - this.header.nativeElement.clientHeight;

    if(this.DEV_MODE){ this.openDagView(); }
    
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
        this.router.navigateByUrl('', {skipLocationChange: true}).then(()=>
        this.router.navigate(["dagview"]));

      }).catch((error: any) => {
        console.log(error);
      });
    }
  }

  public onActivate(componentInstance: any) {
    if (componentInstance instanceof DagComponent) {
      this.dag_view = componentInstance;
    }

    if (componentInstance instanceof RssViewComponent) {
      this.rss_view = componentInstance;
    }

    if(this.rss_view != undefined){
      console.log("Pattern number: " + this.rss_view.getPatternNumber());
    }
  }

  public openDagView(){
    this.router.navigate(['/dagview'], {queryParams: {dev_mode: this.DEV_MODE}});
  }

  public openFullSizeRss(){
    this.router.navigate(['/rssview'], {queryParams: {dev_mode: this.DEV_MODE}});
  }
}
