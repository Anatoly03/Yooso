<template>
    <n-data-table :columns="columns" :data="data" />
</template>

<script setup lang="ts">
import { NButton, NDataTable, NPopover } from 'naive-ui';
import { h, ref } from 'vue';

const columns = ref([
    {
        title: 'ID',
        key: 'id',
        render(row: any) {
            return h(
                NPopover,
                { trigger: 'hover'},
                {
                    // copy ID to clipboard on click
                    trigger: () => h(NButton, {
                        style: {
                            display: 'inline-block',
                            marginLeft: '12px',
                        },
                        onClick: () => navigator.clipboard.writeText(row.id)
                    }, row.id.slice(0, 8)),
                    default: () => h('span', row.id),
                },
            );
        },
    },
    {
        title: 'Components',
        key: 'components',
        render(row: any) {
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
    {
        id: '9179dee9-ad7e-4d74-99d9-186d82a9c932',
        components: [
            { name: 'Superuser', color: 'red' },
            { name: 'EmailAuth', color: 'blue' },
            { name: 'PassAuth', color: 'blue' },
        ],
    },
    {
        id: '7ddae4b2-ccb4-4890-94d4-45339eaaa982',
        components: [
            { name: 'User', color: 'green' },
            { name: 'EmailAuth', color: 'blue' },
            { name: 'PassAuth', color: 'blue' },
        ],
    },
    {
        id: 'cd2db480-141f-47cb-9325-b3107220b0b6',
        components: [
            { name: 'Channel', color: 'purple' },
            { name: 'TextChannel', color: 'gray' },
        ],
    },
    {
        id: 'cd2db480-141f-47cb-9325-b3107220b0b6',
        components: [
            { name: 'Channel', color: 'purple' },
            { name: 'VoiceChannel', color: 'orange' },
        ],
    },
    {
        id: '73295ceb-9220-4b56-8657-08e6282ac815',
        components: [
            { name: 'Message', color: 'purple' },
            { name: 'TextMessage', color: 'gray' },
            { name: 'MessageAttachments', color: 'cyan' },
        ],
    },
]);
</script>
