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

@Component({
  selector: 'app-pattern-summary',
  standalone: true,
  imports: [CommonModule, MatTabsModule, MatTableModule, MatFormFieldModule],
  templateUrl: './pattern-summary.component.html',
  styleUrls: ['./pattern-summary.component.scss']
})
export class PatternSummaryComponent {
  @Input() public pattern: Pattern;

  @ViewChild(MatSort) sort: MatSort;
  private input: HTMLInputElement;
  
  protected displayed_columns: string[] = ['values'];
  // protected data_source: MatTableDataSource<Array<any>>;
  protected data_source;
  protected data_source_length: number;
  protected selected_tab;

  constructor(private cdr: ChangeDetectorRef, private dialog_service: DialogService) {
    this.selected_tab = 0;
  }

  ngOnInit(): void {
    this.update(1); // TODO: Retirar
  }

  public async update(identifier){
    if(identifier == null){
      this.pattern = undefined;
      return;
    }
    
    let pattern;
    if(!environment.dev_mode){
      pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
        console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching pattern.");
      });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/pattern.json'));
      pattern = JSON.parse(rawdata);
    }
    this.pattern = Pattern.fromResponse(pattern);
    
    this.data_source = new MatTableDataSource(this.pattern.dims_values[this.selected_tab]);
    this.data_source_length = this.data_source.data.length;
  }

  protected applyFilter(event: Event) {
    // this.data_source.data = this.pattern.dims_values[this.selected_tab];
    // this.input = (event.target as HTMLInputElement);

    // const filterValue = (event.target as HTMLInputElement).value.trim().toLowerCase();

    // let filteredData = this.data_source.data.filter(item => {
    //     let itemStr = JSON.stringify(item).toLowerCase();
    //     return itemStr.includes(filterValue);
    // });
    
    // this.data_source.data = filteredData;
    // this.data_source_length = this.data_source.data.length;
}

  protected onTabChange(tab_index: number){
    // this.selected_tab = tab_index;
    // let dim = this.pattern.dims_values[tab_index];
    // this.data_source.data = dim;
    // this.data_source_length = this.data_source.data.length;
    // this.data_source.sort = this.sort;
    
    // if (this.input != null){
    //   this.input.value = "";
    // }
  }
}
