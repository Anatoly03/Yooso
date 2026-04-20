<template>
    <span ref="spanRef" :contenteditable="!props.disabled" class="input-span" @input="handleInput" @paste="handlePaste" :class="{disabled: props.disabled}"></span>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue';

const props = defineProps<{
    modelValue: string;
    disabled?: boolean;
}>();

const emit = defineEmits<{
    'update:modelValue': [value: string];
}>();

const spanRef = ref<HTMLSpanElement | null>(null);

function syncDomText(value: string) {
    if (!spanRef.value) {
        return;
    }

    if (spanRef.value.textContent !== value) {
        spanRef.value.textContent = value;
    }
}

// Sync prop changes to DOM
watch(
    () => props.modelValue,
    (newVal) => syncDomText(newVal),
    { flush: 'post' },
);

onMounted(() => syncDomText(props.modelValue));

function handleInput(e: Event) {
    const span = e.target as HTMLSpanElement;
    // Get plain text only, automatically strips any HTML
    const text = span.textContent || '';
    emit('update:modelValue', text);
}

function handlePaste(e: ClipboardEvent) {
    e.preventDefault();
    // Get plain text from clipboard, ignore HTML
    const text = e.clipboardData?.getData('text/plain') || '';
    document.execCommand('insertText', false, text);
}
</script>

<style lang="scss" scoped>
.input-span {
    display: inline-block;
    vertical-align: baseline;
    padding: 5px 4px;
    min-width: 144px;
    width: 100%;
    border: 0;
    border-radius: 2px;
    color: black;

    font: inherit;
    line-height: inherit;
    outline: none;
    background: transparent;
    white-space: pre;

    cursor: text;
    transition: background-color 0.2s;

    &:empty::before {
        content: '\00a0';
    }

    &:hover {
        background-color: rgba(0, 0, 0, 0.05);
    }

    &:focus {
        background-color: rgba(0, 0, 0, 0.1);
    }

    &.disabled {
        color: rgba(0, 0, 0, 0.25);
        background-color: rgba(0, 0, 0, 0.05);
        cursor: not-allowed;
    }
}
</style>
