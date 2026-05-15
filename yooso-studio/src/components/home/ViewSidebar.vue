<template>
    <n-menu :collapsed="props.collapsed" :collapsed-width="64" :collapsed-icon-size="22" v-model:value="selectedKey" :options="menuOptions" :render-label="renderMenuLabel" />
</template>

<script setup lang="ts">
import type { MenuOption } from 'naive-ui';
import { NMenu } from 'naive-ui';
import { FlowerOutline, HomeOutline, LeafOutline, StatsChartOutline } from '@vicons/ionicons5';
import { computed, h, onMounted, ref } from 'vue';
import { RouterLink, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';

const { t, locale } = useI18n();

// Collapse state is managed by parent component (ViewHome) and passed as prop
const props = defineProps<{
    collapsed: boolean;
}>();

// Router instance for navigation
const router = useRouter();

// Current selected menu item key
const selectedKey = ref<string | undefined>(undefined);

// Custom render function for menu labels to support router links
function renderMenuLabel(option: MenuOption) {
    if ('to' in option && typeof option.to === 'string') {
        return h(RouterLink, { to: option.to, activeClass: 'router-link-active' }, () => option.label as string);
    }

    return option.label as string;
}

// Sidebar menu options
const menuOptions = computed(() => {
    void locale.value;

    return [
        { to: '/', key: 'home', label: t('app.menu.home'), icon: () => h(HomeOutline) },
        { to: '/entities', key: 'entities', label: t('app.menu.entities'), icon: () => h(FlowerOutline) },
        { to: '/components', key: 'components', label: t('app.menu.components'), icon: () => h(LeafOutline) },
        { to: '/logs', key: 'logs', label: t('app.menu.logs'), icon: () => h(StatsChartOutline) },
    ];
});

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
