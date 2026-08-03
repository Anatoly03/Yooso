<template>
    <n-layout class="app-content" has-sider>
        <n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed" show-trigger @collapse="collapsed = true" @expand="collapsed = false">
            <side-bar :collapsed="collapsed"></side-bar>
        </n-layout-sider>
        <n-layout>
            <view-entities></view-entities>
        </n-layout>
    </n-layout>
</template>

<script setup lang="ts">
import { NLayout, NLayoutSider } from 'naive-ui';
import { ref } from 'vue';
import SideBar from './SideBar.vue';
import ViewEntities from './ViewEntities.vue';

const collapsed = ref(true);
</script>

<style lang="scss">
html {
    margin: 0;
    width: 100vw;
    height: 100vh;
}

#app,
.app-content {
    width: 100vw;
    height: 100vh;
}
</style>
