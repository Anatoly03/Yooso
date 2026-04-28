<template>
    <div class="view-api-input">
        <span class="label">Input</span>
        <div class="input-type"><slot></slot></div>
    </div>
</template>

<style scoped>
.view-api-input {
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

    .input-type {
        font-family: monospace;
    }
}
</style>
