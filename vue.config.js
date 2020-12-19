const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const path = require('path');

module.exports = {
  configureWebpack: {
    plugins: [
      new WasmPackPlugin({
        crateDirectory: path.join(__dirname, 'wasm-markdown/pkg'),
        withTypeScript: true,
      }),
    ],
  }
}
