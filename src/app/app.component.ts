// https://www.telerik.com/blogs/angular-14-introducing-standalone-components#:~:text=Creating%20a%20Standalone%20Component,ng%20g%20c%20login%20%2D%2Dstandalone
// https://material.angular.io/components/categories
// https://css-tricks.com/snippets/css/a-guide-to-flexbox/
// https://br.pinterest.com/pin/800022321275429738/

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

@Component({
  selector: "app-root",
  templateUrl:'./app.component.html',
  styleUrls:['./app.component.scss'],
  standalone: true,
  imports: [
    CommonModule,
    DagComponent,
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

  public file_path: string;
  public file_name: string = "Open file";
  public matList_height: number;

  length = 50;
  pageSize = 10;
  pageIndex = 0;

  constructor(){}

  ngAfterViewInit(){
    this.matList_height = this.aside.nativeElement.clientHeight - this.header.nativeElement.clientHeight;
  }

  public async openFileDialog(){
    const selected = await open({ multiple: false});
    if (selected === null) { return; } // No file selected
    
    this.file_path = selected.toString();
    this.file_name = this.file_path.split('\\').pop().split('/').pop();
    if (this.file_path == ""){ return; } // No tensor selected
    
    invoke("readFile", {path: this.file_path});
    // this.fileService.openFileDialog().then(path =>{
    //     this.fileService.readFile(path).then(new_dag =>{
    //         let new_mapping = new Map<number, Pattern>();

    //         for (let entry of new Map(Object.entries(new_dag.mapping))){
    //             new_mapping.set(parseInt(entry[0]), entry[1]);
    //         } // unnecessary?

    //         new_dag.mapping = new_mapping;
    //         this.dag = new_dag;
    //     });
    // });
  }
}
