<template>
    <div class="table-border" ref="editorRef">
        <n-data-table remote :loading="props.loading" :bordered="false" :columns="fieldColumns" :data="fields" :pagination="false" :row-props="fieldRowProps" size="small">
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
    client_key?: string;
    name: string;
    field_type: string;
    is_system: boolean;
    created_at: number;
    operation?: 'add' | 'remove' | 'update';
};

type FieldRow = ComponentField & { key: string; operation?: 'add' | 'remove' | 'update' };

const props = defineProps<{
    loading?: boolean;
    isNewComponent?: boolean;
    modelValue: ComponentField[];
}>();

const emit = defineEmits<{
    'update:loading': [value: boolean];
    'update:modelValue': [value: ComponentField[]];
}>();

const draggedIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);
const editorRef = ref<HTMLElement | null>(null);

function getFieldKey(field: Pick<ComponentField, 'id' | 'client_key' | 'created_at'>) {
    return field.id ?? field.client_key ?? `new-${field.created_at}`;
}

function activateCreateRow() {
    const newField: ComponentField = {
        id: undefined,
        client_key: crypto.randomUUID(),
        name: '',
        field_type: 'text',
        is_system: false,
        created_at: Date.now(),
        operation: props.isNewComponent ? undefined : 'add',
    };

    emit('update:modelValue', [...props.modelValue, newField]);

    void nextTick(() => {
        const selector = `[data-field-key="${getFieldKey(newField)}"]`;
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
                'data-field-key': row.key,
                disabled: row.operation === 'remove',
                pattern: /^[a-zA-Z0-9\-]*$/,
                'onUpdate:modelValue': (value) => {
                    const fieldIndex = props.modelValue.findIndex((f) => getFieldKey(f) === row.key);
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
                // TODO: support for type migrations in distant future, currently disabled
                // (number -> text, but not text -> number)
                // DATETIME -> NUMBER
                // NUMBER -> BOOL
                // BOOL -> TEXT
                // NUMBER -> DATETIME
                // DATETIME -> TEXT
                // BOOl -> NUMBER
                disabled: props.isNewComponent ? false : (!row.operation || row.operation !== 'add'),
                modelValue: row.field_type,
                'onUpdate:modelValue': (value) => {
                    const fieldIndex = props.modelValue.findIndex((f) => getFieldKey(f) === row.key);
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
                        if (props.isNewComponent) {
                            emit('update:modelValue', props.modelValue.filter((f) => getFieldKey(f) !== row.key));
                            return;
                        }

                        if (row.id === undefined) {
                            emit('update:modelValue', props.modelValue.filter((f) => getFieldKey(f) !== row.key));
                            return;
                        }

                        updateFieldOperation(row.key, row.operation === 'remove' ? undefined : 'remove');
                    },
                },
                () => h(NIcon, () => h(row.operation === 'remove' ? DismissCircle20Regular : TrashBin)),
            ),
    },
];

function updateFieldOperation(rowKey: string, operation?: FieldRow['operation']) {
    const fieldIndex = props.modelValue.findIndex((f) => getFieldKey(f) === rowKey);
    if (fieldIndex === -1) {
        return;
    }

    const updated = [...props.modelValue];
    updated[fieldIndex] = { ...updated[fieldIndex], operation };
    emit('update:modelValue', updated);
}

const fields = computed<FieldRow[]>(() =>
    props.modelValue.map((f) => ({
        ...f,
        key: getFieldKey(f),
        operation: props.isNewComponent ? undefined : f.operation ?? (f.id === undefined ? 'add' : undefined),
    })),
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
    const rowIndex = fields.value.findIndex((item) => item.key === row.key);
    const rowClass = [row.operation ? `field-row-${row.operation}` : '', dragOverIndex.value === rowIndex ? 'drag-over-row' : '']
        .filter(Boolean)
        .join(' ');

    return {
        draggable: true,
        class: rowClass,
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
    background-color: #f4fff7 !important;
}

:deep(tr.field-row-remove > td) {
    background-color: #ffeeee !important;
}

:deep(tr.field-row-update > td) {
    background-color: #f0fbff !important;
}

:deep(tr.drag-over-row > td) {
    border-top: 2px solid #1890ff;
}

:deep(.drag-handle) {
    cursor: grab;
    user-select: none;
}
</style>
