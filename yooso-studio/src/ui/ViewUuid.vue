<template>
    <n-popover trigger="hover">
        <template #trigger>
            <n-button class="view-uuid" :class="{ ['view-uuid-fill']: props.fill }" :type="active ? 'primary' : 'default'" :ghost="active" @click.stop="copyUuid()">
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
    fill?: boolean;
}>();

function copyUuid() {
    navigator.clipboard.writeText(props.uuid);
}
</script>

<style lang="scss" scoped>
.view-uuid {
    &.view-uuid-fill {
        flex: 1;
        width: 100%;
    }
}
</style>
