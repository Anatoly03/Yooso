<template>
    <div class="view-entities">
        <n-data-table remote :loading="loadingRef" :bordered="false" :columns="columns" :data="data" />
        <div class="view-entities-footer">
            <n-button type="primary" @click="createEntity"> Create Entity </n-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NPopover } from 'naive-ui';
import { h, onMounted, ref } from 'vue';

const loadingRef = ref(true);

const columns = ref([
    {
        title: () => h('span', { style: { marginLeft: '12px' } }, 'ID'),
        key: 'id',
        width: 180,
        render(row: any) {
            return h(
                NPopover,
                { trigger: 'hover' },
                {
                    // copy ID to clipboard on click
                    trigger: () =>
                        h(
                            NButton,
                            {
                                style: {
                                    'font-family': 'monospace',
                                    width: '100%',
                                    display: 'inline-block',
                                    marginLeft: '12px',
                                },
                                onClick: () => navigator.clipboard.writeText(row.id),
                            },
                            row.id.slice(-12),
                        ),
                    default: () => h('span', row.id),
                },
            );
        },
    },
    {
        title: 'Components',
        key: 'components',
        render(row: any) {
            if (!row.components || row.components.length === 0) {
                return h('span', { style: { fontStyle: 'italic', color: '#888' } }, 'No components');
            }

            const tags = row.components.map((component: any) =>
                h(
                    'span',
                    {
                        style: {
                            display: 'inline-block',
                            marginRight: '6px',
                            padding: '4px 8px',
                            backgroundColor: component.color,
                            color: '#fff',
                            borderRadius: '4px',
                        },
                    },
                    component.name,
                ),
            );

            return tags;
        },
    },
]);

const data = ref([
    // {
    //     id: '9179dee9-ad7e-4d74-99d9-186d82a9c932',
    //     components: [
    //         { name: 'Superuser', color: 'red' },
    //         { name: 'EmailAuth', color: 'blue' },
    //         { name: 'PassAuth', color: 'blue' },
    //     ],
    // },
    // {
    //     id: '7ddae4b2-ccb4-4890-94d4-45339eaaa982',
    //     components: [
    //         { name: 'User', color: 'green' },
    //         { name: 'EmailAuth', color: 'blue' },
    //         { name: 'PassAuth', color: 'blue' },
    //     ],
    // },
    // {
    //     id: 'cd2db480-141f-47cb-9325-b3107220b0b6',
    //     components: [
    //         { name: 'Channel', color: 'purple' },
    //         { name: 'TextChannel', color: 'gray' },
    //     ],
    // },
    // {
    //     id: 'cd2db480-141f-47cb-9325-b3107220b0b6',
    //     components: [
    //         { name: 'Channel', color: 'purple' },
    //         { name: 'VoiceChannel', color: 'orange' },
    //     ],
    // },
    // {
    //     id: '73295ceb-9220-4b56-8657-08e6282ac815',
    //     components: [
    //         { name: 'Message', color: 'purple' },
    //         { name: 'TextMessage', color: 'gray' },
    //         { name: 'MessageAttachments', color: 'cyan' },
    //     ],
    // },
]);

async function refreshEntityList() {
    loadingRef.value = true;

    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities/list');
        const result = await response.json();

        if (!result.success) throw new Error(result.message || 'Failed to fetch entities');

        data.value = result.entities;
    } catch (error) {
        console.error('Error fetching entities:', error);
    }

    loadingRef.value = false;
}

async function createEntity() {
    loadingRef.value = true;

    try {
        const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/entities', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Failed to create/update entity');
        }

        const result = await response.json();
        // console.log('Entity created/updated:', result);

        refreshEntityList();
    } catch (error) {
        console.error('Error creating/updating entity:', error);
    }
}

onMounted(() => {
    refreshEntityList();
})
</script>

<style lang="scss" scoped>
.view-entities {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;

    :deep(.n-data-table) {
        flex: 1 1 0;
        min-height: 0;
        overflow: auto;
    }
}

.view-entities-footer {
    flex: 0 0 auto;
    display: flex;
    padding: 10px 24px;
    gap: 8px;

    border-top: 1px solid #eee;
    background-color: #f5f5f5;
}
</style>
