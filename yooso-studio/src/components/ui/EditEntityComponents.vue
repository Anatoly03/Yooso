<template>
    <n-dynamic-tags v-model:value="tags" :render-tag="renderTag">
        <template #input="{ deactivate, submit }">
            <n-auto-complete
                v-model:value="proposeNewComponent"
                :options="autoCompleteOptions"
                :render-label="renderOptionLabel"
                size="small"
                :clear-after-select="true"
                @select="(value) => onSelectComponent(value, submit, deactivate)"
                @keydown.enter.prevent="submitTypedComponent(submit, deactivate)"
                @blur="deactivate"
            >
                <template #default="{ handleInput, handleBlur, handleFocus, value: slotValue }">
                    <n-input type="text" :value="slotValue" placeholder="Add Component" @input="handleInput" @focus="handleFocus" @blur="handleBlur" />
                </template>
            </n-auto-complete>
        </template>
        <template #trigger="{ activate, disabled }">
            <n-button :disabled="disabled" @click="activate()">
                <template #icon>
                    <n-icon>
                        <Add />
                    </n-icon>
                </template>
            </n-button>
        </template>
    </n-dynamic-tags>
</template>

<script setup lang="ts">
import { NAutoComplete, NButton, NDynamicTags, NIcon, NInput, NTag } from 'naive-ui';
import { Add } from '@vicons/ionicons5';
import { computed, h, onMounted, ref, watch } from 'vue';

const props = defineProps<{
    entityId: string;
    components: { id: string; name: string; color: number; is_system: boolean; created_at: number }[];
}>();

export interface ComponentOption {
    id: string;
    label: string;
    value: string;
    color: string;
}

interface AutoCompleteOption {
    label: string;
    value: string;
    color: string;
}

const proposeNewComponentRaw = ref<string | null>('');
const tags = ref<string[]>([]);
const availableComponents = ref<ComponentOption[]>([]);

const proposeNewComponent = computed<string>({
    get: () => proposeNewComponentRaw.value ?? '',
    set: (value) => {
        proposeNewComponentRaw.value = value ?? '';
    },
});

const emit = defineEmits({
    'add-component': (entityId: string, componentId: string) => true,
    'remove-component': (entityId: string, componentId: string) => true,
});

const componentColorByName = computed(() => {
    const map = new Map<string, string>();
    for (const component of availableComponents.value) {
        map.set(component.value.toLowerCase(), component.color);
    }
    return map;
});

const autoCompleteOptions = computed(() => {
    const selectedNames = new Set(tags.value.map((tag) => tag.toLowerCase()));
    const query = proposeNewComponent.value.trim().toLowerCase();

    return availableComponents.value
        .filter((component) => !selectedNames.has(component.value.toLowerCase()))
        .filter((component) => !query || component.value.toLowerCase().includes(query))
        .map((component) => ({
            label: component.label,
            value: component.value,
            color: component.color,
        }));
});

function renderOptionLabel(option: AutoCompleteOption) {
    return h(
        'span',
        {
            style: {
                display: 'inline-block',
                padding: '2px 8px',
                borderRadius: '4px',
                backgroundColor: option.color,
                color: 'black',
            },
        },
        option.label,
    );
}

function submitComponentSelection(value: string | null | undefined, submit: (value: string) => void, deactivate: () => void) {
    const normalizedValue = (value ?? '').trim().toLowerCase();
    if (!normalizedValue) {
        return;
    }

    const component = availableComponents.value.find((item) => item.value.toLowerCase() === normalizedValue);

    if (!component || tags.value.some((tag) => tag.toLowerCase() === normalizedValue)) {
        return;
    }

    emit('add-component', props.entityId, component.id);

    // submit(component.value);
    proposeNewComponent.value = '';
    deactivate();
}

function onSelectComponent(value: string, submit: (value: string) => void, deactivate: () => void) {
    submitComponentSelection(value, submit, deactivate);
}

function submitTypedComponent(submit: (value: string) => void, deactivate: () => void) {
    submitComponentSelection(proposeNewComponent.value, submit, deactivate);
}

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

onMounted(() => {
    fetchComponentOptions();
});

watch(
    () => props.components,
    (components) => {
        tags.value = (components ?? []).map((component) => component.name);
    },
    { immediate: true, deep: true },
);

function renderTag(tag: string, index: number) {
    const color = componentColorByName.value.get(tag.toLowerCase()) || '#C1D1D1';

    return h(
        NTag,
        {
            type: 'default',
            disabled: index > 3,
            closable: true,
            onClose: () => {
                // TODO: invoke call to server to remove component from entity
            },
            style: {
                padding: '17px 8px',
                backgroundColor: color,
                color: 'black',
                borderRadius: '4px',
            },
        },
        {
            default: () => tag,
        },
    );
}
</script>
