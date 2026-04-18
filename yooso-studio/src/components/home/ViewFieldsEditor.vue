<template>
    <n-data-table :columns="fieldColumns" :data="fields" :pagination="false" :row-props="fieldRowProps" size="small" />
</template>

<script setup lang="ts">
import type { DataTableColumns } from 'naive-ui';
import { NDataTable } from 'naive-ui';
import { h, onMounted, ref } from 'vue';

const draggedIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);

type FieldRow = {
    key: string;
    field: string;
    type: string;
};

const fieldColumns: DataTableColumns<FieldRow> = [
    {
        title: '',
        key: 'drag',
        width: 52,
        render: () => h('span', { class: 'drag-handle', title: 'Drag to reorder' }, '::'),
    },
    {
        title: 'Field',
        key: 'field',
    },
];

const fields = ref<FieldRow[]>([
    { key: 'username', field: 'username', type: 'uuid' },
    { key: 'age', field: 'age', type: 'integer' },
    { key: 'updated', field: 'updated', type: 'datetime' },
]);

function fieldRowProps(row: FieldRow) {
    const rowIndex = fields.value.findIndex((item) => item.key === row.key);

    return {
        draggable: true,
        class: dragOverIndex.value === rowIndex ? 'drag-over-row' : '',
        onDragstart: (event: DragEvent) => {
            draggedIndex.value = rowIndex;
            event.dataTransfer?.setData('text/plain', row.key);
            if (event.dataTransfer) {
                event.dataTransfer.effectAllowed = 'move';
            }
        },
        onDragover: (event: DragEvent) => {
            event.preventDefault();
            dragOverIndex.value = rowIndex;
            if (event.dataTransfer) {
                event.dataTransfer.dropEffect = 'move';
            }
        },
        onDrop: (event: DragEvent) => {
            event.preventDefault();
            const from = draggedIndex.value;
            const to = rowIndex;
            if (from === null || from === to) {
                return;
            }

            const next = [...fields.value];
            const [moved] = next.splice(from, 1);
            next.splice(to, 0, moved);
            fields.value = next;

            draggedIndex.value = null;
            dragOverIndex.value = null;
        },
        onDragend: () => {
            draggedIndex.value = null;
            dragOverIndex.value = null;
        },
    };
}
</script>
