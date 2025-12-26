from utils import resize, format_conversion

# resize.resize_image_preserve_format(r"setupClick.png", r"setupClick.ico", (107,107),
#                                     maintain_aspect_ratio=True)

# print(resize.get_ico_sizes(r"D:\Object_\APP\Tauri\work\Click\click_v1\src-tauri\icons\icon.ico"))
# print(resize.get_ico_sizes(r"icon.ico"))

format_conversion.png_to_multi_size_ico(r"setupClick.png", r"setupClick.ico")
# format_conversion.png_to_icns(r"click.png", r"icon.icns")
