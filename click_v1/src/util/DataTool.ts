// ... existing code ...
// 处理单个元素
function processElement(element: string, isSubElement: boolean = false): any {
  element = element.trim();
  
  // 检查是否是数组格式
  if (element.startsWith('[') && element.endsWith(']')) {
    // 提取数组内容并递归解析，标记为子元素
    const content = element.substring(1, element.length - 1);
    return parseStringToArray(content, true);
  }
  
  // 只有在作为子元素时才尝试转换为数字
  if (isSubElement) {
    // 尝试转换为数字
    if (/^-?\d+$/.test(element)) {
      return parseInt(element, 10);
    }
    
    // 尝试转换为浮点数
    if (/^-?\d+\.\d+$/.test(element)) {
      return parseFloat(element);
    }
  }
  
  // 返回字符串
  return element;
}

// 解析包含数组和普通元素的字符串为数组
export function parseStringToArray(str: string, isSubElement: boolean = false): any[] {
  // 移除末尾的逗号和空格
  str = str.replace(/\</g, ",[")
  str = str.replace(/\>/g, "],")
  str = str.replace(/,,/g, ",")
  str = str.trim().replace(/,\s*$/, '');
  
  const result: any[] = [];
  let current = '';
  let bracketDepth = 0;
  let inQuotes = false;
  let quoteChar = '';
  
  for (let i = 0; i < str.length; i++) {
    const char = str[i];
    
    // 处理引号
    if ((char === '"' || char === "'") && (i === 0 || str[i-1] !== '\\')) {
      if (!inQuotes) {
        inQuotes = true;
        quoteChar = char;
        current += char;
      } else if (char === quoteChar) {
        inQuotes = false;
        quoteChar = '';
        current += char;
      } else {
        current += char;
      }
      continue;
    }
    
    // 如果在引号内，直接添加字符
    if (inQuotes) {
      current += char;
      continue;
    }
    
    // 处理方括号
    if (char === '[') {
      bracketDepth++;
      current += char;
    } else if (char === ']') {
      bracketDepth--;
      current += char;
    } else if (char === ',' && bracketDepth === 0) {
      // 只有在非嵌套括号级别且不在引号内时才分割
      result.push(processElement(current.trim(), isSubElement));
      current = '';
    } else {
      current += char;
    }
  }
  
  // 添加最后一个元素
  if (current.trim() !== '') {
    result.push(processElement(current.trim(), isSubElement));
  }
  
  return result;
}
// ... existing code ...