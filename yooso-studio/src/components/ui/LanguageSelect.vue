<template>
    <div class="language-select-menu">
        <n-dropdown :options="dropdownOptions" trigger="click" @select="handleSelect">
            <button type="button" class="n-menu-item-content language-select" :class="{'collapsed': props.collapsed}" :aria-label="selectorAriaLabel">
                <n-icon class="language-select-icon">
                    <Language />
                </n-icon>
                <span class="language-active-span">{{ currentLanguageLabel }}</span>
            </button>
        </n-dropdown>
    </div>
</template>

<script lang="ts" setup>
import 'naive-ui/es/menu/src/styles/index.cssr';

import { computed } from 'vue';
import { Language } from '@vicons/ionicons5';
import { NDropdown, NIcon, type DropdownOption } from 'naive-ui';
import { useI18n } from 'vue-i18n';

const i18n = useI18n();

const props = defineProps<{
    collapsed: boolean;
}>();

type LanguageOption = {
    value: string;
    label: string;
    disabled?: boolean;
};

const languageOptions: LanguageOption[] = [
    { value: 'en', label: 'English (United Kingdom)' },
    { value: 'jp', label: '日本語（日本）' },
    { value: 'zh', label: '中文（中国）', disabled: true },
];

const dropdownOptions = computed<DropdownOption[]>(() =>
    languageOptions.map((option) => ({
        key: option.value,
        label: option.label,
        disabled: option.disabled,
    })),
);

const currentLanguageLabel = computed(() => {
    const currentLocale = i18n.locale.value;
    const currentOption = languageOptions.find((option) => option.value === currentLocale);
    return currentOption?.label ?? currentLocale;
});

const selectorAriaLabel = computed(() => `Change language, current: ${currentLanguageLabel.value}`);

function handleSelect(key: string | number): void {
    i18n.locale.value = String(key);
}
</script>

<style lang="scss" scoped>
.language-select-menu {
    display: flex;
    align-items: center;
    padding: 8px;
}

.language-select {
    box-sizing: border-box;
    max-width: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    border: 0;
    height: var(--n-item-height);
    background-color: transparent;

    padding: 8px;
    padding-left: 26px;
    padding-right: 18px;

    cursor: pointer;

    .language-select-icon {
        width: 22px;
        height: 22px;
        font-size: 20px;
        margin-right: 8px;

        transition: all 0.3s;
    }

    .language-active-span {
        text-wrap: nowrap;
        transition: all 0.3s;
    }

    &:hover {
        background-color: var(--n-item-color-hover, rgb(243, 243, 245));
    }

    &.collapsed {
        justify-content: center;
        padding-left: 21px;

        .language-select-icon {
            width: 22px;
            height: 22px;
            font-size: 22px;
            margin-right: 0;
        }

        .language-active-span {
            width: 0;
            max-width: 0;
            flex-basis: 0;
            overflow: hidden;
            white-space: nowrap;
            opacity: 0;
        }
    }

    transition: all 0.3s;
}
</style>
