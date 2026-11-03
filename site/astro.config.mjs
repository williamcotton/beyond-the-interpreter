import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [
    starlight({
      title: 'Beyond the Interpreter',
      description: 'Build a language, then the tools that bring it to life. An interactive Rust companion to Crafting Interpreters.',
      sidebar: [
        { label: 'Introduction', link: '/' },
        {
          label: '1 · A Third Lox',
          items: [
            { label: '1.1 · A program we can run', link: '/foundations/a-third-lox/first-program/' },
          ],
        },
      ],
      customCss: ['./src/styles/book.css'],
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 2 },
      pagination: true,
      // Search can wait until there is a larger body of chapters to explore.
      pagefind: false,
      favicon: '/favicon.svg',
    }),
    react(),
  ],
});
