import { defineConfig } from 'vite';
import reactRefresh from '@vitejs/plugin-react-refresh';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [reactRefresh()],
    css: {
        preprocessorOptions: {
            less: {
                modifyVars: { '@primary-color': '#1DA57A' },
                javascriptEnabled: true,
            }
        }
    }
});
