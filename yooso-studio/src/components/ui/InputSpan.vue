<template>
    <span ref="spanRef" :contenteditable="!props.disabled" class="input-span" @input="handleInput" @paste="handlePaste" :class="{ disabled: props.disabled }"></span>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue';

const props = defineProps<{
    modelValue: string;
    disabled?: boolean;
    pattern?: RegExp | string;
}>();

const emit = defineEmits<{
    'update:modelValue': [value: string];
}>();

const spanRef = ref<HTMLSpanElement | null>(null);

function getCaretOffset(container: HTMLSpanElement): number | null {
    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0) {
        return null;
    }

    const range = selection.getRangeAt(0);
    if (!container.contains(range.startContainer)) {
        return null;
    }

    const preCaretRange = range.cloneRange();
    preCaretRange.selectNodeContents(container);
    preCaretRange.setEnd(range.startContainer, range.startOffset);
    return preCaretRange.toString().length;
}

function setCaretOffset(container: HTMLSpanElement, offset: number) {
    const targetOffset = Math.max(0, Math.min(offset, container.textContent?.length ?? 0));
    const selection = window.getSelection();
    if (!selection) {
        return;
    }

    const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT);
    let node = walker.nextNode();
    let remaining = targetOffset;

    while (node) {
        const textNode = node as Text;
        const len = textNode.textContent?.length ?? 0;
        if (remaining <= len) {
            const range = document.createRange();
            range.setStart(textNode, remaining);
            range.collapse(true);
            selection.removeAllRanges();
            selection.addRange(range);
            return;
        }

        remaining -= len;
        node = walker.nextNode();
    }

    const range = document.createRange();
    range.selectNodeContents(container);
    range.collapse(false);
    selection.removeAllRanges();
    selection.addRange(range);
}

function syncDomText(value: string, caretOffset?: number) {
    if (!spanRef.value) {
        return;
    }

    if (spanRef.value.textContent !== value) {
        spanRef.value.textContent = value;
    }

    if (caretOffset !== undefined) {
        setCaretOffset(spanRef.value, caretOffset);
    }
}

// Sync prop changes to DOM
watch(
    () => props.modelValue,
    (newVal) => syncDomText(newVal),
    { flush: 'post' },
);

onMounted(() => syncDomText(props.modelValue));

function buildPattern(pattern: RegExp | string): RegExp {
    if (pattern instanceof RegExp) {
        return pattern;
    }

    return new RegExp(pattern);
}

function isAllowedByPattern(text: string): boolean {
    if (!props.pattern) {
        return true;
    }

    const regex = buildPattern(props.pattern);
    return regex.test(text);
}

function handleInput(e: Event) {
    const span = e.target as HTMLSpanElement;
    // Get plain text only, automatically strips any HTML
    const text = span.textContent || '';

    if (!isAllowedByPattern(text)) {
        const caretAfterInvalidInput = getCaretOffset(span) ?? props.modelValue.length;
        const invalidDelta = Math.max(0, text.length - props.modelValue.length);
        const restoreOffset = Math.max(0, caretAfterInvalidInput - invalidDelta);
        syncDomText(props.modelValue, restoreOffset);
        return;
    }

    emit('update:modelValue', text);
}

function handlePaste(e: ClipboardEvent) {
    e.preventDefault();
    // Get plain text from clipboard, ignore HTML
    const text = e.clipboardData?.getData('text/plain') || '';

    if (!spanRef.value) {
        return;
    }

    const currentText = spanRef.value.textContent || '';
    const nextText = `${currentText}${text}`;
    if (!isAllowedByPattern(nextText)) {
        const caretOffset = getCaretOffset(spanRef.value) ?? props.modelValue.length;
        syncDomText(props.modelValue, caretOffset);
        return;
    }

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
