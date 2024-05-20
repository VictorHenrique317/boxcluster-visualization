import { ChangeDetectorRef, Component, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import { MatTabsModule } from '@angular/material/tabs';
import {MatIconModule} from '@angular/material/icon';
import { animate, state, style, transition, trigger } from '@angular/animations';

export interface IntersectedTuple {
  dim_number: String;
  dim_values_preview: Array<String>;
  dim_values: Array<String>;

  needs_expand: boolean;
}

@Component({
  selector: 'app-intersection-details-dialog',
  standalone: true,
  imports: [
    CommonModule,
    MatIconModule,
    MatTabsModule,
    MatSortModule,
    MatTableModule
  ],
  templateUrl: './intersection-details-dialog.component.html',
  styleUrls: ['./intersection-details-dialog.component.scss'],
  animations: [
    trigger('detailExpand', [
      state('collapsed,void', style({height: '0px', minHeight: '0'})),
      state('expanded', style({height: '*'})),
      transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
    ]),
  ],
})
export class IntersectionDetailsDialogComponent {
  public static WIDTH = '500px';
  // public static HEIGHT = '350px';
  public static HEIGHT = '600px';
  // public static HEIGHT = '590px'; 
  
  protected identifier: number;
  protected total_untouched_percentage: number;
  protected total_intersection_percentage: number;
  protected intersections: Map<number, [number, Array<Array<string>>]>;

  protected intersectors_displayed_columns: string[] = ['intersections'];
  protected intersectors_data_source: MatTableDataSource<Array<number>>;
  protected intersector_id: number;

  protected intersector_data_source: IntersectedTuple[];
  protected intersector_displayed_columns = ['dim_number', 'dim_values_preview'];
  protected intersector_displayed_columns_names: Map<String, String> = new Map([
    ['dim_number', 'Dim'],
    ['dim_values_preview', 'Dim preview']
  ]);
  protected intersector_displayed_columns_with_expand = [...this.intersector_displayed_columns, 'expand'];
  protected expanded_element: IntersectedTuple | null;
  private max_dim_values_preview_length = 26;
  
  // protected intersector_data_source: MatTableDataSource<IntersectedTuple[]>;
  
  // expandedElement: IntersectedTuple | null

  

  constructor(public dialogRef: MatDialogRef<IntersectionDetailsDialogComponent>, 
      @Inject(MAT_DIALOG_DATA) public data: {intersection_details: IntersectionDetails}, private cdr: ChangeDetectorRef){

    this.identifier = data.intersection_details.identifier;
    this.total_untouched_percentage = data.intersection_details.total_untouched_percentage;
    this.total_intersection_percentage = data.intersection_details.total_intersection_percentage;

    let sorted_intersections: Map<number, [number, Array<Array<string>>]> = new Map([...data.intersection_details.intersections.entries()]
    .sort((a, b) => {
      return a[1][0] - b[1][0];
    }));
    this.intersections = sorted_intersections;

    let data_source: Array<Array<number>> = Array.from(this.intersections.keys(), key => [key])
    this.intersectors_data_source = new MatTableDataSource(data_source);
  }

  ngOnInit(): void { 
    console.log("Initializing intersection details dialog");
  }

  ngAfterViewInit(){
    let first_intersector = this.intersections.keys().next().value;
    this.selectIntersector(first_intersector); // Selects the first intersector
    this.cdr.detectChanges();
  }

  protected trackColumn(index: number, column: string): any {
    return column;
  }

  protected getColumnName(column: String): String {
    return this.intersector_displayed_columns_names.get(column);
  }

  protected selectIntersector(intersector_id: number){
    this.intersector_id = intersector_id;

    console.log(this.intersections.get(this.intersector_id));
    let intersected_dims: Array<Array<string>> = this.intersections.get(this.intersector_id)[1];

    let i = 0;
    let intersector_data_source: IntersectedTuple[] = [];
    intersected_dims.forEach(dim => {
      let values: Array<String> = [];
      let all_values_length = 0;

      dim.flat().forEach(value => {
        all_values_length += value.length;
        all_values_length += 1; // For the comma
        all_values_length += 1; // For the space
        values.push(" " + value);
      });

      let needs_expand: boolean;
      let dim_values_preview: Array<String> = [];
      if(all_values_length <= this.max_dim_values_preview_length - 2){ // -2 for the last comma and space
        dim_values_preview = values;
        needs_expand = false;
      }else{
        dim_values_preview.push("{" + values.length + " elements...}");
        needs_expand = true;
      }

      intersector_data_source.push(
        {dim_number: 'DIM' + (i+1), dim_values_preview: dim_values_preview, dim_values: values, needs_expand: needs_expand}
        );
      i++;
    });

    this.intersector_data_source = intersector_data_source;
  }

  protected expandRow(element: IntersectedTuple): void {
    if(element.needs_expand === false){
      this.expanded_element = null;
      return;
    }

    this.expanded_element = this.expanded_element === element ? null : element;
  }
  
}