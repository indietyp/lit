import type { Config } from '@jest/types';

const config: Config.InitialOptions = {
    // rootDir: "./src/",
    transform: {
        // "^.+\\.tsx?$": "ts-jest",
        "^.+\\.[jt]sx?$": "babel-jest",
    },
    transformIgnorePatterns: [
        "node_modules\\/(?!(@loopit!\\/engine)\\/)"
    ],
    // moduleFileExtensions: ["js", "ts", "tsx"],
    moduleDirectories: ["node_modules", "src", "engine"],
    // testEnvironment: 'jest-environment-node',
    // extensionsToTreatAsEsm: ['.ts', '.tsx'],
    automock: false,
    resetMocks: false,
    setupFiles: [
        "./setupTests.ts"
    ]
};

export default config;