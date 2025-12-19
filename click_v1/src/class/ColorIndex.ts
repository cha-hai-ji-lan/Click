
export type HexColor = `#${string}`  // #RRGGBB 十六位颜色定义类型
export type RGBColor = [number, number, number]   // RGB颜色定义类型
export type RGBAColor = [number, number, number, number]   // RGB颜色定义类型
export type ColorIndex = RGBColor| RGBAColor | HexColor  // 颜色索引类型

export type ColorSwitch = "Default" | "Light" | "Night"  // 颜色控制器类型 Default == Light
