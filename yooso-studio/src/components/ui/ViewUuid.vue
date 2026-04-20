<template>
    <n-popover trigger="hover">
        <template #trigger>
            <n-button class="view-uuid" :type="active ? 'primary' : 'default'" :ghost="active" @click="copyUuid()" :style="{ marginLeft: props.marginLeft ?? '0px' }">
                {{ uuid.slice(-12) }}
            </n-button>
        </template>
        <div style="font-family: monospace">{{ uuid }}</div>
    </n-popover>
</template>

<script setup lang="ts">
import { NButton, NPopover } from 'naive-ui';

const props = defineProps<{
    uuid: string;
    active?: boolean;
    // optional; home page tables need left margin to prevent overlap with
    // sidebar collapse (toggle) button
    marginLeft?: string;
}>();

function copyUuid() {
    navigator.clipboard.writeText(props.uuid);
}
</script>

<style lang="scss" scoped>
.view-uuid {
    display: inline-block;
    font-family: monospace;
    max-width: 180px;
    width: 100%;
    // margin-left: 12px;
    padding: 0 12px;
    // height: 32px;
    // line-height: 32px;
}
</style>
