<template>
    <div class="view-components table-border" ref="editorRef">
        <n-data-table remote :loading="props.loading" :bordered="false" :columns="columns" :row-props="componentRowProps" :data="data" />
        <div class="view-components-footer">
            <n-button type="primary" @click="emit('new-component')"> {{ $t('app.create.component') }} </n-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { NButton, NDataTable } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import ViewUuid from '../ui/ViewUuid.vue';

export interface Row {
    id: string;
    name: string;
    color: string;
}

const i18n = useI18n();
const props = defineProps<{
    loading?: boolean;
    data: Row[];
}>();
const emit = defineEmits<{
    'view-component': [componentId: string];
    'new-component': [];
}>();

const columns = ref([
    {
        title: () => h('span', { style: { marginLeft: '12px' } }, i18n.t('app.keywords.id')),
        key: 'id',
        width: 180,
        render: (row: Row) => h(ViewUuid, { uuid: row.id, marginLeft: '12px' }),
    },
    {
        title: i18n.t('app.keywords.component', 1),
        key: 'name',
        render(row: Row) {
            return h(
                'span',
                {
                    style: {
                        display: 'inline-block',
                        marginRight: '6px',
                        padding: '4px 8px',
                        backgroundColor: row.color,
                        color: 'black',
                        borderRadius: '4px',
                    },
                },
                row.name,
            );
        },
    },
]);

function componentRowProps(row: Row) {
    return {
        class: 'clickable-row',
        onClick: async () => {
            emit('view-component', row.id);
        },
    };
}
</script>

<style lang="scss" scoped>
.view-components {
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

.view-components-footer {
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
