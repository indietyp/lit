export function convertVariables(variables) {
    let values = Object.values(variables).map(([k, v]) => [k, v.map(i => BigInt(i)).reduce((a, b) => a + b, 0n)]);

    return Object.fromEntries(values);
}