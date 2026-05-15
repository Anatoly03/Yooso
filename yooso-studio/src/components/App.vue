<template>
    <browser-frame v-if="BrowserFrame">
        <n-config-provider class="app-config-provider">
            <router-view />
        </n-config-provider>
    </browser-frame>
    <n-config-provider v-else class="app-config-provider">
        <router-view />
    </n-config-provider>
</template>

<script setup lang="ts">
import { defineAsyncComponent } from 'vue';
import { NConfigProvider } from 'naive-ui';
import ViewHome from './home/ViewHome.vue';
import NotFound from './NotFound.vue';

const BrowserFrame = import.meta.env.VITE_DEMO_MODE === 'true'
  ? defineAsyncComponent(() => import('./demo/BrowserFrame.vue'))
  : null;

defineOptions({
    routes: [
        { path: "/", component: ViewHome, children: ViewHome.routes },
        { path: "/:pathMatch(.*)*", component: NotFound },
    ],
});
</script>

<style lang="scss">
#app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
}

.app-config-provider {
    flex: 1 1 auto;
    min-height: 0;
    display: flex;
    flex-direction: column;
}
</style>
