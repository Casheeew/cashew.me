<script setup lang="ts">
import { useColorMode } from '@vueuse/core'

defineProps<{ projects: Record<string, any> }>()

const mode = useColorMode()

function slug(name: string) {
  return name.toLowerCase().replace(/[\s\\/]+/g, '-')
}

function getThemeImage(item: any) {
  // If item has both light and dark images
  if (item.imageLight && item.imageDark) {
    return mode.value === 'dark' ? item.imageDark : item.imageLight
  }
  // Fallback to single image
  return item.image
}

function getProjectItems(category: any) {
  // Support both old structure (array) and new structure (object with items)
  return Array.isArray(category) ? category : category.items || []
}

function getSubtitle(category: any) {
  // Get subtitle from the category object if it exists
  return !Array.isArray(category) ? category.subtitle : null
}
</script>

<template>
  <div class="max-w-250 mx-auto">
    <p text-center mt--6 mb5 op50 text-lg italic>
      What I've cooked on my own time.
    </p>
    <div class="prose pb5 mx-auto mt10 text-center">
      <div flex="~ gap-2 justify-center">
        <a
          href="https://github.com/Casheeew"
          target="_blank"
          class="group btn-blue inline-block"
        >
          <div
            i-ph-github-logo-duotone
            group-hover="i-ph-github-logo-fill text-blue"
          />
          My GitHub
        </a>
      </div>
      <hr>
    </div>
    <div
      v-for="key, cidx in Object.keys(projects)" :key="key" slide-enter
      :style="{ '--enter-stage': cidx + 1 }"
    >
      <div
        :id="slug(key)"
        select-none relative h18 mt10 mb8 pointer-events-none slide-enter
        :style="{
          '--enter-stage': cidx - 2,
          '--enter-step': '60ms',
        }"
      >
        <span text-5em color-transparent absolute left--1rem top-0rem font-bold leading-1em text-stroke-2 text-stroke-hex-888 op50 dark:op35>{{ key }}</span>
      </div>
      <!-- Subtitle -->
      <div v-if="getSubtitle(projects[key])" class="mt-2 mb8 op50 text-base">
        {{ getSubtitle(projects[key]) }}
      </div>
      <div
        class="project-grid py-2 max-w-500 mx-auto"
        grid="~ cols-1 md:cols-2 gap-5 lg:cols-3"
      >
        <a
          v-for="item, idx in getProjectItems(projects[key])"
          :key="idx"
          class="item relative flex flex-col"
          :class="{ 'has-image': item.image || item.imageLight || item.imageDark }"
          :href="item.link"
          target="_blank"
          :title="item.name"
        >
          <div v-if="item.image || item.imageLight || item.imageDark" class="project-image-wrapper">
            <img :src="getThemeImage(item)" :alt="item.name" class="project-image">
          </div>
          <div class="project-content" :class="{ 'with-image': item.image || item.imageLight || item.imageDark }">
            <div class="flex items-center">
              <div v-if="item.icon" class="pr-3">
                <Slidev v-if="item.icon === 'slidev'" class="text-3xl opacity-80" />
                <VueUse v-else-if="item.icon === 'vueuse'" class="text-3xl opacity-80" />
                <VueReactivity v-else-if="item.icon === 'vue-reactivity'" class="text-3xl opacity-80" />
                <VueDemi v-else-if="item.icon === 'vue-demi'" class="text-3xl opacity-80" />
                <Unocss v-else-if="item.icon === 'unocss'" class="text-3xl opacity-80" />
                <Vitest v-else-if="item.icon === 'vitest'" class="text-3xl opacity-80" />
                <Elk v-else-if="item.icon === 'elk'" class="text-3xl opacity-80" />
                <AnthonyFu v-else-if="item.icon === 'af'" class="text-3xl opacity-80" />
                <div v-else class="text-2xl opacity-80" :class="item.icon || 'i-carbon-unknown'" />
              </div>
              <div class="flex-auto">
                <div class="text-normal">{{ item.name }}</div>
              </div>
            </div>
            <div class="desc text-sm opacity-80 font-normal mt-1" v-html="item.desc" />
          </div>
        </a>
      </div>
    </div>
    <div class="prose pb5 mx-auto mt10 text-center">
      <hr>
    </div>
  </div>
  <div>
    <div class="table-of-contents">
      <div class="table-of-contents-anchor">
        <div class="i-ri-menu-2-fill" />
      </div>
      <ul>
        <li v-for="key of Object.keys(projects)" :key="key">
          <a :href="`#${slug(key)}`">{{ key }}</a>
        </li>
      </ul>
    </div>

    <!-- Call to action -->
    <div class="text-center mt-16 pt-8">
      <p class="text-base text-gray-600 dark:text-gray-400">
        I also write stuff in my free time.
        <br>
        Interested? Check out my <a href="/posts" class="text-blue-600 dark:text-blue-400 hover:underline font-medium">blog</a>.
      </p>
    </div>
  </div>
</template>

<style scoped>
.project-grid a.item {
  background: transparent;
  font-size: 1.1rem;
  width: 100%;
  max-width: 350px;
  border-radius: 6px;
  overflow: hidden;
  transition: all 0.3s ease;
  opacity: 1 !important;
}

.project-grid a.item:not(.has-image) {
  padding: 0.5rem 0.875rem 0.875rem;
}

.project-grid a.item:hover {
  background: #88888822;
}

.project-grid a.item.has-image:hover {
  transform: translateY(-2px);
}

.project-image-wrapper {
  position: relative;
  width: 100%;
  height: 160px;
  overflow: hidden;
}

.project-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.item:hover .project-image {
  transform: scale(1.05);
}

.project-content {
  padding: 0.75rem 0.875rem 0.875rem;
}

.project-content.with-image {
  padding-top: 0.875rem;
}
</style>
