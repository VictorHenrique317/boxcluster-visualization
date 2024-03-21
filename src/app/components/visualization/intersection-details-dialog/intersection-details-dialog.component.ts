import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatDialogRef } from '@angular/material/dialog';

@Component({
  selector: 'app-intersection-details-dialog',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './intersection-details-dialog.component.html',
  styleUrls: ['./intersection-details-dialog.component.scss']
})
export class IntersectionDetailsDialogComponent {
  // constructor(public dialogRef: MatDialogRef<IntersectionDetailsDialogComponent>, 
  //   @Inject(MAT_DIALOG_DATA) public data: {pattern: Pattern},
  //   private cdr: ChangeDetectorRef){}
}
