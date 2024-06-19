import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import PrimeVue from 'primevue/config';
import ConfirmationService from 'primevue/confirmationservice';
import Aura from '@primevue/themes/aura';
//


const app = createApp(App);

app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            cssLayer: {
                name: 'primevue',
                order: 'tailwind-base, primevue, tailwind-utilities'
            }
        }
    }
});

app.use(ConfirmationService);

app.mount('#root')