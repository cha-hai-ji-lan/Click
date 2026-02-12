export class FCArgs {
    private static instance: FCArgs;
    public args: string[] = []

    private constructor() {}

    /**
     * 获取单例实例
     */
    public static getInstance(): FCArgs {
      if (!FCArgs.instance) {
        FCArgs.instance = new FCArgs();
      }
      return FCArgs.instance;
    }
}