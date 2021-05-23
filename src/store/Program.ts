// import { Builder } from '@loopit!/engine';
// import { Module, Expr, Exec } from '@loopit!/engine/schema';
import { Builder } from '../../engine/pkg';
import { Module, Expr, Exec } from '../../engine/pkg/schema';

export class Program {
    public module!: Module;
    public node!: Expr;
    public exec!: Exec;

    constructor(public programCode: string, flags: any = {}) {
        this.module = Builder.parse(programCode);
        this.node = Builder.compile(this.module, flags);
        this.exec = Builder.eval(this.node);
    }
}