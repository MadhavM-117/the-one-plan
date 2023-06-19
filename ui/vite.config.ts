/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-ignore
import path from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
// eslint-disable-next-line import/no-default-export
export default defineConfig({
  // enable the build section for debugging prod issues
  // build: {
  //   minify: false,
  //   sourcemap: "inline",
  // },
  plugins: [react()],
  resolve: {
    alias: {
      // @ts-ignore
      src: path.resolve(__dirname, "./src"),
    },
  },
//  server: {
//    proxy: {
//      "/api/v1": {
//        target: "http://localhost:8000",
//        changeOrigin: true,
//      },
//    },
//  },
  css: {
    preprocessorOptions: {
      sass: {
        includePaths: ["./"],
      },
    },
    modules: {
      localsConvention: "camelCase",
    },
  },
});
