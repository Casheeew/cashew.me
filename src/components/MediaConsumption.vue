<script setup lang="ts">
import type { MediaType } from '../../data/media'
import { media } from '../../data/media'

const route = useRoute()

const type = computed<MediaType>(() => route.query.type as MediaType || 'book')

const isDark = ref(useDark())
</script>

<template>
  <div class="relative min-h-screen width-full">
    <div
      class="absolute inset-0 bg-inherit blur-[4px] bg-blend-overlay -z-1"
      :style="{
        backgroundColor: isDark
          ? 'rgba(0, 0, 0, 0.5)'
          : 'rgba(255, 255, 255, 0.35)',
      }"
    />
    <div
      font-mono
    >
      <div flex="~ gap-2">
        <RouterLink
          v-for="t of Object.keys(media)"
          :key="t"
          :to="{ query: { type: t } }"
          px-2
          class="border-none!"
          :class="type === t ? 'bg-black dark:bg-white text-white! dark:text-black!' : ''"
        >
          {{ t }}
        </RouterLink>
      </div>

      <template v-for="t of Object.keys(media)" :key="t">
        <table v-show="type === t" lang="ja" font-400>
          <tbody>
            <template v-for="m of media[t as MediaType]" :key="m.name">
              <tr v-if="!m.state" v-bind="m.lang ? { lang: m.lang } : {}">
                <td>{{ m.name }}</td>
                <td text-right>
                  {{ m.creator }}
                </td>
                <td v-if="m.note" lt-sm:hidden>
                  {{ m.note }}
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </template>
    </div>
  </div>
</template>

<style scoped>
.background-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: inherit;
  filter: blur(4px);
  background-blend-mode: overlay;
  background-color: rgba(255, 255, 255, 0.25);
  z-index: -1; /* Keep it behind the content */
}

.content {
  position: relative;
  z-index: 1;
  padding: 20px;
}
</style>
