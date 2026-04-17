<template>
    <router-view />
</template>

<script setup lang="ts">
import NotFound from './NotFound.vue';

defineOptions({
    routes: [
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
</style>