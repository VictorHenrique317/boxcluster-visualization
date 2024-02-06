import { Component, Inject, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';

@Component({
  selector: 'app-error-dialog',
  templateUrl: './error-dialog.component.html',
  styleUrls: ['./error-dialog.component.scss']
})
export class ErrorDialogComponent {
  protected error_message: string;

  constructor(public dialogRef: MatDialogRef<ErrorDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {error_message: string}) {
      this.error_message = data.error_message;
  }

}

@NgModule({
  declarations: [ErrorDialogComponent],
  imports: [
    CommonModule],
})
export class ErrorDialogComponentModule {}