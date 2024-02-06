import { Injectable } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { take } from 'rxjs/operators';
import { ErrorDialogComponent } from 'src/app/components/error-dialog/error-dialog.component';

@Injectable({
  providedIn: 'root'
})
export class DialogService {

  constructor(public dialog: MatDialog) { }

  public open(dialog_component, width: string, height: string, dialog_data, closeFunction=null) {
    let enterAnimationDuration = '300ms';
    let exitAnimationDuration = '300ms';

    const dialogRef = this.dialog.open(dialog_component, {
      width: width,
      height: height,
      enterAnimationDuration,
      exitAnimationDuration,
      
      data: dialog_data
    });

    dialogRef.afterClosed().pipe(take(1)).subscribe(result => {
      // Executes when the dialog is closed
      if (result) {
        if (closeFunction){
          closeFunction(result);
        }
      }
    });
  }

  public openErrorDialog(error_message: string) {
    this.open(ErrorDialogComponent, ErrorDialogComponent.WIDTH, ErrorDialogComponent.HEIGHT, 
      {error_message: error_message});
  }
}
