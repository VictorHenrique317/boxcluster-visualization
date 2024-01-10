import { open } from '@tauri-apps/api/dialog';
import {Component, EventEmitter, Inject, NgModule, Output} from '@angular/core';
import {MatDialogRef, MatDialogModule} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-file-selection-dialog',
  templateUrl: './file-selection-dialog.component.html',
  styleUrls: ['./file-selection-dialog.component.scss']
})
export class FileSelectionDialogComponent {
  @Output() modelChange: EventEmitter<any> = new EventEmitter();
  
  private tensor_path: string = "";
  protected tensor_name: string = "";

  private patterns_path: string = "";
  protected patterns_name: string = "";

  constructor(public dialogRef: MatDialogRef<FileSelectionDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {tensor_path: string, patterns_path: string}) {
      this.tensor_path = data.tensor_path;
      this.patterns_path = data.patterns_path;
      this.setNames();
  }

  private isStateValid(): boolean{
    if(this.tensor_path == undefined || this.tensor_path == null || this.tensor_path == ""){
      return false;
    }

    if(this.patterns_path == undefined || this.patterns_path == null || this.patterns_path == ""){
      return false;
    }

    return true;
  }

  private setNames(){
    this.tensor_name = this.tensor_path.split('\\').pop().split('/').pop();
    this.patterns_name = this.patterns_path.split('\\').pop().split('/').pop();
  }

  public async selectTensor(){
    const options = {
      multiple: false
    };
    const selected = await open(options);
    if (selected === null) { return; } // No tensor selected
    
    this.tensor_path = selected.toString();
    this.setNames();
    if (this.tensor_path == ""){ return; } // No tensor selected
}

  public async selectPatterns(){
    const options = {
      multiple: false
    };
    const selected = await open(options);
    if (selected === null) { return; } // No patterns selected
    
    this.patterns_path = selected.toString();
    this.setNames();
    if (this.patterns_path == ""){ return; } // No patterns selected
  }

  protected submit() {
    if (this.isStateValid()){
      this.dialogRef.close({tensor_path: this.tensor_path, patterns_path: this.patterns_path});
    }else{
      this.dialogRef.close({tensor_path: null, patterns_path: null});
    }
   }
}

@NgModule({
  declarations: [FileSelectionDialogComponent],
  imports: [
    MatButtonModule, 
    MatDialogModule,
    MatIconModule],
})
export class FileSelectionDialogComponentModule {}