<template>
    <div class="table-border" ref="editorRef">
        <n-data-table :bordered="false" :columns="fieldColumns" :data="fields" :pagination="false" :row-props="fieldRowProps" size="small">
            <template #empty>
                <!-- empty node to remove template slot -->
                <div></div>
            </template>
        </n-data-table>
        <div class="create-new-field-row">
            <n-button secondary type="success" @click="activateCreateRow()" block> + Add Field </n-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { DataTableColumns } from 'naive-ui';
import { NButton, NDataTable, NIcon } from 'naive-ui';
import { computed, h, nextTick, onMounted, ref } from 'vue';
import { ArrowUndoCircleOutline, TrashBin, ReloadCircleOutline } from '@vicons/ionicons5';
import { DismissCircle20Regular } from '@vicons/fluent';
import InputSpan from '../ui/InputSpan.vue';
import FieldType from '../ui/FieldType.vue';

export type ComponentField = {
    id?: string;
    name: string;
    field_type: string;
    is_system: boolean;
    created_at: number;
};

type FieldRow = ComponentField & { key: string; operation?: 'add' | 'remove' | 'update' };

const props = defineProps<{
    modelValue: ComponentField[];
}>();

const emit = defineEmits<{
    'update:modelValue': [value: ComponentField[]];
}>();

const draggedIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);
const editorRef = ref<HTMLElement | null>(null);

function activateCreateRow() {
    const newField: ComponentField = {
        id: undefined,
        name: '',
        field_type: 'text',
        is_system: false,
        created_at: Date.now(),
    };

    emit('update:modelValue', [...props.modelValue, newField]);

    void nextTick(() => {
        const selector = `[data-field-key="${newField.created_at}"]`;
        const editable = document.querySelector(selector) as HTMLSpanElement | null;
        editable?.focus();
    });
}

const fieldColumns: DataTableColumns<FieldRow> = [
    {
        title: '',
        key: 'drag',
        width: 30,
        render: (row) => h('span', { class: 'drag-handle', title: 'Drag to reorder' }, '::'),
    },
    {
        title: 'Field',
        key: 'field',
        render: (row) =>
            h(InputSpan, {
                modelValue: row.name,
                'data-field-key': row.created_at.toString(),
                disabled: row.operation === 'remove',

                'onUpdate:modelValue': (value) => {
                    const fieldIndex = props.modelValue.findIndex((f) => f.created_at === row.created_at);
                    if (fieldIndex !== -1) {
                        const updated = [...props.modelValue];
                        updated[fieldIndex] = { ...updated[fieldIndex], name: value };
                        emit('update:modelValue', updated);
                    }
                },
            }),
    },
    {
        title: 'Type',
        key: 'type',
        render: (row) =>
            h(FieldType, {
                disabled: row.operation === 'remove',
                modelValue: row.field_type,
                'onUpdate:modelValue': (value) => {
                    const fieldIndex = props.modelValue.findIndex((f) => f.created_at === row.created_at);
                    if (fieldIndex !== -1) {
                        const updated = [...props.modelValue];
                        updated[fieldIndex] = { ...updated[fieldIndex], field_type: value };
                        emit('update:modelValue', updated);
                    }
                },
            }),
    },
    {
        title: '',
        key: 'actions',
        width: 52,
        render: (row) =>
            h(
                NButton,
                {
                    quaternary: true,
                    type: row.operation === 'remove' ? 'default' : 'error',
                    onClick() {
                        switch (row.operation) {
                            case 'update':
                                row.operation = 'remove';
                                break;
                            case 'remove':
                                row.operation = undefined;
                                break;
                            default:
                                if (row.id === undefined) {
                                    emit('update:modelValue', props.modelValue.filter((f) => f.created_at !== row.created_at));
                                } else {
                                    row.operation = 'remove';
                                }
                        }
                    },
                },
                h(NIcon, h(row.operation === 'remove' ? DismissCircle20Regular : TrashBin)),
            ),
    },
];

const fields = computed<FieldRow[]>(() =>
    props.modelValue.map((f) => ({ ...f, key: f.created_at.toString(), operation: undefined as 'add' | 'remove' | 'update' | undefined })),
);

onMounted(() => {
    void nextTick(() => {
        const activeElement = document.activeElement as HTMLElement | null;
        if (!activeElement || !editorRef.value) {
            return;
        }

        if (editorRef.value.contains(activeElement)) {
            activeElement.blur();
        }
    });
});

function fieldRowProps(row: FieldRow) {
    const rowIndex = fields.value.findIndex((item) => item.created_at === row.created_at);
    const rowClass = [row.operation ? `field-row-${row.operation}` : '', dragOverIndex.value === rowIndex ? 'drag-over-row' : '']
        .filter(Boolean)
        .join(' ');

    return {
        draggable: true,
        class: rowClass,
        onDragstart: (event: DragEvent) => {
            draggedIndex.value = rowIndex;
            event.dataTransfer?.setData('text/plain', row.created_at.toString());
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
                draggedIndex.value = null;
                dragOverIndex.value = null;
                return;
            }

            const next = [...props.modelValue];
            const [moved] = next.splice(from, 1);
            next.splice(to, 0, moved);

            emit('update:modelValue', next);

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

<style lang="scss" scoped>
.table-border {
    border: 1px solid rgba(0, 0, 0, 0.12);
    border-radius: 4px;
    overflow: hidden;
}

:deep(.n-data-table-empty) {
    display: none;
}

:deep(tr.field-row-add > td) {
    background-color: #e6ffed !important;
}

:deep(tr.field-row-remove > td) {
    background-color: #e9e9e9 !important;
}

:deep(tr.field-row-update > td) {
    background-color: #e6f8ff !important;
}

:deep(tr.drag-over-row > td) {
    border-top: 2px solid #1890ff;
}

:deep(.drag-handle) {
    cursor: grab;
    user-select: none;
}
</style>
