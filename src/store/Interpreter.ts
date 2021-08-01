import { Program } from './Program';
import init, { Builder, Runtime } from '@loopit!/engine';

export class Interpreter {
    public currentRuntime: Runtime | null = null;

    constructor() {
        // Load WASM Code
        if (import.meta.env.MODE !== 'test') {
            init();
        }
    }

    public runProgram(programCode: string, params: Map<string, number> = new Map()) {
        const program = new Program(programCode);
        // @ts-ignore
        this.currentRuntime = Builder.exec(program!.exec, params);

        while (this.currentRuntime.is_running()) {
            this.currentRuntime.step();
        }

        return this.currentRuntime.context();
    }
}