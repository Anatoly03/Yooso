<template>
    <n-drawer :show="show" @update-show="(b) => emit('update:show', b)" :default-width="720" :min-width="416" placement="right" resizable>
        <n-drawer-content class="view-documentation">
            <n-tabs class="view-documentation-tabs" v-model:value="currentTab" type="line" placement="left">
                <n-tab name="view"> View </n-tab>
                <n-tab name="create"> Create </n-tab>
                <n-tab name="delete"> Delete </n-tab>
            </n-tabs>
            <div class="view-documentation-content">
                <ViewAPIRoute :method="httpMethod">
                    {{ httpUrl }}
                </ViewAPIRoute>

                {{ currentTab }}
            </div>
        </n-drawer-content>
    </n-drawer>
</template>

<script setup lang="ts">
import { NDrawer, NDrawerContent, NLayout, NLayoutContent, NLayoutSider, NTab, NTabs } from 'naive-ui';
import { computed, ref } from 'vue';

import ViewAPIRoute from './ViewAPIRoute.vue';

const currentTab = ref('create');
const props = defineProps<{ show: boolean }>();
const emit = defineEmits<{ 'update:show': [value: boolean] }>();

const httpMethod = computed(() => {
    switch (currentTab.value) {
        case 'create':
            return 'post';
        case 'delete':
            return 'delete';
        case 'view':
        default:
            return 'get';
    }
});

const httpUrl = computed(() => {
    const prefix = import.meta.env.VITE_API_SERVER + '/api/components';

    switch (currentTab.value) {
        case 'create':
            return prefix;
        case 'delete':
        case 'view':
            return prefix + '/:id';
        default:
            return '???';
    }
});
</script>

<style lang="scss" scoped>
.view-documentation {
    :deep(.n-drawer-body-content-wrapper) {
        display: flex;
        width: 100%;
        flex-direction: row;
        gap: 8px;
    }

    .view-documentation-tabs {
        display: flex;
        flex: 0;
        height: 100%;
    }

    .view-documentation-content {
        display: block;
        flex: 1;
    }
}
</style>
