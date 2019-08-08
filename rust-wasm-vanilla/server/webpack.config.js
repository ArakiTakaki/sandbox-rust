const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    // mode: 'production',
    mode: 'development',
    entry: './src/entry.ts',
    output: {
      path: path.resolve('./dist'),
      filename: '[name].js',
      // HTMLにinjectionするパスを設定する
      publicPath: '/',
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "./index.html",
        inject: true,
      }),
    ],
    resolve: {
      // Add `.ts` and `.tsx` as a resolvable extension.
      extensions: ['.ts', '.tsx', '.js','.wasm']
    },
    module: {
      rules: [
        {
        // all files with a `.ts` or `.tsx` extension will be handled by `ts-loader`
          test: /\.ts$/,
          use: 'ts-loader',
        },
        {
          test: /\.wasm$/,
          type: "webassembly/experimental"
        }
      ]
    }
  }