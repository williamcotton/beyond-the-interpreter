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
            { label: '2.3 · Explain input failures', link: '/chapter-2/2.3-input-errors/' },
            { label: '2.4 · Check the input rules', link: '/chapter-2/2.4-input-tests/' },
          ],
        },
        {
          label: '3 · Commands and Clap',
          items: [
            { label: '3.1 · Give the operation a name', link: '/chapter-3/3.1-source-command/' },
            { label: '3.2 · Describe the command with Clap', link: '/chapter-3/3.2-clap/' },
          ],
        },
        {
          label: '4 · Source Locations, Workspaces, and CI',
          items: [
            { label: '4.1 · Locate text with byte ranges', link: '/chapter-4/4.1-byte-ranges/' },
            { label: '4.2 · Identify the source file', link: '/chapter-4/4.2-file-origins/' },
            { label: '4.3 · Extract the workspace', link: '/chapter-4/4.3-workspace/' },
            { label: '4.4 · Run the checks automatically', link: '/chapter-4/4.4-ci/' },
          ],
        },
        {
          label: '5 · Building a Scanner by Hand',
          items: [
            { label: '5.1 · Read a character, emit a token', link: '/chapter-5/5.1-first-tokens/' },
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
