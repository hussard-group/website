import { defineConfig } from 'astro/config';

export default defineConfig({
    output: 'static',
    compressHTML: true,
    trailingSlash: 'never',
    prefetch: {
        prefetchAll: true,
        defaultStrategy: 'hover',
    },
    build: {
        inlineStylesheets: 'auto',
    },
    vite: {
        build: {
            target: 'esnext',
            reportCompressedSize: false,
        },
    },
});
