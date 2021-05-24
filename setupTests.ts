import fetchMock from 'jest-fetch-mock';
import { readFileSync } from 'fs';

/**
 * Set up mock for fetch
 *
 * the package @loopit!/engine uses wasm and therefore loads the lit_bg.wasm file with 'fetch'
 * fetch is not included in node, so it has to be mocked with the help of 'jest-fetch-mock'
 */
const wasmFileName = 'lit_bg.wasm';
const wasmFileLocation = './engine/pkg/' + wasmFileName;

fetchMock.enableMocks();
// Read the .wasm file to memory
const file = readFileSync(wasmFileLocation);

// @ts-ignore
fetch.mockResponse(async request => {
    if (request.url.endsWith(wasmFileName)) {
        return {
            status: 200,
            body: file
        };
    } else {
        return {
            status: 404,
            body: 'Not Found'
        };
    }
});