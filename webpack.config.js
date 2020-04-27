const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
// const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "production",
    entry: {
        index: "./js/index.js"
    },
    output: {
        path: dist,
        filename: "[name].js"
    },
    devServer: {
        contentBase: dist,
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),

        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
        // new webpack.ProvidePlugin({
        //     TextDecoder: ['text-encoding', 'TextDecoder'],
        //     TextEncoder: ['text-encoding', 'TextEncoder']
        // })
    ]
};
