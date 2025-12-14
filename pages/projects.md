---
title: Projects - Cashew
display: Projects
wrapperClass: 'text-center'
art: dots
projects:
  Maintaining:
    subtitle: "My maintained projects, or works-in-progress."
    items:
      - name: 'Yomitan'
        link: 'https://github.com/yomidevs/yomitan'
        desc: 'Powerful and versatile pop-up dictionary loved by 100,000+ language learners.'
        icon: 'i-carbon-book'
        imageLight: 'project-images/yomitan-showcase-light.png'
        imageDark: 'project-images/yomitan-showcase-dark.png'
      - name: 'Yomitan Wiki'
        link: 'https://github.com/yomidevs/yomitan-wiki'
        desc: 'The flagship website of Yomitan.'
        icon: 'i-carbon-mountain'
        imageLight: 'project-images/yomitan-wiki-light.png'
        imageDark: 'project-images/yomitan-wiki-dark.png'
      - name: 'Project Sinoxenica (TBA)'
        link: '#'
        desc: 'Learn Japanese, Chinese, Korean, and Vietnamese all in one app!'
        icon: 'i-carbon-mountain'

  Past Projects:
    subtitle: 'Projects that are mature or no longer active.'
    items:
      - name: 'Split-n-Share!'
        link: 'https://github.com/Casheeew/split-n-share'
        desc: 'Your solution to group buying.'
        icon: 'i-carbon-wallet'
        image: 'project-images/split-n-share-showcase.png'
      - name: 'Learn Chinese'
        link: 'https://casheeew.github.io/learn-chinese/'
        desc: 'The Learn Chinese blog.'
        imageLight: 'project-images/learn-chinese-light.png'
        imageDark: 'project-images/learn-chinese-dark.png'
        icon: 'i-carbon-education'
      - name: 'Wikipedia CSS'
        link: 'https://github.com/Casheeew/wikipedia-css'
        desc: 'Custom Wikipedia CSS for Japanese and Chinese.'
        icon: 'i-carbon-html'
        imageLight: 'project-images/wikipedia-css-light.png'
        imageDark: 'project-images/wikipedia-css-dark.png'
      - name: 'Cashew Wiki'
        link: 'https://github.com/Casheeew/cashew-wiki'
        desc: 'My previous personal website.'
        icon: 'i-carbon-wikis'
      - name: 'CashewBot'
        link: 'https://github.com/Casheeew/CashewBot'
        desc: 'Discord Bot for learning Chinese.'
        icon: 'i-carbon-bot'
      - name: 'DaruScript'
        link: 'https://github.com/DaruScript/DaruScript'
        desc: 'Rust-inspired programming language for the lazy.'
        icon: 'i-carbon-code'
---

<!-- @layout-full-width -->
<ListProjects :projects="frontmatter.projects" />
