<template>
    <div class="view-api-route">
        <span class="http-method">{{ method }}</span>
        <span class="http-route"><slot>/</slot></span>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
    method?: 'get' | 'post' | 'patch' | 'delete';
}>();

const method = computed(() => props.method?.toUpperCase() ?? 'GET');
</script>

<style scoped>
.view-api-route {
    display: flex;
    width: 100%;
    font-family: monospace;
    flex-direction: row;
    padding: 6px;
    margin: 8px;
    box-sizing: border-box;
    border-radius: 4px;
    background-color: #efefef;
    align-items: center;

    .http-method {
        font-weight: bold;
        margin-right: 8px;
        /* background-color: #dfdfdf; */
        width: 64px;
        padding: 0 5px;
        border-radius: 2px;
        text-align: center;
    }
}
</style>
