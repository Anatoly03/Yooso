<template>
    <div class="view-components">
        <n-data-table remote :bordered="false" :loading="loadingRef" :columns="columns" :data="data" />
        <div class="view-components-footer">
            <n-button type="primary" @click="openCreateNewComponentDrawer"> Create Component </n-button>
        </div>
        <n-drawer v-model:show="editComponent" :default-width="612" :min-width="416" placement="right" resizable>
            <n-drawer-content :title="'Edit Component: ' + editComponentName">
                <n-form style="display: flex; flex-direction: column; gap: 5px">
                    <edit-component-label v-model:value="editComponentName" v-model:color="editComponentColor" />
                    <view-fields-editor :component-id="editComponentId" />
                    <n-button-group class="component-action-slot">
                        <n-button v-if="!editComponentIsNew" type="error" @click="deleteComponent"> Delete </n-button>
                        <n-button secondary type="default" @click="editComponent = false">Cancel</n-button>
                        <n-button type="primary" @click="createUpdateComponent">
                            {{ editComponentIsNew ? 'Create' : 'Save' }}
                        </n-button>
                    </n-button-group>
                </n-form>
            </n-drawer-content>
        </n-drawer>
    </div>
</template>

<script setup lang="ts">
import { NButton, NButtonGroup, NDataTable, NForm, NFormItem, NLayout, NPopover, NDrawer, NDrawerContent, DataTableCreateSummary } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import ViewFieldsEditor from './ViewFieldsEditor.vue';
import EditComponentLabel from '../ui/EditComponentLabel.vue';

const editComponent = ref(false);
const editComponentId = ref('');
const editComponentName = ref('');
const editComponentColor = ref('');
const editComponentIsNew = ref(false);
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
            return h(NButtonGroup, null, [
                h(
                    NButton,
                    {
                        type: 'primary',
                        onClick: () => {
                            editComponent.value = true;
                            editComponentId.value = row.id;
                            editComponentName.value = row.name;
                            editComponentColor.value = row.color;
                            editComponentIsNew.value = false;
                        },
                    },
                    'Edit',
                ),
                h(
                    NButton,
                    {
                        type: 'error',
                        onClick: () => {
                            deleteComponent(row.id);
                        },
                    },
                    'Delete',
                ),
            ]);
        },
    },
]);

const data = ref([]);

function openCreateNewComponentDrawer() {
    editComponent.value = true;
    editComponentId.value = '';
    editComponentName.value = 'New Component';
    editComponentColor.value = '#000000';
    editComponentIsNew.value = true;
}

async function refreshComponentList() {
    loadingRef.value = true;

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
}

async function createUpdateComponent() {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/create', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: editComponentName.value,
                is_system: false,
                color: parseInt(editComponentColor.value.replace('#', ''), 16),
            }),
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to create/update component');
        }

        const result = await response.json();
        console.log('Component created/updated:', result);

        refreshComponentList();
    } catch (error) {
        console.error('Error creating/updating component:', error);
    }

    editComponent.value = false;
}

async function deleteComponent(id = editComponentId.value) {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/delete/' + id, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: editComponentName.value,
                is_system: false,
                color: parseInt(editComponentColor.value.replace('#', ''), 16),
            }),
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to delete component');
        }

        const result = await response.json();
        console.log('Component deleted:', result);

        refreshComponentList();
    } catch (error) {
        console.error('Error deleting component:', error);
    }

    editComponent.value = false;
}

onMounted(async () => {
    refreshComponentList();
});
</script>

<style lang="scss" scoped>
.view-components {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;

    :deep(.n-data-table) {
        flex: 1 1 0;
        min-height: 0;
        overflow: auto;
    }
}

.view-components-footer {
    flex: 0 0 auto;
    display: flex;
    padding: 10px 24px;
    gap: 8px;

    border-top: 1px solid #eee;
    background-color: #f5f5f5;
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
