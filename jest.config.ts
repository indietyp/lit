import type { InitialOptionsTsJest } from 'ts-jest/dist/types';

const config: InitialOptionsTsJest = {
    preset: 'ts-jest/presets/js-with-ts-esm',
    transform: {},
    extensionsToTreatAsEsm: ['.ts'],
    globals: {
        'ts-jest': {
            useESM: true,
            astTransformers: {
                before: ['./injectNodeEnvTransformer']
            }
        },
    },
    setupFiles: [
        "./setupTests.ts"
    ]
};

export default config;