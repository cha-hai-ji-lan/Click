export const magick_args = {
    "need-args": ["-resize", "-crop",],
    "before-need-gravity": ["-crop",],
    "before-need-background": ["-rotate",],

}  // magick 需要附件参数的关键参数
export class FCArgs {
    private static instance: FCArgs;
    public args_g1: string[] = []
    public args_g2: string[] = []
    public combiner: string = ""
    public additional_parameters: number = 0  // 关键参数后的附加传参数目
    public pre_args: string = ""  // 上一个关键参数

    private constructor() { }

    public getAt<T>(arr: T[], index: number): T | undefined {
        if (index < 0) {
            // 负索引：从后往前计算
            return arr[arr.length + index];
        }
        // 正索引：正常访问
        return arr[index];
    }

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
        for (let index = 0; index < args.length; index++) {
            const arg = args[index];
            console.log(arg, index)
            if (this.additional_parameters > 0) {
                if (this.pre_args === "") {
                    this.additional_parameters -= 1
                    continue;
                } else {
                    switch (this.pre_args) {
                        case '__bash__':
                            if (arg !== "__end_bash__") {
                                this.args_g2.push(arg)
                                continue;
                            } else {
                                this.additional_parameters = 0 // 遇见结束符表示命令行指令结束
                                continue;
                            }
                        case '-scale':
                        case '-resize':
                            if (arg.includes("%")) {
                                this.args_g2.push(arg)
                                this.additional_parameters = 0
                            } else if (this.combiner === "") {
                                this.combiner += arg + "x";
                                console.log("获取参数", this.combiner)
                            } else {
                                this.combiner += arg;
                                this.args_g2.push(this.combiner)
                                this.pre_args = ""
                                this.combiner = ""
                            }
                            break;
                        case '-gravity':
                            this.args_g2.push(arg)
                            this.pre_args = ""
                            this.combiner = ""
                            break;
                        case '-crop':
                            switch (this.additional_parameters) {
                                case 4:
                                    console.log(this.combiner)

                                    this.combiner = arg + "x"
                                    break;
                                case 3:
                                    console.log(this.combiner)
                                    this.combiner += arg
                                    break;
                                case 2:
                                    if (arg === "") {
                                        console.log(this.combiner)
                                        this.combiner += "+0"
                                    } else {
                                        console.log(this.combiner)
                                        this.combiner += "+" + arg
                                    }
                                    break;
                                case 1:
                                    if (arg === "") {
                                        this.combiner += "+0"
                                    } else {
                                        console.log(this.combiner)
                                        this.combiner += "+" + arg
                                    }
                                    this.args_g2.push(this.combiner)
                                    this.pre_args = ""
                                    this.combiner = ""
                                    break;
                                default:
                                    break;
                            }
                            break;

                        case '-rotate':
                            this.args_g2.push(arg)
                            this.pre_args = ""
                            this.combiner = ""
                            break;
                        default:
                            break;
                    }
                }
                this.additional_parameters -= 1
            }
            switch (arg) {
                case '__bash__':
                    this.pre_args = "__bash__";
                    this.additional_parameters = 1000;  // '-resize'应该有两个附加传参
                    break;
                case '-resize':
                    this.combiner = "" // 以防过去组合器出现内存泄露问题
                    this.additional_parameters = 2;  // '-resize'应该有两个附加传参
                    this.args_g2.push(arg)
                    this.pre_args = arg
                    break;
                case '-scale':
                    this.combiner = "" // 以防过去组合器出现内存泄露问题
                    this.additional_parameters = 2;  // '-resize'应该有两个附加传参
                    this.args_g2.push(arg)
                    this.pre_args = arg
                    break;
                case '-gravity':
                    console.log("获取关键参数", arg)
                    this.additional_parameters = 1;
                    this.args_g2.push(arg);
                    this.pre_args = arg
                    break;
                case '-crop':
                    console.log("获取关键参数", arg)
                    this.combiner = ""
                    this.additional_parameters = 4;  // '-crop'应该有两个附加传参
                    this.args_g2.push(arg);
                    this.pre_args = arg
                    break;
                case '-rotate':
                    console.log("获取关键参数", arg)
                    this.combiner = ""
                    this.additional_parameters = 1;
                    this.args_g2.push(arg);
                    this.pre_args = arg
                    break;
                case '-flop':
                case '-flip':
                    console.log("获取关键参数", arg)
                    this.combiner = ""
                    this.additional_parameters = 0;  // '-crop'应该有两个附加传参
                    this.args_g2.push(arg);
                    this.pre_args = ""
                    break;
                default:
                    break;
            }

        }
    }

    public vid_expend_args1(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }

    public vid_expend_args2(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }
    public aud_expend_args1(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }

    public office_expend_args2(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }
    public office_expend_args1(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
            }
        })
    }

    public aud_expend_args2(args: string[]): void {
        args.forEach(arg => {
            if (arg !== "") {
                this.args_g1.push(arg)
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
export function vidExpendArgs1(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.vid_expend_args1(args));
}
export function vidExpendArgs2(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.vid_expend_args2(args));
}
export function audExpendArgs1(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.aud_expend_args1(args));
}
export function audExpendArgs2(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.aud_expend_args2(args));
}
export function officeExpendArgs1(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.office_expend_args1(args));
}
export function officeExpendArgs2(args: string[]): Promise<void> {
    const manager = FCArgs.getInstance();
    return Promise.resolve(manager.office_expend_args2(args));
}
export function getArgs1(): string[] {
    const manager = FCArgs.getInstance();
    return manager.args_g1;
}
export function getArgs2(): string[] {
    const manager = FCArgs.getInstance();
    return manager.args_g2;
}
export function getAt(arr: any[], index: number): any {
    const manager = FCArgs.getInstance();
    return manager.getAt(arr, index);
}