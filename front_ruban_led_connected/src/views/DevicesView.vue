<script setup lang="ts">
    import { ref, inject } from 'vue';

    const apiClient: ApiClient = inject('apiClient');

    interface device {
        name: String,
        address: String, 
        connected: boolean,
        paired: boolean
    };

    const devicesList = ref<device[]>([]);

    const refreshDevices = () => {
        apiClient.get('/bluetooth/devices?connected=false')
        .then((response) => {
            devicesList.value = response.data.device;
        });
    };

    refreshDevices();
</script>

<template>
    <div class="row">
        <nav class="col-md-3 col-lg-2 d-md-block sidebar collapse">
            <div>
                <router-link class="btn btn-primary" :to="{path: `/scan/`}">Scan device</router-link>
            </div>
            <div v-for="device in devicesList">
                <router-link :to="{path: `/devices/${device.address}/`}" class="nav-link">
                    {{ device.name }} 
                </router-link>
            </div>
        </nav>
        <section class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
            <router-view/>
        </section>
    </div>
</template>