<template>
    <div class="field-type-select">
        <n-select
            :disabled="props.disabled"
            :options="options"
            :render-label="renderLabel"
            :render-tag="renderSingleSelectTag"
            :default-value="props.modelValue"
            @update-value="emit('update:modelValue', $event)"
        />
    </div>
</template>

<script setup lang="ts">
import { CalendarNumberOutline, KeyOutline, MailOutline, Snow, Text, Toggle } from "@vicons/ionicons5"
import { BookNumber16Regular } from "@vicons/fluent"
import type { SelectRenderLabel, SelectRenderTag } from "naive-ui"
import { NSelect, NText, NIcon } from "naive-ui"
import { h, PropType, ref } from "vue"

const props = defineProps<{
    modelValue: string;
    disabled?: boolean;
}>();

const emit = defineEmits<{
    'update:modelValue': [value: string];
}>();

const options = [
    // {
    //     value: "uuid",
    //     label: "UUID",
    //     icon: Snow,
    //     description: "Universally Unique Identifier",
    // },
    {
        value: "text",
        label: "Text",
        icon: Text,
        description: "Plain text value",
    },
    {
        value: "integer",
        label: "Number",
        icon: BookNumber16Regular,
        description: "Integer value",
    },
    // {
    //     value: "email",
    //     label: "Email",
    //     icon: MailOutline,
    //     description: "Email address",
    // },
    {
        value: "boolean",
        label: "Boolean",
        icon: Toggle,
        description: "Binary value, either truthy or false.",
    },
    // {
    //     value: "password",
    //     label: "Password",
    //     icon: KeyOutline,
    //     description: "Protected text value (hashed)",
    // },
    // {
    //     value: "datetime",
    //     label: "Date Time",
    //     icon: CalendarNumberOutline,
    //     description: "Date and time value",
    // },
]

const modelValue = ref(
    options.find((o) => o.value === props.modelValue)?.value ?? "text",
)

const renderSingleSelectTag: SelectRenderTag = ({ option }) => {
    return h(
        "div",
        {
            style: {
                display: "flex",
                gap: "8px",
                alignItems: "center",
            },
        },
        [
            h(NIcon, { size: 18 }, { default: () => (option.icon ? h(option.icon) : null) }),
            option.label as string,
        ],
    )
}

const renderLabel: SelectRenderLabel = (option) => {
    return h(
        "div",
        {
            style: {
                display: "flex",
                alignItems: "center",
            },
        },
        [
            h(NIcon, { size: 18 }, { default: () => (option.icon ? h(option.icon) : null) }),
            h(
                "div",
                {
                    style: {
                        marginLeft: "12px",
                        padding: "4px 0",
                    },
                },
                [
                    h("div", null, [option.label as string]),
                    // h(
                    //     NText,
                    //     { depth: 3, tag: "div" },
                    //     {
                    //         default: () => option.description as string,
                    //     },
                    // ),
                ],
            ),
        ],
    )
}
</script>
<!-- 
<style lang="scss" scoped>
.field-type-select {
    // max-width: 100px;

    * {
        border: 0;
    }
}
</style> -->
