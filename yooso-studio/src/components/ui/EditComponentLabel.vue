<template>
    <n-color-picker v-model:value="modelColor">
        <template #trigger="{ value, onClick }">
            <div class="edit-component-label" :style="{ color: 'white', backgroundColor: value ?? '' }">
                <input-span class="edit-component-input" v-model="modelValue" />
                <n-icon class="edit-component-color" @click="onClick">
                    <ColorWand />
                </n-icon>
            </div>
        </template>
    </n-color-picker>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { NColorPicker, NIcon } from 'naive-ui';
import { ColorWand } from '@vicons/ionicons5';
import InputSpan from './InputSpan.vue';

const props = defineProps<{
    value: string;
    color: string;
}>();

const emit = defineEmits<{
    (e: 'update:value', value: string): void;
    (e: 'update:color', value: string): void;
}>();

const modelValue = computed({
    get: () => props.value,
    set: (value: string) => emit('update:value', value),
});

const modelColor = computed({
    get: () => props.color,
    set: (value: string | null) => emit('update:color', value ?? ''),
});
</script>

<style lang="scss" scoped>
.edit-component-label {
    position: relative;
    display: flex;
    align-items: stretch;
    width: 100%;
    border-radius: 4px;
    color: black;

    .edit-component-input {
        flex: 1 1 auto;
        min-width: 0;
        width: 100%;
        padding: 8px 34px 8px 6px;
        border-radius: 4px;
        box-sizing: border-box;
    }

    .edit-component-color {
        position: absolute;
        right: 8px;
        top: 50%;
        transform: translateY(-50%);
        z-index: 1;
        cursor: pointer;
        flex: none;
        color: black;
    }
}
</style>