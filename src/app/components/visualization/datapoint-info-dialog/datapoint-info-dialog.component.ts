import { CommonModule } from '@angular/common';
import { open } from '@tauri-apps/api/dialog';
import {Component, EventEmitter, Inject, NgModule, Output} from '@angular/core';
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
  
  @ViewChild(MatPaginator) paginator: MatPaginator;
  @ViewChild(MatSort) sort: MatSort;
  protected displayed_columns: string[] = ['values'];
  protected data_sources: Array<MatTableDataSource<Array<string>>>;
  protected selected_tab;

  constructor(public dialogRef: MatDialogRef<DatapointInfoDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {pattern: Pattern}) {

      this.pattern = data.pattern;
      this.data_sources = this.createDataSources();
      // this.onTabChange(0);
  }

  ngAfterViewInit() {
    this.data_sources = this.createDataSources();
  }

  private createDataSources(): Array<MatTableDataSource<Array<string>>>{
    let data_sources = new Array<MatTableDataSource<Array<string>>>();
    
    this.pattern.dims_values.forEach(dim => {
      let data_source: MatTableDataSource<Array<string>> = new MatTableDataSource(dim);
      data_source.paginator = this.paginator;
      data_source.sort = this.sort;

      data_sources.push(data_source);
    });

    return data_sources;
  }

  protected applyFilter(event: Event) {
    // const filterValue = (event.target as HTMLInputElement).value;
    // this.data_source.filter = filterValue.trim().toLowerCase();

    // if (this.data_source.paginator) {
    //   this.data_source.paginator.firstPage();
    // }
  }
}
