/**
 * Shiki highlighting subprocess.
 *
 * Reads JSON from stdin: { code: string, lang: string }
 * Writes JSON to stdout: { html: string }
 *
 * Configuration matches cashew.me/vite.config.ts lines 88-104.
 */
import { createHighlighter } from 'shiki'
import {
  transformerNotationDiff,
  transformerNotationHighlight,
  transformerNotationWordHighlight,
} from '@shikijs/transformers'
import { rendererRich, transformerTwoslash } from '@shikijs/twoslash'

let highlighter = null

async function getHighlighter() {
  if (!highlighter) {
    highlighter = await createHighlighter({
      themes: ['vitesse-dark', 'vitesse-light'],
      langs: [
        'javascript', 'typescript', 'jsx', 'tsx', 'vue', 'vue-html',
        'html', 'css', 'scss', 'json', 'yaml', 'toml', 'markdown',
        'bash', 'shell', 'python', 'rust', 'go', 'java', 'c', 'cpp',
        'ruby', 'php', 'swift', 'kotlin', 'sql', 'graphql', 'diff',
        'xml', 'svelte', 'astro', 'wasm', 'lua', 'zig',
      ],
    })
  }
  return highlighter
}

async function highlight(code, lang) {
  const h = await getHighlighter()

  // Check if the language is loaded, fallback to 'text' if not
  const loadedLangs = h.getLoadedLanguages()
  const actualLang = loadedLangs.includes(lang) ? lang : 'text'

  const html = h.codeToHtml(code, {
    lang: actualLang,
    themes: {
      dark: 'vitesse-dark',
      light: 'vitesse-light',
    },
    defaultColor: false,
    cssVariablePrefix: '--s-',
    transformers: [
      transformerTwoslash({
        explicitTrigger: true,
        renderer: rendererRich(),
      }),
      transformerNotationDiff(),
      transformerNotationHighlight(),
      transformerNotationWordHighlight(),
    ],
  })

  return html
}

// Read stdin
let input = ''
process.stdin.setEncoding('utf-8')
process.stdin.on('data', chunk => {
  input += chunk
})
process.stdin.on('end', async () => {
  try {
    const { code, lang } = JSON.parse(input)
    const html = await highlight(code, lang)
    process.stdout.write(JSON.stringify({ html }))
  } catch (err) {
    process.stderr.write(err.message)
    process.exit(1)
  }
})
