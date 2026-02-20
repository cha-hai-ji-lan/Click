# converter.py
import shutil

import win32com.client
import os
import pythoncom


class OfficeConverter:
    """Office文档转换器 - 使用win32com实现docx和xlsx互转"""

    def __init__(self):
        # 初始化COM库
        pythoncom.CoInitialize()
        self.word_app = None
        self.excel_app = None

    def __enter__(self):
        return self

    def __exit__(self):
        self.close()

    def close(self):
        """关闭所有Office应用程序"""
        if self.word_app:
            self.word_app.Quit()
            self.word_app = None
        if self.excel_app:
            self.excel_app.Quit()
            self.excel_app = None
        pythoncom.CoUninitialize()

    def docx_to_xlsx(self, docx_path: str, xlsx_path: str) -> bool:
        """
        将docx文件转换为xlsx文件
        注意：这种转换会丢失大部分格式，仅保留文本内容
        """
        try:
            if not os.path.exists(docx_path):
                raise FileNotFoundError(f"源文件不存在: {docx_path}")

            # 创建Word应用程序对象
            if not self.word_app:
                self.word_app = win32com.client.Dispatch("Word.Application")
                self.word_app.Visible = False

            # 打开docx文档
            doc = self.word_app.Documents.Open(docx_path)

            # 先保存为HTML格式作为中间步骤
            html_path = xlsx_path.replace('.xlsx', '.html')
            html_file_path = xlsx_path.replace('.xlsx', '.files')
            doc.SaveAs2(html_path, FileFormat=8)  # 8 = HTML格式
            doc.Close()

            # 使用Excel打开HTML并保存为xlsx
            if not self.excel_app:
                self.excel_app = win32com.client.Dispatch("Excel.Application")
                self.excel_app.Visible = False

            workbook = self.excel_app.Workbooks.Open(html_path)
            workbook.SaveAs(xlsx_path, FileFormat=51)  # 51 = xlsx格式
            workbook.Close()

            # 删除临时HTML文件
            if os.path.exists(html_path):
                os.remove(html_path)
            if os.path.exists(html_file_path):
                shutil.rmtree(html_file_path)

            print(f"成功将 {docx_path} 转换为 {xlsx_path}")
            print("-end")
            return True

        except Exception as e:
            print(f"docx转xlsx失败: {str(e)}")
            return False

    def xlsx_to_docx(self, xlsx_path: str, docx_path: str) -> bool:
        """
        将xlsx文件转换为docx文件
        注意：这种转换会丢失表格格式，仅保留数据内容
        """
        try:
            if not os.path.exists(xlsx_path):
                raise FileNotFoundError(f"源文件不存在: {xlsx_path}")

            # 创建Excel应用程序对象
            if not self.excel_app:
                self.excel_app = win32com.client.Dispatch("Excel.Application")
                self.excel_app.Visible = False

            # 打开xlsx工作簿
            workbook = self.excel_app.Workbooks.Open(os.path.abspath(xlsx_path))

            # 先保存为HTML格式作为中间步骤
            html_path = docx_path.replace('.docx', '.html')
            workbook.SaveAs(os.path.abspath(html_path), FileFormat=44)  # 44 = HTML格式
            workbook.Close()

            # 使用Word打开HTML并保存为docx
            if not self.word_app:
                self.word_app = win32com.client.Dispatch("Word.Application")
                self.word_app.Visible = False

            doc = self.word_app.Documents.Open(os.path.abspath(html_path))
            doc.SaveAs2(os.path.abspath(docx_path), FileFormat=16)  # 16 = docx格式
            doc.Close()

            # 删除临时HTML文件
            if os.path.exists(html_path):
                os.remove(html_path)

            print(f"成功将 {xlsx_path} 转换为 {docx_path}")
            return True

        except Exception as e:
            print(f"xlsx转docx失败: {str(e)}")
            return False


# 使用示例
def example_usage():
    converter = OfficeConverter()

    try:
        # docx转xlsx
        converter.docx_to_xlsx("input.docx", "output.xlsx")

        # xlsx转docx
        converter.xlsx_to_docx("input.xlsx", "output.docx")

    finally:
        converter.close()


if __name__ == "__main__":
    example_usage()
