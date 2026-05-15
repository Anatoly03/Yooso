<!-- Browser Frame for Demo Mode -->
<!-- This component is for the `VITE_DEMO_MODE` flag and will wrap the
     app window in simple browser UI. -->

<template>
    <div class="app-browser-frame">
        <div class="header">
            <div class="buttons">
                <span class="button red"></span>
                <span class="button yellow"></span>
                <span class="button green"></span>
            </div>
            <div class="url">
                <span>{{ sampleUrl }}{{ $route.fullPath }}</span>
            </div>
            <div class="buttons">
                <n-icon><ChevronBack /></n-icon>
                <n-icon><ChevronForward /></n-icon>
                <n-icon><Refresh /></n-icon>
            </div>
        </div>
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { NIcon } from 'naive-ui';
import { ChevronBack, ChevronForward, Refresh } from '@vicons/ionicons5';

const sampleUrl = 'https://yooso.demo';
</script>

<style lang="scss" scoped>
.app-browser-frame {
    display: flex;
    max-width: calc(100vw - 20px);
    flex-direction: column;
    width: 100%;
    height: 100%;
    margin: 10px;
    border: 1px solid gray;
    border-radius: 8px;
    overflow: hidden;

    .header {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 6px 12px;
        background-color: #f0f0f0;
        border-bottom: 1px solid #d0d0d0;

        .buttons {
            display: flex;
            gap: 8px;
            flex: 0 0 auto;

            .button {
                width: 12px;
                height: 12px;
                border-radius: 50%;
                background-color: gray;

                &.red { background-color: #ff5f56; }
                &.yellow { background-color: #ffbd2e; }
                &.green { background-color: #27c93f; }
            }
        }

        .url {
            flex: 1 1 auto;
            min-width: 0; /* allow ellipsis on overflow */
            font-size: 14px;
            color: #555;
            padding: 4px 12px;
            background-color: #eaeaea;
            border-radius: 6px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    }
}
</style>
