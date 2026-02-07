import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';

/**
 * 创建系统托盘图标
 * @returns Promise<TrayIcon | null> 返回创建的托盘图标实例或null
 */
export async function createTrayIcon(): Promise<TrayIcon | null> {
  try {
    // 获取应用默认图标
    const appIcon = await defaultWindowIcon();
    
    // 构建托盘选项，只在图标存在时添加icon属性
    const options: { icon?: any; tooltip?: string } = {
      tooltip: 'Click期待不断进步'
    };
    
    // 只有当appIcon不为null时才设置icon属性
    if (appIcon !== null) {
      options.icon = appIcon;
    }
    
    // 创建托盘图标
    const tray = await TrayIcon.new(options);
    return tray;
  } catch (error) {
    console.error('创建托盘图标失败:', error);
    return null;
  }
}