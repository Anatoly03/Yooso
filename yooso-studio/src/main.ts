import { createApp } from "vue";
import { createWebHistory, createRouter } from "vue-router";

import App from "./components/App.vue";

const router = createRouter({
    history: createWebHistory(),
    routes: App.routes,
});

createApp(App).use(router).mount("#app");
