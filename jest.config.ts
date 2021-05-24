import type { InitialOptionsTsJest } from 'ts-jest/dist/types';

const config: InitialOptionsTsJest = {
    preset: 'ts-jest/presets/js-with-ts-esm',
    transform: {},
    extensionsToTreatAsEsm: ['.ts'],
    globals: {
        'ts-jest': {
            useESM: true,
            astTransformers: {
                before: ['./src/test-utils/injectNodeEnvTransformer']
            }
        },
    },
};

export default config;