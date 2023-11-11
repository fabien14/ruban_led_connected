import { reactive } from "vue";
import { io, Socket } from "socket.io-client";

interface ApiWebSocketOptions {
    url?: string
    token?: string
}

export class ApiWebSocket {
    url: string = '';
    instance: WebSocket | null = null;
    connected: boolean = false;

    constructor(options: ApiWebSocketOptions) {
        if (options.url) {
            this.url = options.url;
        }

        this.instance = new WebSocket(this.url);

        this.instance.addEventListener('open', () => {
            console.log("OPEN");
        });

        this.instance.addEventListener('close', () => {
            console.log("CLOSE");
        });
    }

    getWebSocket() {
        return this.instance;
    }
}

function setup(options: ApiWebSocketOptions) {
    return new ApiWebSocket(options);
}

export default setup;
