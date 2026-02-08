import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';

// 单例实例变量
let trayInstance: TrayIcon | null = null;

/**
 * 托盘管理器类 - 实现单例模式
 */
class TrayManager {
  private static instance: TrayManager;
  private tray: TrayIcon | null = null;
  private tooltip: string = 'Click期待不断进步';
  private isCreating: boolean = false; // 防重标志
  private creationPromise: Promise<boolean> | null = null; // 防止重复创建的Promise

  private constructor() {}

  /**
   * 获取单例实例
   */
  public static getInstance(): TrayManager {
    if (!TrayManager.instance) {
      TrayManager.instance = new TrayManager();
    }
    return TrayManager.instance;
  }

  /**
   * 创建系统托盘图标
   */
  public async createTrayIcon(): Promise<boolean> {
    console.log("创建图标");
    
    // 如果正在创建中，返回现有的Promise
    if (this.isCreating && this.creationPromise) {
      console.warn('托盘图标正在创建中...');
      return this.creationPromise;
    }
    
    // 如果已经存在托盘实例，直接返回成功
    if (this.tray) {
      console.log('托盘图标已存在');
      return Promise.resolve(true);
    }

    this.isCreating = true;
    
    // 创建新的Promise并缓存
    this.creationPromise = this.createTrayInternal()
      .then(result => {
        this.isCreating = false;
        this.creationPromise = null;
        return result;
      })
      .catch(error => {
        console.error('创建托盘图标失败:', error);
        this.isCreating = false;
        this.creationPromise = null;
        return false;
      });

    return this.creationPromise;
  }

  /**
   * 内部创建托盘图标的实现
   */
  private async createTrayInternal(): Promise<boolean> {
    try {
      // 确保销毁旧图标
      await this.destroyTrayIcon();

      // 添加超时控制
      const appIconPromise = Promise.race([
        defaultWindowIcon(),
        new Promise<null>(resolve => setTimeout(() => resolve(null), 5000)) // 5秒超时
      ]);

      const appIcon = await appIconPromise;
      
      // 构建托盘选项
      const options: { icon?: any; tooltip?: string } = {
        tooltip: this.tooltip
      };
      
      // 只有当appIcon不为null时才设置icon属性
      if (appIcon !== null) {
        options.icon = appIcon;
      }
      
      // 创建托盘图标，添加超时控制
      const trayPromise = Promise.race([
        TrayIcon.new(options),
        new Promise<TrayIcon>((_, reject) => 
          setTimeout(() => reject(new Error('创建托盘图标超时')), 5000)
        )
      ]);

      this.tray = await trayPromise;
      trayInstance = this.tray; // 同步到全局变量
      console.log('托盘图标创建成功');
      return true;
    } catch (error) {
      console.error('创建托盘图标失败:', error);
      // 确保清理状态
      this.tray = null;
      trayInstance = null;
      throw error;
    }
  }

  /**
   * 销毁托盘图标
   */
  public async destroyTrayIcon(): Promise<void> {
    if (this.tray) {
      try {
        console.log('正在销毁托盘图标...');
        // 添加超时控制
        await Promise.race([
          this.tray.close(),
          new Promise<void>(resolve => setTimeout(resolve, 3000)) // 3秒超时
        ]);
        console.log('托盘图标销毁完成');
      } catch (error) {
        console.warn('销毁托盘图标时出现警告:', error);
      } finally {
        // 无论如何都要清理引用
        this.tray = null;
        trayInstance = null;
      }
    }
  }

  /**
   * 安全地销毁托盘图标（用于应用关闭时）
   */
  public async safeDestroyTrayIcon(): Promise<void> {
    try {
      // 取消任何正在进行的创建操作
      this.isCreating = false;
      this.creationPromise = null;
      
      // 销毁现有托盘
      await this.destroyTrayIcon();
    } catch (error) {
      console.warn('安全销毁托盘图标时出现错误:', error);
    }
  }

  /**
   * 更新托盘提示文本
   */
  public async updateTooltip(newTooltip: string): Promise<boolean> {
    this.tooltip = newTooltip;
    if (this.tray) {
      try {
        await this.tray.setTooltip(newTooltip);
        return true;
      } catch (error) {
        console.error('更新托盘提示失败:', error);
        return false;
      }
    }
    return false;
  }
  
  /**
   * 重置托盘提示文本
   */
  public async resiteTooltip(): Promise<boolean> {
    this.tooltip = 'Click期待不断进步';
    if (this.tray) {
      try {
        await this.tray.setTooltip('Click期待不断进步');
        return true;
      } catch (error) {
        console.error('更新托盘提示失败:', error);
        return false;
      }
    }
    return false;
  }

  /**
   * 更新托盘图标
   */
  public async updateIcon(icon: string | any): Promise<boolean> {
    if (this.tray) {
      try {
        await this.tray.setIcon(icon);
        return true;
      } catch (error) {
        console.error('更新托盘图标失败:', error);
        return false;
      }
    }
    return false;
  }

  /**
   * 获取当前托盘实例
   */
  public getTrayInstance(): TrayIcon | null {
    return this.tray;
  }

  /**
   * 检查托盘是否存在
   */
  public hasTray(): boolean {
    return this.tray !== null;
  }

  /**
   * 检查是否正在创建托盘
   */
  public isCreatingTray(): boolean {
    return this.isCreating;
  }
}

/**
 * 导出单例实例的便捷函数
 */
export const trayManager = TrayManager.getInstance();

/**
 * 兼容旧API的导出函数
 */
export async function createTrayIcon(): Promise<TrayIcon | null> {
  const manager = TrayManager.getInstance();
  const success = await manager.createTrayIcon();
  return success ? manager.getTrayInstance() : null;
}

export async function destroyTrayIcon(): Promise<void> {
  const manager = TrayManager.getInstance();
  await manager.destroyTrayIcon();
}

export async function safeDestroyTrayIcon(): Promise<void> {
  const manager = TrayManager.getInstance();
  await manager.safeDestroyTrayIcon();
}

export async function updateTrayTooltip(tooltip: string): Promise<boolean> {
  return await TrayManager.getInstance().updateTooltip(tooltip);
}

export async function resiteTrayTooltip(): Promise<boolean> {
  return await TrayManager.getInstance().resiteTooltip();
}

export async function updateTrayIcon(icon: string | any): Promise<boolean> {
  return await TrayManager.getInstance().updateIcon(icon);
}

export async function hasTrayNow(): Promise<boolean> {
  return TrayManager.getInstance().hasTray();
}

export function isCreatingTrayNow(): Promise<boolean> {
  return Promise.resolve(TrayManager.getInstance().isCreatingTray());
}

// 导出全局托盘实例（向后兼容）
export { trayInstance };