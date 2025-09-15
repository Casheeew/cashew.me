<script setup lang="ts">
import { ref } from 'vue'

const isDark = ref(useDark())
</script>

<template>
  <div
    :style="{
      position: 'absolute',
      top: '0',
      left: '0',
      bottom: '0',
      width: '100vw',
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
