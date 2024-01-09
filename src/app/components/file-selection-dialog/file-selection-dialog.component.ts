import { open } from '@tauri-apps/api/dialog';
import {Component, EventEmitter, NgModule, Output} from '@angular/core';
import {MatDialogRef, MatDialogModule} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';

@Component({
  selector: 'app-file-selection-dialog',
  templateUrl: './file-selection-dialog.component.html',
  styleUrls: ['./file-selection-dialog.component.scss']
})
export class FileSelectionDialogComponent {
  @Output() modelChange: EventEmitter<any> = new EventEmitter();
  
  private tensor_path: string = "";
  private tensor_name: string = "";

  private patterns_path: string = "";
  private patterns_name: string = "";

  constructor(public dialogRef: MatDialogRef<FileSelectionDialogComponent>) {}

  private isStateValid(): boolean{
    if(this.tensor_path == undefined || this.tensor_path == null || this.tensor_path == ""){
      return false;
    }

    if(this.patterns_path == undefined || this.patterns_path == null || this.patterns_path == ""){
      return false;
    }

    return true;
  }

  public async selectTensor(){
    const options = {
      multiple: false
    };
    const selected = await open(options);
    if (selected === null) { return; } // No tensor selected
    
    this.tensor_path = selected.toString();
    this.tensor_name = this.tensor_path.split('\\').pop().split('/').pop();
    if (this.tensor_path == ""){ return; } // No tensor selected
}

  public async selectPatterns(){
    const options = {
      multiple: false
    };
    const selected = await open(options);
    if (selected === null) { return; } // No patterns selected
    
    this.patterns_path = selected.toString();
    this.patterns_name = this.patterns_path.split('\\').pop().split('/').pop();
    if (this.patterns_path == ""){ return; } // No patterns selected
    
    // if (this.tensor_path != "" && this.patterns_path != ""){
    //   // Both are defined
    //   invoke("initApplication", {tensorPath: this.tensor_path, patternsPath: this.patterns_path}).then((result: any) =>{
    //     this.upload_file_mode = "tensor";
    //     this.model_loaded = true;

    //     // Forcing a reload
    //     this.router.navigateByUrl('', {skipLocationChange: true}).then(()=>{
    //       this.openDagView();
    //     });

    //   }).catch((error: any) => {
    //     console.log(error);
    //   });
    // }
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