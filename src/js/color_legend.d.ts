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
  