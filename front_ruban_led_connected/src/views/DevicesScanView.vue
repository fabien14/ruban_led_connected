<script setup lang="ts">
    import { ref, inject, onBeforeUnmount } from 'vue';
    import apiWebSocketSetup from "@/services/web-socket";
    import { useRouter } from 'vue-router';

    onBeforeUnmount(() => {
        webSocket.close();
    });

    const router = useRouter()
    const apiClient: ApiClient = inject('apiClient');

    interface scan {
        starting: boolean,
        started_time: Date | null,
        timeout: Number
    };

    const scanInfo = ref<scan>({
        starting: false,
        started_time: null,
        timeout: 0
    });

    interface device {
        name: String,
        address: String, 
        connected: boolean,
        paired: boolean
    };

    const devicesList = ref<device[]>([]);
    let activeChange = true;

    let apiWebSocket = apiWebSocketSetup({
        url: 'ws://localhost:8080/bluetooth/scan-stream'
    });
    let webSocket = apiWebSocket.getWebSocket();
    
    if (webSocket) {
        webSocket.onmessage = (event) => {
            const obj = JSON.parse(event.data);
            devicesList.value = obj.device;
        };
    }
    
    const changeIsScanRunning = () => {
        if (!scanInfo.value.starting && activeChange) {
            apiClient.post('/bluetooth/scan')
            .then((response) => {
                scanInfo.value = response.data;
                initScanTimer();
                switchButtonScanRunning(scanInfo.value.starting);
            });
        }
    };

    const getScanInfo = () => {
        activeChange = false;
        apiClient.get('/bluetooth/scan')
        .then((response) => {
            scanInfo.value = response.data;
            initScanTimer();
            switchButtonScanRunning(scanInfo.value.starting);
            activeChange = true;
        });
    };

    const switchButtonScanRunning = (switchButton:boolean) => {
        if (switchButton) {
            document.getElementById('isScanRunning').switchButton('on');
            document.getElementById('isScanRunning').switchButton('disable');
        }
        else {
            document.getElementById('isScanRunning').switchButton('off');
            document.getElementById('isScanRunning').switchButton('enable');
        }
    };

    let scanTimer = ref(0);
    let scanTimerStr = ref("");

    const initScanTimer = () => {
        if (!scanInfo.value.started_time) {
            console.log(scanInfo.value.started_time);
            scanTimer.value = 0;
            scanTimerStr.value = "";
            return;
        }
        
        const dateNow = Date.now();
        const deltaDate = dateNow - Date.parse(scanInfo.value.started_time);
        const deltaDateSecond = Math.floor(deltaDate/1000);
        const resteTimerSecond = scanInfo.value.timeout - deltaDateSecond;

        if (resteTimerSecond < scanInfo.value.timeout) {
            scanTimer.value = resteTimerSecond;
            scanTimerDown();
        }
        else {
            scanTimer.value = 0;
            scanTimerStr.value = "00:00";
        }
    };

    const scanTimerDown = () => {
        if (scanTimer.value > 0) {
            setTimeout(() => {
                scanTimer.value -= 1;
                scanTimerStrFormat();
                scanTimerDown();
            }, 1000);
        }
        else {
            getScanInfo();
        }
    };

    const scanTimerStrFormat = () => {
        const minutes = Math.floor(scanTimer.value / 60);
        const seconds = scanTimer.value % 60;
        scanTimerStr.value = `${minutes}:${seconds}`;
    };

    const connectDevice = (deviceAddress:String) => {
        apiClient.get(`/bluetooth/devices/${deviceAddress}/connect`)
        .then((response) => {
            router.push("/devices");
        });
    };

    getScanInfo();



</script>

<template>
    <p>Scan Timer: {{ scanTimerStr }}</p>
    <input type="checkbox" 
        data-toggle="switchbutton" 
        data-onstyle="success" 
        data-onlabel='<i class="bi bi-play-fill"></i> Scanning'
        data-offlabel='<i class="bi bi-stop-fill"></i> Scan'
        data-width="140"
        id="isScanRunning"
        @change="changeIsScanRunning">

    <ol class="list-group list-group-numbered d-flex justify-content-center vw-50">
        <li class="list-group-item d-flex justify-content-between align-items-start" v-for="device in devicesList" >
            <div class="ms-2 me-auto">
                <div class="fw-bold">{{ device.name }}</div>
                {{ device.address }}
            </div>
            <div class="ms-2 me-auto">
                <div>Connected : <i class="bi bi-circle-fill" v-bind:class="[device.connected ? 'text-success': 'text-danger']"></i></div>
                <div>Paired : <i class="bi bi-circle-fill" v-bind:class="[device.paired ? 'text-success': 'text-danger']"></i></div>
            </div>
            <button type="button" class="btn btn-success" @click="connectDevice(device.address);">Connect</button>
        </li>
    </ol>

    <pre>{{ scanInfo }}</pre>
    <pre>{{ devicesList }}</pre>
</template>