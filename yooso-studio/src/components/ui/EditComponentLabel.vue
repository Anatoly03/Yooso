<template>
    <n-color-picker v-model:value="componentColor">
        <template #trigger="{ value, onClick }">
            <div class="edit-component-label" :style="{ color: 'white', backgroundColor: value ?? '' }">
                <input-span class="edit-component-input" v-model="componentName" />
                <n-icon class="edit-component-color" @click="onClick">
                    <ColorWand />
                </n-icon>
            </div>
        </template>
    </n-color-picker>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { NColorPicker, NIcon } from 'naive-ui';
import { ColorWand } from '@vicons/ionicons5';
import InputSpan from './InputSpan.vue';

const props = defineProps<{
    name: string;
    color: string;
}>();

const componentColor = ref(props.color);
const componentName = ref(props.name);
</script>

<style lang="scss" scoped>
.edit-component-label {
    position: relative;
    display: flex;
    align-items: stretch;
    width: 100%;
    border-radius: 4px;

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
    }
}
</style>