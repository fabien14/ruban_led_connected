import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import apiClientSetup, { ApiClient } from './services/api-client';

import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap/dist/js/bootstrap.js';
import 'bootstrap-icons/font/bootstrap-icons.css';
import 'bootstrap-switch-button/css/bootstrap-switch-button.min.css';
import 'bootstrap-switch-button/dist/bootstrap-switch-button.min.js';



const apiClient = apiClientSetup({
    baseUrl: 'http://localhost:8080/',
});

const app = createApp(App);
app.use(router);
app.provide('apiClient', apiClient);
app.mount('#app');
