<script setup lang="ts">
import { ref } from 'vue'

const isDark = ref(useDark())
</script>

<template>
  <div
    :style="{
      position: 'absolute',
      top: '0',
      left: '-100vw',
      right: '-100vw',
      bottom: '0',
      backgroundImage: `url('${isDark ? '/media-dark.png' : '/media-light.png'}')`,
      backgroundPosition: 'center center',
      backgroundRepeat: 'no-repeat',
      backgroundSize: 'cover',
      backgroundAttachment: 'fixed',
      filter: 'blur(0px)',
      backgroundBlendMode: 'overlay',
      backgroundColor: 'rgba(255, 255, 255, 0)',
      zIndex: '-1',
    }"
  />
</template>
