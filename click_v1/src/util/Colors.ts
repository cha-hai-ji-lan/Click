import { type HexColor, RGBColor, RGBAColor, ColorSwitch } from '@/class/ColoeIndex'
import { ref } from 'vue'
    ;
export function ColorCtr() {
    // 特殊色彩
    const colseIconColor = ref<HexColor>("#fda5a5")                         // 关闭按钮颜色
    const colseIconHoverColor = ref<HexColor>("#ff5252")                    // 关闭按钮悬停颜色
    const colseIconActiveColor = ref<HexColor>("#f90303")                   // 关闭按钮活动颜色
    const minSizeIconColor = ref<HexColor>("#b3e7fc")                       // 最小化按钮颜色
    const minSizeIconHoverColor = ref<HexColor>("#56cfff")                  // 最小化按钮悬停颜色
    const minSizeIconActiveColor = ref<HexColor>("#00b7ff")                 // 最小化按钮活动颜色
    const maxSizeIconColor = ref<HexColor>("#f9d682")                       // 最大化按钮颜色
    const maxSizeIconHoverColor = ref<HexColor>("#ffc02d")                  // 最大化按钮悬停颜色
    const maxSizeIconActiveColor = ref<HexColor>("#fab104ff")               // 最大化按钮活动颜色

    const colseIconColorRGB = ref<RGBColor>([253, 165, 165])
    const colseIconHoverColorRGB = ref<RGBColor>([255, 82, 82])
    const colseIconActiveColorRGB = ref<RGBColor>([249, 3, 3])
    const minSizeIconColorRGB = ref<RGBColor>([179, 231, 252])
    const minSizeIconHoverColorRGB = ref<RGBColor>([86, 207, 255])
    const minSizeIconActiveColorRGB = ref<RGBColor>([0, 183, 255])
    const maxSizeIconColorRGB = ref<RGBColor>([249, 214, 130])
    const maxSizeIconHoverColorRGB = ref<RGBColor>([255, 192, 45])
    const maxSizeIconActiveColorRGB = ref<RGBColor>([250, 177, 4])

    const colseIconColorRGBA = ref<RGBAColor>([253, 165, 165, 1])
    const colseIconHoverColorRGBA = ref<RGBAColor>([255, 82, 82, 1])
    const colseIconActiveColorRGBA = ref<RGBAColor>([249, 3, 3, 1])
    const minSizeIconColorRGBA = ref<RGBAColor>([179, 231, 252, 1])
    const minSizeIconHoverColorRGBA = ref<RGBAColor>([86, 207, 255, 1])
    const minSizeIconActiveColorRGBA = ref<RGBAColor>([0, 183, 255, 1])
    const maxSizeIconColorRGBA = ref<RGBAColor>([249, 214, 130, 1])
    const maxSizeIconHoverColorRGBA = ref<RGBAColor>([255, 192, 45, 1])
    const maxSizeIconActiveColorRGBA = ref<RGBAColor>([250, 177, 4, 1])



    // 一般色彩     
    const titleBarColor = ref<HexColor>("#FFFFFF")                      // 标题栏颜色
    const toolBarColor = ref<HexColor>("#FFFFFF")                       // 工具栏颜色
    const foreGroundColor = ref<HexColor>("#FFFFFF")                    // 前景色
    const midGroundColor = ref<HexColor>("#FFFFFF")                     // 中景色
    const backGroundColor = ref<HexColor>("#FFFFFF")                    // 背景色
    const borderColor = ref<HexColor>("#FFFFFF")                        // 边框色
    const iconColor = ref<HexColor>("#FFFFFF")                          // 按钮颜色
    const iconHoverColor = ref<HexColor>("#FFFFFF")                     // 按钮悬停颜色
    const iconActiveColor = ref<HexColor>("#FFFFFF")                    // 按钮活动颜色
    const fontColor = ref<HexColor>("#FFFFFF")                          // 一般文字颜色

    const titleBarColorRGB = ref<RGBColor>([255, 255, 255])
    const toolBarColorRGB = ref<RGBColor>([255, 255, 255])
    const foreGroundColorRGB = ref<RGBColor>([255, 255, 255])
    const midGroundColorRGB = ref<RGBColor>([255, 255, 255])
    const backGroundColorRGB = ref<RGBColor>([255, 255, 255])
    const borderColorRGB = ref<RGBColor>([255, 255, 255])
    const iconColorRGB = ref<RGBColor>([255, 255, 255])
    const iconHoverColorRGB = ref<RGBColor>([255, 255, 255])
    const iconActiveColorRGB = ref<RGBColor>([255, 255, 255])
    const fontColorRGB = ref<RGBColor>([255, 255, 255])

    const titleBarColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const toolBarColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const foreGroundColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const midGroundColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const backGroundColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const borderColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const iconColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const iconHoverColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const iconActiveColorRGBA = ref<RGBAColor>([255, 255, 255, 1])
    const fontColorRGBA = ref<RGBAColor>([255, 255, 255, 1])

    const colorSign = ref<ColorSwitch>("Default")

    // 函数声明（重载签名）
    function switch_color(inputColor: RGBColor): HexColor;
    function switch_color(inputColor: HexColor): RGBColor;
    // 函数实现
    function switch_color(inputColor: RGBColor | HexColor): HexColor | RGBColor {
        if (Array.isArray(inputColor)) {
            // RGB to Hex conversion
            return "#" + inputColor.map((color) =>
                color.toString(16).padStart(2, "0")
            ).join("") as HexColor;
        } else {
            return [
                parseInt(inputColor.substring(1, 3), 16),
                parseInt(inputColor.substring(3, 5), 16),
                parseInt(inputColor.substring(5, 7), 16)
            ] as RGBColor;
        }
    }

    function set_color_palette() {
        switch (colorSign.value) {
            case "Light":
                // HEX
                titleBarColor.value = "#f9f8e2"
                toolBarColor.value = "#ecece7"
                foreGroundColor.value = "#fdf9dd"
                midGroundColor.value = "#7cc4e0"
                backGroundColor.value = "#fbfbf4"
                borderColor.value = "#e2f0f0"
                iconColor.value = "#45494a"
                iconHoverColor.value = "#f5f57f"
                iconActiveColor.value = "#f0f02c"
                fontColor.value = "#42494a"
                // RGB
                titleBarColorRGB.value = [249, 248, 226]
                toolBarColorRGB.value = [236, 236, 231]
                foreGroundColorRGB.value = [253, 249, 221]
                midGroundColorRGB.value = [226, 240, 240]
                backGroundColorRGB.value = [251, 251, 244]
                borderColorRGB.value = [35, 169, 242]
                iconColorRGB.value = [69, 73, 74]
                iconHoverColorRGB.value = [245, 245, 127]
                iconActiveColorRGB.value = [240, 240, 44]
                fontColorRGB.value = [66, 73, 74]
                // RGBA
                titleBarColorRGBA.value = [249, 248, 226, 1]
                toolBarColorRGBA.value = [236, 236, 231, 1]
                foreGroundColorRGBA.value = [253, 249, 221, 1]
                midGroundColorRGBA.value = [226, 240, 240, 1]
                backGroundColorRGBA.value = [251, 251, 244, 1]
                borderColorRGBA.value = [35, 169, 242, 1]
                iconColorRGBA.value = [69, 73, 74, 1]
                iconHoverColorRGBA.value = [245, 245, 127, 1]
                iconActiveColorRGBA.value = [240, 240, 44, 1]
                fontColorRGBA.value = [66, 73, 74, 1]
                break;
            case "Night":
                titleBarColor.value = "#3c3f41"     //  
                toolBarColor.value = "#2b2b2b"      // PANTONE 19-4305TPG Pirate Black
                foreGroundColor.value = "#414342"   //  PANTONE 19-4305TPG Pirate Black
                midGroundColor.value = "#3d4144"    //  3TREES B28605-1 青骊
                backGroundColor.value = "#373838"   //  
                iconColor.value = "#cccccc"
                iconHoverColor.value = "#3c3c3c"
                // RGB
                titleBarColorRGB.value = [60, 63, 65]
                toolBarColorRGB.value = [43, 43, 43]
                foreGroundColorRGB.value = [65, 67, 66]
                midGroundColorRGB.value = [61, 65, 68]
                backGroundColorRGB.value = [55, 56, 56]
                iconColorRGB.value = [204, 204, 204]
                iconHoverColorRGB.value = [60, 60, 60]
                // RGBA
                titleBarColorRGBA.value = [60, 63, 65, 1]
                toolBarColorRGBA.value = [43, 43, 43, 1]
                foreGroundColorRGBA.value = [65, 67, 66, 1]
                midGroundColorRGBA.value = [61, 65, 68, 1]
                backGroundColorRGBA.value = [55, 56, 56, 1]
                iconColorRGBA.value = [204, 204, 204, 1]
                iconHoverColorRGBA.value = [60, 60, 60, 1]
                break;
            default:
                // HEX
                titleBarColor.value = "#f3f0ea"     //  NIPPON OW025-4 菱花白
                toolBarColor.value = "#ecece7"      // RAL 9003 信号白
                foreGroundColor.value = "#fffdf0"    //  NIPPON OW033-4 珍珠白 
                midGroundColor.value = "#e2e3e9"   //  RAL 9006 白铝色
                backGroundColor.value = "#fbfbf4"   //  NIPPON OW038-4 象牙白
                iconColor.value = "#45494a"
                iconHoverColor.value = "#e0e0db"
                iconActiveColor.value = "#42494a"
                // RGB
                titleBarColorRGB.value = [249, 248, 226]
                toolBarColorRGB.value = [236, 236, 231]
                foreGroundColorRGB.value = [253, 249, 221]
                midGroundColorRGB.value = [226, 240, 240]
                backGroundColorRGB.value = [251, 251, 244]
                borderColorRGB.value = [35, 169, 242]
                iconColorRGB.value = [69, 73, 74]
                iconHoverColorRGB.value = [245, 245, 127]
                iconActiveColorRGB.value = [240, 240, 44]
                fontColorRGB.value = [66, 73, 74]
                // RGBA
                titleBarColorRGBA.value = [249, 248, 226, 1]
                toolBarColorRGBA.value = [236, 236, 231, 1]
                foreGroundColorRGBA.value = [253, 249, 221, 1]
                midGroundColorRGBA.value = [226, 240, 240, 1]
                backGroundColorRGBA.value = [251, 251, 244, 1]
                borderColorRGBA.value = [35, 169, 242, 1]
                iconColorRGBA.value = [69, 73, 74, 1]
                iconHoverColorRGBA.value = [245, 245, 127, 1]
                iconActiveColorRGBA.value = [240, 240, 44, 1]
                fontColorRGBA.value = [66, 73, 74, 1]
                break;
        }
    }
    return {
        // 特殊色彩
        colseIconColor,
        colseIconHoverColor,
        colseIconActiveColor,
        minSizeIconColor,
        minSizeIconHoverColor,
        minSizeIconActiveColor,
        maxSizeIconColor,
        maxSizeIconHoverColor,
        maxSizeIconActiveColor,
        colseIconColorRGB,
        colseIconHoverColorRGB,
        colseIconActiveColorRGB,
        minSizeIconColorRGB,
        minSizeIconHoverColorRGB,
        minSizeIconActiveColorRGB,
        maxSizeIconColorRGB,
        maxSizeIconHoverColorRGB,
        maxSizeIconActiveColorRGB,
        colseIconColorRGBA,
        colseIconHoverColorRGBA,
        colseIconActiveColorRGBA,
        minSizeIconColorRGBA,
        minSizeIconHoverColorRGBA,
        minSizeIconActiveColorRGBA,
        maxSizeIconColorRGBA,
        maxSizeIconHoverColorRGBA,
        maxSizeIconActiveColorRGBA,

        // 一般色彩
        titleBarColor,
        toolBarColor,
        foreGroundColor,
        midGroundColor,
        backGroundColor,
        borderColor,
        iconColor,
        iconHoverColor,
        iconActiveColor,
        fontColor,

        titleBarColorRGB,
        toolBarColorRGB,
        foreGroundColorRGB,
        midGroundColorRGB,
        backGroundColorRGB,
        borderColorRGB,
        iconColorRGB,
        iconHoverColorRGB,
        iconActiveColorRGB,
        fontColorRGB,

        titleBarColorRGBA,
        toolBarColorRGBA,
        foreGroundColorRGBA,
        midGroundColorRGBA,
        backGroundColorRGBA,
        borderColorRGBA,
        iconColorRGBA,
        iconHoverColorRGBA,
        iconActiveColorRGBA,
        fontColorRGBA,

        colorSign,

        switch_color,
        set_color_palette
    } as const
}
