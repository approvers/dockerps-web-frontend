import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

import { ConfigurationFactory, CliConfigOptions } from 'webpack';
import { Options as TsLoaderOptions } from 'ts-loader';
import { resolve } from 'path';

const baseDirectory: string = __dirname;
const publicDirectory: string = resolve(baseDirectory, 'public');

const config: ConfigurationFactory = (
    env: string,
    argv: CliConfigOptions
) => ({
    entry: resolve(baseDirectory, 'index.ts'),
    output: {
        path: publicDirectory,
        filename: 'bundle.js',
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'ts-loader',
                options: <TsLoaderOptions>{
                    configFile: 'tsconfig.loader.json',
                },
            },
            {
                test: /\.s[ac]ss$/,
                loaders: [
                    'style-loader',
                    'css-loader',
                    'sass-loader',
                ],
            },
        ],
    },
    resolve: {
        extensions: ['.wasm', '.js', '.ts', '.sass', '.scss'],
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: baseDirectory,
        }),
    ],
    devServer: {
        contentBase: publicDirectory,
        compress: argv.mode === 'production',
        port: 8080,
    },
});

// noinspection JSUnusedGlobalSymbols
export default config;
