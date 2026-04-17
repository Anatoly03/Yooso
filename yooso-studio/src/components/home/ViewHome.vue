<template>
    <n-layout has-sider>
        <n-layout-sider collapse-mode="width" :collapsed-width="64" :width="240" show-trigger="bar" :collapsed="collapsed" @collapse="collapsed = true" @expand="collapsed = false" bordered>
            <n-menu :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22" v-model:value="selectedKey" :options="menuOptions" :render-label="renderMenuLabel" />
        </n-layout-sider>
        <n-layout>
            <span>Content</span>
        </n-layout>
    </n-layout>
</template>

<script setup lang="ts">
import type { MenuOption } from 'naive-ui';
import { NIcon, NLayout, NLayoutSider, NMenu } from 'naive-ui';
import { HomeOutline } from '@vicons/ionicons5';
import { h, onMounted, ref } from 'vue';
import { RouterLink, useRouter } from 'vue-router';

// Router instance for navigation
const router = useRouter();

// State of left sidebar
const collapsed = ref(true);

// Current selected menu item key
const selectedKey = ref<string | undefined>(undefined);

// Custom render function for menu labels to support router links
function renderMenuLabel(option: MenuOption) {
    if ('to' in option && typeof option.to === 'string') {
        return h(RouterLink, { to: option.to, activeClass: 'router-link-active' }, [option.label as string]);
    }

    return option.label as string;
}

// Sidebar menu options
// TODO: move to separate component/ file
const menuOptions: MenuOption[] = [
    {
        key: 'HomeOutline',
        label: 'Home',
        to: '/', // Route path for navigation
        icon: () => h(NIcon, null, { default: () => h(HomeOutline) }),
    },
];

// Select current path as active menu item
onMounted(() => {
    const currentPath = router.currentRoute.value.path;
    const activeMenuItem = menuOptions.find((option) => 'to' in option && option.to === currentPath);
    if (activeMenuItem) {
        activeMenuItem.key = activeMenuItem.key || activeMenuItem.to; // Ensure key is set for active item
        selectedKey.value = activeMenuItem.key;
    }
});
</script>
