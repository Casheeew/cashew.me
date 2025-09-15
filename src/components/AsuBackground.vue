<script setup lang="ts">
import { ref } from 'vue'

const isDark = ref(useDark())
</script>

<template>
  <div
    :style="{
      position: 'fixed',
      top: '0',
      left: '0',
      bottom: '0',
      width: '100vw',
      height: '100vh',
      backgroundImage: `url('${isDark ? '/media-dark.webp' : '/media-light.webp'}')`,
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
