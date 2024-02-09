import { resolveResource } from '@tauri-apps/api/path'
import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, Input, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatListModule } from '@angular/material/list';
import { MatRippleModule } from '@angular/material/core';
import { PatternSummaryComponent } from './pattern-summary/pattern-summary.component';
import { Pattern } from 'src/app/models/pattern';
import { MatIconModule } from '@angular/material/icon';
import { invoke } from '@tauri-apps/api/tauri'
import { MatDividerModule } from '@angular/material/divider';
import { environment } from 'src/environments/environment';
import { fs } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';

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

  constructor(private cdr: ChangeDetectorRef, private dialog_service: DialogService){}

  async ngAfterViewInit(){
    console.log("Initializing paginator");

    this.navigation_options_height = this.navigation_options.nativeElement.clientHeight;
    this.available_height = this.matList_height - this.navigation_options_height;

    // console.log("matList_height: %d, navigation_options_height: %d, available_height: %d", this.matList_height, this.navigation_options_height, this.available_height);

    let sounding_pattern;
    if(!environment.dev_mode){
      sounding_pattern = await invoke("getSoundingPattern").catch((error: any) => {
        console.error(error);
        this.dialog_service.openErrorDialog("Could not load paginator.");
      });

    }else if(environment.dev_mode){
      let dims_values: Array<Array<string>> = [["a", "b", "c"], ["a", "b", "c"], ["a", "b", "c"]];
      sounding_pattern = new Pattern(1, dims_values, 0.8, 2);
    }

    console.log("Received sounding pattern:");
    console.log(sounding_pattern);

    this.page_items = [sounding_pattern];
    this.cdr.detectChanges();
  }

  protected async refreshPageSize(page_size:number){
    // This method is (only) called each time a pattern-summary component is created,
    // but its fully executed just once, when the paginator is initialized.

    if (this.page_size == page_size){ return; }

    // Here its necessary to resize the paginator
    this.page_size = page_size;

    if(!environment.dev_mode){
      let data = await invoke("refreshPageSize", {pageSize: this.page_size}).catch((error: any) => {
        console.error(error);
        this.dialog_service.openErrorDialog("Could not refresh paginator.");
      });

      this.page_items = data[0];
      this.current_page = data[1];
      this.total_pages = data[2] + 1;

    }else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/page_patterns.json'));
      let page_items = JSON.parse(rawdata);

      this.page_items = page_items;
      this.current_page = 0;
      this.total_pages = 1;
    }

    this.cdr.detectChanges();
  }

  public goToPage(page_index:number){
    invoke("goToPage", {page_index: page_index}).then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

      this.cdr.detectChanges();

    }).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("Could not go to page: " + page_index + ".");
    });
  }

  public goToFirstPage(){
    invoke("goToFirstPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

      this.cdr.detectChanges();

    }).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("Could not go to first page.");
    });
  }

  public goToLastPage(){
    invoke("goToLastPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

      this.cdr.detectChanges();

    }).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("Could not go to last page.");
    });
  }

  public nextPage(){
    invoke("nextPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

      this.cdr.detectChanges();

    }).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("Could not go to next page.");
    });
  }

  public previousPage(){
    invoke("previousPage").then((result: any) =>{
      this.page_items = result[0];
      this.current_page = result[1];
      this.total_pages = result[2] + 1;

      this.cdr.detectChanges();

    }).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("Could not go to previous page.");
    });
  }
}
