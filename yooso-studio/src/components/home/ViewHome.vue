<!-- Not to be confused with [ViewHomeGeneral] which is reffered to
 as 'Home' in the admin UI. 'Home' is the main landing page for the
 application, the 'home' route of the 'ViewHome' component. -->

<template>
    <n-layout has-sider>
        <n-layout-sider collapse-mode="width" :collapsed-width="64" :width="240" show-trigger="bar" :collapsed="collapsed" @collapse="collapsed = true" @expand="collapsed = false" bordered>
            <ViewSidebar :collapsed="collapsed" :routes="$options.routes" />
        </n-layout-sider>
        <n-layout>
            <router-view />
        </n-layout>
    </n-layout>
</template>

<script setup lang="ts">
import { NIcon, NLayout, NLayoutSider } from 'naive-ui';
import { FlowerOutline, HomeOutline } from '@vicons/ionicons5';
import { h, ref } from 'vue';
import ViewSidebar from './ViewSidebar.vue';
import ViewHomeGeneral from './ViewHomeGeneral.vue';
import ViewHomeEntities from './ViewHomeEntities.vue';

// State of left sidebar
const collapsed = ref(true);

// Routes
defineOptions({
    routes: [
        { path: '/', component: ViewHomeGeneral, name: 'Home', icon: () => h(HomeOutline) },
        { path: '/entities', component: ViewHomeEntities, name: 'Entities', icon: () => h(FlowerOutline) },
    ],
});
</script>
