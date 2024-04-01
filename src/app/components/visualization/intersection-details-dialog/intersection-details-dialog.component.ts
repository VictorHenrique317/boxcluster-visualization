import { ChangeDetectorRef, Component, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import { MatTabsModule } from '@angular/material/tabs';

export interface IntersectedTuple{
    tuple: String;
    dim: String;
}

@Component({
  selector: 'app-intersection-details-dialog',
  standalone: true,
  imports: [
    CommonModule,
    MatTabsModule,
    MatSortModule,
    MatTableModule
  ],
  templateUrl: './intersection-details-dialog.component.html',
  styleUrls: ['./intersection-details-dialog.component.scss']
})
export class IntersectionDetailsDialogComponent {
  public static WIDTH = '500px';
  public static HEIGHT = '590px';
  
  protected identifier: number;
  protected total_untouched_percentage: number;
  protected total_intersection_percentage: number;
  protected intersections: Map<number, [number, Array<Array<string>>]>;

  protected intersectors_displayed_columns: string[] = ['intersections'];
  protected intersectors_data_source: MatTableDataSource<Array<number>>;

  protected intersector_displayed_columns: string[] = ['tuple', 'dim']
  protected intersector_data_source: MatTableDataSource<IntersectedTuple[]>;

  protected intersector_id: number;



  constructor(public dialogRef: MatDialogRef<IntersectionDetailsDialogComponent>, 
      @Inject(MAT_DIALOG_DATA) public data: {intersection_details: IntersectionDetails}, private cdr: ChangeDetectorRef){

    this.identifier = data.intersection_details.identifier;
    this.total_untouched_percentage = data.intersection_details.total_untouched_percentage;
    this.total_intersection_percentage = data.intersection_details.total_intersection_percentage;
    this.intersections = data.intersection_details.intersections;

    let data_source: Array<Array<number>> = Array.from(this.intersections.keys(), key => [key])
    this.intersectors_data_source = new MatTableDataSource(data_source);
  }

  ngOnInit(): void {
    console.log("Initializing intersection details dialog");

    let sorted_intersections: Map<number, [number, Array<Array<string>>]> = new Map([...this.intersections.entries()].sort((a, b) => {
      return a[1][0] - b[1][0];
    }));
    this.intersections = sorted_intersections;
  }

  protected selectIntersector(intersector_id: number){
    this.intersector_id = parseInt(intersector_id[0]);

    let intersected_dims: Array<Array<string>> = this.intersections.get(this.intersector_id)[1];

    let i = 0;
    let intersector_data_source: Array<IntersectedTuple> = [];
    intersected_dims.forEach(dim => {
      let values: Array<String> = dim.flat();
      for (let j = 0; j < values.length; j++){
        intersector_data_source.push({tuple: values[j], dim: 'DIM' + i});
      }
      i++;
    });

    this.intersector_data_source = new MatTableDataSource(Array.from(intersector_data_source, x => [x]));
  }
}
