import { Component, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { ApiService } from 'src/app/services/api/api.service';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-search-dialog',
  standalone: true,
  imports: [CommonModule, MatFormFieldModule, MatSelectModule, MatTableModule, FormsModule],
  templateUrl: './search-dialog.component.html',
  styleUrls: ['./search-dialog.component.scss']
})
export class SearchDialogComponent {
  public static WIDTH = '45vw';
  public static HEIGHT = '50vh';

  protected nb_of_dims: number[];
  protected dims_values: string[][];
  protected selectedValues: string[][];

  protected displayedColumns: string[];

  constructor(public dialogRef: MatDialogRef<SearchDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {}, private api_service: ApiService) {
        this.loadData();
  }

  private async loadData(){
    this.dims_values = await this.api_service.getDimsValues();
    this.nb_of_dims =  Array(this.dims_values.length).fill(0).map((_, i) => i);
    this.displayedColumns = this.nb_of_dims.map(i => 'dim' + (i + 1));
    this.resetSelectedValues();
  }

  protected onSelectionChange(){
    this.selectedValues = this.selectedValues.map(value => value);
  }

  protected ok(): Array<Array<string>>{
    this.dialogRef.close(this.selectedValues);
    return this.selectedValues; // Return the selected values
  }

  private resetSelectedValues(){
    this.selectedValues = [this.dims_values.map(values => "")];
  }
}