import { Component, Inject, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';

@Component({
  selector: 'app-error-dialog',
  templateUrl: './error-dialog.component.html',
  styleUrls: ['./error-dialog.component.scss']
})
export class ErrorDialogComponent {
  public static WIDTH = '400px';
  public static HEIGHT = '250px';

  protected error_message: string;
  
  constructor(public dialogRef: MatDialogRef<ErrorDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {error_message: string}) {
      this.error_message = data.error_message;
  }

  protected submit(){
    this.dialogRef.close();
  }

}

@NgModule({
  declarations: [ErrorDialogComponent],
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule
  ],
})
export class ErrorDialogComponentModule {}