<!-- components/ListExperience.vue -->
<script setup lang="ts">
import { computed } from 'vue'

interface ExperienceItem {
  duration: string
  year?: string // Optional – will be computed if not provided
  type: 'education' | 'work'
  title: string
  org: string
  orgLink?: string
  location?: string
  desc?: string
  highlights?: string[]
  research?: {
    duration?: string
    lab: string
    labLink?: string
    advisor: string
    advisorLink?: string
    project: string
    details: string[]
  }
  impact?: string
  tech?: string[]
}

const props = defineProps<{
  experience: ExperienceItem[]
}>()

// Extract the **end** year (or current year if ongoing)
function getEndYear(duration: string): string {
  // If it contains "Present", use current year (2025 as of now, but you can hardcode or make dynamic)
  if (duration.toLowerCase().includes('present')) {
    return '2025'
  }
  // Find the last 4-digit year in the string (e.g., "2021 - 2022" → 2022)
  const matches = duration.match(/\d{4}/g)
  if (matches && matches.length > 0) {
    return matches[matches.length - 1]
  }
  return 'Present'
}

// Compute year for grouping (use provided year or extract end year)
const sortedExperiences = computed(() => {
  const withYear = props.experience.map(item => ({
    ...item,
    year: item.year || getEndYear(item.duration),
  }))
  return [...withYear].sort((a, b) => b.year.localeCompare(a.year))
})
</script>

<template>
  <div class="max-w-4xl mx-auto pb-20 px-4">
    <!-- Subtitle -->
    <div class="text-center mb-12">
      <p text-center mt--6 mb5 op50 text-lg italic>
        See what I've been doing and where I've been over the years.
      </p>
      <hr class="mt-8 mb-24 w-12 mx-auto border-gray-300 dark:border-gray-700">
    </div>

    <!-- Timeline -->
    <div class="timeline-wrapper relative">
      <!-- Continuous vertical line -->
      <div class="timeline-line absolute left-40 top-0 bottom-0 w-px bg-gray-300 dark:bg-gray-700 hidden md:block" />

      <div class="space-y-12">
        <template v-for="(item, idx) in sortedExperiences" :key="idx">
          <!-- Year header (uses end year or current year for ongoing) -->
          <div v-if="idx === 0 || item.year !== sortedExperiences[idx - 1].year" class="my-12 text-center">
            <span text-4em color-transparent font-bold font-mono leading-1em text-stroke-2 text-stroke-hex-888 op50 dark:op35>
              {{ item.year }}
            </span>
          </div>

          <!-- Experience item -->
          <div class="relative" :class="{ 'mt-3': idx > 0 && item.year === sortedExperiences[idx - 1].year }">
            <!-- Duration badge on the left -->
            <div class="absolute left-0 top-2 w-40 text-right hidden md:block">
              <div
                class="inline-block px-4 py-1.5 text-sm font-mono font-bold bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg"
              >
                {{ item.duration }}
              </div>
            </div>

            <!-- Mobile duration badge -->
            <div class="md:hidden mb-2 text-left">
              <div
                class="inline-block px-4 py-1.5 text-sm font-mono font-bold bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg"
              >
                {{ item.duration }}
              </div>
            </div>

            <!-- Timeline dot (gray) -->
            <div
              class="absolute left-40 top-4 w-4 h-4 rounded-full bg-gray-300 dark:bg-gray-700 ring-8 ring-white dark:ring-gray-950 hidden md:block"
            />

            <!-- Content card -->
            <div class="md:pl-56">
              <div
                class="experience-card p-5 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 bg-white dark:bg-gray-900 bg-opacity-50 hover:bg-opacity-70 transition-all duration-300 backdrop-blur"
              >
                <div class="flex items-start gap-3 mb-3">
                  <div
                    class="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center"
                    :class="item.type === 'education' ? 'bg-purple-100 dark:bg-purple-900' : 'bg-blue-100 dark:bg-blue-900'"
                  >
                    <div
                      v-if="item.type === 'education'"
                      class="i-carbon-education text-lg text-purple-600 dark:text-purple-400"
                    />
                    <div v-else class="i-carbon-code text-lg text-blue-600 dark:text-blue-400" />
                  </div>

                  <div class="flex-1 min-w-0">
                    <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100 mb-1">
                      <a
                        v-if="item.orgLink" :href="item.orgLink" target="_blank" rel="noopener noreferrer"
                        class="hover:text-blue-600 dark:hover:text-blue-400 transition-colors inline-flex items-center gap-1.5"
                      >
                        {{ item.org }}
                        <span class="i-carbon-launch text-base" />
                      </a>
                      <span v-else>{{ item.org }}</span>
                    </h3>
                    <div class="text-base font-medium text-gray-700 dark:text-gray-300">
                      {{ item.title }}
                      <span v-if="item.location" class="text-gray-600 dark:text-gray-400 font-normal"> · {{
                        item.location }}</span>
                    </div>
                  </div>
                </div>

                <p v-if="item.desc" class="text-sm text-gray-700 dark:text-gray-300 leading-relaxed mb-3">
                  {{ item.desc }}
                </p>

                <div
                  v-if="item.research"
                  class="p-4 rounded-lg bg-purple-50 dark:bg-purple-900 dark:bg-opacity-30 border-l-3 border-purple-500 mb-3"
                >
                  <div class="flex items-center justify-between mb-2">
                    <div class="flex items-center gap-2">
                      <div class="i-carbon-chemistry text-base text-purple-600 dark:text-purple-400" />
                      <span class="text-sm font-bold text-gray-800 dark:text-gray-200">
                        <a
                          v-if="item.research.labLink" :href="item.research.labLink" target="_blank"
                          rel="noopener noreferrer"
                          class="hover:text-purple-600 dark:hover:text-purple-400 transition-colors inline-flex items-center gap-1.5"
                        >
                          {{ item.research.lab }}
                          <span class="i-carbon-launch text-xs" />
                        </a>
                        <span v-else>{{ item.research.lab }}</span>
                      </span>
                    </div>
                    <span class="text-xs font-mono text-gray-600 dark:text-gray-400">{{ item.research.duration }}</span>
                  </div>
                  <div class="text-xs text-gray-600 dark:text-gray-400 mb-1">
                    Undergraduate Researcher · Advised by
                    <a v-if="item.research.advisorLink" :href="item.research.advisorLink" target="_blank" rel="noopener noreferrer" class="italic hover:text-purple-600 dark:hover:text-purple-400 transition-colors inline-flex items-center gap-1">
                      {{ item.research.advisor }}
                      <span class="i-carbon-launch text-[8px]" />
                    </a>
                    <span v-else class="italic">{{ item.research.advisor }}</span>
                  </div>
                  <div class="text-xs font-medium text-gray-700 dark:text-gray-300 mb-2">
                    {{ item.research.project }}
                  </div>
                  <ul class="space-y-1">
                    <li
                      v-for="detail in item.research.details" :key="detail"
                      class="flex items-start gap-1.5 text-xs text-gray-600 dark:text-gray-400"
                    >
                      <span class="i-carbon-dot-mark text-xs mt-0.5 flex-shrink-0" />
                      <span>{{ detail }}</span>
                    </li>
                  </ul>
                </div>

                <ul v-if="item.highlights" class="space-y-1 mb-3">
                  <li
                    v-for="highlight in item.highlights" :key="highlight"
                    class="flex items-start gap-2 text-sm text-gray-600 dark:text-gray-400"
                  >
                    <span class="i-carbon-arrow-right text-sm mt-0.5 flex-shrink-0 text-gray-400 dark:text-gray-500" />
                    <span v-html="highlight.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')" />
                  </li>
                </ul>

                <div
                  v-if="item.impact"
                  class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-green-100 dark:bg-green-900 dark:bg-opacity-40 text-xs font-medium mb-3"
                >
                  <div class="i-carbon-trophy text-green-600 dark:text-green-400" />
                  <span class="text-gray-700 dark:text-gray-300">{{ item.impact }}</span>
                </div>

                <div v-if="item.tech" class="flex flex-wrap gap-2">
                  <span
                    v-for="t in item.tech" :key="t"
                    class="px-2 py-1 text-xs font-mono bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
                  >
                    {{ t }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>

    <div class="prose pb5 mx-auto mt10 text-center">
      <hr>
    </div>

    <!-- Call to action -->
    <div class="text-center mt-16 pt-8">
      <p class="text-base text-gray-600 dark:text-gray-400">
        Want to learn more about my personal projects? Check out
        <a href="/projects" class="text-blue-600 dark:text-blue-400 hover:underline font-medium">projects</a>!
      </p>
    </div>
  </div>
</template>

<style scoped>
.timeline-wrapper {
  position: relative;
}

.experience-card {
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.experience-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.dark .experience-card {
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

.dark .experience-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}

@media (max-width: 768px) {
  .timeline-line {
    display: none;
  }
}
</style>
