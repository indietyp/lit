import { Program } from './Program';
// import init, { Builder, Runtime } from '@loopit!/engine';
import init, { Builder, Runtime } from '../../engine/pkg';

export class Interpreter {
    public variableMap: Map<string, number> = new Map();
    public currentRuntime: Runtime | null = null;

    constructor() {
        // Load WASM Code
        // if (import.meta.env.MODE !== 'test') {
        //     init();
        // }
    }

    public runProgram(programCode: string) {
        const program = new Program(programCode);
        this.currentRuntime = Builder.exec(program!.exec, this.variableMap);

        while (this.currentRuntime.is_running()) {
            this.currentRuntime.step();
        }

        return this.currentRuntime.context();
    }
}