import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [
    starlight({
      title: 'Beyond the Interpreter',
      description: 'Build a language, then the tools that bring it to life. An interactive Rust companion to Crafting Interpreters.',
      sidebar: [{ label: 'Introduction', link: '/' }],
      customCss: ['./src/styles/book.css'],
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 2 },
      pagination: false,
      // Search becomes useful once there is more than one chapter to find.
      pagefind: false,
      favicon: '/favicon.svg',
    }),
    react(),
  ],
});
