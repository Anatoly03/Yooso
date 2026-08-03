<template>
    <div style="padding: 8px">
        <n-button type="primary" @click="createEntity()"> Create </n-button>
        <ul class="debug-entity-display">
            <li v-for="e in entities">{{ e.id }} created at {{ e.created_at }}</li>
        </ul>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { NButton } from 'naive-ui';

let entities = ref([] as any[]);

async function createEntity() {
    const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities', {
        method: 'POST',
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
