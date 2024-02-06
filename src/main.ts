import { enableProdMode, importProvidersFrom } from "@angular/core";
import { bootstrapApplication } from "@angular/platform-browser";
import { AppComponent } from "./app/app.component";

import { environment } from "./environments/environment";
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import {routes} from './routes';
import { provideRouter } from "@angular/router";
import { MatDialogModule } from "@angular/material/dialog";

if (environment.production) {
  enableProdMode();
}

bootstrapApplication(AppComponent, {
    providers: [importProvidersFrom(
      BrowserAnimationsModule, MatDialogModule), 
      provideRouter(routes),]
}).catch((err) => console.error(err));
