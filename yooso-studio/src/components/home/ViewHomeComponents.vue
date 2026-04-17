<template>
    <n-data-table :columns="columns" :data="data" />
</template>

<script setup lang="ts">
import { NButton, NDataTable, NPopover } from 'naive-ui';
import { h, ref } from 'vue';

const columns = ref([
    {
        title: 'Component Name',
        key: 'name',
        render(row: any) {
            return h(
                'span',
                {
                    style: {
                        display: 'inline-block',
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
    // {
    //     title: 'Components',
    //     key: 'components',
    //     render(row: any) {
    //         const tags = row.components.map((component: any) =>
    //             h(
    //                 'span',
    //                 {
    //                     style: {
    //                         display: 'inline-block',
    //                         marginRight: '6px',
    //                         padding: '4px 8px',
    //                         backgroundColor: component.color,
    //                         color: '#fff',
    //                         borderRadius: '4px',
    //                     },
    //                 },
    //                 component.name,
    //             ),
    //         );

    //         return tags;
    //     },
    // },
]);

const data = ref([
    {
        name: 'Superuser',
        color: 'red',
    },
    {
        name: 'EmailAuth',
        color: 'blue',
    },
    {
        name: 'PassAuth',
        color: 'blue',
    },
    {
        name: 'User',
        color: 'green',
    },
    {
        name: 'Channel',
        color: 'purple',
    },
    {
        name: 'TextChannel',
        color: 'gray',
    },
    {
        name: 'VoiceChannel',
        color: 'orange',
    },
    {
        name: 'Message',
        color: 'purple',
    },
    {
        name: 'TextMessage',
        color: 'gray',
    },
    {
        name: 'MessageAttachments',
        color: 'cyan',
    },
]);
</script>
