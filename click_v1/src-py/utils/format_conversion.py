from PIL import Image
import os


def resize_image_preserve_format(input_path, output_path=None, size=(800, 600), maintain_aspect_ratio=False,
                                 sizes=None):
    """
    缩放图片并保持原格式

    :param sizes:  ico 尺寸
    :param input_path: 输入文件路径
    :param output_path: 输出文件路径，如果为 None，则在原文件名后添加 '_resized'
    :param size: 目标尺寸
    :param maintain_aspect_ratio: 是否保持宽高比
    :return: 保存的文件路径
    """
    # 如果没有提供输出路径，则基于输入路径生成
    if sizes is None:
        # 默认的 ICO 尺寸，Windows 推荐的常见尺寸
        sizes = [16, 24, 32, 48, 64, 128, 256]
    if output_path is None:
        name, ext = os.path.splitext(input_path)
        output_path = f"{name}_resized{ext}"

    try:
        with Image.open(input_path) as img:
            # 获取原始格式
            original_format = img.format or get_format_from_extension(input_path)
            output_format = get_format_from_extension(output_path)

            # 根据格式处理透明度
            if original_format == 'JPEG' and img.mode in ('RGBA', 'LA', 'P'):
                img = img.convert('RGB')

            # 根据参数调整尺寸
            if maintain_aspect_ratio:
                img.thumbnail(size, Image.Resampling.LANCZOS)
            else:
                img = img.resize(size, Image.Resampling.LANCZOS)

            if output_format == 'ICO':
                if img.mode != 'RGBA':
                    img = img.convert('RGBA')

                    # 创建包含不同尺寸的图片列表
                icon_images = []
                for size in sizes:
                    # 调整图片尺寸
                    resized_img = img.resize((size, size), Image.Resampling.LANCZOS)
                    icon_images.append(resized_img)
                    print(f"已保存 {size}x{size} 的图片")
                # 保存为 ICO 文件，包含所有尺寸
                icon_images[0].save(
                    output_path,
                    format='ICO',
                    append_images=icon_images[1:],
                    sizes=[(s, s) for s in sizes]
                )
            else:
                # 保存图片，保持原格式
                img.save(output_path, format=original_format, optimize=True)

        print(f"已将 {input_path} 缩放为 {size} 并保持原格式保存为 {output_path}")
        return output_path

    except Exception as e:
        print(f"转换失败: {e}")
        return None


def get_format_from_extension(file_path):
    """
    根据文件扩展名获取 PIL 格式名称
    """
    extension = os.path.splitext(file_path)[1].lower()
    format_mapping = {
        '.jpg': 'JPEG',
        '.jpeg': 'JPEG',
        '.png': 'PNG',
        '.bmp': 'BMP',
        '.tiff': 'TIFF',
        '.tif': 'TIFF',
        '.webp': 'WEBP',
        '.gif': 'GIF',
        '.ppm': 'PPM',
        '.pgm': 'PGM',
        '.pbm': 'PBM',
        '.ico': 'ICO',
    }
    return format_mapping.get(extension, 'JPEG')


from PIL import Image


def png_to_multi_size_ico(png_path, ico_path, sizes=None):
    """
    将PNG图片转换为多尺寸的ICO文件

    Args:
        png_path: PNG文件路径
        ico_path: 输出的ICO文件路径
        sizes: 要生成的尺寸列表，默认为常见的ICO尺寸
    """
    if sizes is None:
        sizes = [(16, 16), (32, 32), (128, 128), (256, 256)]

    # 打开原始PNG图像
    img = Image.open(png_path)

    img.save(ico_path, format='ICO', sizes=sizes)


def png_to_icns(png_path, icns_path):
    """
    将PNG图片转换为多尺寸的ICNS文件

    Args:
        png_path: PNG文件路径
        icns_path: 输出的ICNS文件路径
    """
    from PIL import Image
    import os

    # macOS ICNS 常见的尺寸
    icns_sizes = [
        (16, 16),
        (32, 32),
        (64, 64),
        (128, 128),
        (256, 256),
        (512, 512),
        (1024, 1024)
    ]
    img = Image.open(png_path)
    img.save(icns_path, format='ICNS', sizes=icns_sizes)
