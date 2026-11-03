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
            { label: '1.1 · A program we can run', link: '/chapter-1/1.1-first-program/' },
          ],
        },
        {
          label: '2 · Reading Source',
          items: [
            { label: '2.1 · Give the program a filename', link: '/chapter-2/2.1-filename/' },
            { label: '2.2 · Read the source', link: '/chapter-2/2.2-read-source/' },
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
