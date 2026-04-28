<template>
    <div class="view-api-output">
        <span class="label">Output</span>
        <div class="output-type"><slot></slot></div>
    </div>
</template>

<style scoped>
.view-api-output {
    display: flex;
    width: 100%;
    flex-direction: column;
    padding: 6px;
    margin: 8px;
    box-sizing: border-box;
    border-radius: 4px;
    background-color: #efefef;

    .label {
        font-size: 0.75em;
    }

    .output-type {
        font-family: monospace;
    }
}
</style>
