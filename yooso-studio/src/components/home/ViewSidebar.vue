<template>
    <n-menu :collapsed="props.collapsed" :collapsed-width="64" :collapsed-icon-size="22" v-model:value="selectedKey" :options="menuOptions" :render-label="renderMenuLabel" />
</template>

<script setup lang="ts">
import type { MenuOption } from 'naive-ui';
import { NIcon, NMenu } from 'naive-ui';
import { computed, h, onMounted, ref } from 'vue';
import { RouterLink, useRouter } from 'vue-router';

// Sidebar route definition
interface SidebarRoute {
    path: string;
    name: string;
    icon: () => ReturnType<typeof h>;
}

// Collapse state is managed by parent component (ViewHome) and passed as prop
const props =defineProps<{
    collapsed: boolean;
    routes: SidebarRoute[];
}>();

// Router instance for navigation
const router = useRouter();

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
const menuOptions = computed(() => props.routes.map(route => ({
     key: route.path, // Use path as unique key
     label: route.name,
     to: route.path,
     icon: () => h(NIcon, null, { default: () => route.icon() }),
})));

// Select current path as active menu item
onMounted(() => {
    const currentPath = router.currentRoute.value.path;
    const activeMenuItem = menuOptions.value.find((option) => 'to' in option && option.to === currentPath);
    if (activeMenuItem) {
        activeMenuItem.key = activeMenuItem.key || activeMenuItem.to; // Ensure key is set for active item
        selectedKey.value = activeMenuItem.key;
    }
});
</script>
