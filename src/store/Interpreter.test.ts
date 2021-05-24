import { Interpreter } from './Interpreter';
import init from '../../engine/pkg';
import { mockFetchWasmFile } from '../test-utils/mockFetchWasmFile';

beforeAll(async () => {
    mockFetchWasmFile();
    await init('lit_bg.wasm');
})

describe('Interpreter', () => {
    let interpreter!: Interpreter;

    beforeEach(() => {
        interpreter = new Interpreter();
    })

    it('runs program code w/o input parameters', () => {
        const programCode = "x := 1\nLOOP x DO\nx := x + 1\nEND";

        const result = interpreter.runProgram(programCode);

        // @ts-ignore
        expect(result['x'][0]).toBe(2);
    });

    it('runs program code w/ parameters', () => {
        const programCode = "LOOP x DO\nx := x + 1\nEND";
        const params: Map<string, number> = new Map(Object.entries({ x: 3 }));

        const result = interpreter.runProgram(programCode, params);

        // @ts-ignore
        expect(result['x'][0]).toBe(6);
    })
});