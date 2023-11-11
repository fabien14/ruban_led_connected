// api-client.ts

import axios, { type AxiosInstance } from 'axios'
import type { App } from 'vue'

interface ApiClientOptions {
    baseUrl?: string
    token?: string
}

export class ApiClient {
    baseUrl: string = '';
    instance: AxiosInstance | null = null;

    constructor(options: ApiClientOptions) {
        if (options.baseUrl) {
            this.baseUrl = options.baseUrl;
        }

        this.instance = axios.create({
            baseURL: this.baseUrl,
            withCredentials: false,
            headers: {
                "Access-Control-Allow-Origin": "*",
                "Access-Control-Allow-Methods": "GET, POST, PATCH, PUT, DELETE, OPTIONS",
                "Access-Control-Allow-Headers": "Origin, Content-Type, X-Auth-Token",
                "Content-Type": 'application/json',
                Authorization: options.token ? `Bearer ${options.token}` : '',
            }
        });
    }

    get(uri: string) {
        if (!this.instance) {
            throw Error('Axios instance not instanciated');
        }
        return this.instance.get(uri);
    }

    post(uri: string, data: object = {}) {
        if (!this.instance) {
            throw Error('Axios instance not instanciated');
        }
        return this.instance.post(uri, data);
    }
}

function setup(options: ApiClientOptions) {
    return new ApiClient(options);
}

export default setup;
