import requests


def download_image(url, save_path):
    """
    从指定URL下载图片并保存到本地
    """
    try:
        # 发送GET请求
        response = requests.get(url, stream=True)
        response.raise_for_status()  # 检查请求是否成功
        print(f"正在下载图片:")
        # 保存图片到本地
        with open(save_path, 'wb') as file:
            for chunk in response.iter_content(chunk_size=8192):
                file.write(chunk)

        print(f"图片已成功下载并保存到: {save_path}")
        return True
    except requests.exceptions.RequestException as e:
        print(f"下载失败: {e}")
        return False


# 使用示例
num = 1

while True:
    url = f"https://file2.acgnngca.com/nh2/2025092705/3511165_{num}.webp"
    save_path = fr"D:\Downloads\[MISAAKI] Kono-suba [AI Generated] - 中文版同人H漫画_工口漫画[第1页]\{num}.webp"
    state = download_image(url, save_path)
    num += 1
    if not state:
        break
