<script setup lang="ts">
  import TopMenu from '@/components/TopMenu.vue'
</script>

<template> 
  <header class="bd-header">
    <TopMenu />
  </header>

  <main class="container-fluid">
    <router-view/>
  </main>
</template>


