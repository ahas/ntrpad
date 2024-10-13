import "@mdi/font/css/materialdesignicons.min.css";
import { createVuetify } from "vuetify";
import "vuetify/styles";

export default defineNuxtPlugin((nuxtApp) => {
  const vuetify = createVuetify({
    ssr: true,
  });
  nuxtApp.vueApp.use(vuetify);
});
