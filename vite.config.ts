import { defineConfig } from "vite";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import solidPlugin from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";
import environment from "vite-plugin-environment";

// https://vitejs.dev/config/
export default defineConfig({
  root: "./frontend",
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
  envDir: "../",
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  plugins: [
    TanStackRouterVite({ target: "solid", autoCodeSplitting: true }),
    solidPlugin(),
    tailwindcss(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
});
