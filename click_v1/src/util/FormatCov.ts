export class FCArgs {
    private static instance: FCArgs;
    public args_g1: string[] = []
    public args_g2: string[] = []
    public combiner: string = ""

    private constructor() { }

    /**
     * 获取单例实例
     */
    public static getInstance(): FCArgs {
        if (!FCArgs.instance) {
            FCArgs.instance = new FCArgs();
        }
        return FCArgs.instance;
    }

    public securityReview(formatType: string[]) {
        console.log(formatType)
        if (formatType[1] === "ico") {
            this.args_g2.push("-resize")
            this.args_g2.push("256x256")
        }

    }
    public clean_args1(): void {
        this.args_g1 = []
    }
    public clean_args2(): void {
        this.args_g2 = []
    }
    public clean_args(): void {
        this.args_g1 = []
        this.args_g2 = []
    }

    public push_args1(arg: string): void {
        this.args_g1.push(arg)
    }
    public push_args2(arg: string): void {
        this.args_g2.push(arg)
    }
    public expend_args1(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }
    public expend_args2(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g2.push(arg)

            }
        })
    }
    public img_expend_args1(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }
    public img_expend_args2(args: string[]): void {
        args.forEach((arg, index) => {
            console.log(arg, index)
            if (arg !== "") {
                switch (index) {
                    case 0:
                        this.combiner = "" // 以防过去组合器出现内存泄露问题
                        if (arg.includes("%")) {
                            this.args_g2.push("-resize")
                            this.args_g2.push(arg)
                        } else {
                            this.combiner += arg + "x";
                        }
                        break;
                    case 1:
                        if (this.combiner !== "") {
                            this.combiner += arg;
                            this.args_g2.push("-resize")
                            this.args_g2.push(this.combiner)
                        }
                        this.combiner = "" // 不管有没有成功压入参数都要清空组合器
                        break;
                    default:
                        break;
                }

            }
        })
    }
}

export const FCArgsManager = FCArgs.getInstance();
export function securityReview(formatType: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.securityReview(formatType));
}
export function cleanArgs(): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.clean_args());
}
export function cleanArgs1(): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.clean_args1());
}
export function cleanArgs2(): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.clean_args2());
}
export function pushArgs1(arg: string): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.push_args1(arg));
}
export function pushArgs2(arg: string): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.push_args2(arg));
}
export function expendArgs1(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.expend_args1(args));
}
export function expendArgs2(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.expend_args2(args));
}
export function imgExpendArgs1(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.img_expend_args1(args));
}
export function imgExpendArgs2(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.img_expend_args2(args));
}
export function getArgs1(): string[] {
    const manager = FCArgs.getInstance();
    return manager.args_g1;
}
export function getArgs2(): string[] {
    const manager = FCArgs.getInstance();
    return manager.args_g2;
}