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

  protected selected_input: number;
  protected filtered_values:string[]; 

  private previous_filters: string[][];
  
  protected nb_of_dims: number[];
  protected dims_values: string[][];
  protected selectedValues: string[][];
  private any_value_mask: boolean[];

  protected displayedColumns: string[];

  constructor(public dialogRef: MatDialogRef<SearchDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {previous_filters: string[][]}, private api_service: ApiService) {
      this.loadData(data.previous_filters);
  }

  protected resetFilteredValues(dim_index: number){
    this.selected_input = dim_index;
    this.filtered_values = this.dims_values[dim_index];
  }

  protected onKey(event: KeyboardEvent) { 
    const inputValue = (event.target as HTMLInputElement).value;
    this.filtered_values = this.search(inputValue);
  }

  protected search(value: string) { 
    let filter = value.toLowerCase();
    return this.dims_values[this.selected_input].filter(option => option.toLowerCase().includes(filter));
  }

  private async loadData(previous_filters: string[][]){
    this.previous_filters = previous_filters;
    this.dims_values = await this.api_service.getAllDimsValues();
    this.nb_of_dims =  Array(this.dims_values.length).fill(0).map((_, i) => i);
    this.displayedColumns = this.nb_of_dims.map(i => 'dim' + (i + 1));
    this.resetSelectedValues();

    this.any_value_mask = [];
    if (this.previous_filters){
      this.previous_filters.forEach((filter, i) => {
        this.selectedValues[i] = filter;
        
        if(filter.includes("Any value")){
          this.any_value_mask[i] = true;
        }else{
          this.any_value_mask[i] = false;
        }
      });
    }
  }

  protected onSelectionChange(value, dim_index){
    if (value == "Any value"){
      this.selectedValues[dim_index] = [];
      this.any_value_mask[dim_index] = true;
      this.selectedValues[dim_index].push(value);
      return;
    }

    if(!this.any_value_mask[dim_index]){ // Only adds the value if the "Any value" option is not selected
      this.selectedValues[dim_index].push(value);
    }
  }

  protected deleteValue(dim_index: number, value_index: number){
    if(this.selectedValues[dim_index][value_index] == "Any value"){
      this.any_value_mask[dim_index] = false;
    }

    this.selectedValues[dim_index].splice(value_index, 1);
  }

  protected clearFilters(){
    this.resetSelectedValues();
    this.dims_values.forEach((_, i) => {
      this.selectedValues[i] = ["Any value"];
      this.any_value_mask[i] = true;
    });
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