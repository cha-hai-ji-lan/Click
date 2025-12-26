from PIL import Image
import os


def resize_image_keep_format(input_path, output_path, size=(800, 600)):
    """
    缩放图片到指定尺寸并保持原格式保存

    :param input_path: 输入文件路径
    :param output_path: 输出文件路径
    :param size: 目标尺寸，格式为 (width, height)
    """
    try:
        # 打开原始图片
        with Image.open(input_path) as img:
            # 根据输出路径的扩展名确定格式
            output_format = get_format_from_extension(output_path)

            # 根据原图格式进行适当的模式转换
            if output_format in ['JPEG'] and img.mode in ('RGBA', 'LA', 'P'):
                # JPEG 不支持透明度，需要转换为 RGB
                img = img.convert('RGB')
            elif output_format in ['PNG', 'WEBP'] and img.mode not in ('RGBA', 'LA'):
                # PNG 和 WEBP 支持透明度，如果原图支持则保持
                pass  # 保持原图模式

            # 调整图片大小
            resized_img = img.resize(size, Image.Resampling.LANCZOS)

            # 保存为原格式
            resized_img.save(output_path, format=output_format, optimize=True)

        print(f"已将 {input_path} 缩放为 {size} 并保存为 {output_path}")

    except Exception as e:
        print(f"转换失败: {e}")


def resize_image_preserve_format(input_path, output_path=None, size=(800, 600), maintain_aspect_ratio=False):
    """
    缩放图片并保持原格式

    :param input_path: 输入文件路径
    :param output_path: 输出文件路径，如果为 None，则在原文件名后添加 '_resized'
    :param size: 目标尺寸
    :param maintain_aspect_ratio: 是否保持宽高比
    :return: 保存的文件路径
    """
    # 如果没有提供输出路径，则基于输入路径生成
    if output_path is None:
        name, ext = os.path.splitext(input_path)
        output_path = f"{name}_resized{ext}"

    try:
        with Image.open(input_path) as img:
            # 获取原始格式
            original_format = img.format or get_format_from_extension(input_path)

            # 根据格式处理透明度
            if original_format == 'JPEG' and img.mode in ('RGBA', 'LA', 'P'):
                img = img.convert('RGB')

            # 根据参数调整尺寸
            if maintain_aspect_ratio:
                img.thumbnail(size, Image.Resampling.LANCZOS)
            else:
                img = img.resize(size, Image.Resampling.LANCZOS)

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
    }
    return format_mapping.get(extension, 'JPEG')



def get_ico_sizes(ico_path):
    """获取 ICO 文件中的所有尺寸信息"""
    with Image.open(ico_path) as img:
        # 获取图像信息
        if hasattr(img, 'apng'):
            # 如果是 APNG 格式
            sizes = []
            for frame in range(img.n_frames):
                img.seek(frame)
                sizes.append((img.width, img.height))
            return sizes
        else:
            # 对于 ICO 文件，获取所有可用的尺寸
            sizes = []
            # 尝试获取不同尺寸的图像
            try:
                # 获取图像的 info 属性中的尺寸信息
                if hasattr(img, 'info') and 'sizes' in img.info:
                    return img.info['sizes']
                else:
                    # 直接返回当前尺寸
                    sizes.append((img.width, img.height))
                    # 尝试获取其他尺寸
                    frame = 0
                    while frame < img.n_frames:
                        img.seek(frame)
                        sizes.append((img.width, img.height))
                        frame += 1
                    return sizes
            except:
                return [(img.width, img.height)]

# 使用示例

