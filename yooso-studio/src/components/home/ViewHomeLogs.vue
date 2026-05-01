<template>
    <div class="view-logs">
        <div class="padding"></div>
        <div class="heatmap-anchor">
            <n-heatmap
                :data="heatmapData"
                size="large"
                :x-gap="4"
                :y-gap="4"
                :show-week-labels="false"
                :show-month-labels="false"
            />
        </div>
        <ul class="view-log-entries">
            <li v-for="log in logs" :key="log.id" class="request-log" :class="{'request-log-error': log.status.toString().charAt(0) === '5'}">
                <span class="request-method" :class="'method-' + log.method.toLowerCase()">{{ log.method }}</span>
                <a :href="log.href" class="request-uri" target="_blank" rel="noopener noreferrer">{{ log.uri }}</a>
                <span
                    class="response-status"
                    :class="['status-' + log.status, 'status-' + log.status.toString().charAt(0) + 'xx']"
                >
                    {{ log.status }}
                </span>
            </li>
        </ul>
    </div>
</template>

<script setup lang="ts">
import type { HeatmapDataItem } from 'naive-ui'
import { NHeatmap } from 'naive-ui'
import { onMounted, ref } from 'vue'

const heatmapData = ref<HeatmapDataItem[]>([]);
const logs = ref<any[]>([]);

function toUTCDayStartMs(epochMs: number): number {
    const date = new Date(epochMs);
    return Date.UTC(date.getFullYear(), date.getMonth(), date.getDate());
}

onMounted(async () => {
    const dayMs = 24 * 60 * 60 * 1000;
    const totalDays = 365 * 3;
    const start = toUTCDayStartMs(Date.now() - totalDays * dayMs);
    const heatmapIndexByDay = new Map<number, number>();

    // Generate one value per day over ~3 years so the heatmap is long.
    for (let i = 0; i <= totalDays; i++) {
        const timestamp = start + i * dayMs;

        const entry = { timestamp, value: 0 };
        heatmapData.value.push(entry);
        heatmapIndexByDay.set(timestamp, i);
    }

    const response = await fetch(import.meta.env.VITE_API_SERVER + '/api/logs?limit=1000');
    const json_logs = await response.json();
    logs.value = json_logs.map((log: any) => ({
        ...log,
        href: import.meta.env.VITE_API_SERVER + log.request_uri,
    }));

    for (const log of json_logs) {
        const createdMs = log.created < 1_000_000_000_000 ? log.created * 1000 : log.created;
        const dayTimestamp = toUTCDayStartMs(createdMs);
        const index = heatmapIndexByDay.get(dayTimestamp);

        if (index === undefined) continue;
        const current = heatmapData.value[index];
        current.value = (current.value ?? 0) + 1;
    }
})
</script>

<style lang="scss" scoped>
.padding {
    width: 100%;
    min-height: 50px;
    flex: 0 0 60px;
}

.view-logs {
    width: 100%;
    max-height: 100vh;
    min-height: 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    justify-content: center;
}

.heatmap-anchor {
    display: flex;
    position: relative;
    right: 50px;
    justify-content: flex-end;
}

.view-log-entries {
    display: flex;
    flex-direction: column;
    flex: 1 1 auto;
    min-height: 0;
    border: 1px solid gray;
    overflow-y: auto;
    padding: 0;

    margin: 30px;
    margin-bottom: -1px;
    border-top-left-radius: 10px;
    border-top-right-radius: 10px;
    border-bottom: none;

    list-style: none;

    li:nth-child(even) {
        background: #f5f5f5;
    }

    li.request-log-error {
        background: #ffe0e0;
    }

    li {
        display: flex;
        padding: 10px;
        gap: 20px;

        .request-method {
            width: 70px;
            text-align: center;
            padding: 0 5px;
            border-radius: 5px;
            background-color: #e0e0e0;
            font-weight: bold;

            &.method-get {
                background-color: #bbdefb;
            }

            &.method-post {
                background-color: #c8e6c9;
            }

            &.method-put {
                background-color: #fff9c4;
            }

            &.method-delete {
                background-color: #ffcdd2;
            }

            &.method-patch {
                background-color: #d1c4e9;
            }
        }

        .request-uri {
            flex: 1;
            text-decoration: none;
            color: inherit;

            &:hover {
                text-decoration: underline;
            }
        }
    }
}

.response-status {
    padding: 0 5px;
    border-radius: 5px;
    background-color: #e0e0e0;

    &.status-2xx {
        background-color: #c8e6c9;
    }

    &.status-4xx {
        background-color: #ffcdd2;
    }

    &.status-5xx {
        background-color: #ff8a80;
    }
}
</style>
