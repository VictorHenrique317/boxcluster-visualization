import { Injectable } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { take } from 'rxjs/operators';
import { ErrorDialogComponent } from 'src/app/components/error-dialog/error-dialog.component';

@Injectable({
  providedIn: 'root'
})
export class ErrorService {

  constructor(public error_dialog: MatDialog) { }

  public openErrorDialog(error_message: string) {
    let enterAnimationDuration = '300ms';
    let exitAnimationDuration = '300ms';

    const dialogRef = this.error_dialog.open(ErrorDialogComponent, {
      width: '500px',
      height: '400px',
      enterAnimationDuration,
      exitAnimationDuration,
      
      data: {
        error_message: error_message,
      }
    });

    dialogRef.afterClosed().pipe(take(1)).subscribe(result => {
      // Executes when the dialog is closed
      if (result) {  }
    });
  }
}
