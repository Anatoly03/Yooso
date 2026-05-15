<template>
    <div class="view-components-home">
        <view-components
            :loading="loadingRef"
            :data="data"
            @view-component="openEditComponentDrawer"
            @new-component="openCreateNewComponentDrawer"
            @view-documentation="openDocumentationDrawer = true"
        />
        <view-component-documentation v-model:show="openDocumentationDrawer" />
        <n-drawer v-model:show="editComponent" :default-width="612" :min-width="416" placement="right" resizable>
            <n-drawer-content :title="editComponentIsNew ? $t('app.create.component') : $t('app.actions.edit') + ': ' + editComponentName">
                <n-form style="display: flex; flex-direction: column; gap: 5px">
                    <edit-component-label v-model:value="editComponentName" v-model:color="editComponentColor" />
                    <view-fields-editor v-model:loading="editComponentLoadingRef" v-model:model-value="editComponentFields" :component-id="editComponentId" :is-new-component="editComponentIsNew" />
                    <span class="error-message" v-if="editComponentError">{{ editComponentError }}</span>
                    <n-button-group class="component-action-slot">
                        <n-button type="error" @click="deleteComponent()" v-if="!editComponentIsNew"> {{ $t('app.actions.delete') }} </n-button>
                        <n-button secondary type="default" @click="editComponent = false"> {{ $t('app.actions.cancel') }} </n-button>
                        <n-button type="primary" :loading="editComponentSubmittingRef" @click="patchComponent" v-if="!editComponentIsNew"> {{ $t('app.actions.save') }} </n-button>
                        <n-button type="primary" :loading="editComponentSubmittingRef" @click="createComponent" v-else> {{ $t('app.actions.create') }} </n-button>
                    </n-button-group>
                </n-form>
            </n-drawer-content>
        </n-drawer>
    </div>
</template>

<script setup lang="ts">
import { NButton, NButtonGroup, NForm, NDrawer, NDrawerContent } from 'naive-ui';
import { onMounted, ref } from 'vue';
import yooso from '../../services/yooso';

import EditComponentLabel from '../ui/EditComponentLabel.vue';
import ViewFieldsEditor, { type ComponentField } from '../tables/ViewFieldsEditor.vue';
import ViewComponents from '../tables/ViewComponents.vue';
import ViewComponentDocumentation from '../docs/ViewComponentDocumentation.vue';

const editComponent = ref(false);
const editComponentId = ref('');
const editComponentName = ref('');
const editComponentColor = ref('');
const editComponentCreatedAt = ref(0);
const editComponentFields = ref<ComponentField[]>([]);
const editComponentIsNew = ref(false);
const loadingRef = ref(true);
const editComponentLoadingRef = ref(false);
const editComponentSubmittingRef = ref(false);
const editComponentError = ref<string | null>(null);
const openDocumentationDrawer = ref(false);
const data = ref<any[]>([]);

async function openEditComponentDrawer(id: string) {
    const row = data.value.find((c) => c.id === id);

    editComponent.value = true;
    editComponentId.value = row.id;
    editComponentName.value = row.name;
    editComponentColor.value = row.color;
    editComponentCreatedAt.value = row.createdAt;
    editComponentIsNew.value = false;
    editComponentLoadingRef.value = true;
    editComponentSubmittingRef.value = false;

    const componentData = await viewComponent(row.id);
    if (componentData) {
        editComponentFields.value = componentData.fields.map((field: any) => ({
            ...field,
            original_name: field.name,
        }));
    }
    editComponentLoadingRef.value = false;
}

function openCreateNewComponentDrawer() {
    editComponent.value = true;
    editComponentId.value = '';
    editComponentName.value = 'new_component';
    editComponentColor.value = '#C1D1E1';
    editComponentIsNew.value = true;
    editComponentFields.value = [];
    editComponentLoadingRef.value = false;
    editComponentError.value = null;
}

async function refreshComponentList() {
    yooso
        .components()
        .subscribeLoadingRef(loadingRef)
        .list()
        .then((result) => {
            data.value = result.map((component: any) => ({
                id: component.id,
                name: component.component_name,
                color: '#' + component.color.toString(16).padStart(6, '0'),
                createdAt: component.created_at,
            }));
        });
}

async function createComponent() {
    yooso
        .components()
        .subscribeLoadingRef(editComponentSubmittingRef)
        .subscribeErrorRef(editComponentError)
        .create({
            name: editComponentName.value,
            is_system: false,
            fields: editComponentFields.value,
            color: parseInt(editComponentColor.value.replace('#', ''), 16),
        })
        .then(() => {
            refreshComponentList();
            editComponent.value = false;
        });
}

async function viewComponent(id = editComponentId.value) {
    return yooso.components().subscribeErrorRef(editComponentError).view(id);
}

async function patchComponent() {
    editComponentSubmittingRef.value = true;

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
                // send fields that have an operation (add, update, remove)
                fields: editComponentFields.value.filter((f) => f.operation),
            }),
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.error || 'Failed to patch component');
        }

        await response.json();
        // console.log('Component patched:', result);

        refreshComponentList();
    } catch (error: any) {
        console.error('Error patching component:', error);
        editComponentSubmittingRef.value = false;
        editComponentError.value = error.message || String(error);
        return;
    }

    editComponent.value = false;
    editComponentError.value = null;
}

async function deleteComponent(id = editComponentId.value) {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/' + id, {
            method: 'DELETE',
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.error || 'Failed to delete component');
        }

        await response.json();
        // console.log('Component deleted:', result);

        refreshComponentList();
    } catch (error: any) {
        console.error('Error deleting component:', error);
        editComponentError.value = error.message || String(error);
        return;
    }

    editComponent.value = false;
    editComponentError.value = null;
}

onMounted(async () => {
    refreshComponentList();
});
</script>

<style lang="scss" scoped>
.view-components-home {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    padding: 12px;
    padding-left: 24px;
    width: 100%;
    height: 100%;

    :deep(.n-data-table) {
        flex: 1 1 0;
        min-height: 0;
        overflow: auto;
    }

    :deep(tr.clickable-row) {
        cursor: pointer;
    }
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

.error-message {
    color: red;
    font-size: 0.9em;
}
</style>
