import { AfterViewInit, Component, ElementRef, Input, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatListModule } from '@angular/material/list';
import { MatRippleModule } from '@angular/material/core';
import { PatternSummaryComponent } from './pattern-summary/pattern-summary.component';
import { Pattern } from 'src/app/models/pattern';
import { MatIconModule } from '@angular/material/icon';
import { invoke } from '@tauri-apps/api/tauri'
import { MatDividerModule } from '@angular/material/divider';

@Component({
  selector: 'app-dynamic-paginator',
  standalone: true,
  templateUrl: './dynamic-paginator.component.html',
  styleUrls: ['./dynamic-paginator.component.scss'],
  imports: [
    CommonModule,
    PatternSummaryComponent,
    MatListModule,
    MatRippleModule,
    MatDividerModule,
    MatIconModule]
})
export class DynamicPaginatorComponent implements AfterViewInit {
  // @Input() public dag: Dag;
  @Input() public matList_height: number;
  @ViewChild("navigation_options") navigation_options: ElementRef<HTMLDivElement>;

  private page_size: number = 1;
  
  public page_items: Array<Pattern>;
  public current_page: number = 0;
  public total_pages: number = 1;
  public available_height: number;
  public navigation_options_height: number;

  constructor(){}

  ngAfterViewInit(){
    console.log("Initializing paginator");

    this.navigation_options_height = this.navigation_options.nativeElement.clientHeight;
    this.available_height = this.matList_height - this.navigation_options_height;

    console.log("matList_height: %d, navigation_options_height: %d, available_height: %d", this.matList_height, this.navigation_options_height, this.available_height);

    invoke("getSoundingPattern").then((sounding_pattern: Pattern) =>{
      this.page_items = [sounding_pattern];
      console.log("Sounding pattern: %o", sounding_pattern);

    }).catch((error: any) => {
      console.log(error);
    });
  }

  public refreshPageSize(page_size:number){
    console.log("Refreshing page size to %d", page_size);

    if (this.page_size == page_size){ return; }

    // Here its necessary to resize the paginator
    this.page_size = page_size;
    invoke("refreshPageSize", {pageSize: this.page_size}).then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

    }).catch((error: any) => {
      console.log(error);
    });
  }

  public goToPage(page_index:number){
    invoke("goToPage", {page_index: page_index}).then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

    }).catch((error: any) => {
      console.log(error);
    });
  }

  public goToFirstPage(){
    invoke("goToFirstPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

    }).catch((error: any) => {
      console.log(error);
    });
  }

  public goToLastPage(){
    invoke("goToLastPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

    }).catch((error: any) => {
      console.log(error);
    });
  }

  public nextPage(){
    invoke("nextPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

    }).catch((error: any) => {
      console.log(error);
    });
  }

  public previousPage(){
    invoke("previousPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

    }).catch((error: any) => {
      console.log(error);
    });
  }
}
