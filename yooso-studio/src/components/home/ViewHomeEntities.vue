<template>
    <div class="view-entities">
        <n-data-table remote :loading="loadingRef" :bordered="false" :columns="columns" :data="data" />
        <div class="view-entities-footer">
            <n-button type="primary" @click="createEntity"> Create Entity </n-button>
        </div>
        <n-drawer v-model:show="addComponentDrawer" :default-width="612" :min-width="416" placement="right" resizable>
            <n-drawer-content :title="'Add Component: ' + addComponentName">
                <n-form style="display: flex; flex-direction: column; gap: 5px" label-placement="left" label-width="auto" size="small">
                    <n-card size="small">
                        <small>Preview</small>
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
                        <n-button secondary type="default" @click="addComponentDrawer = false">Cancel</n-button>
                        <n-button type="primary" :loading="addComponentSubmittingRef" @click="submitAddComponent">Add</n-button>
                    </n-button-group>
                </n-form>
            </n-drawer-content>
        </n-drawer>
    </div>
</template>

<script setup lang="ts">
import { NButton, NButtonGroup, NCard, NDataTable, NDrawer, NDrawerContent, NForm, NFormItem, NInput, NInputNumber, NPopover, NSwitch } from 'naive-ui';
import { computed, h, onMounted, ref } from 'vue';
import EditEntityComponents from '../ui/EditEntityComponents.vue';
import ViewUuid from '../ui/ViewUuid.vue';

const loadingRef = ref(true);
const addComponentDrawerRef = ref(false);
const addComponentName = ref('');
const addComponentColor = ref('');
const addComponentEntityId = ref('');
const addComponentId = ref('');
const addComponentFields = ref<any[]>([]);
const addComponentData = ref<Record<string, any>>({});
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
            addComponentSubmittingRef.value = false;
        }
        addComponentDrawerRef.value = value;
    },
});

const columns = ref([
    {
        title: () => h('span', { style: { marginLeft: '12px' } }, 'ID'),
        key: 'id',
        width: 180,
        render(row: any) {
            return h(ViewUuid, { uuid: row.id, active: row.id === addComponentEntityId.value, marginLeft: '12px' });
        },
    },
    {
        title: 'Components',
        key: 'components',
        render(row: any) {
            const componentRenderKey = `${row.id}:${(row.components ?? []).map((c: any) => c.id).join(',')}`;

            return h(EditEntityComponents, {
                key: componentRenderKey,
                entityId: row.id,
                components: row.components,
                onAddComponent: async (entityId: string, componentId: string) => {
                    addComponentEntityId.value = entityId;
                    addComponentId.value = componentId;
                    addComponentName.value = componentId;
                    addComponentColor.value = '';

                    // Fetch component details to get color for preview
                    try {
                        const response = await fetch(import.meta.env.VITE_API_SERVER + `/api/components/view/${componentId}`);
                        const result = await response.json();

                        addComponentName.value = result.metadata.name;
                        addComponentColor.value = '#' + result.metadata.color.toString(16).padStart(6, '0');

                        addComponentFields.value = result.fields;
                        addComponentData.value = result.fields.reduce((acc: Record<string, any>, field: any) => {
                            if (field.field_type === 'boolean') {
                                acc[field.name] = false;
                            } else if (field.field_type === 'number' || field.field_type === 'integer') {
                                acc[field.name] = 0;
                            } else {
                                acc[field.name] = '';
                            }

                            return acc;
                        }, {});
                    } catch (error) {
                        console.error('Error fetching component details:', error);
                        addComponentEntityId.value = '';
                        addComponentId.value = '';
                        return;
                    }

                    addComponentDrawer.value = true;
                },
            });
        },
    },
]);

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

async function submitAddComponent() {
    if (!addComponentEntityId.value || !addComponentId.value) {
        return;
    }

    addComponentSubmittingRef.value = true;

    try {
        const response = await fetch(
            import.meta.env.VITE_API_SERVER + `/api/entities/${addComponentEntityId.value}/component/${addComponentId.value}`,
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(addComponentData.value),
            },
        );

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
.view-entities {
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
