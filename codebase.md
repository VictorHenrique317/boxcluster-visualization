### DIRECTORY src/ FOLDER STRUCTURE ###
/
    main.ts
    polyfills.ts
    style.scss
    index.html
    routes.ts
    theme.scss
assets/
    angular.svg
    tauri.svg
    js/
        mds.js
app/
    app.component.scss
    app.component.ts
    app.component.html
    components/
        visualization/
            visualization.component.scss
            visualization.component.ts
            svg-feature.module.ts
            intersection-mode-feature.module.ts
            visualization.component.html
            visualization.component.spec.ts
            intersection-details-dialog/
                intersection-details-dialog.component.spec.ts
                intersection-details-dialog.component.scss
                intersection-details-dialog.component.ts
                intersection-details-dialog.component.html
            datapoint-tooltip/
                datapoint-tooltip.component.scss
                datapoint-tooltip.component.ts
                datapoint-tooltip.component.html
                datapoint-tooltip.component.spec.ts
        main_options/
            rss-view/
                rss-view.component.html
                rss-view.component.scss
                rss-view.component.spec.ts
                rss-view.component.ts
            file-selection-dialog/
                file-selection-dialog.component.ts
                file-selection-dialog.component.html
                file-selection-dialog.component.spec.ts
                file-selection-dialog.component.scss
        pattern-summary/
            pattern-summary.component.html
            pattern-summary.component.spec.ts
            pattern-summary.component.scss
            pattern-summary.component.ts
            pattern-dim-dialog/
                pattern-dim-dialog.component.ts
                pattern-dim-dialog.component.scss
                pattern-dim-dialog.component.html
                pattern-dim-dialog.component.spec.ts
        error-dialog/
            error-dialog.component.scss
            error-dialog.component.ts
            error-dialog.component.html
            error-dialog.component.spec.ts
    models/
        color.ts
        intersection_details.ts
        pattern.ts
        svg.ts
        datapoint.ts
    services/
        svg/
            svg.service.spec.ts
            svg.service.ts
        dialog/
            dialog.service.ts
            dialog.service.spec.ts
        api/
            api.service.spec.ts
            api.service.ts
environments/
    environment.ts
    environment.prod.ts
js/
    circle_legend.d.ts
    color_legend.js
    color_legend.d.ts
    circle_legend.js
### DIRECTORY src/ FOLDER STRUCTURE ###

### DIRECTORY src/ FLATTENED CONTENT ###
### src/main.ts BEGIN ###
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

### src/main.ts END ###

### src/polyfills.ts BEGIN ###
/**
 * This file includes polyfills needed by Angular and is loaded before the app.
 * You can add your own extra polyfills to this file.
 *
 * This file is divided into 2 sections:
 *   1. Browser polyfills. These are applied before loading ZoneJS and are sorted by browsers.
 *   2. Application imports. Files imported after ZoneJS that should be loaded before your main
 *      file.
 *
 * The current setup is for so-called "evergreen" browsers; the last versions of browsers that
 * automatically update themselves. This includes recent versions of Safari, Chrome (including
 * Opera), Edge on the desktop, and iOS and Chrome on mobile.
 *
 * Learn more in https://angular.io/guide/browser-support
 */

/***************************************************************************************************
 * BROWSER POLYFILLS
 */

/**
 * By default, zone.js will patch all possible macroTask and DomEvents
 * user can disable parts of macroTask/DomEvents patch by setting following flags
 * because those flags need to be set before `zone.js` being loaded, and webpack
 * will put import in the top of bundle, so user need to create a separate file
 * in this directory (for example: zone-flags.ts), and put the following flags
 * into that file, and then add the following code before importing zone.js.
 * import './zone-flags';
 *
 * The flags allowed in zone-flags.ts are listed here.
 *
 * The following flags will work for all browsers.
 *
 * (window as any).__Zone_disable_requestAnimationFrame = true; // disable patch requestAnimationFrame
 * (window as any).__Zone_disable_on_property = true; // disable patch onProperty such as onclick
 * (window as any).__zone_symbol__UNPATCHED_EVENTS = ['scroll', 'mousemove']; // disable patch specified eventNames
 *
 *  in IE/Edge developer tools, the addEventListener will also be wrapped by zone.js
 *  with the following flag, it will bypass `zone.js` patch for IE/Edge
 *
 *  (window as any).__Zone_enable_cross_context_check = true;
 *
 */

/***************************************************************************************************
 * Zone JS is required by default for Angular itself.
 */
import "zone.js"; // Included with Angular CLI.

/***************************************************************************************************
 * APPLICATION IMPORTS
 */

### src/polyfills.ts END ###

### src/style.scss BEGIN ###

// Custom Theming for Angular Material
// For more information: https://material.angular.io/guide/theming
@use '@angular/material' as mat;
// Plus imports for other components in your app.

// Include the common styles for Angular Material. We include this here so that you only
// have to load a single css file for Angular Material in your app.
// Be sure that you only ever include this mixin once!
@include mat.core();

// Include theme styles for core and each component used in your app.
// Alternatively, you can import and @include the theme mixins for each component
// that you are using.

@import 'theme.scss';

@include mat.all-component-themes($theme);

body{
  margin: 0 0 0 0;
  app-root{
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
  }
}
### src/style.scss END ###

### src/index.html BEGIN ###
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title> Boxcluster Visualization </title>
    <base href="/" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="icon" type="image/x-icon" href="favicon.ico" />
      <link rel="preconnect" href="https://fonts.gstatic.com">
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500&display=swap" rel="stylesheet">
    <link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">
</head>

  <body class="mat-typography">
    <app-root></app-root>
  </body>
</html>

### src/index.html END ###

### src/routes.ts BEGIN ###
import { Route } from '@angular/router';
import { VisualizationComponent } from './app/components/visualization/visualization.component';
import { RssViewComponent } from './app/components/main_options/rss-view/rss-view.component';

export const routes: Route[] = [
  // {
  //   path: '',
  //   component: DagComponent,
  // },
  {
    path: 'visualizationView',
    component: VisualizationComponent,
  },
  {
    path: 'rssView',
    component: RssViewComponent,
  },
  {
    path: '**',
    redirectTo: '',
  },
];
### src/routes.ts END ###

### src/theme.scss BEGIN ###
$deep-blue-palette: (
 50: #deefff,
 100: #bcd9f4,
 200: #9fbddc,
 300: #7fa1c4,
 400: #678cb2,
 500: #234768,
 600: #345d81,
 700: #0f3553,
 800: #0d3250,
 900: #031f33,

 contrast: (
   50: black,
   100: rgb(0, 0, 0),
   200: rgb(27, 27, 27),
   300: rgb(53, 53, 53),
   400: rgb(80, 80, 80),
   500: rgb(110, 110, 110),
   600: rgb(150, 150, 150),
   700: rgb(184, 184, 184),
   800: rgb(216, 216, 216),
   900: white,
 )
);

// (50: #deefff, 100: #bcd9f4, 200: #9fbddc, 300: #7fa1c4, 400: #678cb2, 500: #4e79a0, 600: #3f6a8f, 700: #305778, 800: #214461, 900: #0c3049, contrast: (50: rgba(0, 0, 0, 0.87), 100: rgba(0, 0, 0, 0.87), 200: rgba(0, 0, 0, 0.87), 300: rgba(0, 0, 0, 0.87), 400: rgba(0, 0, 0, 0.87), 500: rgba(0, 0, 0, 0.87), 600: white, 700: white, 800: white, 900: white), default: #4e79a0, lighter: #bcd9f4, darker: #305778, text: #4e79a0, default-contrast: rgba(0, 0, 0, 0.87), lighter-contrast: rgba(0, 0, 0, 0.87), darker-contrast: white, "50-contrast": rgba(0, 0, 0, 0.87), "100-contrast": rgba(0, 
// 0, 0, 0.87), "200-contrast": rgba(0, 0, 0, 0.87), "300-contrast": rgba(0, 0, 0, 0.87), "400-contrast": rgba(0, 0, 0, 0.87), "500-contrast": rgba(0, 0, 0, 0.87), "600-contrast": white, "700-contrast": white, "800-contrast": white, "900-contrast": white, "contrast-contrast": null)

$primary-palette: mat.define-palette($deep-blue-palette);
$accent-palette: mat.define-palette(mat.$blue-palette, A200, A100, A400);
$warn-palette: mat.define-palette(mat.$red-palette);

// background-color: mat.get-color-from-palette($primary-palette, 900);

// Create the theme object. A theme consists of configurations for individual
// theming systems such as "color" or "typography".
$theme: mat.define-light-theme((
  color: (
    primary: $primary-palette,
    accent: $accent-palette,
    warn: $warn-palette,
  )
));

$palette-color: map-get($theme, color);
$background: map-get($palette-color, background);
$background-color: map-get($background, background);

.mdc-list-item:hover .mdc-list-item__primary-text {
  color: white !important;
}
### src/theme.scss END ###

### src/assets/angular.svg BEGIN ###
<?xml version="1.0" encoding="utf-8"?>
<!-- Generator: Adobe Illustrator 19.1.0, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
<svg version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
	 viewBox="0 0 250 250" style="enable-background:new 0 0 250 250;" xml:space="preserve">
<style type="text/css">
	.st0{fill:#DD0031;}
	.st1{fill:#C3002F;}
	.st2{fill:#FFFFFF;}
</style>
<g>
	<polygon class="st0" points="125,30 125,30 125,30 31.9,63.2 46.1,186.3 125,230 125,230 125,230 203.9,186.3 218.1,63.2 	"/>
	<polygon class="st1" points="125,30 125,52.2 125,52.1 125,153.4 125,153.4 125,230 125,230 203.9,186.3 218.1,63.2 125,30 	"/>
	<path class="st2" d="M125,52.1L66.8,182.6h0h21.7h0l11.7-29.2h49.4l11.7,29.2h0h21.7h0L125,52.1L125,52.1L125,52.1L125,52.1
		L125,52.1z M142,135.4H108l17-40.9L142,135.4z"/>
</g>
</svg>

### src/assets/angular.svg END ###

### src/assets/tauri.svg BEGIN ###
<svg width="206" height="231" viewBox="0 0 206 231" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M143.143 84C143.143 96.1503 133.293 106 121.143 106C108.992 106 99.1426 96.1503 99.1426 84C99.1426 71.8497 108.992 62 121.143 62C133.293 62 143.143 71.8497 143.143 84Z" fill="#FFC131"/>
<ellipse cx="84.1426" cy="147" rx="22" ry="22" transform="rotate(180 84.1426 147)" fill="#24C8DB"/>
<path fill-rule="evenodd" clip-rule="evenodd" d="M166.738 154.548C157.86 160.286 148.023 164.269 137.757 166.341C139.858 160.282 141 153.774 141 147C141 144.543 140.85 142.121 140.558 139.743C144.975 138.204 149.215 136.139 153.183 133.575C162.73 127.404 170.292 118.608 174.961 108.244C179.63 97.8797 181.207 86.3876 179.502 75.1487C177.798 63.9098 172.884 53.4021 165.352 44.8883C157.82 36.3744 147.99 30.2165 137.042 27.1546C126.095 24.0926 114.496 24.2568 103.64 27.6274C92.7839 30.998 83.1319 37.4317 75.8437 46.1553C74.9102 47.2727 74.0206 48.4216 73.176 49.5993C61.9292 50.8488 51.0363 54.0318 40.9629 58.9556C44.2417 48.4586 49.5653 38.6591 56.679 30.1442C67.0505 17.7298 80.7861 8.57426 96.2354 3.77762C111.685 -1.01901 128.19 -1.25267 143.769 3.10474C159.348 7.46215 173.337 16.2252 184.056 28.3411C194.775 40.457 201.767 55.4101 204.193 71.404C206.619 87.3978 204.374 103.752 197.73 118.501C191.086 133.25 180.324 145.767 166.738 154.548ZM41.9631 74.275L62.5557 76.8042C63.0459 72.813 63.9401 68.9018 65.2138 65.1274C57.0465 67.0016 49.2088 70.087 41.9631 74.275Z" fill="#FFC131"/>
<path fill-rule="evenodd" clip-rule="evenodd" d="M38.4045 76.4519C47.3493 70.6709 57.2677 66.6712 67.6171 64.6132C65.2774 70.9669 64 77.8343 64 85.0001C64 87.1434 64.1143 89.26 64.3371 91.3442C60.0093 92.8732 55.8533 94.9092 51.9599 97.4256C42.4128 103.596 34.8505 112.392 30.1816 122.756C25.5126 133.12 23.9357 144.612 25.6403 155.851C27.3449 167.09 32.2584 177.598 39.7906 186.112C47.3227 194.626 57.153 200.784 68.1003 203.846C79.0476 206.907 90.6462 206.743 101.502 203.373C112.359 200.002 122.011 193.568 129.299 184.845C130.237 183.722 131.131 182.567 131.979 181.383C143.235 180.114 154.132 176.91 164.205 171.962C160.929 182.49 155.596 192.319 148.464 200.856C138.092 213.27 124.357 222.426 108.907 227.222C93.458 232.019 76.9524 232.253 61.3736 227.895C45.7948 223.538 31.8055 214.775 21.0867 202.659C10.3679 190.543 3.37557 175.59 0.949823 159.596C-1.47592 143.602 0.768139 127.248 7.41237 112.499C14.0566 97.7497 24.8183 85.2327 38.4045 76.4519ZM163.062 156.711L163.062 156.711C162.954 156.773 162.846 156.835 162.738 156.897C162.846 156.835 162.954 156.773 163.062 156.711Z" fill="#24C8DB"/>
</svg>

### src/assets/tauri.svg END ###

### src/assets/js/mds.js BEGIN ###
var numeric = require('numeric');

(function(mds) {
    "use strict";
    /// given a matrix of distances between some points, returns the
    /// point coordinates that best approximate the distances using
    /// classic multidimensional scaling
    mds.classic = function(distances, dimensions) {
        dimensions = dimensions || 2;

        // square distances
        var M = numeric.mul(-0.5, numeric.pow(distances, 2));

        // double centre the rows/columns
        function mean(A) { return numeric.div(numeric.add.apply(null, A), A.length); }
        var rowMeans = mean(M),
            colMeans = mean(numeric.transpose(M)),
            totalMean = mean(rowMeans);

        for (var i = 0; i < M.length; ++i) {
            for (var j =0; j < M[0].length; ++j) {
                M[i][j] += totalMean - rowMeans[i] - colMeans[j];
            }
        }

        // take the SVD of the double centred matrix, and return the
        // points from it
        var ret = numeric.svd(M),
            eigenValues = numeric.sqrt(ret.S);
        return ret.U.map(function(row) {
            return numeric.mul(row, eigenValues).splice(0, dimensions);
        });
    };

    /// draws a scatter plot of points, useful for displaying the output
    /// from mds.classic etc
    mds.drawD3ScatterPlot = function(element, xPos, yPos, labels, params) {
        params = params || {};
        var padding = params.padding || 32,
            w = params.w || Math.min(720, document.documentElement.clientWidth - padding),
            h = params.h || w,
            xDomain = [Math.min.apply(null, xPos),
                       Math.max.apply(null, xPos)],
            yDomain = [Math.max.apply(null, yPos),
                       Math.min.apply(null, yPos)],
            pointRadius = params.pointRadius || 3;

        if (params.reverseX) {
            xDomain.reverse();
        }
        if (params.reverseY) {
            yDomain.reverse();
        }

        var xScale = d3.scale.linear().
                domain(xDomain)
                .range([padding, w - padding]),

            yScale = d3.scale.linear().
                domain(yDomain)
                .range([padding, h-padding]),

            xAxis = d3.svg.axis()
                .scale(xScale)
                .orient("bottom")
                .ticks(params.xTicks || 7),

            yAxis = d3.svg.axis()
                .scale(yScale)
                .orient("left")
                .ticks(params.yTicks || 7);

        var svg = element.append("svg")
                .attr("width", w)
                .attr("height", h);

        svg.append("g")
            .attr("class", "axis")
            .attr("transform", "translate(0," + (h - padding + 2*pointRadius) + ")")
            .call(xAxis);

        svg.append("g")
            .attr("class", "axis")
            .attr("transform", "translate(" + (padding - 2*pointRadius) + ",0)")
            .call(yAxis);

        var nodes = svg.selectAll("circle")
            .data(labels)
            .enter()
            .append("g");
        
        nodes.append("circle")
            .attr("r", pointRadius)
            .attr("cx", function(d, i) { return xScale(xPos[i]); })
            .attr("cy", function(d, i) { return yScale(yPos[i]); });

        nodes.append("text")
            .attr("text-anchor", "middle")
            .text(function(d) { return d; })
            .attr("x", function(d, i) { return xScale(xPos[i]); })
            .attr("y", function(d, i) { return yScale(yPos[i]) - 2 *pointRadius; });
    };
}(window.mds = window.mds || {}));


### src/assets/js/mds.js END ###

### src/app/app.component.scss BEGIN ###
@use '@angular/material' as mat;
// @import '../style.scss';
@import '../theme.scss';

// $primary-palette: map-get($map: , $key: )

$max_nav_height: 5vh;

body{
    // $background: map-get($theme, background);
    overflow: hidden;
    background-color: $background-color;
    // position: absolute;
    z-index: 1;
    // top: 0px;
    // right: 0;
    // left: 0;
    // bottom: 0px;
    display: flex;
    flex-direction: row;

    width: 100vw;
    height: 100vh;

    aside{
        z-index: 2;
        background-color: mat.get-color-from-palette($primary-palette, 900);
        color: mat.get-color-from-palette($primary-palette, '900-contrast');
        // flex: 1;
        display: flex;
        flex-direction: column;

        width: 25vw;
        max-height: 100vh;
        min-height: 100vh;

        // background-color: green;

        #main-options{
            display: flex;
            flex-direction: row;
            justify-content: space-around;
            // background-color: brown;

            padding-top: 0.5em;
            padding-bottom: 0.5em;
            padding-left: 0.5em;
            padding-right: 0.5em;

            height: 10vh;
            
            #matlist-placeholder{
                min-height: $max_nav_height;
                max-height: $max_nav_height;
                padding-top: 1em;
                padding-bottom: 1em;
                padding-left: 1em;
                padding-right: 1em;
            }

            .main-option{
                min-height: $max_nav_height;
                max-height: $max_nav_height;
                padding-top: 1em;
                padding-bottom: 1em;
                padding-left: 1em;
                padding-right: 1em;
    
                user-select: none;
    
                display: flex;
                flex-direction: row;
                align-items: center;
    
                border-radius: 10px;

                background-color: mat.get-color-from-palette($primary-palette, 900);
    
                mat-icon{
                    font-size: 2.5em;
                    width: 1em;
                    height: 1em;
                }
    
                h1{
                    max-width: 80%;
    
                    margin: 0 0 0 0;
    
                    padding-top: 0.5em;
                    padding-bottom: 0.5em;
                    padding-left: 0.5em;
                    padding-right: 0.5em;
                    
                    word-wrap: break-word;
                    font-size: 1.25em;
                }
            }
    
            .main-option:hover{
                background-color: mat.get-color-from-palette($primary-palette, 800);
                cursor: pointer;
            }

            #truncate-model-button{
                mat-icon{
                    transform: scaleX(-1);
                }
            }
        }

        #content-cover{
            width: 25vw;
            height: 90vh;
            
            position: absolute;
            top: 10vh;

            z-index: 2;

            background-color: rgba(0, 0, 0, 1); 
        }

        .sidenav-content{
            height: 90vh;
            background-color: mat.get-color-from-palette($primary-palette, 900);

            #settings-wrapper-lower-content{
                width: 100%;
                height: 100%;
            }

            #settings-wrapper-upper-content{
                background-color: mat.get-color-from-palette($primary-palette, 900);
                width: 80%;

                nav{
                    display: flex;
                    flex-direction: column;

                    .setting{
                        color: white;
                        background-color: mat.get-color-from-palette($primary-palette, 800);

                        margin: 0.5em 1em 0.5em 1em;
                    }
                }
            }

            #pattern-summary{
                width: 100%;
                height: 100%;
                color: white;
            }

            app-dynamic-paginator{
                flex: 4;
            }
        }

        h2{
            padding-left: 1em;
            padding-top: 1em;
        }
    }

    #main-app{
        // background-color: green;
        width: 75vw;
        height: 100vh;
        flex: 4;
        display: flex;
        flex-direction: column;

        #rss_view {
            z-index: 2;

            position: absolute;
            top: 10vh;
            left: 0vw;

            height: 90vh;
            width: 25vw;
        }

        #visualization{
            width: 100%;
            height: 100%;
            // flex: 14;

            #select-model-warning-wrapper{
                width: 100%;
                height: 100%;

                display: flex;
                align-items: center;
                justify-content: center;

                #button-wrapper{
                    display: flex;
                    align-items: center;
                    justify-content: center;

                    padding-top: 1em;
                    padding-bottom: 1em;
                    padding-left: 1em;
                    padding-right: 1em;
        
                    user-select: none;
        
                    display: flex;
                    flex-direction: row;
                    align-items: center;
        
                    border-radius: 10px;

                    background-color: mat.get-color-from-palette($primary-palette, 800);

                    width: 40em;
                    height: 10em;

                    h1{
                        font-size: 2em;
                        color: white;
                    }
                }

                #button-wrapper:hover{
                    background-color: mat.get-color-from-palette($primary-palette, 600);
                    cursor: pointer;
                }
                
            }

            #progress-spinner{
                width: 100%;
                height: 100%;

                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;

                h1{
                    user-select: none;
                    padding: 2em 0 0 0;
                }
            }
        }

        footer{
            // flex: 1;
            // background-color: brown;
        }
    }
}

.floatable {
    z-index: 1;
    position: absolute;
}

.buttonToggled{
    background-color: mat.get-color-from-palette($primary-palette, 800) !important;
}
### src/app/app.component.scss END ###

### src/app/app.component.ts BEGIN ###
// https://www.telerik.com/blogs/angular-14-introducing-standalone-components#:~:text=Creating%20a%20Standalone%20Component,ng%20g%20c%20login%20%2D%2Dstandalone
// https://material.angular.io/components/categories
// https://css-tricks.com/snippets/css/a-guide-to-flexbox/
// https://br.pinterest.com/pin/800022321275429738/
// import * as numeric from 'numeric';

import { AfterViewInit, ChangeDetectorRef, Component, ElementRef, ViewChild , Renderer2} from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import { VisualizationComponent } from "./components/visualization/visualization.component";
import { MatSlideToggleModule } from '@angular/material/slide-toggle';
import {MatTabsModule} from '@angular/material/tabs';
import {MatButtonToggleModule} from '@angular/material/button-toggle';
import {MatDividerModule} from '@angular/material/divider';
import {MatListModule} from '@angular/material/list';
import {MatSelectModule} from '@angular/material/select';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatMenuModule} from '@angular/material/menu';
import {MatButtonModule} from '@angular/material/button';
import {MatRippleModule} from '@angular/material/core';
import {MatPaginatorModule} from '@angular/material/paginator';
import {MatIconModule} from '@angular/material/icon';
import { CommonModule } from "@angular/common";
import { open } from '@tauri-apps/api/dialog';
import { RssViewComponent } from "./components/main_options/rss-view/rss-view.component";
import { provideRouter, Router, RouterOutlet} from "@angular/router";
import { environment } from "src/environments/environment";
import {MatSidenav, MatSidenavModule} from '@angular/material/sidenav'
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatTooltipModule } from '@angular/material/tooltip';
import { FileSelectionDialogComponent } from './components/main_options/file-selection-dialog/file-selection-dialog.component';
import { take } from "rxjs/operators";
import { DialogService } from "./services/dialog/dialog.service";
import { ErrorDialogComponent } from "./components/error-dialog/error-dialog.component";
import { PatternSummaryComponent } from "./components/pattern-summary/pattern-summary.component";
import { Pattern } from "./models/pattern";
import { fs } from "@tauri-apps/api";
import { resolveResource } from "@tauri-apps/api/path";
import { ApiService } from "./services/api/api.service";
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';

export enum MainOption {
  MODEL_SELECTOR,
  SETTINGS,
  TRUNCATE_MODEL,
  INTERSECTION_MODE
};

export enum ApplicationStatus {
  UNLOADED,
  LOADING,
  LOADED
};

@Component({
    selector: "app-root",
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
    standalone: true,
    animations: [
        trigger('slideInOut', [
            state('void', style({ transform: 'translateX(-100%)', opacity: 0 })),
            state('in', style({ transform: 'translateX(0)', opacity: 1 })),
            state('out', style({ transform: 'translateX(-100%)', opacity: 0 })),
            transition('void => in', [animate('0.4s ease-in-out')]),
            transition('in => out', [animate('0.4s ease-in-out')]),
            transition('out => in', [animate('0.4s ease-in-out')])
        ])
    ],
    imports: [RouterOutlet, CommonModule, VisualizationComponent, RssViewComponent,
        MatSlideToggleModule, MatTabsModule, MatButtonToggleModule, MatDividerModule,
        MatListModule, MatSelectModule, MatSlideToggleModule, MatCheckboxModule, MatMenuModule, MatButtonModule,
        MatRippleModule, MatPaginatorModule, MatSidenavModule, MatIconModule, MatTooltipModule, PatternSummaryComponent,
        MatProgressSpinnerModule]
})

export class AppComponent implements AfterViewInit{
  protected all_application_status = Object.values(ApplicationStatus);
  protected application_status: ApplicationStatus = ApplicationStatus.UNLOADED;

  protected MainOption = MainOption;
  protected settings_enabled: boolean = false;
  protected truncate_model_enabled: boolean;
  protected intersection_mode_enabled: boolean = false;

  @ViewChild("aside") aside: ElementRef<HTMLElement>;
  public matList_height: number;

  @ViewChild("sidenav") sidenav: MatSidenav;
  @ViewChild("model_selector") model_selector: ElementRef<HTMLElement>;
  private last_opened_folder: string = "";
  protected tensor_path: string = "";
  protected patterns_path: string = "";
  @ViewChild('rss_view') rss_view: RssViewComponent;
  @ViewChild('pattern_summary') pattern_summary: PatternSummaryComponent;
  
  @ViewChild('visualization_view') visualization_view: VisualizationComponent;
  protected hovered_pattern: Pattern;
  
  constructor(private cdr: ChangeDetectorRef, private dialog_service: DialogService, private api_service: ApiService){}

  ngAfterViewInit(){
    // this.matList_height = this.aside.nativeElement.clientHeight - this.model_selector.nativeElement.clientHeight;

    if(environment.dev_mode){ 
      this.application_status = ApplicationStatus.LOADED;
      this.cdr.detectChanges();
    }
  }

  // private reloadApplication(){
  //   this.application_status = ApplicationStatus.UNLOADED;
  //   this.cdr.detectChanges();

  //   this.application_status = ApplicationStatus.LOADED;
  //   this.cdr.detectChanges();
  // }

  private async handleModelChange(event: any){
    console.log("Handling model change");
    if (event.tensor_path == null || event.patterns_path == null){ return; }
    this.application_status = ApplicationStatus.LOADING;
    this.cdr.detectChanges();

    this.toggleMainOption(null);
    this.togglePatternSummary(null);
    this.updatePatternSummary(null);
    
    this.last_opened_folder = event.last_opened_folder;

    this.tensor_path = event.tensor_path;
    this.patterns_path = event.patterns_path;
    
    await this.api_service.initApplication(this.tensor_path, this.patterns_path);
    
    this.application_status = ApplicationStatus.LOADED;
    this.cdr.detectChanges();
    // this.reloadApplication();
  }

  protected toggleMainOption(option: MainOption | null){
    this.deactivateMainOptionsExcept(option);

    switch(option){
      case MainOption.MODEL_SELECTOR:
        this.openModelSelection();
        break;
      case MainOption.SETTINGS:
        this.toggleSettings();
        break;
      case MainOption.TRUNCATE_MODEL:
        this.toggleTruncateModel();
        break;
      case null:
        break
      // case MainOption.INTERSECTION_MODE:
      //   this.toggleIntersectionMode();
      //   break;
    }
  }

  private deactivateMainOptionsExcept(option: MainOption){
    if(this.settings_enabled && option != MainOption.SETTINGS){ this.toggleSettings(); }
    if(this.truncate_model_enabled && option != MainOption.TRUNCATE_MODEL){ this.toggleTruncateModel(); }
    // if(this.intersection_mode_enabled && option != MainOption.INTERSECTION_MODE){ this.toggleIntersectionMode(); }
  }

  private openModelSelection(): void {
    let dialog_data = {
      last_opened_folder: this.last_opened_folder,
      tensor_path: this.tensor_path,
      patterns_path: this.patterns_path
    };
    this.dialog_service.open(FileSelectionDialogComponent, 
      FileSelectionDialogComponent.WIDTH, 
      FileSelectionDialogComponent.HEIGHT, 
      dialog_data, 
      this.handleModelChange.bind(this));
  }

  private toggleSettings(){
    this.settings_enabled = !this.settings_enabled;
    this.sidenav.toggle();
  }
  
  private toggleTruncateModel(){
    if(this.truncate_model_enabled == undefined){ return; }

    this.truncate_model_enabled = !this.truncate_model_enabled;
    this.cdr.detectChanges();
  }

  // private toggleIntersectionMode(){
  //   this.intersection_mode_enabled = !this.intersection_mode_enabled;
  //   this.visualization_view.toggleIntersectionMode();
  //  }

  protected disableRssView(){
    this.truncate_model_enabled = false;
    this.cdr.detectChanges();
  }

  protected onTruncation(event){
    this.rss_view.disableSlider(); 
    setTimeout(() => { 
        this.rss_view.enableSlider();
    }, 1100);

    this.visualization_view.onTruncation(event);
  }

  protected updatePatternSummary(identifier){
    this.pattern_summary.update(identifier);
  }

  protected togglePatternSummary(identifier){
    this.pattern_summary.toggleLock(identifier);
  }

  get applicationStatusUnloaded(): ApplicationStatus {
    return ApplicationStatus.UNLOADED;
  }

  get applicationStatusLoading(): ApplicationStatus {
    return ApplicationStatus.LOADING;
  }

  get applicationStatusLoaded(): ApplicationStatus {
    return ApplicationStatus.LOADED;
  }
}

### src/app/app.component.ts END ###

### src/app/app.component.html BEGIN ###
<body #app_body id="app_body">
    <aside #aside>
        <div id="main-options">
            <header class="main-option" id="model-selection-button" *ngIf="application_status == applicationStatusLoaded" #model_selector matRipple [matRippleCentered]="true"
                (click)="toggleMainOption(MainOption.MODEL_SELECTOR)"
                matTooltip="Change resume">
                <div id="matlist-placeholder" *ngIf="application_status == applicationStatusUnloaded"></div>
                <mat-icon aria-hidden="false" aria-label="open_file" fontIcon="upload_file" ></mat-icon>
            </header>

            <header class="main-option" id="truncate-model-button" 
                [ngClass]="{'buttonToggled': this.truncate_model_enabled}"
                *ngIf="application_status == applicationStatusLoaded"
                (click)="toggleMainOption(MainOption.TRUNCATE_MODEL)"
                matTooltip="Truncate resume">
                <mat-icon aria-hidden="false" aria-label="settings" fontIcon="show_chart"></mat-icon>
            </header>
<!-- 
            <header class="main-option" id="settings-button" matRipple [matRippleCentered]="true" 
                [ngClass]="{'buttonToggled': this.settings_enabled}"
                *ngIf="model_loaded"
                (click)="toggleMainOption(MainOption.SETTINGS)"
                matTooltip="Settings">
                <mat-icon aria-hidden="false" aria-label="settings" fontIcon="settings"></mat-icon>
            </header> -->
        </div>
        
        <div id="content-cover" [hidden]="!truncate_model_enabled"></div>
        <mat-sidenav-container #content class="sidenav-content" autosize [ngClass]="{'faded': truncate_model_enabled}">
            <div id="settings-wrapper-lower-content">
                <app-pattern-summary id="pattern-summary" #pattern_summary></app-pattern-summary>
            </div>
        
            <mat-sidenav #sidenav id="settings-wrapper-upper-content" mode="over" fixedInViewport="false">
                <nav>
                    <button class="setting" mat-raised-button [matMenuTriggerFor]="scaleMenu">Scale</button>
                    <mat-menu class="setting" #scaleMenu="matMenu">
                        <button mat-menu-item>Linear</button>
                        <button mat-menu-item>Logarithmic</button>
                    </mat-menu>
        
                    <button class="setting" mat-raised-button [matMenuTriggerFor]="areaMenu">Area attribute</button>
                    <mat-menu class="setting" #areaMenu="matMenu">
                        <button mat-menu-item>Pattern size</button>
                        <button mat-menu-item>Density</button>
                        <button mat-menu-item>G</button>
                    </mat-menu>
        
                    <mat-menu class="setting" #groupMenu="matMenu">
                        <button mat-menu-item>Flat</button>
                        <button mat-menu-item>Group by fonts</button>
                    </mat-menu>
                </nav>
            </mat-sidenav>
        </mat-sidenav-container>

        
    </aside>

    <div id="main-app">
        <app-rss-view #rss_view id="rss_view" class="floatable" 
                (onTruncation)="onTruncation($event)"
                (initialized)="disableRssView()"
                [@slideInOut]="truncate_model_enabled ? 'in' : 'out'"
                *ngIf="application_status == applicationStatusLoaded">
        </app-rss-view>

        <div id="visualization">
            <!-- <router-outlet 
                (activate)="onActivate($event)"
                (onTruncationFinished)="onTruncationFinished()"
                >
            </router-outlet> -->
            <div id="select-model-warning-wrapper" *ngIf="application_status == applicationStatusUnloaded">
                <div id="button-wrapper" (click)="toggleMainOption(MainOption.MODEL_SELECTOR)">
                    <h1> Select resume to visualize </h1>
                </div>
            </div>

            <div id="progress-spinner" *ngIf="application_status == applicationStatusLoading">
                <mat-spinner></mat-spinner>
                <h1>Loading visualization...</h1>
            </div>
            
            <app-visualization #visualization_view
                (onTruncation)="onTruncation($event)"
                (datapoint_hover_in)="updatePatternSummary($event)"
                (datapoint_hover_out)="updatePatternSummary(null)"
                (datapoint_click)="togglePatternSummary($event)"
                *ngIf="application_status == applicationStatusLoaded">
            </app-visualization>
        </div>
    </div>
</body>
### src/app/app.component.html END ###

### src/app/components/visualization/visualization.component.scss BEGIN ###
@use '@angular/material' as mat;
@import '../../../theme.scss';

html, body{
    display: flex;
    flex-direction: row;
    justify-content: center;
    // background-color: yellow;
    height: 100vh;
    width: 100;

    overflow: hidden;

    position: relative;

    section{
        display: flex;
        flex-grow: 4;

        justify-content: center;

        position: relative;

        #vizualization_div{
            z-index: 0;
            position: relative;
            object-fit: cover;
            width: 100%;
            height: 100%;

            // background-color: red;
            object-fit:contain;
        }

        #intersection_details{
            // width: 10vw;
            // height: 10vh;

            bottom: 5%;
            right: 5%;

            button{
                // background-color: mat.get-color-from-palette($primary-palette, 100);
            }
        }
    }

    #datapoint_tooltip{
        position: absolute;
        top: 0;
        left: 0;

        width: 15%;
        height: 15%;
    }
}

.floatable {
    z-index: 1;
    position: absolute;
}

.dashed-outline {
    border:  2px dashed #000;
}
### src/app/components/visualization/visualization.component.scss END ###

### src/app/components/visualization/visualization.component.ts BEGIN ###
import * as d3Tip from "d3-tip";
import { resolveResource } from '@tauri-apps/api/path'
import { ChangeDetectorRef, Component, ComponentFactoryResolver, EventEmitter, InjectionToken, Input, OnDestroy, Output, Renderer2, ViewContainerRef } from '@angular/core';
import { ComponentPortal, PortalModule } from '@angular/cdk/portal';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/app/models/datapoint';
import { event, fs, invoke } from '@tauri-apps/api';
import { BaseDirectory } from "@tauri-apps/api/fs";
import { SvgService } from 'src/app/services/svg/svg.service';
import { Subscription, take } from 'rxjs';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';
import { ActivatedRoute } from '@angular/router';
import { RssViewComponent } from 'src/app/components/main_options/rss-view/rss-view.component';
import { environment } from '../../../environments/environment';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { DataPointTooltipComponent } from "./datapoint-tooltip/datapoint-tooltip.component";
import { Pattern } from "src/app/models/pattern";
import { DialogService } from "src/app/services/dialog/dialog.service";
import { legendCircle } from 'src/js/circle_legend.js';
import { Legend } from 'src/js/color_legend.js';
import { IntersectionModeFeatureModule } from 'src/app/components/visualization/intersection-mode-feature.module';
import { SvgFeatureModule } from "./svg-feature.module";
import {MatButtonModule} from '@angular/material/button';
import { ApiService } from "src/app/services/api/api.service";

@Component({
    selector: 'app-visualization',
    standalone: true,
    templateUrl: './visualization.component.html',
    styleUrls: ['./visualization.component.scss'],
    animations: [
        trigger('slideInOut', [
            state('void', style({
                transform: 'translateX(100%)',
                opacity: 0
            })),
            state('in', style({
                transform: 'translateX(0)',
                opacity: 1
            })),
            state('out', style({
                transform: 'translateX(100%)',
                opacity: 0
            })),
            transition('void => in', [
                animate('0.5s ease-in-out')
            ]),
            transition('in => out', [
                animate('0.5s ease-in-out')
            ]),
            transition('out => in', [
                animate('0.5s ease-in-out')
            ])
        ])
    ],
    imports: [
        CommonModule,
        MatCardModule,
        PortalModule,
        RssViewComponent,
        DataPointTooltipComponent,
        MatButtonModule
    ]
})

export class VisualizationComponent implements AfterViewInit, OnDestroy{
  @Output() datapoint_hover_in = new EventEmitter<number>();
  @Output() datapoint_hover_out = new EventEmitter<number>();
  @Output() datapoint_click = new EventEmitter();

  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  @ViewChild('vizualization_div') visualization_div: ElementRef<HTMLDivElement>;

  private svg_feature: SvgFeatureModule;
  protected intersection_mode_feature: IntersectionModeFeatureModule;

  constructor(private api_service: ApiService, private dialog_service: DialogService, private cdr: ChangeDetectorRef){ }

  ngOnInit(): void {
    this.intersection_mode_feature = new IntersectionModeFeatureModule(null, null, null);
  }

  async ngAfterViewInit() {
    console.log("Initializing visualization component");

    let svg_width = this.body.nativeElement.clientWidth;
    let svg_height = this.body.nativeElement.clientHeight;
    
    this.svg_feature = new SvgFeatureModule(this.cdr);
    this.svg_feature.init(this.visualization_div, svg_width, svg_height);
    this.svg_feature.datapoint_hover_in.subscribe(identifier => this.onDatapointHoverIn(identifier));
    this.svg_feature.datapoint_hover_out.subscribe(identifier => this.onDatapointHoverOut(identifier));
    this.svg_feature.datapoint_click.subscribe(identifier => this.onDatapointClick(identifier));
    
    let datapoints = await this.api_service.getDataPoints();
    this.svg_feature.drawDataPoints(datapoints);

    this.intersection_mode_feature = new IntersectionModeFeatureModule(this.svg_feature, this.dialog_service, this.api_service);
  }

  ngOnDestroy() {
    this.svg_feature.datapoint_hover_in.unsubscribe();
    this.svg_feature.datapoint_hover_out.unsubscribe();
    this.svg_feature.datapoint_click.unsubscribe();
  }

  public onResize(event) {
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.svg_feature.resizeSvg(width, height);
  }

  public async onTruncation(event){
    let new_size = event - 1; // -1 because the first point is the null model rss
    let truncated_datapoints = await this.api_service.truncateModel(new_size);

    this.intersection_mode_feature.toggleIntersections(null);
    this.svg_feature.drawDataPoints(truncated_datapoints);
  }

  private onDatapointHoverIn(identifier: number){
    this.datapoint_hover_in.emit(identifier); // To communicate with pattern summary
  }

  private onDatapointHoverOut(identifier: number){
    this.datapoint_hover_out.emit(identifier); // To communicate with pattern summary
  }

  private onDatapointClick(identifier: number){
    this.datapoint_click.emit(identifier); // To communicate with pattern summary
    this.intersection_mode_feature.toggleIntersections(identifier);
  }

  // public toggleIntersectionMode(){
  //   this.intersection_mode_feature.toggleIntersectionMode();
  // }
}

### src/app/components/visualization/visualization.component.ts END ###

### src/app/components/visualization/svg-feature.module.ts BEGIN ###
import * as d3Tip from "d3-tip";
import { ChangeDetectorRef, ElementRef, EventEmitter, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import * as d3 from 'd3';
import { DataPoint } from 'src/app/models/datapoint';
import { legendCircle } from 'src/js/circle_legend.js';
import { Legend } from 'src/js/color_legend.js';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class SvgFeatureModule {
  public datapoint_hover_in = new EventEmitter<number>();
  public datapoint_hover_out = new EventEmitter<number>();
  public datapoint_click = new EventEmitter<number>();

  private locked_datapoint: DataPoint;
  private datapoints: Array<DataPoint>;
  
  private visualization_div: ElementRef;
  public plot: any;
  public svg: any;

  private zoom_level: number;
  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;

  private tooltip;
  private transition_duration = 300;

  private cdr: ChangeDetectorRef;

  constructor(cdr: ChangeDetectorRef){ 
    this.cdr = cdr;
  }

  public init(visualization_div: ElementRef, svg_width: number, svg_height: number){
    this.visualization_div = visualization_div;
    this.svg_width = svg_width;
    this.svg_height = svg_height;

    this.tooltip = d3Tip.default()
      .attr('class', 'd3-tip')
      .offset([-10, 0])
      .html(function(d) {
        return "\
          <div style='background-color:#ededed; padding: 0.5em 0.5em 0.5em 0.5em; border-radius: 10px; border: 1px dashed black;'>\
            <strong>ID:</strong> <span style='color:#BC2602'>" + d.identifier + "</span><br>\
            <strong>Size:</strong> <span style='color:#BC2602'>" + d.pattern_size + "</span><br>\
            <strong>Density:</strong> <span style='color:#BC2602'>" + Math.round(d.density * 100) / 100 + "</span>\
          </div>\
          ";
      });
    
    this.svg = this.createSvg();
    this.resizeSvg(this.svg_width, this.svg_height);
    this.cdr.detectChanges();
    
    this.zoom_level = this.initial_scale;
  }

  public createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
      .append('svg')
        .attr('width', this.svg_width)
        .attr('height',this.svg_height)
        .on('dblclick', () => {  });

    return svg;
  }

  public resizeSvg(width: number, height: number){
    this.svg
      .attr('width', width)
      .attr('height', height);

    let x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, (height - this.y_correction)/1]);

    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([(height - this.y_correction)/1, 0]);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.svg_width = width;
    this.svg_height = height;

    this.createPlot();
    this.drawDataPoints(this.datapoints);
  }

  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g")
      .attr("id", "plot")
      .on('dblclick', () => {  });
    
    let panning_zoom = d3.zoom()
      .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
      .translateExtent([[0, 0], [this.svg_height, this.svg_height]])
      .on("start", (event, d) => { this.svg.attr("cursor", "grabbing"); })
      .on("zoom", (event) => { 
        this.plot.attr("transform", event.transform); 
        if(event.sourceEvent instanceof WheelEvent){
          this.zoom_level = event.transform.k;
          this.drawCircleLegend();
        }
      })
      .on("end", (event, d) => {this.svg.attr("cursor", "default")});

    this.svg.call(panning_zoom);

    // Apply initial zoom level
    let x_translation_factor = 0.0;
    let y_translation_factor = 0.2;
    let initial_transform = d3.zoomIdentity
      .translate(-this.svg_width*(x_translation_factor), -this.svg_height*(y_translation_factor))
      .scale(this.initial_scale);
    this.svg.call(panning_zoom.transform, initial_transform);
    
    this.drawGridLines();
    this.drawUnselectionRect();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
    let grey_tonality = 220;
    let color = `rgb(${grey_tonality}, ${grey_tonality}, ${grey_tonality})`;
    
    this.plot.append("g") // Add the X gridlines
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", color)
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    this.plot.append("g") // Add the Y gridlines
      .attr("class", "grid")
      .attr("color", color)
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }

  private drawUnselectionRect(){
    this.plot.append('rect')
        .attr('id', 'overlay')
        .attr('x', 0)
        .attr('y', 0)
        .attr('width', this.svg_width)
        .attr('height', this.svg_height)
        .style('fill', 'rgba(0, 0, 0, 0)')
        .lower()
        .on('click', (event, d) => { 
          this.locked_datapoint = undefined;
          this.toggleHighlight(undefined);
          this.datapoint_click.emit(null) 
        });
  }

  private drawCircleLegend(){
    let min_pattern_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let max_pattern_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let mean_pattern_size = 0;
    for(let i = 0; i < this.datapoints.length; i++){
      mean_pattern_size += this.datapoints[i].pattern_size;
    }
    mean_pattern_size /= this.datapoints.length;
    mean_pattern_size = Math.round(mean_pattern_size);

    let min_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;
    let max_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;

    let legend = legendCircle(null)
      .scale(
        d3.scaleLinear()
            .domain([min_pattern_size, max_pattern_size])
            .range([min_size, max_size])
      )
      .tickValues([min_pattern_size, mean_pattern_size, max_pattern_size])
      .tickFormat((d, i, e) => `${d}${i === e.length - 1 ? " Cells" : ""}`)
      .tickSize(max_size); // defaults to 5
    
    const legend_x_padding = 10;
    const legend_y_padding = 10;
  
    this.svg.select("#circle_legend").remove();
    this.svg.append("g")
      .attr('id', 'circle_legend')
      .attr('transform', `translate(${legend_x_padding}, ${legend_y_padding})`)
      .call(legend);
  }

  private drawColorLegend(){
    let oldLegend = document.getElementById("color_legend");
    if(oldLegend){
        oldLegend.parentNode.removeChild(oldLegend);
    }

    let svg_width = this.svg.attr('width');
    let legend_width = 320;
    const legend_x_padding = 10;

    let legend_x = svg_width - (legend_width + legend_x_padding);

    let legend = Legend(d3.scaleLinear([0, 1], ["rgba(255,0,0,0)", "red"]), {
      title: "Density",
      width: legend_width,
    })

    let legendGroup = this.svg.append('g')
      .attr('id', 'color_legend')
      .attr("transform", `translate(${legend_x}, 0)`);
      
    legendGroup.node().appendChild(legend);
  }

  private scalingFunction(datapoints: Array<DataPoint>) {
    let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));
    let max_module = Math.max(x_max_module, y_max_module);

    let scaled_datapoints: Array<DataPoint> = datapoints;
    let screen_coverage = 0.5;
    datapoints.forEach(datapoint => {
        let result_x = datapoint.x / x_max_module;
        let result_y = datapoint.y / y_max_module;

      if (isNaN(result_x) || !isFinite(result_x)) { result_x = datapoint.x; }
      if (isNaN(result_y) || !isFinite(result_y)) { result_y = datapoint.y; }

        datapoint.x = result_x / ((1-screen_coverage) + 1);
        datapoint.y = result_y / ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  private toggleHighlight(datapoint: DataPoint){
    if(this.locked_datapoint){ return; }
    
    if(datapoint){ // Add a EMPTY circle with id highlight, the circle should not block mouse hover and click events
       // Draw a new blue circle on the coordinates of datapoint
      let highlight_radius = datapoint.size * 1.8;
      let highlight_color = 'rgba(114, 232, 247)';
      let highlight_opacity = 0.8;
      let stroke_width = highlight_radius/3;

      let highlight_circle = this.plot.select('#highlight');
      if(highlight_circle){ highlight_circle.remove(); }
      this.plot.append('circle')
        .attr('id', 'highlight')
        .attr('cx', this.x_scale(datapoint.x))
        .attr('cy', this.y_scale(datapoint.y))
        .attr('r', highlight_radius)
        .attr('fill', 'none')
        .attr('stroke', highlight_color)
        .attr('stroke-width', stroke_width)
        .attr('opacity', highlight_opacity)
        .style('pointer-events', 'none');
    }else{
      // Remove the circle with id highlight
      let highlight_circle = this.plot.select('#highlight');
      if(highlight_circle){ highlight_circle.remove(); }
    }
  }

  public drawDataPoints(datapoints: Array<DataPoint>) {
    if(datapoints == undefined || datapoints == null){ return; }
    if(this.plot == undefined){ return; }
    
    console.log("Drawing " + datapoints.length + " datapoints");
    this.datapoints = datapoints;

    this.plot.call(this.tooltip);

    let scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.plot.selectAll('datapoint')
        .data(scaled_datapoints, d => d.identifier);

    circles.exit()
        .transition()
        .duration(this.transition_duration)
        .attr('r', 0)
        .remove(); 

    circles.transition()
        .duration(this.transition_duration) 
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)));

    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('class', 'datapoint')
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', 0)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer')
        .style('stroke', 'rgba(255, 0, 0, 1')
        .on('mouseover', (event, d) => { 
          this.toggleHighlight(d);
          this.tooltip.show(d, event.currentTarget);
          this.datapoint_hover_in.emit(d.identifier);
        })
        .on('mouseout', (event, d) => { 
          this.toggleHighlight(undefined);
          this.tooltip.hide(d, event.currentTarget); 
          this.datapoint_hover_out.emit(d.identifier);
        })
        .on('click', (event, d) => {
          this.locked_datapoint = undefined;
          this.toggleHighlight(d);
          this.locked_datapoint = d;

          this.datapoint_click.emit(d.identifier);
         })
        .transition()
        .duration(this.transition_duration)
        .attr('r', d => d.size);
    
    this.drawCircleLegend();
    this.drawColorLegend();
  }

  public resetDatapointEvents(){
    let circles = this.plot.selectAll('datapoint'); 
    circles
        .on('mouseover', (event, d) => { 
          this.tooltip.show(d, event.currentTarget);
          this.datapoint_hover_in.emit(d.identifier)
         })
        .on('mouseout', (event, d) => { 
          this.tooltip.hide(d, event.currentTarget); 
          this.datapoint_hover_out.emit(d.identifier);
         })
        .on('click', (event, d) => {
          this.locked_datapoint = undefined;
          this.toggleHighlight(d);
          this.locked_datapoint = d;

          this.datapoint_click.emit(d.identifier);
         });
  }

  public xScale(x: number){
    return this.x_scale(x);
  }

  public yScale(y: number){
    return this.y_scale(y);
  }

  public getDrawnDataPoints(){
    return this.datapoints;
  }

  public getSvgWidth(){
    return this.svg_width;
  }

  public getSvgHeight(){
    return this.svg_height;
  }
}
### src/app/components/visualization/svg-feature.module.ts END ###

### src/app/components/visualization/intersection-mode-feature.module.ts BEGIN ###
import { EventEmitter, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';
import { SvgFeatureModule } from './svg-feature.module';
import * as d3 from 'd3';
import { environment } from 'src/environments/environment';
import { fs, invoke } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { IntersectionDetailsDialogComponent } from './intersection-details-dialog/intersection-details-dialog.component';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import { ApiService } from 'src/app/services/api/api.service';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class IntersectionModeFeatureModule {
  private intersection_mode:boolean = false;
  private old_clicked_datapoint = null;
  private clicked_datapoint_data: DataPoint = null;
  private intersection_details: IntersectionDetails;
  private transition_duration: number = 300;

  private svg_feature: SvgFeatureModule;
  private dialog_service: DialogService;
  private api_service: ApiService

  constructor(svg_feature: SvgFeatureModule, dialog_service: DialogService, api_service: ApiService) {
    this.svg_feature = svg_feature;
    this.dialog_service = dialog_service;
    this.api_service = api_service;
  }

  private connectDatapoints(center: DataPoint, intersections:Map<number, number>, intersections_colors: Map<number, string>){
    let circles = new Map<number, DataPoint>(this.svg_feature.plot.selectAll('datapoint').data()
      .map(d => [d.identifier, d]));

    for(let [identifier, percentage] of intersections.entries()){
      if(identifier == this.clicked_datapoint_data.identifier){ continue; } // itself
      if(identifier == 0){ continue; } // Excess intersections

      let stroke_width = 6 * percentage + 2; // 2 to 8

      let x1 = this.svg_feature.xScale(center.x);
      let y1 = this.svg_feature.yScale(center.y);
      let line = this.svg_feature.plot.append('line')
        .datum({x1: x1, y1: y1})  // Bind the original coordinates to the line
        .attr('class', 'intersection_line')
        .attr('pointer-events', 'none')
        .attr('x1', this.svg_feature.xScale(center.x))  // Start position (x) of the line
        .attr('y1', this.svg_feature.yScale(center.y))  // Start position (y) of the line
        .attr('x2', this.svg_feature.xScale(center.x))  // Initially, end position (x) is the same as start position
        .attr('y2', this.svg_feature.yScale(center.y))  // Initially, end position (y) is the same as start position
        .attr('stroke', intersections_colors.get(identifier))
        .attr('stroke-width', stroke_width);
      
      let related_circle = circles.get(identifier) || null;
      if (related_circle == null) { continue; } // Related circle is a subpattern
      line
        .transition('mouseover')
        .duration(this.transition_duration*2)
        .attr('x2', this.svg_feature.xScale(related_circle.x))  // Actual end position (x) of the line
        .attr('y2', this.svg_feature.yScale(related_circle.y));  // Actual end position (y) of the line
    }
  }

  private highlightDatapoints(identifiers: Array<number>, intersections_colors: Map<number, string>){
    let identifiers_set = new Set(identifiers);
    let circles_visibility = 0.2;

    this.svg_feature.plot.selectAll('datapoint')
      .raise()
      .transition('mouseover')
      // .duration(this.transition_duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
      .style('stroke', `rgba(255, 0, 0, 1)`)
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${circles_visibility})`)
      .style('stroke', `rgba(255, 0, 0, ${circles_visibility})`);

    let highligthed_circles = this.svg_feature.plot.selectAll('datapoint')
      .filter(d => identifiers_set.has(d.identifier));

    highligthed_circles
      .raise()
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('fill', d => intersections_colors.get(d.identifier))
      .style('stroke', d=> intersections_colors.get(d.identifier));
  }

  private createIntesectionColorMapping(intersections: Map<number, number>): Map<number, string>{
    let colors = ["#eb4da3", "#a614b3", "#8a4aed", "#1731e8", "#3ea6ed", "#0c8e81"]

    let sorted_intersections = new Map(Array.from(intersections.entries()).sort((a, b) => b[1] - a[1]));

    let intersections_colors: Map<number, string> = new Map();
    let i = 0;
    for(const [identifier, percentage] of sorted_intersections.entries()){
      if(identifier == this.clicked_datapoint_data.identifier){
        intersections_colors.set(identifier, "#d71610");
        continue;
      }

      if(i > colors.length - 1){ console.warn("Not enough colors for intersections.");}
      intersections_colors.set(identifier, colors[i % colors.length]);
      i++;
    }

    return intersections_colors;
  }

  private expandCircle(clicked_circle, expansion_factor, intersections, intersections_colors){
    clicked_circle
      .attr('r', this.clicked_datapoint_data.size)
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('r', this.clicked_datapoint_data.size * expansion_factor)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, 1)`);
  }

  private createIntersectionChart(clicked_circle: any, intersections: Map<number, number>, original_radius: number, chart_radius: number, 
    intersections_colors: Map<number, string>){
    let pie = d3.pie()
      .value((d: any) => d.value);

    let data: Array<any> = Array.from(intersections, ([key, value]) => ({key, value}));
    let pie_data = pie(data);
    
    let original_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => original_radius);
    let pie_chart_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(chart_radius);

    let pie_group = this.svg_feature.plot.append('g')
      .attr('class', 'pie_chart')
      .attr('transform', `translate(${clicked_circle.attr('cx')}, ${clicked_circle.attr('cy')})`);

    pie_group.selectAll('path')
      .data(pie_data)
      .enter()
      .append('path')
      .attr('pointer-events', 'none')
      .attr('d', original_arc)
      .attr('fill', 'red')
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('d', pie_chart_arc)
      .attr('fill', (d: any) => {
        let color = intersections_colors.get(d.data.key);
        return color;
      });
  }

  private createIntersectionCharts(identifiers: Array<number>, intersections: Map<number, number>, intersections_colors: Map<number, string>){
    let clicked_datapoint = this.svg_feature.plot.selectAll('datapoint')
      .filter(d => d.identifier == this.clicked_datapoint_data.identifier);
    let empty = new Map<number, number>();
    empty.set(this.clicked_datapoint_data.identifier, 1);
    let original_radius = this.clicked_datapoint_data.size;
    let chart_radius = this.clicked_datapoint_data.size;
    this.createIntersectionChart(clicked_datapoint, empty, original_radius, chart_radius, intersections_colors);
  
    let identifiers_set = new Set(identifiers);
    let circles = this.svg_feature.plot.selectAll('datapoint')
      .filter(d => identifiers_set.has(d.identifier));

    circles.each((d, i, nodes) => {
      let intersection_data: Map<number, number> = new Map<number, number>();
      let parent_current_percentage = intersections.get(d.identifier); // Colored with the parent color
      let complement_percentage = 1 - parent_current_percentage; // Colored with current circle color

      intersection_data.set(this.clicked_datapoint_data.identifier, parent_current_percentage);
      intersection_data.set(d.identifier, complement_percentage);

      original_radius = d.size;
      chart_radius = d.size;
      this.createIntersectionChart(d3.select(nodes[i]), intersection_data, original_radius, chart_radius, intersections_colors);
    });
  }

  private async showIntersections(){
    if(this.clicked_datapoint_data == null){ return };

    let intersections = await this.api_service.getIntersectionsPercentages(this.clicked_datapoint_data.identifier);
    let intersections_colors = this.createIntesectionColorMapping(intersections);

    let relationed_datapoints: Array<number> = Array.from(intersections.keys())
      .filter(d => (d != this.clicked_datapoint_data.identifier) && (d != 0));
    this.highlightDatapoints(relationed_datapoints, intersections_colors);
    this.connectDatapoints(this.clicked_datapoint_data, intersections, intersections_colors);
    let expansion_factor = 1;
    // this.expandCircle(clicked_circle, expansion_factor, intersections, intersections_colors);
    this.createIntersectionCharts(relationed_datapoints, intersections, intersections_colors);
  }

  private async hideIntersections(){
    let intersection_lines = this.svg_feature.svg.selectAll('.intersection_line');
    intersection_lines
      .transition('mouseout')
      .duration(this.transition_duration)
      .attr('x2', d => d.x1)  // End position (x) becomes the start position
      .attr('y2', d => d.y1)  // End position (y) becomes the start position
      .remove();

    let circles = this.svg_feature.plot.selectAll('datapoint');
    circles
      .transition('mouseout')
      .duration(this.transition_duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
      .attr('r', d => d.size)
      .style('stroke', `rgba(255, 0, 0, 1)`);

    if(this.clicked_datapoint_data != null){
      let circle_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => this.clicked_datapoint_data.size);

      let pie_chart = this.svg_feature.svg.selectAll('.pie_chart');
      pie_chart.selectAll('path')
        .transition('mouseout')
        .duration(this.transition_duration)
        .attr('d', d=> d.size)
        .remove();  // Remove the paths after the transition
    }
  }

  public async toggleIntersections(identifier: number){
    this.hideIntersections();
    await this.updateClickedDatapoint(identifier);

    // if(identifier == null || identifier==undefined){return;}

    // if((this.old_clicked_datapoint != null) && (identifier == this.old_clicked_datapoint.identifier)){ // Datapoint was clicked again
    //   await this.updateClickedDatapoint(null);
    // }

    // this.showIntersections();
  }

  private async updateClickedDatapoint(identifier: number) {
    this.old_clicked_datapoint = this.clicked_datapoint_data;

    if(identifier == null){
      this.clicked_datapoint_data = null;
      this.intersection_details = null;
      return;
    }

    console.log(this.svg_feature.plot.selectAll('datapoint'))
    let clicked_circle = this.svg_feature.plot.selectAll('datapoint')
      // .filter(d => d.identifier == 13); // Fix black color
      .filter(d => d.identifier == identifier);
    this.clicked_datapoint_data = clicked_circle.node().__data__;

    this.intersection_details = await this.api_service.getIntersectionDetails(this.clicked_datapoint_data.identifier);
  }

  public clickedPatternHasIntersections(): boolean {
    if(this.intersection_details == null){
      return false;
    }

    return this.intersection_details.intersections.size > 0;
  }

  public async showIntersectionDetails(){
    if(this.clicked_datapoint_data == null){
      console.warn("No clicked datapoint to show details.");
      return;
    }

    let intersection_details = await this.api_service.getIntersectionDetails(this.clicked_datapoint_data.identifier);

    let dialog_data = {
      intersection_details: intersection_details
    }

    this.dialog_service.open(IntersectionDetailsDialogComponent, 
      IntersectionDetailsDialogComponent.WIDTH, 
      IntersectionDetailsDialogComponent.HEIGHT, 
      dialog_data);
  }
}

### src/app/components/visualization/intersection-mode-feature.module.ts END ###

### src/app/components/visualization/visualization.component.html BEGIN ###
<body #body (window:resize)="onResize($event)">
    <section>
        <div #vizualization_div id="vizualization_div"></div>

        <div class="floatable" id="intersection_details" *ngIf="intersection_mode_feature.clickedPatternHasIntersections()">
            <button mat-raised-button (click)="intersection_mode_feature.showIntersectionDetails()"> Show intersections </button>
        </div>
    </section>
</body>
### src/app/components/visualization/visualization.component.html END ###

### src/app/components/visualization/visualization.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { VisualizationComponent } from './visualization.component';

describe('VisualizationComponent', () => {
  let component: VisualizationComponent;
  let fixture: ComponentFixture<VisualizationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ VisualizationComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(VisualizationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/visualization/visualization.component.spec.ts END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IntersectionDetailsDialogComponent } from './intersection-details-dialog.component';

describe('IntersectionDetailsDialogComponent', () => {
  let component: IntersectionDetailsDialogComponent;
  let fixture: ComponentFixture<IntersectionDetailsDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ IntersectionDetailsDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IntersectionDetailsDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.spec.ts END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.scss BEGIN ###
@use '@angular/material' as mat;
@import '../../../../theme.scss';

body{
    margin: 0 0 0 0;
    padding: 0 0 0 0;


    width: 100%;
    height: 100%;
    
    display: flex;
    flex-direction: column;
    align-items: center;

    overflow-x: hidden;
    overflow-y: scroll;

    // background-color: green;

    header{
        width: 100%;
        height: fit-content;

        padding: 1em 1em 1em 4em;

        // background-color: blue;
    }

    section{
        width: 95%;
        height: 90%;

        padding: 1em 1em 1em 2em;

        display: flex;
        flex-direction: row;
        align-items: flex-start;
        justify-content: center;
        // background-color: yellow;

        // overflow: hidden;

        #intersectors_table_wrapper{
            width: fit-content;
            max-height: 100%;
            overflow: auto;

            padding: 0 5% 0 0;            

            // background-color: red;
            
            table{
                user-select: none;
                cursor: pointer;

                .mat-column-intersections{
                    text-align: center;
                }

                th{ // Table header
                    cursor: default;
                }

                .intersectors_data_row:hover{
                    background: whitesmoke;
                }
            }
        }

        #intersector_table_wrapper{
            width: 70%;
            overflow: auto;
            
            // background-color: blue;
            table{
                max-width: 90%;

                .mat-column-dim_number{ 
                    text-align: center;
                    width: 5em;
                }

                .mat-column-dim_values_preview{
                    text-align: center;
                    width: 5em;
                    // background-color: blue;
                }

                .mat-column-expand{
                    // background-color: brown;
                    width: 3em;
                }

                th{ // Table header
                    user-select: none;
                    cursor: default;
                }

                tr.data-row{
                    // Selector for the rows
                }

                tr.data-row:not(.expanded-row):hover {
                    background: whitesmoke;
                }
                
                tr.data-row:not(.expanded-row):active {
                    background: #efefef;
                }

                .data-row td {
                    border-bottom-width: 0;
                }

                .mat-column-expand{
                    button{
                        // width: 2em;
                        mat-icon{
                            // font-size: 1em;
                        }
                    }
                    
                }

                tr.detail-row {
                    height: 0;
                }
                
                .detail {
                    display: flex;

                    // background-color: red;
                    width: 100%;
                    word-break: normal;
                }
                
                .detail-value {
                    padding: 16px;
                }
                
                .detail-value-attribution {
                    opacity: 0.5;
                }
            }
        }
    }
}

.expanded-row{
    
}

.selected-row{
    background: whitesmoke;
}
### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.scss END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.ts BEGIN ###
import { ChangeDetectorRef, Component, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import {MatSort, MatSortModule} from '@angular/material/sort';
import {MatTableDataSource, MatTableModule} from '@angular/material/table';
import { MatTabsModule } from '@angular/material/tabs';
import {MatIconModule} from '@angular/material/icon';
import { animate, state, style, transition, trigger } from '@angular/animations';

export interface IntersectedTuple {
  dim_number: String;
  dim_values_preview: Array<String>;
  dim_values: Array<String>;

  needs_expand: boolean;
}

@Component({
  selector: 'app-intersection-details-dialog',
  standalone: true,
  imports: [
    CommonModule,
    MatIconModule,
    MatTabsModule,
    MatSortModule,
    MatTableModule
  ],
  templateUrl: './intersection-details-dialog.component.html',
  styleUrls: ['./intersection-details-dialog.component.scss'],
  animations: [
    trigger('detailExpand', [
      state('collapsed,void', style({height: '0px', minHeight: '0'})),
      state('expanded', style({height: '*'})),
      transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
    ]),
  ],
})
export class IntersectionDetailsDialogComponent {
  public static WIDTH = '40vw';
  public static HEIGHT = '80vh';
  
  protected identifier: number;
  protected total_untouched_percentage: number;
  protected total_intersection_percentage: number;
  protected intersections: Map<number, [number, Array<Array<string>>]>;

  protected intersectors_displayed_columns: string[] = ['intersections'];
  protected intersectors_data_source: MatTableDataSource<Array<number>>;
  protected intersector_id: number;

  protected intersector_data_source: IntersectedTuple[];
  protected intersector_displayed_columns = ['dim_number', 'dim_values_preview'];
  protected intersector_displayed_columns_names: Map<String, String> = new Map([
    ['dim_number', 'Dim'],
    ['dim_values_preview', 'Dim preview']
  ]);
  protected intersector_displayed_columns_with_expand = [...this.intersector_displayed_columns, 'expand'];
  protected expanded_element: IntersectedTuple | null;
  private max_dim_values_preview_length = 26;
  
  // protected intersector_data_source: MatTableDataSource<IntersectedTuple[]>;
  
  // expandedElement: IntersectedTuple | null

  

  constructor(public dialogRef: MatDialogRef<IntersectionDetailsDialogComponent>, 
      @Inject(MAT_DIALOG_DATA) public data: {intersection_details: IntersectionDetails}, private cdr: ChangeDetectorRef){

    this.identifier = data.intersection_details.identifier;
    this.total_untouched_percentage = data.intersection_details.total_untouched_percentage;
    this.total_intersection_percentage = data.intersection_details.total_intersection_percentage;

    let sorted_intersections: Map<number, [number, Array<Array<string>>]> = new Map([...data.intersection_details.intersections.entries()]
    .sort((a, b) => {
      return a[1][0] - b[1][0];
    }));
    this.intersections = sorted_intersections;

    let data_source: Array<Array<number>> = Array.from(this.intersections.keys(), key => [key])
    this.intersectors_data_source = new MatTableDataSource(data_source);
  }

  ngOnInit(): void { 
    console.log("Initializing intersection details dialog");
  }

  ngAfterViewInit(){
    let first_intersector = this.intersections.keys().next().value;
    this.selectIntersector(first_intersector); // Selects the first intersector
    this.cdr.detectChanges();
  }

  protected trackColumn(index: number, column: string): any {
    return column;
  }

  protected getColumnName(column: String): String {
    return this.intersector_displayed_columns_names.get(column);
  }

  protected selectIntersector(intersector_id: number){
    this.intersector_id = intersector_id;

    console.log(this.intersections.get(this.intersector_id));
    let intersected_dims: Array<Array<string>> = this.intersections.get(this.intersector_id)[1];

    let i = 0;
    let intersector_data_source: IntersectedTuple[] = [];
    intersected_dims.forEach(dim => {
      let values: Array<String> = [];
      let all_values_length = 0;

      dim.flat().forEach(value => {
        all_values_length += value.length;
        all_values_length += 1; // For the comma
        all_values_length += 1; // For the space
        values.push(" " + value);
      });

      let needs_expand: boolean;
      let dim_values_preview: Array<String> = [];
      if(all_values_length <= this.max_dim_values_preview_length - 2){ // -2 for the last comma and space
        dim_values_preview = values;
        needs_expand = false;
      }else{
        dim_values_preview.push("{" + values.length + " elements...}");
        needs_expand = true;
      }

      intersector_data_source.push(
        {dim_number: 'DIM' + (i+1), dim_values_preview: dim_values_preview, dim_values: values, needs_expand: needs_expand}
        );
      i++;
    });

    this.intersector_data_source = intersector_data_source;
  }

  protected expandRow(element: IntersectedTuple): void {
    if(element.needs_expand === false){
      this.expanded_element = null;
      return;
    }

    this.expanded_element = this.expanded_element === element ? null : element;
  }
  
}
### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.ts END ###

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.html BEGIN ###
<body>
    <header>
        <h1>Intersections for pattern {{identifier}}</h1>
        <span>Total un-intersected percentage: {{total_untouched_percentage*100 | number:'1.2-2'}}%</span> <br>
        <span>Total intersected percentage: {{total_intersection_percentage*100 | number:'1.2-2'}}%</span>
    </header>

    <section>
        <div id="intersectors_table_wrapper">
            <table mat-table [dataSource]="intersectors_data_source">
                
                <!-- Intersections column -->
                <ng-container matColumnDef="intersections">
                <th mat-header-cell *matHeaderCellDef>Intersections ({{ intersectors_data_source.data.length}})</th>
                <td mat-cell *matCellDef="let row"> {{row}} </td>
                </ng-container>
            
                <tr mat-header-row *matHeaderRowDef="intersectors_displayed_columns; sticky: true"></tr>
                <tr class="intersectors_data_row"
                    mat-row
                    (click)="selectIntersector(row[0])"
                    [class.selected-row]="this.intersector_id === row[0]"
                    *matRowDef="let row; columns: intersectors_displayed_columns;"
                ></tr>
                
            </table>
        </div>

        <div id="intersector_table_wrapper">
            
            <!-- Declares a table. The multiTemplateDataRows attribute allows multiple <ng-container> elements per row -->
            <table mat-table [dataSource]="intersector_data_source" multiTemplateDataRows>
                <!-- Creates a column for each item in the array.  -->
                <ng-container *ngFor="let column of intersector_displayed_columns; trackBy: trackColumn" matColumnDef="{{column}}" sticky>
                    <th mat-header-cell *matHeaderCellDef> {{getColumnName(column)}} </th>
                    <td mat-cell *matCellDef="let element"> {{element[column]}} </td>
                </ng-container>

                <!-- Defines an additional column for the expand/collapse button. The button changes its icon based on whether the
                current row is expanded or not -->
                <ng-container matColumnDef="expand">
                    <th mat-header-cell *matHeaderCellDef aria-label="row actions">&nbsp;</th>
                    <td mat-cell *matCellDef="let element">
                    <button mat-icon-button aria-label="expand row" 
                        (click)="(expanded_element = expanded_element === element ? null : element); 
                        $event.stopPropagation()"
                        [hidden]="!element.needs_expand">
                        
                        <mat-icon *ngIf="expanded_element === element">keyboard_arrow_up</mat-icon>
                        <mat-icon *ngIf="expanded_element !== element">keyboard_arrow_down</mat-icon>

                    </button>
                    </td>
                </ng-container>

                <!-- Expanded Content Column - The detail row is made up of this one column that spans across all columns -->
                <ng-container matColumnDef="expandedDetail">
                    <td mat-cell *matCellDef="let element" [attr.colspan]="intersector_displayed_columns_with_expand.length">
                    <div class="detail"

                    [@detailExpand]="element == expanded_element ? 'expanded' : 'collapsed'">
                        <div class="detail-value">
                            {{element.dim_values}}
                        </div>

                    </div>
                    </td>
                </ng-container>
                
                <!-- Creates a header row for the table. -->
                <tr mat-header-row *matHeaderRowDef="intersector_displayed_columns_with_expand"></tr>
                <!-- Defines the data rows. When a row is clicked, it toggles the expanded state of the row -->
                <tr mat-row *matRowDef="let element; columns: intersector_displayed_columns_with_expand;"
                    class="data-row"
                    [style.cursor]="element.needs_expand ? 'pointer' : 'default'"
                    [class.expanded-row]="expanded_element === element"
                    (click)="expandRow(element)">
                </tr>
                <!-- Defines the expanded detail row. -->
                <tr mat-row *matRowDef="let row; columns: ['expandedDetail']" class="detail-row"></tr>
            </table>

        </div>
    </section>
    
    <footer></footer>
</body>

### src/app/components/visualization/intersection-details-dialog/intersection-details-dialog.component.html END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.scss BEGIN ###
html, body{
    margin: 0 0 0 0;
    padding: 0 0 0 0;
    
    width: 100%;
    height: 100%;

    background-color: red;
}
### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.scss END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.ts BEGIN ###
import { Component, ElementRef, Input, Renderer2 } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';

@Component({
  selector: 'app-datapoint-tooltip',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './datapoint-tooltip.component.html',
  styleUrls: ['./datapoint-tooltip.component.scss']
})
export class DataPointTooltipComponent {
  protected visible: boolean = false;
  private datapoint: DataPoint;

  constructor(private elementRef: ElementRef, private renderer: Renderer2) {}

  ngOnInit(): void {
    console.log(this.datapoint)
  }

  public toggleVisibility(){
    this.visible = !this.visible;
  }

  public setDatapoint(datapoint: DataPoint){
    this.datapoint = datapoint;
  }

  public setPosition(top: number, left: number) {
    let top_str = top + 'px';
    let left_str = left + 'px';
    this.renderer.setStyle(this.elementRef.nativeElement, 'top', top_str);
    this.renderer.setStyle(this.elementRef.nativeElement, 'left', left_str);
  }
}

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.ts END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.html BEGIN ###
<body [hidden]="!visible">
    
</body>
### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.html END ###

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DataPointTooltipComponent } from './datapoint-tooltip.component';

describe('DatapointTooltipComponent', () => {
  let component: DataPointTooltipComponent;
  let fixture: ComponentFixture<DataPointTooltipComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ DataPointTooltipComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DataPointTooltipComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/visualization/datapoint-tooltip/datapoint-tooltip.component.spec.ts END ###

### src/app/components/main_options/rss-view/rss-view.component.html BEGIN ###
<body #rssWindow (window:resize)="onResize($event)">
    <div id="visualization_div" #visualization_div></div>
    <div id="slider-div">
        <mat-slider 
            class="example-margin"
            [disabled]="sliderDisabled"
            (input)="onSliderDrag($event)"
            (change)="onSliderChange($event)"
            [max]="rss_evolution.length"
            [min]="1"
            [step]="1"
            [discrete]="false"
            [showTickMarks]="true"
            >
        <input matSliderThumb [(ngModel)]="pattern_number" #slider>
        </mat-slider>

        <p>Number of patterns: {{pattern_number - 1}} </p>
    </div>
</body> 
### src/app/components/main_options/rss-view/rss-view.component.html END ###

### src/app/components/main_options/rss-view/rss-view.component.scss BEGIN ###
@use '@angular/material' as mat;
@import 'src/theme.scss';

html,body{
    height: 100%;
    width: 100%;
    padding: 0 0 0 0;
    margin : 0 0 0 0;

    overflow: hidden;
    
    background-color: mat.get-color-from-palette($primary-palette, 900);
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    user-select: none;

    #drawer{
        z-index: 0;
        background-color: red;
        width: 10%;
        height: 10%;
        user-select: none;
    }

    #drawer:hover{
        cursor: pointer;
    }

    #visualization_div{
        z-index: 0;
        position: relative;
        object-fit:contain;
        width: 90%;
        height: 25%; // Of available space

        margin-top: 2em;

        border: 4px solid mat.get-color-from-palette($primary-palette, 700);
        user-select: none;
        background-color: mat.get-color-from-palette($primary-palette, '900-contrast');
    }

    #slider-div{
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        width: 87%;
        height: 15%;

        padding-right: 1%;
        padding-left: 1%;
        padding-top: 0%;
        // padding-bottom: 1%;

        mat-slider{
            // color: white;
        }

        .mat-mdc-slider {
            width: 100%;
            // background-color: brown;
            
        }      

        p{
            user-select: none;
            color: mat.get-color-from-palette($primary-palette, '900-contrast');
        }
        // background-color: yellow;
    }

    #placeholder{
        height: 60%;
        background-color: red;
    }
}
### src/app/components/main_options/rss-view/rss-view.component.scss END ###

### src/app/components/main_options/rss-view/rss-view.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RssViewComponent } from './rss-view.component';

describe('RssViewComponent', () => {
  let component: RssViewComponent;
  let fixture: ComponentFixture<RssViewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ RssViewComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RssViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/main_options/rss-view/rss-view.component.spec.ts END ###

### src/app/components/main_options/rss-view/rss-view.component.ts BEGIN ###
import { resolveResource } from '@tauri-apps/api/path'
import * as d3 from 'd3';
import { Component, ElementRef, EventEmitter, Output, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSliderModule} from '@angular/material/slider';
import { SvgService } from 'src/app/services/svg/svg.service';
import { FormsModule } from '@angular/forms';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatCardModule} from '@angular/material/card';
import { DataPoint } from 'src/app/models/datapoint';
import { fs, invoke } from '@tauri-apps/api';
import { ChangeDetectorRef } from '@angular/core';
import { AfterViewInit } from '@angular/core'
import { Color } from 'src/app/models/color';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs';
import { environment } from 'src/environments/environment';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { ApiService } from 'src/app/services/api/api.service';

@Component({
  selector: 'app-rss-view',
  standalone: true,
  imports: [
    CommonModule,
    MatSliderModule,
    FormsModule
  ],
  templateUrl: './rss-view.component.html',
  styleUrls: ['./rss-view.component.scss']
})
export class RssViewComponent implements AfterViewInit{
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;

  @ViewChild('visualization_div') visualization_div: ElementRef<HTMLDivElement>;
  private svg: any;
  private plot: any;

  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;
  
  protected sliderDisabled: boolean = false;
  @Output() onTruncation: EventEmitter<any> = new EventEmitter();
  @Output() initialized: EventEmitter<any> = new EventEmitter();

  public rss_evolution: Array<number> = [];
  private datapoints: Array<DataPoint>;
  private scaled_datapoints: Array<DataPoint>;
  protected pattern_number;

  constructor(private route: ActivatedRoute, private dialog_service: DialogService, private api_service: ApiService){}
  
  async ngAfterViewInit() {
    this.rss_evolution = await this.api_service.getFullRssEvolution();
    let subpatterns_identifiers: number[] = await this.api_service.getAllSubpatternsIdentifiers();
    
    this.pattern_number = this.rss_evolution.length;
    this.datapoints = this.wrapIntoDatapoints(this.rss_evolution, subpatterns_identifiers);
    
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg = this.createSvg();
    this.resizeSvg(width, height, 0);
    this.drawDataPoints();

    this.connectDatapoints();
    this.initialized.emit();
  }

  private wrapIntoDatapoints(rss_evolution: Array<number>, subpatterns_identifiers: number[]): Array<DataPoint>{
    let datapoints: DataPoint[] = [];
    let subpatterns_identifiers_set: Set<number> = new Set(subpatterns_identifiers);

    let gray_shade = 160;
    for (let i = 0; i < rss_evolution.length; i++){
      let identifier = i; // i because the first index is the null model rss
      let x = undefined;
      let y = undefined;

      let r = 0;
      let g = 0;
      let b = 0;
      let a = 1;
      if(subpatterns_identifiers_set.has(identifier)){
        r = gray_shade;
        g = gray_shade;
        b = gray_shade;
      }

      let datapoint = new DataPoint(i, 10, 10, 0, 0, x, y, r, g, b, a);
      datapoints[i] = datapoint;
    }

    return datapoints;
  }

  private scalingFunction(datapoints: Array<DataPoint>): Array<any>{
    let min_rss = Math.min(...this.rss_evolution.map(rss => Math.abs(rss)));
    let max_rss = Math.max(...this.rss_evolution.map(rss => Math.abs(rss)));

    let max_y = max_rss;
    let y_range = max_rss - min_rss;
    let length = datapoints.length;

    let lateral_screen_coverage = 1;
    let vertical_screen_coverage = 0.9;
    let scaled_datapoints: Array<DataPoint> = datapoints;
    for (let i = 0; i < datapoints.length; i++){
      let datapoint: DataPoint = datapoints[i];
      let rss = this.rss_evolution[i];
      
      let x = ((i + 0.5)/length) * 2 - 1; // scale x to be between -1 and 1
      x /= ((1-lateral_screen_coverage) + 1)
      
      let y = (rss - min_rss) / y_range; // Scale y to be between 0 and 1
      y = y * 2 - 1; // Scale y to be between -1 and 1
      y /= ((1-vertical_screen_coverage) + 1)
      
      let radius = 3;
      let scaled_datapoint = new DataPoint(i, radius, 10, 0, 0, x, y, datapoint.r, datapoint.g, datapoint.b, datapoint.a);
      scaled_datapoints[i] = scaled_datapoint;
    }

    return scaled_datapoints;
  }

  private drawDataPoints() {
    if(this.plot == undefined){ return; }
  
    this.scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.plot.selectAll('circle')
        .data(this.scaled_datapoints, d => d.identifier); // Each datapoint has a unique identifier
  
    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', d => d.size)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer'); // Set cursor to pointer
  }

  private connectDatapoints(){
    console.log("Connecting datapoints");
    if(this.scaled_datapoints.length < 2){ return; }

    let line = d3.line<DataPoint>()
      .x(d => this.x_scale(d.x))
      .y(d => this.y_scale(d.y));

    for(let i = 0; i < this.scaled_datapoints.length - 1; i++) {
      let point1 = this.scaled_datapoints[i];
      let point2 = this.scaled_datapoints[i+1];

      this.plot.append('path')
        .attr('d', line([point1, point2]))
        .attr('stroke', 'black')
        .attr('stroke-width', 2)
        .attr('fill', 'none');
    }
  }

  public enableSlider(){ this.sliderDisabled = false; }
  public disableSlider(){ this.sliderDisabled = true; }
  
  protected onSliderChange(event: any) {
    this.onTruncation.emit(this.pattern_number);
  }

  protected onSliderDrag(event: any) {
    let x = this.datapoints[this.pattern_number - 1].x;
    this.drawVerticalLine(x);
  }

  public onResize(event) {
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.resizeSvg(width, height);
    this.drawDataPoints();
    this.connectDatapoints();
  }
  
  public getPatternNumber(): number{  
    return this.pattern_number;
  }

  public setPatternNumber(pattern_number: number){
    this.pattern_number = pattern_number;
  }

  // ========================= SVG FUNCTIONS ========================= //

  private createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
    .append('svg')
      .attr('width', this.svg_width)
      .attr('height',this.svg_height);

    return svg;
  }

  public resizeSvg(width: number, height: number, y_correction=0){
    this.svg
      .attr('width', width)
      .attr('height', height);

    let x_scale;

    x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, (width/1)]);

    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([(height - y_correction)/1, 0]);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.svg_width = width;
    this.svg_height = height;

    this.createPlot();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }

    // Add the X gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", "lightgrey")
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    // Add the Y gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("color", "lightgrey")
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }
  
  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g").attr("id", "plot");
    
    this.drawGridLines();
  }

  private drawVerticalLine(x: number) {
    this.plot.selectAll('#vertical-line').remove();

    this.plot.append('line')
        .attr('id', 'vertical-line')
        .attr('x1', this.x_scale(x))
        .attr('y1', 0)
        .attr('x2', this.x_scale(x))
        .attr('y2', this.svg_height)
        .attr('stroke', 'red')
        .attr('stroke-width', 2);
  }

  // ========================= SVG FUNCTIONS ========================= //
}



### src/app/components/main_options/rss-view/rss-view.component.ts END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.ts BEGIN ###
import { open } from '@tauri-apps/api/dialog';
import {Component, EventEmitter, Inject, Input, NgModule, Output} from '@angular/core';
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
  public static WIDTH = '45vw';
  public static HEIGHT = '50vh';
  @Output() modelChange: EventEmitter<any> = new EventEmitter();

  private last_opened_folder: string;
  
  private tensor_path: string = "";
  protected tensor_name: string = "";

  private patterns_path: string = "";
  protected patterns_name: string = "";

  constructor(public dialogRef: MatDialogRef<FileSelectionDialogComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: {last_opened_folder: string, tensor_path: string, patterns_path: string}) {
      this.last_opened_folder = data.last_opened_folder;
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
      multiple: false,
      defaultPath: this.last_opened_folder
    };
    const selected = await open(options);
    if (selected === null) { return; } // No tensor selected
  
    this.tensor_path = selected.toString();
    this.setNames();
    if (this.tensor_path == ""){ return; } // No tensor selected

    this.last_opened_folder = this.tensor_path;
  }
  
  public async selectPatterns(){
    const options = {
      multiple: false,
      defaultPath: this.last_opened_folder
    };
    const selected = await open(options);
    if (selected === null) { return; } // No patterns selected
    
    this.patterns_path = selected.toString();
    this.setNames();
    if (this.patterns_path == ""){ return; } // No patterns selected

    this.last_opened_folder = this.patterns_path;
  }

  protected submit() {
    if (this.isStateValid()){
      this.dialogRef.close({last_opened_folder: this.last_opened_folder, tensor_path: this.tensor_path, patterns_path: this.patterns_path});
    }else{
      this.dialogRef.close({last_opened_folder: "", tensor_path: null, patterns_path: null});
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
### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.ts END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.html BEGIN ###
<body>
    <h1 id="title" mat-dialog-title>Select resume</h1>

    <div id="dialog-content" mat-dialog-content>
        Select the source tensor file and the patterns you want to visualize.
    </div>

    <div id="file-selection-wrapper">
        <div class="file-selector" id="tensor-selection">
            <h2>Tensor</h2>
            <div class="clickable">
                <mat-icon (click)="selectTensor()"> attach_file </mat-icon>
                <p>{{this.tensor_name}}</p>
            </div>
        </div>
        <div class="file-selector" id="patterns-selection">
            <h2>Patterns</h2>
            <div class="clickable">
                <mat-icon (click)="selectPatterns()"> attach_file </mat-icon>
                <p>{{this.patterns_name}}</p>
            </div>
        </div>
    </div>

    <div id="dialog-actions" mat-dialog-actions>
        <button mat-button mat-dialog-close cdkFocusInitial>Close</button>
        <button mat-button mat-dialog-close (click)="submit()">Ok</button>
    </div>
</body>
### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.html END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FileSelectionDialogComponent } from './file-selection-dialog.component';

describe('FileSelectionDialogComponent', () => {
  let component: FileSelectionDialogComponent;
  let fixture: ComponentFixture<FileSelectionDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ FileSelectionDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FileSelectionDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.spec.ts END ###

### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.scss BEGIN ###
@use '@angular/material' as mat;
@import 'src/theme.scss';
body{
    margin: 0 0 0 0;
    padding: 0 0 0 0;
    
    display: flex;
    flex-direction: column;
    // background-color: red;

    width: 100%;
    height: 100%;

    #title{
        margin: 0 0 0 0;
        flex: 0.5;
        // background-color: blue;
    }

    #dialog-content{
        flex: 0.75;
        padding: 1em 1em 1em 1em;
        // background-color: green;
    }

    #file-selection-wrapper{
        flex: 3;
        // background-color: yellow;
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        align-items: center;

        .file-selector{
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;

            // background-color: orange;
            width: 12em;
            height: 12em;

            h2{
                margin: 0 0 0 0;

                font-size: 1.25em;
                text-align: center;
            }

            .clickable{
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                width: 100%;
                height: 100%;

                padding: 1em 0 0 0;

                // background-color: green;
                
                mat-icon{
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;

                    font-size: 3.5em;
                    width: 50%;
                    height: 50%;

                    padding: 0.15em 0.10em 0.15em 0.10em;
                    
                    color: mat.get-color-from-palette($primary-palette, '50-contrast');
                    background-color: mat.get-color-from-palette($primary-palette, 50);

                    border: 2px solid mat.get-color-from-palette($primary-palette, 100);
                    border-radius: 10px;
                }

                mat-icon:hover{
                    background-color: mat.get-color-from-palette($primary-palette, 100);
                    cursor: pointer;
                }

                p{  
                    padding: 1em 0 0 0;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                    
                    max-width: 100%;

                    // background-color: red;
                }
            }
        }
    }

    #dialog-actions{
        display: flex;
        flex-direction: row;
        justify-content: flex-end;
        align-items: center;

        flex: 0.5;
        // background-color: purple;
    }
}
### src/app/components/main_options/file-selection-dialog/file-selection-dialog.component.scss END ###

### src/app/components/pattern-summary/pattern-summary.component.html BEGIN ###
<body>
    <div id="pattern-preview" *ngIf="pattern">
        <div id="info">
            <h1>Pattern {{pattern.identifier}}</h1>
            <div id="details">
                <span>Size: {{pattern.size}}</span> <br>
                <span>Density: {{pattern.density}}</span>
            </div>
        </div>
    
        <section>
            <div id="dim-summaries">
                <div class="dim-summary" *ngFor="let dim_values of this.pattern.dims_values; let i = index" (click)="openDimDialog(i)">
                    <div class="dim-summary-header">
                        <h2>DIM {{i+1}}</h2>
                        <span>({{dim_values.length}} elements)</span> <br>
                    </div>
                    <div class="dim-summary-content">
                        <span class="element"> {{this.formatDimValues(dim_values)}} </span>
                    </div>
                </div>
            </div>
        </section>
        
        <footer></footer>
    </div>
</body>

### src/app/components/pattern-summary/pattern-summary.component.html END ###

### src/app/components/pattern-summary/pattern-summary.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PatternSummaryComponent } from './pattern-summary.component';

describe('PatternSummaryComponent', () => {
  let component: PatternSummaryComponent;
  let fixture: ComponentFixture<PatternSummaryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ PatternSummaryComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PatternSummaryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/pattern-summary/pattern-summary.component.spec.ts END ###

### src/app/components/pattern-summary/pattern-summary.component.scss BEGIN ###
@use '@angular/material' as mat;
@import '../../../theme.scss';

body{
    padding: 0 0 0 0;
    margin: 0 0 0 0;

    width: 100%;
    height: 100%;
    // background-color: red;

    overflow-x: hidden; /* Hide horizontal scrollbar */

    #info{
        padding: 0em 0 1em 2em;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: flex-start;

        // background-color: yellow;

        h1{
            margin: 0 0 0 0;
            // background-color: red;
        }

        #details{
            // background-color: green;
            display: flex;
            flex-direction: row;

            align-items: center;
            justify-content: flex-start;

            span{
                
                padding-right: 1em;
            }
        }
    }

    section{
        height: 100%;

        #dim-summaries{
            .dim-summary{
                padding: 1em 0 1em 2em;
                
                width: 100%;
                max-height: 15em;
                // background-color: red;

                overflow: hidden;
                user-select: none;
                
                .dim-summary-header{
                    display: flex;
                    flex-direction: row;

                    align-items: center;

                    h2,span{
                        margin: 0 0 0 0;
                        padding: 0 0 0 0;
                    }

                    h2{
                        padding-right: 0.5em;
                    }
                }

                .dim-summary-content{
                    display: inline-block;
                    width: 85%;

                    // white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    word-wrap: break-word;

                    .element{
                        word-wrap: break-word;
                        overflow: hidden;
                        text-overflow: ellipsis;
                    }
                }
            }

            .dim-summary:hover{
                cursor: pointer;
                background-color: mat.get-color-from-palette($primary-palette, 800);
            }
        }
    }

    
}
### src/app/components/pattern-summary/pattern-summary.component.scss END ###

### src/app/components/pattern-summary/pattern-summary.component.ts BEGIN ###
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

  async ngOnInit(): Promise<void> {
    console.log("Initializing PatternSummaryComponent");
    // await this.update(1); // TODO: Retirar
    // this.openDimDialog(1); // TODO: Retirar
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

    if((!this.pattern) || (identifier != this.pattern.identifier)){ // Lock on another pattern
      this.locked = false;
      this.update(identifier);
    }

    this.locked = !this.locked;
  }
}

### src/app/components/pattern-summary/pattern-summary.component.ts END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.ts BEGIN ###
import { Component, Inject, Input, OnInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';

@Component({
  selector: 'app-pattern-dim-dialog',
  standalone: true,
  imports: [CommonModule, MatFormFieldModule, MatPaginatorModule, 
    MatInputModule, MatTableModule],
  templateUrl: './pattern-dim-dialog.component.html',
  styleUrls: ['./pattern-dim-dialog.component.scss']
})
export class PatternDimDialogComponent implements OnInit{
  public static WIDTH = '40vw';
  public static HEIGHT = '60vh';

  private dim_values: string[];
  protected displayed_columns: string[] = ['Elements'];
  // protected data_source: MatTableDataSource<Array<any>>;
  protected data_source;

  @ViewChild("input") input: HTMLInputElement;
  
  constructor(public dialogRef: MatDialogRef<PatternDimDialogComponent>, @Inject(MAT_DIALOG_DATA) public data: {dim_values: string[]}) {
    this.dim_values = data.dim_values;
    this.data_source = new MatTableDataSource(data.dim_values);
  }

  ngOnInit(): void {
    
  }

  protected applyFilter(event: Event) {
    this.data_source.data = this.dim_values;
    this.input = (event.target as HTMLInputElement);

    const filterValue = (event.target as HTMLInputElement).value.trim().toLowerCase();

    let filteredData = this.data_source.data.filter(item => {
        let itemStr = JSON.stringify(item).toLowerCase();
        return itemStr.includes(filterValue);
    });
    
    this.data_source.data = filteredData;
  }
}

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.ts END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.scss BEGIN ###
body{
    padding: 0 0 0 0;
    margin: 0 0 0 0;

    width: 100%;
    height: 100%;

    // background-color: red;

    display: flex;
    flex-direction: row;

    overflow: hidden;
    
    #table-wrapper{
        width: 70%;
        height: 90%;
        overflow-y: auto;
        overflow-x: auto;
    }

    #filter{
        overflow: hidden;
        padding-top: 1em;
        
        padding-left: 2em;
        width: 20%;
        height: 20%;
    }
}
### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.scss END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.html BEGIN ###
<body>
    <div id="table-wrapper">
        <table mat-table [dataSource]="data_source">
            <!-- Elements column -->
            <ng-container matColumnDef="Elements">
            <th mat-header-cell *matHeaderCellDef>Elements ({{ data_source.data.length}})</th>
            <td mat-cell *matCellDef="let row"> {{row}} </td>
            </ng-container>
        
            <tr mat-header-row *matHeaderRowDef="displayed_columns; sticky: true"></tr>
            <tr class="intersectors_data_row" mat-row *matRowDef="let row; columns: displayed_columns;"></tr>
        </table>
    </div>

    <mat-form-field id="filter" class="filter">
        <mat-label>Filter</mat-label>
        <input #input matInput (keyup)="applyFilter($event)" placeholder="">
    </mat-form-field>
</body>
### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.html END ###

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PatternDimDialogComponent } from './pattern-dim-dialog.component';

describe('PatternDimDialogComponent', () => {
  let component: PatternDimDialogComponent;
  let fixture: ComponentFixture<PatternDimDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ PatternDimDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PatternDimDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/pattern-summary/pattern-dim-dialog/pattern-dim-dialog.component.spec.ts END ###

### src/app/components/error-dialog/error-dialog.component.scss BEGIN ###
body{
    overflow: hidden;
    padding: 0 0 0 0;
    margin: 0 0 0 0;
    
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    // background-color: red;

    width: 100%;
    height: 100%;

    header{
        width: 100%;
        display: flex;
        flex-direction: column;
        // background-color: blue;
        
        #title{
            display: flex;
            flex-direction: row;
            align-items: center;
            padding: 1em 0 0 1em;

            // background-color: green;
            
            mat-icon{
                color: red;
            }
    
            h1{
                margin: 0 0 0 0;
                padding: 0 0 0 0.5em;
                // background-color: yellow;
                // height: 20%;
            }
        }
    
        span{
            padding: 0.5em 0 0 1em;
            width: 100%;
            height: 10%;
            // background-color: blue;
        }
    }

    section{
        padding: 1em 0 0 2em;
        width: 100%;
        height: 40%;
        // background-color: green;
        
        color: red;
    }

    #dialog-actions{
        // height: 10%;
        // background-color: yellow;
    }
}
### src/app/components/error-dialog/error-dialog.component.scss END ###

### src/app/components/error-dialog/error-dialog.component.ts BEGIN ###
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
  public static WIDTH = '30vw';
  public static HEIGHT = '30vh';

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
### src/app/components/error-dialog/error-dialog.component.ts END ###

### src/app/components/error-dialog/error-dialog.component.html BEGIN ###
<body>
    <header>
        <div id="title">
            <mat-icon>error</mat-icon>
            <h1>ERROR</h1>
        </div>
        
        <span>An error occurred while executing the application:</span>
    </header>
    
    <section>{{this.error_message}}</section>

    <div id="dialog-actions" mat-dialog-actions>
        <button mat-button mat-dialog-close cdkFocusInitial (click)="submit()">Close</button>
    </div>
</body>
### src/app/components/error-dialog/error-dialog.component.html END ###

### src/app/components/error-dialog/error-dialog.component.spec.ts BEGIN ###
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ErrorDialogComponent } from './error-dialog.component';

describe('ErrorDialogComponent', () => {
  let component: ErrorDialogComponent;
  let fixture: ComponentFixture<ErrorDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ ErrorDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ErrorDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

### src/app/components/error-dialog/error-dialog.component.spec.ts END ###

### src/app/models/color.ts BEGIN ###
export class Color{
    r: number;
    g: number;
    b: number;
}
### src/app/models/color.ts END ###

### src/app/models/intersection_details.ts BEGIN ###
export class IntersectionDetails{
    identifier: number;
    total_untouched_percentage: number;
    total_intersection_percentage: number;
    intersections: Map<number, [number, Array<Array<string>>]>;

    constructor (identifier: number, total_untouched_percentage: number, total_intersection_percentage: number, 
        intersections: Map<number, [number, Array<Array<string>>]>){

        this.identifier = identifier;
        this.total_untouched_percentage = total_untouched_percentage;
        this.total_intersection_percentage = total_intersection_percentage;
        this.intersections = intersections;
    }
}
### src/app/models/intersection_details.ts END ###

### src/app/models/pattern.ts BEGIN ###
export class Pattern{
    identifier:number;
    dims_values: Array<Array<string>>;
    density: number;
    size: number;

    constructor(identifier: number, dims_values: Array<Array<string>>, density: number, size: number){
        this.identifier = identifier;
        this.dims_values = dims_values;
        this.density = density;
        this.size = size;
    }

    public static fromResponse(response: any): Pattern{
        return new Pattern(response.identifier, response.dims_values, response.density, response.size);
    }
}
### src/app/models/pattern.ts END ###

### src/app/models/svg.ts BEGIN ###
// import * as d3 from 'd3';
// import { ElementRef } from '@angular/core';
// import { DataPoint } from './datapoint';
// import { event } from '@tauri-apps/api';

// export class Svg {
//     public d3_svg: any;
//     public plot: any;

//     private width: number;
//     private height: number;

//     private x_scale: any;
//     private y_scale: any;

//     private gridlines: boolean;
//     private number_of_gridlines: number;

//     private pannable: boolean;

//     private initial_scale: number;

//     constructor(vizualization_div: ElementRef<HTMLDivElement>, width: number, height: number, 
//                 number_of_gridlines: number = 40, gridlines: boolean = true, pannable: boolean = true){

//         this.width = width;
//         this.height = height;
//         this.number_of_gridlines = number_of_gridlines;
//         this.gridlines = gridlines;
//         this.pannable = pannable;
//         this.create(vizualization_div);
//     }

//     private create(vizualization_div: ElementRef<HTMLDivElement>){
//       this.d3_svg = d3.select(vizualization_div.nativeElement)
//       .append('svg')
//         .attr('width', this.width)
//         .attr('height',this.height);
//     }

//     public resize(width: number, height: number, y_correction=0){
//       this.d3_svg
//         .attr('width', width)
//         .attr('height', height);

//       let x_scale;

//       if(this.pannable){ // Only the pannable visualization will have square aspect ratio
//         x_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([0, (height - y_correction)/1]);

//       }else if(!this.pannable){
//         x_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([0, (width/1)]);
//       }
  
//       let y_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([(height - y_correction)/1, 0]);

//       this.x_scale = x_scale;
//       this.y_scale = y_scale;
//       this.width = width;
//       this.height = height;
  
//       this.createPlot();
//     }

//     private drawGridLines() {
//       let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
//       let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
  
//       // Add the X gridlines
//       this.plot.append("g")			
//         .attr("class", "grid")
//         .attr("transform", "translate(0," + this.height + ")")
//         .attr("color", "lightgrey")
//         .call(makeXGridlines()
//             .tickSize(-this.height)
//             .tickFormat(() => "")
//         )
  
//       // Add the Y gridlines
//       this.plot.append("g")			
//         .attr("class", "grid")
//         .attr("color", "lightgrey")
//         .call(makeYGridlines()
//             .tickSize(-1 * this.width)
//             // .tickSize(-300)
//             .tickFormat(() => "")
//         )
//     }
    
//     private createPlot(){
//       if(this.plot != undefined){ this.d3_svg.select("#plot").remove(); }
//       this.plot = this.d3_svg.append("g").attr("id", "plot");
      
//       if(this.pannable){ // Only the pannable square visualization will execute this
//         let panning_zoom = d3.zoom()
//           .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
//           // .translateExtent([[0, 0], [this.height, this.height/1.2]])
//           .translateExtent([[0, 0], [this.height, this.height]])
//           .on("start", (event, d) => { this.d3_svg.attr("cursor", "grabbing"); })
//           .on("zoom", (event) => { this.plot.attr("transform", event.transform); })
//           .on("end", (event, d) => {this.d3_svg.attr("cursor", "default")});
    
//         this.d3_svg.call(panning_zoom);

//         // Apply initial zoom level
//         this.initial_scale=  1.4;
//         let x_translation_factor = 0.0;
//         // let y_translation_factor = 0.15;
//         let y_translation_factor = 0.2;
//         let initial_transform = d3.zoomIdentity
//           .translate(-this.width*(x_translation_factor), -this.height*(y_translation_factor))
//           // .translate(-this.width*(x_translation_factor), 0)
//           .scale(this.initial_scale);
//         this.d3_svg.call(panning_zoom.transform, initial_transform);
//       }
      
//       if(this.gridlines){ this.drawGridLines(); }
//       // this.drawDataPoints();
//     }

//     public drawVerticalLine(x: number) {
//       // Remove any existing line
//       this.plot.selectAll('#vertical-line').remove();
  
//       // Draw a new line
//       this.plot.append('line')
//           .attr('id', 'vertical-line')
//           .attr('x1', this.x_scale(x))
//           .attr('y1', 0)
//           .attr('x2', this.x_scale(x))
//           .attr('y2', this.height)
//           .attr('stroke', 'red')
//           .attr('stroke-width', 2);
//   }

//   public getXScale(){
//     return this.x_scale;
//   }

//   public getYScale(){
//     return this.y_scale;
//   }

//   public getInitialScale(){
//     return this.initial_scale;
//   }
// }
### src/app/models/svg.ts END ###

### src/app/models/datapoint.ts BEGIN ###
export class DataPoint{
    identifier: number;
    size: number;
    pattern_size: number;
    density: number;
    stroke_width: number;

    x: number;
    y: number;

    r: number;
    g: number;
    b: number;
    a: number;

    constructor(identifier: number, size: number,  pattern_size: number, density: number, stroke_width: number, x: number, y: number, r: number, g: number, b: number, a:number){
        this.identifier = identifier;
        this.pattern_size = pattern_size;
        this.size = size;
        this.density = density;
        this.stroke_width = stroke_width;
        this.x = x;
        this.y = y;
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }
}
### src/app/models/datapoint.ts END ###

### src/app/services/svg/svg.service.spec.ts BEGIN ###
import { TestBed } from '@angular/core/testing';

import { SvgService } from './svg.service';

describe('SvgService', () => {
  let service: SvgService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SvgService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

### src/app/services/svg/svg.service.spec.ts END ###

### src/app/services/svg/svg.service.ts BEGIN ###
import { ElementRef, Injectable } from '@angular/core';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';

@Injectable({
  providedIn: 'root'
})
export class SvgService {

  constructor() { }

  



  // private drawGridLines() {
  //   let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(40) }
  //   let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(40) }

  //   // Add the X gridlines
  //   this.plot.append("g")			
  //     .attr("class", "grid")
  //     .attr("transform", "translate(0," + this.height + ")")
  //     .attr("color", "grey")
  //     .call(makeXGridlines()
  //         .tickSize(-this.height)
  //         .tickFormat(() => "")
  //     )

  //   // Add the Y gridlines
  //   this.plot.append("g")			
  //     .attr("class", "grid")
  //     .attr("color", "grey")
  //     .call(makeYGridlines()
  //         .tickSize(-1 * this.width)
  //         .tickFormat(() => "")
  //     )
  // }

  // private createPlot(svg: any, width: number, height: number){
  //   svg.select("#plot").remove();
  //   let plot = svg.append("g").attr("id", "plot");
  
  //   let panning_zoom = d3.zoom()
  //     .scaleExtent([1, 10]) // This control how much you can unzoom (x1) and zoom (x10)
  //     .translateExtent([[0, 0], [width, height]])
  //     .on("start", (event, d) => { svg.attr("cursor", "grabbing"); })
  //     .on("zoom", (event) => { plot.attr("transform", event.transform); })
  //     .on("end", (event, d) => {svg.attr("cursor", "default")});
  
  //   svg.call(panning_zoom);
  
  //   // Apply initial zoom level
  //   let initial_scale = 1.2;
  //   let translation_factor = 0.1;
  //   let initial_transform = d3.zoomIdentity
  //     .translate(-width*(translation_factor), -height*(translation_factor))
  //     .scale(initial_scale);
  //   svg.call(panning_zoom.transform, initial_transform);
  
  //   this.drawGridLines();
  //   this.drawDataPoints();
  // }


}

### src/app/services/svg/svg.service.ts END ###

### src/app/services/dialog/dialog.service.ts BEGIN ###
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

### src/app/services/dialog/dialog.service.ts END ###

### src/app/services/dialog/dialog.service.spec.ts BEGIN ###
import { TestBed } from '@angular/core/testing';

import { DialogService } from './dialog.service';

describe('DialogService', () => {
  let service: DialogService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DialogService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

### src/app/services/dialog/dialog.service.spec.ts END ###

### src/app/services/api/api.service.spec.ts BEGIN ###
import { TestBed } from '@angular/core/testing';

import { ApiService } from './api.service';

describe('ApiService', () => {
  let service: ApiService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ApiService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

### src/app/services/api/api.service.spec.ts END ###

### src/app/services/api/api.service.ts BEGIN ###
import { Injectable } from '@angular/core';
import { fs, invoke } from '@tauri-apps/api';
import { Pattern } from 'src/app/models/pattern';
import { environment } from 'src/environments/environment';
import { DialogService } from '../dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { DataPoint } from 'src/app/models/datapoint';
import { IntersectionDetails } from 'src/app/models/intersection_details';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  constructor(private dialog_service: DialogService) { }

  public async initApplication(tensor_path: string, patterns_path: string){
    await invoke("initApplication", {tensorPath: tensor_path, patternsPath: patterns_path}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("ERROR Could not read tensor or patterns.");
      throw error;
    });
  }

  public async getFullRssEvolution(): Promise<Array<number>> {
    console.log("Initializing rss view component");
    console.log("Invoking getFullRssEvolution");

    let rss_evolution;
    if(!environment.dev_mode){
      rss_evolution = await invoke("getFullRssEvolution").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Could not load rss graph.");
        throw error;
      });

    } else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/rss_evolution.json'));
      rss_evolution = JSON.parse(rawdata);
    }

    console.log("Received rss_evolution:");
    console.log(rss_evolution);

    return rss_evolution;
  }

  public async truncateModel(new_size: number): Promise<any>{
    console.log("Truncating datapoints to only: " + new_size);
    let truncated_datapoints;
    if(!environment.dev_mode){
      await invoke("truncateModel", {newSize: new_size}).catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while truncating datapoints.");
        throw error;
      });
  
      truncated_datapoints = await this.getDataPoints();
    }
    else if(environment.dev_mode) {
      let datapoints = await this.getDataPoints(); // Getting all original datapoints in dev mode
      truncated_datapoints = datapoints.slice(0, new_size);
    }

    return truncated_datapoints;
  }

  public async getIntersectionsPercentages(identifier: number): Promise<Map<number, number>> {
    let raw_data;
    if(!environment.dev_mode){
      raw_data = await invoke("getIntersectionsPercentages", {identifier: identifier})
        .catch((error: any) => {
          // console.error(error);
          this.dialog_service.openErrorDialog("Error while getting intersections.");
          throw error;
      });
      
    }else{
      raw_data = await fs.readTextFile(await resolveResource('resources/intersections2.json'));
      raw_data = JSON.parse(raw_data);
    }

    let intersections = new Map<number, number>();
    for (let key in raw_data) { intersections.set(Number(key), Number(raw_data[key])); }

    return intersections;
  }

  public async getIntersectionDetails(identifier: number): Promise<IntersectionDetails>{
    let data: any;
    if(!environment.dev_mode){
      data = await invoke("getIntersectionDetails", {identifier: identifier}).catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching intersection details.");
        throw error;
      });

    }else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/intersection_details.json'));
      data = JSON.parse(rawdata);
    }

    let intersections: Map<number, [number, Array<Array<string>>]> = new Map();
    for (let key in data.intersections) { 
      let value = data.intersections[key];
      let percentage = Math.round(value[0]*100)/100;
      let dims_intersections = value[1];
      intersections.set(Number(key), [percentage, dims_intersections]);
    }

    let intersection_details: IntersectionDetails = new IntersectionDetails(
      data.identifier,
      Math.round(data.total_untouched_percentage * 100)/100,
      Math.round(data.total_intersection_percentage * 100)/100,
      intersections
    );

    return intersection_details;
  }

  public async getPattern(identifier: number): Promise<Pattern> {
    let pattern;
    if(!environment.dev_mode){
      pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching pattern.");
        throw error;
      });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/pattern.json'));
      pattern = JSON.parse(rawdata);
      pattern.identifier = identifier;
    }
    
    return Pattern.fromResponse(pattern);
  }

  public async getDataPoints(): Promise<Array<DataPoint>> {
    console.log("Invoking getDataPoints");
    let datapoints;
    if(!environment.dev_mode){
      datapoints = await invoke("getDataPoints").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching data points.");
        throw error;
      });

    } else if (environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/datapoints2.json'));
      datapoints = JSON.parse(rawdata);
    }

    console.log("Received datapoints:");
    console.log(datapoints);

    return datapoints;
  }

  public async getAllSubpatternsIdentifiers(): Promise<Array<number>> {
    let subpatterns_identifiers;
    if(!environment.dev_mode){
      subpatterns_identifiers = await invoke("getAllSubPatternsIdentifiers").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching subpatterns identifiers.");
        throw error;
      });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/subpatterns_identifiers.json'));
      subpatterns_identifiers = JSON.parse(rawdata);
    }

    return subpatterns_identifiers;
  }
  
}

### src/app/services/api/api.service.ts END ###

### src/environments/environment.ts BEGIN ###
// This file can be replaced during build by using the `fileReplacements` array.
// `ng build` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,
  dev_mode: true,
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/plugins/zone-error';  // Included with Angular CLI.

### src/environments/environment.ts END ###

### src/environments/environment.prod.ts BEGIN ###
export const environment = {
  production: true,
  dev_mode: false,
};

### src/environments/environment.prod.ts END ###

### src/js/circle_legend.d.ts BEGIN ###
// https://observablehq.com/@harrystevens/circle-legend

declare module 'src/js/circle_legend.js' {
    export function legendCircle(context: any): any;
  }  
### src/js/circle_legend.d.ts END ###

### src/js/color_legend.js BEGIN ###
import * as d3 from 'd3';
// Copyright 2021, Observable Inc.
// Released under the ISC license.
// https://observablehq.com/@d3/color-legend
export const Legend = function (color, {
    title,
    tickSize = 6,
    width = 320, 
    height = 44 + tickSize,
    marginTop = 18,
    marginRight = 0,
    marginBottom = 16 + tickSize,
    marginLeft = 0,
    ticks = width / 64,
    tickFormat,
    tickValues
  } = {}) {
  
    function ramp(color, n = 256) {
      const canvas = document.createElement("canvas");
      canvas.width = n;
      canvas.height = 1;
      const context = canvas.getContext("2d");
      for (let i = 0; i < n; ++i) {
        context.fillStyle = color(i / (n - 1));
        context.fillRect(i, 0, 1, 1);
      }
      return canvas;
    }
  
    const svg = d3.create("svg")
        .attr("width", width)
        .attr("height", height)
        .attr("viewBox", [0, 0, width, height])
        .style("overflow", "visible")
        .style("display", "block");
  
    let tickAdjust = g => g.selectAll(".tick line").attr("y1", marginTop + marginBottom - height);
    let x;
  
    // Continuous
    if (color.interpolate) {
      const n = Math.min(color.domain().length, color.range().length);
  
      x = color.copy().rangeRound(d3.quantize(d3.interpolate(marginLeft, width - marginRight), n));
  
      svg.append("image")
          .attr("x", marginLeft)
          .attr("y", marginTop)
          .attr("width", width - marginLeft - marginRight)
          .attr("height", height - marginTop - marginBottom)
          .attr("preserveAspectRatio", "none")
          .attr("xlink:href", ramp(color.copy().domain(d3.quantize(d3.interpolate(0, 1), n))).toDataURL());
    }
  
    // Sequential
    else if (color.interpolator) {
      x = Object.assign(color.copy()
          .interpolator(d3.interpolateRound(marginLeft, width - marginRight)),
          {range() { return [marginLeft, width - marginRight]; }});
  
      svg.append("image")
          .attr("x", marginLeft)
          .attr("y", marginTop)
          .attr("width", width - marginLeft - marginRight)
          .attr("height", height - marginTop - marginBottom)
          .attr("preserveAspectRatio", "none")
          .attr("xlink:href", ramp(color.interpolator()).toDataURL());
  
      // scaleSequentialQuantile doesn’t implement ticks or tickFormat.
      if (!x.ticks) {
        if (tickValues === undefined) {
          const n = Math.round(ticks + 1);
          tickValues = d3.range(n).map(i => d3.quantile(color.domain(), i / (n - 1)));
        }
        if (typeof tickFormat !== "function") {
          tickFormat = d3.format(tickFormat === undefined ? ",f" : tickFormat);
        }
      }
    }
  
    // Threshold
    else if (color.invertExtent) {
      const thresholds
          = color.thresholds ? color.thresholds() // scaleQuantize
          : color.quantiles ? color.quantiles() // scaleQuantile
          : color.domain(); // scaleThreshold
  
      const thresholdFormat
          = tickFormat === undefined ? d => d
          : typeof tickFormat === "string" ? d3.format(tickFormat)
          : tickFormat;
  
      x = d3.scaleLinear()
          .domain([-1, color.range().length - 1])
          .rangeRound([marginLeft, width - marginRight]);
  
      svg.append("g")
        .selectAll("rect")
        .data(color.range())
        .join("rect")
          .attr("x", (d, i) => x(i - 1))
          .attr("y", marginTop)
          .attr("width", (d, i) => x(i) - x(i - 1))
          .attr("height", height - marginTop - marginBottom)
          .attr("fill", d => d);
  
      tickValues = d3.range(thresholds.length);
      tickFormat = i => thresholdFormat(thresholds[i], i);
    }
  
    // Ordinal
    else {
      x = d3.scaleBand()
          .domain(color.domain())
          .rangeRound([marginLeft, width - marginRight]);
  
      svg.append("g")
        .selectAll("rect")
        .data(color.domain())
        .join("rect")
          .attr("x", x)
          .attr("y", marginTop)
          .attr("width", Math.max(0, x.bandwidth() - 1))
          .attr("height", height - marginTop - marginBottom)
          .attr("fill", color);
  
      tickAdjust = () => {};
    }
  
    svg.append("g")
        .attr("transform", `translate(0,${height - marginBottom})`)
        .call(d3.axisBottom(x)
          .ticks(ticks, typeof tickFormat === "string" ? tickFormat : undefined)
          .tickFormat(typeof tickFormat === "function" ? tickFormat : undefined)
          .tickSize(tickSize)
          .tickValues(tickValues))
        .call(tickAdjust)
        .call(g => g.select(".domain").remove())
        .call(g => g.append("text")
          .attr("x", marginLeft)
          .attr("y", marginTop + marginBottom - height - 6)
          .attr("fill", "currentColor")
          .attr("text-anchor", "start")
          .attr("font-weight", "bold")
          .attr("class", "title")
          .text(title));
  
    return svg.node();
  }
### src/js/color_legend.js END ###

### src/js/color_legend.d.ts BEGIN ###
declare module 'src/js/color_legend.js' {
    export function Legend(
      color: any,
      options?: {
        title?: string,
        tickSize?: number,
        width?: number,
        height?: number,
        marginTop?: number,
        marginRight?: number,
        marginBottom?: number,
        marginLeft?: number,
        ticks?: number,
        tickFormat?: any,
        tickValues?: any[]
      }
    ): any;
  }
  
### src/js/color_legend.d.ts END ###

### src/js/circle_legend.js BEGIN ###
// https://observablehq.com/@harrystevens/circle-legend

export const legendCircle = function(context){
    let scale,
        tickValues,
        tickFormat = d => d,
        tickSize = 5;
    
    function legend(context){
      let g = context.select("g");
      if (!g._groups[0][0]){
        g = context.append("g");
      }
      g.attr("transform", `translate(${[1, 1]})`);
      
      const ticks = tickValues || scale.ticks();
      
      const max = ticks[ticks.length - 1];
      
      g.selectAll("circle")
          .data(ticks.slice().reverse())
        .enter().append("circle")
          .attr("fill", "none")
          .attr("stroke", "currentColor")
          .attr("cx", scale(max))
          .attr("cy", scale)
          .attr("r", scale);
      
      g.selectAll("line")
          .data(ticks)
        .enter().append("line")
          .attr("stroke", "currentColor")
          .attr("stroke-dasharray", "4, 2")
          .attr("x1", scale(max))
          .attr("x2", tickSize + scale(max) * 2)
          .attr("y1", d => scale(d) * 2)
          .attr("y2", d => scale(d) * 2);
      
      g.selectAll("text")
          .data(ticks)
        .enter().append("text")
          .attr("font-family", "'Helvetica Neue', sans-serif")
          .attr("font-size", 11)
          .attr("dx", 3)
          .attr("dy", 4)
          .attr("x", tickSize + scale(max) * 2)
          .attr("y", d => scale(d) * 2)
          .text(tickFormat);
    }
    
    legend.tickSize = function(_){
      return arguments.length ? (tickSize = +_, legend) : tickSize;
    }
    
    legend.scale = function(_){
      return arguments.length ? (scale = _, legend) : scale;
    }
  
    legend.tickFormat = function(_){
      return arguments.length ? (tickFormat = _, legend) : tickFormat;
    }
    
    legend.tickValues = function(_){
      return arguments.length ? (tickValues = _, legend) : tickValues;
    }
    
    return legend;
  }
### src/js/circle_legend.js END ###

### DIRECTORY src/ FLATTENED CONTENT ###
