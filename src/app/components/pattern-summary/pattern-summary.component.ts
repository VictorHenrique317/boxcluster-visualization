import { ChangeDetectorRef, Component, Input, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Pattern } from 'src/app/models/pattern';
import { environment } from 'src/environments/environment';
import { fs, invoke } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import {MatTabsModule} from '@angular/material/tabs';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatPaginator, MatPaginatorModule} from '@angular/material/paginator';
import { MatInputModule } from '@angular/material/input';
import {MatSelectChange, MatSelectModule} from '@angular/material/select';
import { ApiService } from 'src/app/services/api/api.service';
import { PatternDimDialogComponent } from './pattern-dim-dialog/pattern-dim-dialog.component';

const MAX_VALUE_STRING_LENGTH = 140;

@Component({
  selector: 'app-pattern-summary',
  standalone: true,
  imports: [CommonModule, MatTabsModule, MatTableModule, MatFormFieldModule, MatPaginatorModule, 
    MatInputModule, MatSelectModule],
  templateUrl: './pattern-summary.component.html',
  styleUrls: ['./pattern-summary.component.scss']
})
export class PatternSummaryComponent {
  @Input() public pattern: Pattern;
  private locked: boolean = false;
  
  private input: HTMLInputElement;

  constructor(private api_service: ApiService, private dialog_service: DialogService) {}

  ngOnInit(): void {
    this.update(1); // TODO: Retirar
  }

  protected formatDimValues(dims_values: string[]): string {
    let formated_string = dims_values.join(", ");

    if(formated_string.length > MAX_VALUE_STRING_LENGTH){
      formated_string = formated_string.slice(0, MAX_VALUE_STRING_LENGTH) + " (...)";
    }

    return formated_string;
  }

  protected openDimDialog(dim_index: number): void {
    let dialog_data = {
      dim_values: this.pattern.dims_values[dim_index]
    };

    this.dialog_service.open(PatternDimDialogComponent, 
      PatternDimDialogComponent.WIDTH, 
      PatternDimDialogComponent.HEIGHT, 
      dialog_data);
  }

  public async update(identifier){
    if (this.locked){ return; }

    if(identifier == null){
      this.pattern = undefined;
      return;
    }

    this.pattern = await this.api_service.getPattern(identifier);
  }

  public toggleLock(identifier: number){
    if(identifier == null){ // De-select current pattern
      this.locked = false;
      this.update(null);
      return;
    }

    if(identifier != this.pattern.identifier){ // Lock on another pattern
      this.locked = false;
      this.update(identifier);
    }

    this.locked = !this.locked;
  }
}
