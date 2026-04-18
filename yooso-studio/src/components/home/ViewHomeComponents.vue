<template>
    <n-data-table remote :loading="loadingRef" :columns="columns" :data="data" />
</template>

<script setup lang="ts">
import { NButton, NDataTable, NPopover } from 'naive-ui';
import { h, onMounted, ref } from 'vue';

const loadingRef = ref(true);

const columns = ref([
    {
        title: 'Component',
        key: 'name',
        render(row: any) {
            return h(
                'span',
                {
                    style: {
                        display: 'inline-block',
                        marginLeft: '12px',
                        marginRight: '6px',
                        padding: '4px 8px',
                        backgroundColor: row.color,
                        color: '#fff',
                        borderRadius: '4px',
                    },
                },
                row.name,
            );
        },
    },
]);

const data = ref([]);

onMounted(async () => {
    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/components/list');
        const result = await response.json();

        if (!result.success) throw new Error(result.message || 'Failed to fetch components');

        // Map color to HTML approved format
        data.value = result.components.map((component: any) => {
            let htmlColor = '#' + component.color.toString(16).padStart(6, '0');

            return {
                name: component.name,
                color: htmlColor,
            };
        });
    } catch (error) {
        console.error('Error fetching components:', error);
    }

    loadingRef.value = false;
});
</script>
