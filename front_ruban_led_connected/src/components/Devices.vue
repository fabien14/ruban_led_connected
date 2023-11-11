<script setup lang="ts">
    import { reactive } from 'vue';
    import ColorPicker from '@radial-color-picker/vue-color-picker';
    import '@radial-color-picker/vue-color-picker/dist/vue-color-picker.min.css';

    
    const color = reactive({
        hue: 50,
        saturation: 100,
        luminosity: 50,
        alpha: 1,
    });

    const updateColor = (value:number) => {
        color.hue = value;
    };

    const selectColor = (value:number) => {
        console.log(value);
    };
</script>

<template>
    Devices components
    <div>
        <color-picker v-bind="color" @input="updateColor" @select="selectColor"></color-picker>
        <pre v-text="JSON.stringify(color, null, 4)"></pre>
    </div>
</template>