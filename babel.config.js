const {resolve} = require('path');

module.exports = {
    env: {
        test: {
            presets: [
                ["@babel/preset-env", {"targets": {"node": "current"}}],
                "@babel/preset-react",
                "@babel/preset-typescript",
            ],
            plugins: ['babel-plugin-transform-import-meta'],
        }
    }
}