<script setup lang="ts">
    import { ref, reactive , onBeforeUnmount, onMounted } from 'vue';
    import { useRoute } from 'vue-router';
    import apiWebSocketSetup from "@/services/web-socket";
    import ColorPicker from '@radial-color-picker/vue-color-picker';
    import '@radial-color-picker/vue-color-picker/dist/vue-color-picker.min.css';

    const route = useRoute();
    const adressDevice = ref(route.params.id);

    let apiWebSocket = null;
    let webSocket = null;

    let message = ref('');

    const sendMessage = () => {
        if (webSocket) {
            webSocket.send(message.value);
        }
    };
    
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

    onMounted(() => {
        console.log(route.params.id);
        adressDevice.value = route.params.id;
        apiWebSocket = new apiWebSocketSetup({
            url: `ws://localhost:8080/bluetooth/devices/${adressDevice.value}/stream`
        });

        webSocket = apiWebSocket.getWebSocket();

        webSocket.onmessage = (event) => {
            console.log(event.data);
            //const obj = JSON.parse(event.data);
            
        };
    });

    onBeforeUnmount(() => {
        if (webSocket) {
            webSocket.close();
        }
    });


</script>

<template>
    Devices components
    <div>
        <color-picker v-bind="color" @input="updateColor" @select="selectColor"></color-picker>
        <pre v-text="JSON.stringify(color, null, 4)"></pre>
    </div>
    {{route.params.id}}

    <input v-model="message" @keyup.enter="sendMessage" />
</template>