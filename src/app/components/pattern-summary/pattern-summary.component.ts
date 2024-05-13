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

  protected selected_dim = 1;
  
  private input: HTMLInputElement;

  @ViewChild(MatSort) sort: MatSort;
  protected displayed_columns: string[] = ['values'];
  // protected data_source: MatTableDataSource<Array<any>>;
  protected data_source;
  protected data_source_length: number;

  constructor(private api_service: ApiService, private cdr: ChangeDetectorRef) {
    this.selected_dim = 0;
    this.data_source = new MatTableDataSource();
  }

  ngOnInit(): void {
    // this.update(1); // TODO: Retirar
  }

  protected applyFilter(event: Event) {
    this.data_source.data = this.pattern.dims_values[this.selected_dim];
    this.input = (event.target as HTMLInputElement);

    const filterValue = (event.target as HTMLInputElement).value.trim().toLowerCase();

    let filteredData = this.data_source.data.filter(item => {
        let itemStr = JSON.stringify(item).toLowerCase();
        return itemStr.includes(filterValue);
    });
    
    this.data_source.data = filteredData;
    this.data_source_length = this.data_source.data.length;
}

  public async update(identifier){
    if (this.locked){ return; }

    if(identifier == null){
      this.pattern = undefined;
      return;
    }

    this.pattern = await this.api_service.getPattern(identifier);

    this.selected_dim = 0;
    this.data_source.data = this.pattern.dims_values[this.selected_dim];
    this.data_source_length = this.data_source.data.length;
  }

  protected onSelectionChange(event: MatSelectChange){
    this.selected_dim = event.value;
    this.data_source.data = this.pattern.dims_values[this.selected_dim];
    this.data_source_length = this.data_source.data.length;

    if (this.input != null){
      this.input.value = "";
    }
  }

  public toggle(identifier: number){
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
