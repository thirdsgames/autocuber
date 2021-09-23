const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
  },
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.(s(a|s)ss|css)$/,
        use: ['style-loader', 'css-loader', 'sass-loader'],
      },
    ],
  },
  mode: 'development',
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: 'index.html', to: 'index.html' }],
    }),
  ],
  resolve: {
    extensions: ['.sass', '.ts', '.js'],
  },
  experiments: {
    syncWebAssembly: true,
  },
};
