<!-- Not to be confused with [ViewHome] which is the wrapper for the
 main home page. -->

<template>
    <div class="view-home-general">
        <n-page-header class="view-home-general" title="Yooso Studio" subtitle="要素スタジオ">
            <!-- <n-grid :cols="5">
                <n-gi>
                    <n-statistic label="Episodes" value="125" />
                </n-gi>
                <n-gi>
                    <n-statistic label="Guests" value="22" />
                </n-gi>
                <n-gi>
                    <n-statistic label="Apologies" value="36" />
                </n-gi>
                <n-gi>
                    <n-statistic label="Topics" value="83" />
                </n-gi>
                <n-gi>
                    <n-statistic label="Reference Links" value="2,346" />
                </n-gi>
            </n-grid> -->
            <!-- <template #avatar>
                <n-avatar />
            </template> -->
        </n-page-header>
        <p>{{ $t('hello') }}</p>
    </div>
</template>

<script lang="ts" setup>
import { NPageHeader } from 'naive-ui';
</script>

<style lang="scss" scoped>
.view-home-general {
    padding: 12px;
}
</style>
