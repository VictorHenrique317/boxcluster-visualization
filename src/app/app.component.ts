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
  @ViewChild("aside") aside: ElementRef<HTMLElement>;
  @ViewChild("header") header: ElementRef<HTMLElement>;
  
  protected dag_view: DagComponent;
  private rss_view: RssViewComponent;

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
    // this.router.navigate(['dagview']);
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
      console.log(this.rss_view.getPatternNumber());
    }
  }

  // public svgWheelHandler(event: WheelEvent){
  //   if (this.router.url == '/dagview'){
  //     // this.dag.wheelHandler(event);
  //   }
  // }

  public openDagView(){
    this.router.navigate(['/dagview']);
  }

  public openFullSizeRss(){
    this.router.navigate(['/rssview']);
  }

//   private classicMds(distances: number[][], dimensions: number = 2) {
//     // square distances
//     let M = distances.map(row => row.map(value => numeric.mul(-0.5, numeric.pow(value, 2))));

//     // double centre the rows/columns
//     function mean(A: number[]) { return numeric.div(numeric.add.apply(null, A), A.length); }
//     let rowMeans = M.map(row => mean(row)),
//         colMeans = numeric.transpose(M).map(col => mean(col)),
//         totalMean = mean(rowMeans);

//     for (let i = 0; i < M.length; ++i) {
//         for (let j =0; j < M[0].length; ++j) {
//             M[i][j] += totalMean - rowMeans[i] - colMeans[j];
//         }
//     }

//     // take the SVD of the double centred matrix, and return the
//     // points from it
//     let ret = numeric.svd(M),
//         eigenValues = numeric.sqrt(ret.S);
//     return ret.U.map(function(row: number[]) {
//         return numeric.mul(row, eigenValues).splice(0, dimensions);
//     });
// }

}
