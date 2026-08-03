<template>
    <div class="view-entities">
        <div class="debug-header">
            <n-button type="primary" @click="createEntity()"> Create </n-button>
        </div>
        <n-data-table class="view-entities-table" :columns="columns" :data="entities">
            <template #empty>
                <n-empty description="Custom your icon">
                    <template #icon>
                        <n-icon>
                            <FlowerOutline />
                        </n-icon>
                    </template>
                    <template #extra>
                        <n-button size="small" @click="createEntity()"> Create Entity </n-button>
                    </template>
                </n-empty>
            </template>
        </n-data-table>
    </div>
</template>

<script setup lang="ts">
import { h, onMounted, ref } from 'vue';
import { NButton, NDataTable, NEmpty, NIcon } from 'naive-ui';
import ViewUuid from '../ui/ViewUuid.vue';
import { FlowerOutline, TrashBinOutline } from '@vicons/ionicons5';

const columns = [
    {
        title: 'ID',
        key: 'id',
        width: 180,
        render(row: any) {
            return h(ViewUuid, { uuid: row.id, fill: true });
        },
    },
    { title: 'Created At', key: 'created_at' },
    {
        title: '',
        key: 'delete',
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
                        deleteEntity(row.id);
                    },
                },
                h(NIcon, h(TrashBinOutline))
            );
        },
    },
];
let entities = ref([] as any[]);

async function createEntity() {
    const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities', {
        method: 'POST',
    });
    entities.value = await loadEntities();
}

async function deleteEntity(id: string) {
    const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities/' + id, {
        method: 'DELETE',
    });
    entities.value = await loadEntities();
}

async function loadEntities(): Promise<any[]> {
    const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities');
    const list = await response.json();
    return list;
}

onMounted(async () => {
    entities.value = await loadEntities();
});
</script>

<style lang="scss" scoped>
.view-entities {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px;
    height: 100%;
    box-sizing: border-box;

    .view-entities-table {
        flex: 1;
    }
}
</style>
