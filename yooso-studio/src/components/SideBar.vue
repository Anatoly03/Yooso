<template>
    <n-menu :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions" :expand-icon="expandIcon" :value="'menu-entities'" />
</template>

<script setup lang="ts">
import type { MenuOption } from 'naive-ui';
import { CaretDownOutline, FlowerOutline, HomeOutline, LeafOutline, SettingsOutline, StatsChartOutline } from '@vicons/ionicons5';
import { NIcon, NMenu } from 'naive-ui';
import { Component, h } from 'vue';

const props = defineProps<{
    collapsed?: boolean;
}>();

const menuOptions: MenuOption[] = [
    {
        label: 'Home',
        key: 'menu-general',
        icon: renderIcon(HomeOutline),
        disabled: true,
    },
    {
        label: 'Entities',
        key: 'menu-entities',
        icon: renderIcon(FlowerOutline),
    },
    {
        label: 'Components',
        key: 'menu-components',
        icon: renderIcon(LeafOutline),
        disabled: true,
    },
    {
        label: 'Settings',
        key: 'menu-settings',
        icon: renderIcon(SettingsOutline),
        disabled: true,
    },
    {
        label: 'Logs',
        key: 'menu-logs',
        icon: renderIcon(StatsChartOutline),
        disabled: true,
    },
];

function renderIcon(icon: Component) {
    return () => h(NIcon, null, { default: () => h(icon) });
}

function expandIcon() {
    return h(NIcon, null, { default: () => h(CaretDownOutline) });
}
</script>
