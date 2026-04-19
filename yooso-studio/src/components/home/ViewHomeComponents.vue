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
                    <view-fields-editor v-model:model-value="editComponentFields" :component-id="editComponentId" :is-new-component="editComponentIsNew" />
                    <n-button-group class="component-action-slot">
                        <n-button type="error" @click="deleteComponent()" v-if="!editComponentIsNew"> Delete </n-button>
                        <n-button secondary type="default" @click="editComponent = false">Cancel</n-button>
                        <n-button type="primary" @click="patchComponent" v-if="!editComponentIsNew"> Patch </n-button>
                        <n-button type="primary" @click="createComponent" v-if="editComponentIsNew"> Create </n-button>
                    </n-button-group>
                </n-form>
            </n-drawer-content>
        </n-drawer>
    </div>
</template>

<script setup lang="ts">
import { NButton, NButtonGroup, NDataTable, NForm, NFormItem, NLayout, NPopover, NDrawer, NDrawerContent, DataTableCreateSummary } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import ViewFieldsEditor, { type ComponentField } from './ViewFieldsEditor.vue';
import EditComponentLabel from '../ui/EditComponentLabel.vue';

const editComponent = ref(false);
const editComponentId = ref('');
const editComponentName = ref('');
const editComponentColor = ref('');
const editComponentCreatedAt = ref(0);
const editComponentFields = ref<ComponentField[]>([]);
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
                        onClick: async () => {
                            editComponent.value = true;
                            editComponentId.value = row.id;
                            editComponentName.value = row.name;
                            editComponentColor.value = row.color;
                            editComponentCreatedAt.value = row.createdAt;
                            editComponentIsNew.value = false;

                            const componentData = await viewComponent(row.id);
                            if (componentData) {
                                editComponentFields.value = componentData.fields;
                            }
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
    editComponentName.value = 'new-component';
    editComponentColor.value = '#000000';
    editComponentIsNew.value = true;
    editComponentFields.value = [];
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

            // console.log(`Component: ${component.name} (${component.id}), Original Color: ${component.color}, HTML Color: ${htmlColor}`);
            return {
                id: component.id,
                name: component.name,
                color: htmlColor,
                createdAt: component.created_at,
            };
        });
    } catch (error) {
        console.error('Error fetching components:', error);
    }

    loadingRef.value = false;
}

async function createComponent() {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: editComponentName.value,
                is_system: false,
                fields: editComponentFields.value,
                color: parseInt(editComponentColor.value.replace('#', ''), 16),
            }),
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to create/update component');
        }

        const result = await response.json();
        // console.log('Component created/updated:', result);

        refreshComponentList();
    } catch (error) {
        console.error('Error creating/updating component:', error);
    }

    editComponent.value = false;
}

async function viewComponent(id = editComponentId.value): Promise<any> {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/view/' + id);

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to create/update component');
        }

        const result = await response.json();

        refreshComponentList();

        return result;
    } catch (error) {
        console.error('Error creating/updating component:', error);
    }

    return null;
}

async function patchComponent() {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components', {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                id: editComponentId.value,
                name: editComponentName.value,
                is_system: false,
                color: parseInt(editComponentColor.value.replace('#', ''), 16),
                created_at: editComponentCreatedAt.value,
            }),
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to patch component');
        }

        const result = await response.json();
        // console.log('Component patched:', result);

        refreshComponentList();
    } catch (error) {
        console.error('Error patching component:', error);
    }

    editComponent.value = false;
}

async function deleteComponent(id = editComponentId.value) {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/' + id, {
            method: 'DELETE',
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to delete component');
        }

        const result = await response.json();
        // console.log('Component deleted:', result);

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
