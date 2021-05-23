import { Interpreter } from './Interpreter';
import init from '../../engine/pkg';

it('runs program', async () => {
    await init('lit_bg.wasm');
    const interpreter = new Interpreter();
    const programCode = "x := 1\nLOOP x DO\nx := x + 1\nEND";


    const result = interpreter.runProgram(programCode);

    expect(result['x'][0]).toBe(2);
})