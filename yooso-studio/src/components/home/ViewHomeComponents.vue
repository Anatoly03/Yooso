<template>
    <div class="view-components">
        <n-data-table remote :loading="loadingRef" :columns="columns" :data="data" />
        <n-drawer v-model:show="editComponent" :default-width="612" :min-width="416" placement="right" resizable>
            <n-drawer-content :title="'Edit Component: ' + editComponentName">
                <n-form style="display: flex; flex-direction: column; gap: 5px;">
                    <edit-component-label :name="editComponentName" :color="editComponentColor" />
                    <view-fields-editor :component-id="editComponentId" />
                    <n-button-group class="component-action-slot">
                        <n-button type="error" @click="editComponent = false">Delete</n-button>
                        <n-button secondary type="default" @click="editComponent = false">Cancel</n-button>
                        <n-button type="primary" @click="editComponent = false">Save</n-button>
                    </n-button-group>
                </n-form>
            </n-drawer-content>
        </n-drawer>
    </div>
</template>

<script setup lang="ts">
import { NButton, NButtonGroup, NDataTable, NForm, NFormItem, NLayout, NPopover, NDrawer, NDrawerContent } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import ViewFieldsEditor from './ViewFieldsEditor.vue';
import EditComponentLabel from '../ui/EditComponentLabel.vue';

const editComponent = ref(false);
const editComponentId = ref('');
const editComponentName = ref('');
const editComponentColor = ref('');
const loadingRef = ref(true);

const columns = ref([
    {
        title: 'Component',
        key: 'name',
        render(row: any) {
            return h(
                'span',
                {
                    style: {
                        display: 'inline-block',
                        marginLeft: '12px',
                        marginRight: '6px',
                        padding: '4px 8px',
                        backgroundColor: row.color,
                        color: '#fff',
                        borderRadius: '4px',
                    },
                },
                row.name,
            );
        },
    },
    {
        title: 'Actions',
        key: 'actions',
        render(row: any) {
            return h(
                NButton,
                {
                    type: 'primary',
                    onClick: () => {
                        editComponent.value = true;
                        editComponentId.value = row.id;
                        editComponentName.value = row.name;
                        editComponentColor.value = row.color;
                    },
                },
                'Edit',
            );
        },
    },
]);

const data = ref([]);

onMounted(async () => {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/list');
        const result = await response.json();

        if (!result.success) throw new Error(result.message || 'Failed to fetch components');

        // Map color to HTML approved format
        data.value = result.components.map((component: any) => {
            let htmlColor = '#' + component.color.toString(16).padStart(6, '0');

            console.log(`Component: ${component.name} (${component.id}), Original Color: ${component.color}, HTML Color: ${htmlColor}`);
            return {
                id: component.id,
                name: component.name,
                color: htmlColor,
            };
        });
    } catch (error) {
        console.error('Error fetching components:', error);
    }

    loadingRef.value = false;
});
</script>

<style lang="scss" scoped>
.view-components {
    display: flex;
    flex-direction: column;
    width: 100%;
}

.component-action-slot {
    width: 100%;
    margin-top: 12px;
    display: flex;
    justify-content: flex-end;

    :deep(.n-layout-scroll-container) {
        flex: 1;
    }

    :deep(.n-button) {
        flex: 1 0 auto;
    }
}
</style>
