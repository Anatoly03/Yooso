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
        </div>
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
const sampleUrl = "https://yooso.demo";
</script>

<style lang="scss" scoped>
.app-browser-frame {
    display: flex;
    max-width: calc(100vw - 40px);
    flex-direction: column;
    width: 100%;
    height: 100%;
    margin: 20px;
    border: 1px solid gray;
    border-radius: 8px;
    overflow: hidden;

    .header {
        display: flex;
        align-items: center;
        padding: 4px 12px;
        background-color: #f0f0f0;
        border-bottom: 1px solid gray;

        .buttons {
            display: flex;
            gap: 8px;
            margin-right: 16px;

            .button {
                width: 12px;
                height: 12px;
                border-radius: 50%;
                background-color: gray;

                &.red {
                    background-color: #ff5f56;
                }
                &.yellow {
                    background-color: #ffbd2e;
                }
                &.green {
                    background-color: #27c93f;
                }
            }
        }

        .url {
            flex: 1;
            font-size: 14px;
            color: #555;
            padding: 2px 10px;
            background-color: #e0e0e0;
            border-radius: 4px;
        }
    }
}
</style>