/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')

const buildDirectory = '../dist'

module.exports = [
   {
      mode: 'production',
      target: 'web',
      entry: path.resolve(__dirname, './src/ssr.tsx'),
      output: {
         publicPath: 'public',
         path: path.resolve(__dirname, buildDirectory),
         library: 'SSR',
         libraryTarget: 'var',
         filename: 'index.js',
      },
      resolve: {
         extensions: ['.js', '.jsx', '.json', '.ts', '.tsx'],
      },
      module: {
         rules: [
            {
               test: /\.ts(x?)$/,
               exclude: /node_modules/,
               use: [
                  {
                     loader: 'ts-loader',
                  },
               ],
            },
            {
               test: /\.(png|jp(e*)g|svg|gif)$/,
               use: [
                  {
                     loader: 'file-loader',
                     options: {
                        name: './images/[hash]-[name].[ext]',
                     },
                  },
               ],
            },
            {
               test: /\.css$/i,
               use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'],
            },
         ],
      },
      plugins: [
         new MiniCssExtractPlugin({
            filename: './styles/ssr.css',
         }),
         new CleanWebpackPlugin({
            cleanOnceBeforeBuildPatterns: [
               path.join(__dirname, buildDirectory),
            ],
            dangerouslyAllowCleanPatternsOutsideProject: true,
         }),
      ],
      experiments: {
         asyncWebAssembly: true,
         syncWebAssembly: true,
      },
   },
   {
      mode: 'production',
      target: 'web',
      entry: path.resolve(__dirname, './src/index.tsx'),
      output: {
         path: path.resolve(__dirname, buildDirectory),
         filename: 'scripts/bundle.js',
      },
      resolve: {
         extensions: ['.js', '.jsx', '.json', '.ts', '.tsx'],
      },
      module: {
         rules: [
            {
               test: /\.ts(x?)$/,
               exclude: /node_modules/,
               use: [
                  {
                     loader: 'ts-loader',
                  },
               ],
            },
            {
               test: /\.(png|jp(e*)g|svg|gif)$/,
               use: [
                  {
                     loader: 'file-loader',
                     options: {
                        name: './images/[hash]-[name].[ext]',
                     },
                  },
               ],
            },
            {
               test: /\.css$/i,
               use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'],
            },
         ],
      },
      plugins: [
         new MiniCssExtractPlugin({
            filename: './styles/ssr.css',
         }),
      ],
      experiments: {
         asyncWebAssembly: true,
         syncWebAssembly: true,
      },
   },
]
