import { createApp } from 'vue'
import App from './App.vue'
import { initContract } from './near/utils'

import { Buffer } from "buffer";
global.Buffer = Buffer;

window.nearInitPromise = initContract()
.then(() => {
    const app = createApp(App)

    app.mount('#app')
})

