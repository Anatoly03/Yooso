<template>
    <n-config-provider class="app-config-provider">
        <router-view />
    </n-config-provider>
</template>

<script setup lang="ts">
import { NConfigProvider } from 'naive-ui';
import ViewHome from './home/ViewHome.vue';
import NotFound from './NotFound.vue';

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
