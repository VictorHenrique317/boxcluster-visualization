import { Component, Inject } from '@angular/core';
import { AsyncPipe, CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { ApiService } from 'src/app/services/api/api.service';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';
import {MatAutocompleteModule} from '@angular/material/autocomplete';

@Component({
  selector: 'app-search-dialog',
  standalone: true,
  imports: [CommonModule, MatFormFieldModule, MatSelectModule, MatTableModule, FormsModule, MatIconModule, MatTooltipModule,
            FormsModule, MatAutocompleteModule,ReactiveFormsModule, AsyncPipe],
  templateUrl: './search-dialog.component.html',
  styleUrls: ['./search-dialog.component.scss']
})
export class SearchDialogComponent {
  public static WIDTH = '60vw';
  public static HEIGHT = '70vh';

  private previous_filters: string[][];
  
  protected nb_of_dims: number[];
  protected dims_values: string[][];
  protected selectedValues: string[][];

  protected displayedColumns: string[];

  constructor(public dialogRef: MatDialogRef<SearchDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {previous_filters: string[][]}, private api_service: ApiService) {
      this.previous_filters = data.previous_filters;
      this.loadData();
  }

  private async loadData(){
    this.dims_values = await this.api_service.getAllDimsValues();
    this.nb_of_dims =  Array(this.dims_values.length).fill(0).map((_, i) => i);
    this.displayedColumns = this.nb_of_dims.map(i => 'dim' + (i + 1));
    this.resetSelectedValues();

    if (this.previous_filters){
      this.previous_filters.forEach((filter, i) => {
        this.selectedValues[i] = filter;
      });
    }
  }

  protected onSelectionChange(value, dim_index){
    this.selectedValues[dim_index].push(value);
  }

  protected deleteValue(dim_index: number, value_index: number){
    this.selectedValues[dim_index].splice(value_index, 1);
  }

  protected clearFilters(){
    this.resetSelectedValues();
  }

  protected close(){
    this.dialogRef.close();
  }

  protected ok(): Array<Array<string>>{
    this.dialogRef.close(this.selectedValues);
    return this.selectedValues; // Return the selected values
  }

  private resetSelectedValues(){
    this.selectedValues = [];
    this.nb_of_dims.forEach(i => this.selectedValues.push([]));
  }
}