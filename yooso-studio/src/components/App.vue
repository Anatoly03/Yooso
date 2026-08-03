<template>
    <n-layout class="app-content" has-sider>
        <n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed" show-trigger @collapse="collapsed = true" @expand="collapsed = false">
            <n-menu
                :collapsed="collapsed"
                :collapsed-width="64"
                :collapsed-icon-size="22"
                :options="menuOptions"
                :expand-icon="expandIcon"
            />
        </n-layout-sider>
        <n-layout>
            <span>Content</span>
        </n-layout>
    </n-layout>
</template>

<script setup lang="ts">
import type { MenuOption } from 'naive-ui';
import { BookmarkOutline, CaretDownOutline, FlowerOutline, HomeOutline, LeafOutline, StatsChartOutline } from '@vicons/ionicons5';
import { NIcon, NLayout, NLayoutSider, NMenu } from 'naive-ui';
import { h, ref } from 'vue';

const menuOptions: MenuOption[] = [
    {
        label: 'General',
        key: 'menu-general',
        icon: renderIcon(HomeOutline),
    },
    {
        label: 'Entities',
        key: 'menu-entities',
        icon: renderIcon(FlowerOutline),
        disabled: true,
    },
    {
        label: 'Components',
        key: 'menu-components',
        icon: renderIcon(LeafOutline),
        disabled: true,
    },
    {
        label: 'Logs',
        key: 'menu-logs',
        icon: renderIcon(StatsChartOutline),
        disabled: true,
    },
];

const collapsed = ref(true);

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

function expandIcon() {
    return h(NIcon, null, { default: () => h(CaretDownOutline) });
}
</script>

<style lang="scss">
html {
    margin: 0;
    width: 100vw;
    height: 100vh;
}

#app, .app-content {
    width: 100vw;
    height: 100vh;
}
</style>