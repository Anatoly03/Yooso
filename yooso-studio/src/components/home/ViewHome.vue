<!-- Not to be confused with [ViewHomeGeneral] which is reffered to
 as 'Home' in the admin UI. 'Home' is the main landing page for the
 application, the 'home' route of the 'ViewHome' component. -->

<template>
    <n-layout has-sider>
        <n-layout-sider :content-style="{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100%' }" collapse-mode="width" :collapsed-width="64" :width="240" show-trigger="bar" :collapsed="collapsed" @collapse="collapsed = true" @expand="collapsed = false" bordered>
            <ViewSidebar :collapsed="collapsed" />
            <LanguageSelect :collapsed="collapsed" />
        </n-layout-sider>
        <n-layout>
            <router-view />
        </n-layout>
    </n-layout>
</template>

<script setup lang="ts">
import { NLayout, NLayoutSider } from 'naive-ui';
import { ref } from 'vue'

import LanguageSelect from '../ui/LanguageSelect.vue';
import ViewSidebar from './ViewSidebar.vue';
import ViewHomeGeneral from './ViewHomeGeneral.vue';
import ViewHomeEntities from './ViewHomeEntities.vue';
import ViewHomeComponents from './ViewHomeComponents.vue';
import ViewHomeLogs from './ViewHomeLogs.vue';

// State of left sidebar
// TODO: set to true if more components flood the screen
const collapsed = ref(false);

// Routes
defineOptions({
    routes: [
        { path: '/', component: ViewHomeGeneral },
        { path: '/entities', component: ViewHomeEntities },
        { path: '/components', component: ViewHomeComponents },
        { path: '/logs', component: ViewHomeLogs },
    ],
});
</script>
