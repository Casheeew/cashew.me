import { createGenerator } from '@unocss/core'
import { presetAttributify, presetIcons, presetWind3, presetWebFonts } from 'unocss'
import { createLocalFontProcessor } from '@unocss/preset-web-fonts/local'
import transformerDirectives from '@unocss/transformer-directives'
import fs from 'fs'
import { glob } from 'fs/promises'
import MagicString from 'magic-string'

const config = {
  shortcuts: [
    {
      'bg-base': 'bg-white dark:bg-black',
      'color-base': 'text-black dark:text-white',
      'border-base': 'border-[#8884]',
    },
    [/^btn-(\w+)$/, ([_, color]) => `op50 px2.5 py1 transition-all duration-200 ease-out no-underline! hover:(op100 text-${color} bg-${color}/10) border border-base! rounded`],
  ],
  rules: [
    [/^slide-enter-(\d+)$/, ([_, n]) => ({
      '--enter-stage': n,
    })],
  ],
  presets: [
    presetIcons({
      extraProperties: {
        'display': 'inline-block',
        'height': '1.2em',
        'width': '1.2em',
        'vertical-align': 'text-bottom',
      },
      collections: {
        ri: () => import('@iconify-json/ri/icons.json', { with: { type: 'json' } }).then(m => m.default),
        uil: () => import('@iconify-json/uil/icons.json', { with: { type: 'json' } }).then(m => m.default),
        carbon: () => import('@iconify-json/carbon/icons.json', { with: { type: 'json' } }).then(m => m.default),
        ph: () => import('@iconify-json/ph/icons.json', { with: { type: 'json' } }).then(m => m.default),
        'simple-icons': () => import('@iconify-json/simple-icons/icons.json', { with: { type: 'json' } }).then(m => m.default),
      },
    }),
    presetAttributify(),
    presetWind3(),
    presetWebFonts({
      fonts: {
        sans: 'Inter',
        mono: 'DM Mono',
        condensed: 'Roboto Condensed',
        wisper: 'Bad Script',
      },
      processors: createLocalFontProcessor(),
    }),
  ],
  transformers: [
    transformerDirectives(),
  ],
  safelist: [
    'i-ri-menu-2-fill',
    'i-ri-arrow-up-line',
    'i-ri-lightbulb-line',
    'i-ri-article-line',
    'i-ri-camera-3-line',
    'i-ri-moon-line',
    'i-ri-sun-line',
    'i-uil-github-alt',
    'i-uil-linkedin-alt',
  ],
}

const generator = await createGenerator(config)

// 1. Collect all HTML content for utility generation
const htmlFiles = []
for await (const file of glob('dist/**/*.html')) {
  htmlFiles.push(file)
}

let allContent = ''
for (const file of htmlFiles) {
  allContent += fs.readFileSync(file, 'utf-8') + '\n'
}

// 2. Generate utility CSS from HTML
const { css } = await generator.generate(allContent)
fs.mkdirSync('dist/styles', { recursive: true })
fs.writeFileSync('dist/styles/uno.css', css)

// 3. Process CSS files to expand --uno: / --at-apply: / @apply directives
const transformer = transformerDirectives()
const cssFiles = ['dist/styles/main.css', 'dist/styles/prose.css', 'dist/styles/markdown.css']
let cssTransformed = 0

for (const file of cssFiles) {
  if (!fs.existsSync(file)) continue
  const original = fs.readFileSync(file, 'utf-8')
  if (!original.includes('--uno:') && !original.includes('--at-apply:') && !original.includes('@apply')) continue

  const s = new MagicString(original)
  await transformer.transform(s, file, { uno: generator, tokens: new Set() })
  const transformed = s.toString()

  if (transformed !== original) {
    fs.writeFileSync(file, transformed)
    cssTransformed++
  }
}

// Count results
const iconCount = (css.match(/--un-icon/g) || []).length
const totalUtilities = (css.match(/\{/g) || []).length
console.log(`Generated ${totalUtilities} rules (including ${iconCount} icons) to dist/styles/uno.css`)
if (cssTransformed > 0) {
  console.log(`Expanded directives in ${cssTransformed} CSS files`)
}
