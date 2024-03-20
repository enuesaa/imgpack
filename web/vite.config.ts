import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import routify from '@roxi/routify/vite-plugin'
import { preprocessMeltUI, sequence } from '@melt-ui/pp'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [
    routify({
      render: {
        ssr: { enable: false }
      }
    }),
    svelte({
      compilerOptions: {
        hydratable: !!process.env.ROUTIFY_SSR_ENABLE
      },
      preprocess: sequence([vitePreprocess(), preprocessMeltUI()])
    })
  ]
})
