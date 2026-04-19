import { createApp } from 'vue';
import { createWebHistory, createRouter } from 'vue-router';
import { createI18n } from 'vue-i18n';

import App from './components/App.vue';

import en from './locales/en.json';
import jp from './locales/jp.json';

const i18n = createI18n({
    legacy: false,
    globalInjection: true,
    locale: 'en',
    fallbackLocale: 'en',
    messages: { en, jp },
});

const router = createRouter({
    history: createWebHistory(),
    routes: App.routes,
});

createApp(App).use(router).use(i18n).mount('#app');
