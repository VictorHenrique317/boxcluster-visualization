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
  public static WIDTH = '45vw';
  public static HEIGHT = '50vh';

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
