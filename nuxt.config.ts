import { transformAssetUrls } from "vite-plugin-vuetify";
import vuetify from "vite-plugin-vuetify";

const isDev = process.env.NODE_ENV === "development";

export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",

  rootDir: "src-nuxt/",
  srcDir: ".",

  devtools: { enabled: true },
  ssr: false,
  devServer: { host: process.env.TAURI_DEV_HOST || "localhost" },
  ignore: ["./src-tauri/**/*"],

  app: {
    rootId: "__root",
    buildAssetsDir: "_assets",
  },

  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
    },
    css: {
      preprocessorOptions: {
        scss: { api: "modern-compiler" },
      },
    },
    vue: {
      template: { transformAssetUrls },
    },
  },

  modules: [
    "@pinia/nuxt",
    "@vueuse/nuxt",
    "@nuxtjs/google-fonts",
    (_options, nuxt) => {
      nuxt.hooks.hook("vite:extendConfig", (config) => {
        config.plugins.push(vuetify({ autoImport: true }));
      });
    },
  ],

  build: {
    transpile: ["vuetify"],
  },

  router: {
    options: {
      // hashMode: !isDev,
      hashMode: true,
    },
  },

  css: ["~/assets/styles/main.scss"],

  googleFonts: {
    download: false,
    families: {
      Roboto: {
        ital: [100, 300, 400, 500, 700, 900],
        wght: [100, 300, 400, 500, 700, 900],
      },
      "Noto+Sans+KR": {
        ital: [100, 300, 400, 500, 700, 900],
        wght: [100, 300, 400, 500, 700, 900],
      },
    },
  },
});
