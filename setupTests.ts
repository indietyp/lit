import { enableFetchMocks } from "jest-fetch-mock";
import { readFileSync } from "fs";

const wasmFileName = "lit_bg.wasm";
const wasmFileLocation = "./engine/pkg/" + wasmFileName;

enableFetchMocks();
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
            body: "Not Found"
        };
    }
});