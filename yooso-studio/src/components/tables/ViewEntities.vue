<template>
    <div class="view-entities table-border" ref="editorRef">
        <n-data-table remote :loading="props.loading" :bordered="false" :columns="columns" :row-props="entityRowProps" :data="data" />
        <div class="view-entities-footer">
            <n-button type="primary" @click="emit('new-entity')"> {{ $t('app.create.entity') }} </n-button>
            <n-button type="info" @click="emit('view-documentation')" disabled> {{ $t('app.documentation.api') }} </n-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { NButton, NDataTable } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ViewUuid from '../ui/ViewUuid.vue';
import EditEntityComponents, { ComponentOption } from '../ui/EditEntityComponents.vue';

export interface Row {
    id: string;
    name: string;
    color: string;
}

const i18n = useI18n();
const props = defineProps<{
    loading?: boolean;
    data: Row[];
    'active-entity'?: string;
}>();
const availableComponents = ref<ComponentOption[]>([]);
const emit = defineEmits<{
    'view-entity': [entityId: string];
    'new-entity': [];
    'add-component': [entityId: string, componentId: string];
    'edit-component': [entityId: string, componentId: string];
    'remove-component': [entityId: string, componentId: string];
    'delete-entity': [entityId: string];
    'view-documentation': [];
}>();

const columns = ref([
    {
        title: () => h('span', { style: { marginLeft: '12px' } }, i18n.t('app.keywords.id')),
        key: 'id',
        width: 180,
        render(row: any) {
            return h(ViewUuid, { uuid: row.id, active: row.id === props['active-entity'], marginLeft: '12px' });
        },
    },
    {
        title: i18n.t('app.keywords.component', 2),
        key: 'components',
        render(row: any) {
            const componentRenderKey = `${row.id}:${(row.components ?? []).map((c: any) => c.id).join(',')}`;

            return h(EditEntityComponents, {
                key: componentRenderKey,
                entityId: row.id,
                components: row.components,
                allComponents: availableComponents.value,
                onAddComponent: async (entityId: string, componentId: string) => {
                    emit('add-component', entityId, componentId);
                },
                onEditComponent: async (entityId: string, componentId: string) => {
                    emit('edit-component', entityId, componentId);
                },
                onRemoveComponent: async (entityId: string, componentId: string) => {
                    emit('remove-component', entityId, componentId);
                },
            });
        },
    },
    {
        title: '',
        key: 'delete-entity',
        width: 110,
        align: 'center' as const,
        render(row: any) {
            return h(
                NButton,
                {
                    type: 'error',
                    secondary: true,
                    size: 'small',
                    onClick: (event: MouseEvent) => {
                        event.stopPropagation();
                        emit('delete-entity', row.id);
                    },
                },
                {
                    default: () => i18n.t('app.actions.delete'),
                },
            );
        },
    },
]);


async function fetchComponentOptions() {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/list');
        const result = await response.json();

        if (!result.success) {
            throw new Error(result.message || 'Failed to fetch components');
        }

        availableComponents.value = result.components.map((component: any) => ({
            id: component.id,
            label: component.name,
            value: component.name,
            color: '#' + component.color.toString(16).padStart(6, '0'),
        }));
    } catch (error) {
        console.error('Error fetching components:', error);
    }
}

function entityRowProps(row: Row) {
    return {
        class: 'clickable-row',
        onClick: async () => {
            emit('view-entity', row.id);
        },
    };
}

onMounted(() => {
    fetchComponentOptions();
});
</script>

<style lang="scss" scoped>
.view-entities {
    box-sizing: border-box;
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

.table-border {
    border: 1px solid rgba(0, 0, 0, 0.12);
    border-radius: 4px;
    overflow: hidden;
}
</style>
