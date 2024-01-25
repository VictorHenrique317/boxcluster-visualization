import { CommonModule } from '@angular/common';
import { open } from '@tauri-apps/api/dialog';
import {ChangeDetectorRef, Component, ElementRef, EventEmitter, Inject, NgModule, Output} from '@angular/core';
import {MatDialogRef, MatDialogModule} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';
import { DataPoint } from 'src/app/models/datapoint';
import { Pattern } from 'src/app/models/pattern';
import {MatTabsModule} from '@angular/material/tabs';
import {AfterViewInit, ViewChild} from '@angular/core';
import {MatPaginator, MatPaginatorModule} from '@angular/material/paginator';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import { animate, state, style, transition, trigger } from '@angular/animations';

@Component({
  selector: 'app-datapoint-info-dialog',
  standalone: true,
  imports: [
    CommonModule,
    MatTabsModule,
    MatFormFieldModule, 
    MatInputModule, 
    MatTableModule,
    MatSortModule, 
    MatPaginatorModule
  ],
  templateUrl: './datapoint-info-dialog.component.html',
  styleUrls: ['./datapoint-info-dialog.component.scss']
})
export class DatapointInfoDialogComponent {
  protected pattern;
  
  @ViewChild(MatSort) sort: MatSort;
  private input: HTMLInputElement;
  
  protected displayed_columns: string[] = ['values'];
  protected data_source: MatTableDataSource<Array<any>>;
  protected selected_tab;

  constructor(public dialogRef: MatDialogRef<DatapointInfoDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {pattern: Pattern},
    private cdr: ChangeDetectorRef) {
      this.selected_tab = 0;
      this.pattern = data.pattern;
      this.data_source = new MatTableDataSource(this.pattern.dims_values[this.selected_tab]);
  }

  ngAfterViewInit() {
    this.data_source.sort = this.sort;
  }

  protected applyFilter(event: Event) {
    this.data_source.data = this.pattern.dims_values[this.selected_tab];
    this.input = (event.target as HTMLInputElement);

    const filterValue = (event.target as HTMLInputElement).value.trim().toLowerCase();

    let filteredData = this.data_source.data.filter(item => {
        let itemStr = JSON.stringify(item).toLowerCase();
        return itemStr.includes(filterValue);
    });
    
    this.data_source.data = filteredData;
}


  protected onTabChange(tab_index: number){
    this.selected_tab = tab_index;
    let dim = this.pattern.dims_values[tab_index];
    this.data_source.data = dim;
    this.data_source.sort = this.sort;
    
    if (this.input != null){
      this.input.value = "";
    }
  }

}
