<script setup lang="ts">
import { ref } from 'vue'

const isDark = ref(useDark())
onMounted(() => {
  // Preload both images
  const dark = new Image()
  dark.src = '/media-dark.png'

  const light = new Image()
  light.src = '/media-light.png'
})
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
