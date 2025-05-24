import { defineConfig } from 'tailwindcss'

export default defineConfig({
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        'mono': ['JetBrains Mono', 'Courier New', 'monospace'],
      },
    },
  },
})
