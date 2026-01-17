export interface PathItem extends Array<string> {
    0: string; // 路径
    1: string; // 名称
}

export interface ComVisibility {
  setting: {
      'setting-open': boolean;
      'setting-close': boolean;
  };
}

export type AllowIconType = ".jpg" | ".jepg" | ".JPG" | ".JEPG" | ".png" | ".PNG"


// 定义FloatingWindow的类型
export interface FloatingWindowState {
  "choose-path-pool"?: boolean;
  "choose-path-pool-close"?: boolean;
  "choose-function"?:  boolean,
  "choose-function-close"?:  boolean,
  "work-bench"?:  boolean,  
  "work-bench-close"?:  boolean,
}