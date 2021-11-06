const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "production",
    entry: {
        index: "./src/index.js"
    },
    output: {
        path: dist,
        filename: "[name].js"
    },
    devServer: {
        static: {
            directory: path.join(__dirname, 'dist'),
        }
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: path.resolve(__dirname, "static"), to: dist }
            ]
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true
    }
}
