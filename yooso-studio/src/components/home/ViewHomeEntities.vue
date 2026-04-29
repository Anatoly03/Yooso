<template>
    <div class="view-entities-home">
        <view-entities :loading="loadingRef" :data="data" :active-entity="addComponentEntityId" @view-entity="console.log" @new-entity="createEntity" @add-component="openAddComponentDrawer" @edit-component="openEditComponentDrawer" @remove-component="removeComponent" />
        <!-- <n-data-table remote :loading="loadingRef" :bordered="false" :columns="columns" :data="data" /> -->
        <n-drawer v-model:show="addComponentDrawer" :default-width="612" :min-width="416" placement="right" resizable>
            <n-drawer-content :title="(addComponentIsEdit ? $t('app.edit.component') : $t('app.add.component')) + ': ' + addComponentName">
                <n-form style="display: flex; flex-direction: column; gap: 5px" label-placement="left" label-width="auto" size="small">
                    <n-card size="small">
                        <small>{{ $t('app.actions.preview') }}</small>
                        <div>
                            <view-uuid active :uuid="addComponentEntityId" />
                            <div
                                class="view-component-label"
                                :style="{
                                    display: 'inline-block',
                                    padding: '5px 8px',
                                    backgroundColor: addComponentColor || '#c1d1d1',
                                    color: 'black',
                                    borderRadius: '4px',
                                    marginLeft: '6px',
                                }"
                            >
                                {{ addComponentName }}
                            </div>
                        </div>
                    </n-card>
                    <!-- <n-card size="small" v-if="addComponentFields.length"> -->
                    <n-form-item v-for="field in addComponentFields" :key="field.id" :path="field.id" :label="field.name" :show-feedback="false" style="padding: 2px 0">
                        <n-input v-if="field.field_type === 'text'" v-model:value="addComponentData[field.name]" />
                        <n-input-number v-else-if="field.field_type === 'number' || field.field_type === 'integer'" v-model:value="addComponentData[field.name]" />
                        <n-switch v-else-if="field.field_type === 'boolean'" v-model:value="addComponentData[field.name]" />
                    </n-form-item>
                    <!-- </n-card> -->
                    <!-- <edit-component-label v-model:value="editComponentName" v-model:color="editComponentColor" />
                    <view-fields-editor v-model:loading="editComponentLoadingRef" v-model:model-value="editComponentFields" :component-id="editComponentId" :is-new-component="editComponentIsNew" /> -->
                    <n-button-group class="entity-action-slot">
                        <n-button secondary type="default" @click="addComponentDrawer = false">{{ $t('app.actions.cancel') }}</n-button>
                        <n-button type="primary" :loading="addComponentSubmittingRef" @click="submitAddComponent">{{ addComponentIsEdit ? $t('app.actions.save') : $t('app.actions.add') }}</n-button>
                    </n-button-group>
                </n-form>
            </n-drawer-content>
        </n-drawer>
    </div>
</template>

<script setup lang="ts">
import { NButton, NButtonGroup, NCard, NDrawer, NDrawerContent, NForm, NFormItem, NInput, NInputNumber, NSwitch } from 'naive-ui';
import { computed, h, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ViewEntities from '../tables/ViewEntities.vue';
import ViewUuid from '../ui/ViewUuid.vue';

const loadingRef = ref(true);
const addComponentDrawerRef = ref(false);
const addComponentName = ref('');
const addComponentColor = ref('');
const addComponentEntityId = ref('');
const addComponentId = ref('');
const addComponentFields = ref<any[]>([]);
const addComponentData = ref<Record<string, any>>({});
const addComponentIsEdit = ref(false);
const addComponentSubmittingRef = ref(false);

const addComponentDrawer = computed({
    get: () => addComponentDrawerRef.value,
    set: (value) => {
        if (!value) {
            addComponentEntityId.value = '';
            addComponentId.value = '';
            addComponentName.value = '';
            addComponentColor.value = '';
            addComponentFields.value = [];
            addComponentData.value = {};
            addComponentIsEdit.value = false;
            addComponentSubmittingRef.value = false;
        }
        addComponentDrawerRef.value = value;
    },
});

const data = ref([
    // {
    //     id: '9179dee9-ad7e-4d74-99d9-186d82a9c932',
    //     components: [
    //         { name: 'Superuser', color: 'red' },
    //         { name: 'EmailAuth', color: 'blue' },
    //         { name: 'PassAuth', color: 'blue' },
    //     ],
    // },
    // {
    //     id: '7ddae4b2-ccb4-4890-94d4-45339eaaa982',
    //     components: [
    //         { name: 'User', color: 'green' },
    //         { name: 'EmailAuth', color: 'blue' },
    //         { name: 'PassAuth', color: 'blue' },
    //     ],
    // },
    // {
    //     id: 'cd2db480-141f-47cb-9325-b3107220b0b6',
    //     components: [
    //         { name: 'Channel', color: 'purple' },
    //         { name: 'TextChannel', color: 'gray' },
    //     ],
    // },
    // {
    //     id: 'cd2db480-141f-47cb-9325-b3107220b0b6',
    //     components: [
    //         { name: 'Channel', color: 'purple' },
    //         { name: 'VoiceChannel', color: 'orange' },
    //     ],
    // },
    // {
    //     id: '73295ceb-9220-4b56-8657-08e6282ac815',
    //     components: [
    //         { name: 'Message', color: 'purple' },
    //         { name: 'TextMessage', color: 'gray' },
    //         { name: 'MessageAttachments', color: 'cyan' },
    //     ],
    // },
]);

async function refreshEntityList() {
    loadingRef.value = true;

    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities/list');
        const result = await response.json();

        if (!result.success) throw new Error(result.message || 'Failed to fetch entities');

        data.value = result.entities.map((entity: any) => ({
            ...entity,
            components: entity.components ?? [],
        }));

        console.log('Fetched entities:', result.entities);
    } catch (error) {
        console.error('Error fetching entities:', error);
    }

    loadingRef.value = false;
}

async function createEntity() {
    loadingRef.value = true;

    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to create/update entity');
        }

        const result = await response.json();
        // console.log('Entity created/updated:', result);

        refreshEntityList();
    } catch (error) {
        console.error('Error creating/updating entity:', error);
    }
}

function createDefaultComponentData(fields: any[]) {
    return fields.reduce((acc: Record<string, any>, field: any) => {
        if (field.field_type === 'boolean') {
            acc[field.name] = false;
        } else if (field.field_type === 'number' || field.field_type === 'integer') {
            acc[field.name] = 0;
        } else {
            acc[field.name] = '';
        }

        return acc;
    }, {});
}

function normalizeEditComponentData(fields: any[], existingValues: Record<string, any>) {
    return fields.reduce((acc: Record<string, any>, field: any) => {
        const snakeCaseFieldName = field.name.replace(/-/g, '_');

        if (existingValues[field.name] !== undefined) {
            acc[field.name] = existingValues[field.name];
        } else if (existingValues[snakeCaseFieldName] !== undefined) {
            acc[field.name] = existingValues[snakeCaseFieldName];
        } else if (field.field_type === 'boolean') {
            acc[field.name] = false;
        } else if (field.field_type === 'number' || field.field_type === 'integer') {
            acc[field.name] = 0;
        } else {
            acc[field.name] = '';
        }

        return acc;
    }, {});
}

function normalizeComponentValue(field: any, value: any) {
    if (field.field_type === 'boolean') {
        if (typeof value === 'boolean') {
            return value;
        }

        if (typeof value === 'number') {
            return value !== 0;
        }

        if (typeof value === 'string') {
            const normalized = value.trim().toLowerCase();
            if (normalized === 'true' || normalized === '1') {
                return true;
            }

            if (normalized === 'false' || normalized === '0' || normalized === '') {
                return false;
            }
        }

        return Boolean(value);
    }

    if (field.field_type === 'number' || field.field_type === 'integer') {
        if (typeof value === 'number') {
            return value;
        }

        if (typeof value === 'string') {
            const parsed = Number(value);
            return Number.isNaN(parsed) ? 0 : parsed;
        }

        return 0;
    }

    if (value === null || value === undefined) {
        return '';
    }

    return String(value);
}

function buildComponentPayload(fields: any[], data: Record<string, any>) {
    return fields.reduce((payload: Record<string, any>, field: any) => {
        payload[field.name] = normalizeComponentValue(field, data[field.name]);
        return payload;
    }, {});
}

async function openComponentDrawer(entityId: string, componentId: string, isEdit: boolean) {
    addComponentEntityId.value = entityId;
    addComponentId.value = componentId;
    addComponentName.value = componentId;
    addComponentColor.value = '';
    addComponentIsEdit.value = isEdit;

    // Fetch component details to get schema and color for preview
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + `/api/components/view/${componentId}`);
        const result = await response.json();

        addComponentName.value = result.metadata.name;
        addComponentColor.value = '#' + result.metadata.color.toString(16).padStart(6, '0');

        addComponentFields.value = result.fields;
        addComponentData.value = createDefaultComponentData(result.fields);

        if (isEdit) {
            const entityResponse = await fetch(import.meta.env.VITE_API_SERVER + `/api/entities/view/${entityId}`);
            const entityResult = await entityResponse.json();

            if (!entityResponse.ok || !entityResult.components) {
                throw new Error(entityResult.error || entityResult.message || 'Failed to fetch entity component values');
            }

            const componentKey = addComponentName.value.replace(/-/g, '_');
            const existingValues = entityResult.components[componentKey];

            if (!existingValues) {
                throw new Error(`Component ${addComponentName.value} is not set for entity ${entityId}`);
            }

            addComponentData.value = normalizeEditComponentData(result.fields, existingValues);
        }
    } catch (error) {
        console.error('Error fetching component details:', error);
        addComponentEntityId.value = '';
        addComponentId.value = '';
        addComponentName.value = '';
        addComponentColor.value = '';
        addComponentFields.value = [];
        addComponentData.value = {};
        addComponentIsEdit.value = false;
        return;
    }

    addComponentDrawer.value = true;
}

async function openAddComponentDrawer(entityId: string, componentId: string) {
    await openComponentDrawer(entityId, componentId, false);
}

async function openEditComponentDrawer(entityId: string, componentId: string) {
    await openComponentDrawer(entityId, componentId, true);
}

async function removeComponent(entityId: string, componentId: string) {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + `/api/entities/${entityId}/component/${componentId}`, {
            method: 'DELETE',
        });

        const result = await response.json();
        if (!response.ok || !result.success) {
            throw new Error(result.error || result.message || 'Failed to remove component from entity');
        }
    } catch (error) {
        console.error('Error removing component from entity:', error);
    }

    await refreshEntityList();
}

async function submitAddComponent() {
    if (!addComponentEntityId.value || !addComponentId.value) {
        return;
    }

    addComponentSubmittingRef.value = true;

    try {
        const payload = buildComponentPayload(addComponentFields.value, addComponentData.value);
        const response = await fetch(import.meta.env.VITE_API_SERVER + `/api/entities/${addComponentEntityId.value}/component/${addComponentId.value}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        });

        const result = await response.json();
        if (!response.ok || !result.success) {
            throw new Error(result.error || result.message || 'Failed to add component to entity');
        }

        addComponentDrawer.value = false;
    } catch (error) {
        console.error('Error adding component to entity:', error);
    }

    addComponentSubmittingRef.value = false;

    await refreshEntityList();
}

onMounted(() => {
    refreshEntityList();
});
</script>

<style lang="scss" scoped>
.view-entities-home {
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
}

.view-entities-footer {
    flex: 0 0 auto;
    display: flex;
    padding: 10px 24px;
    gap: 8px;

    border-top: 1px solid #eee;
    background-color: #f5f5f5;
}

.entity-action-slot {
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
