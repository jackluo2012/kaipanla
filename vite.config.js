import { defineConfig } from 'vite';

export default defineConfig({
  // 环境变量前缀
  envPrefix: ['VITE_', 'TAURI_'],

  // 开发服务器配置
  server: {
    port: 1420,
    strictPort: true,
    host: 'localhost',
  },

  // 构建配置
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    outDir: 'dist',
    assetsDir: 'assets',
    sourcemap: false,
    minify: 'esbuild',
  },

  // 清屏
  clearScreen: true,

  // 清除控制台输出(生产环境)
  esbuild: {
    drop: process.env.NODE_ENV === 'production' ? ['console', 'debugger'] : [],
  },
});
