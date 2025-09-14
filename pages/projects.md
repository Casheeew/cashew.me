---
title: Projects - Anthony Fu
display: Projects
description: List of projects that I am proud of
wrapperClass: 'text-center'
art: dots
projects:
  Maintaining:
    - name: 'Yomitan'
      link: 'https://github.com/yomidevs/yomitan'
      desc: 'Powerful and versatile pop-up dictionary loved by language learners.'
      icon: 'i-simple-icons-vite'
    - name: 'Yomitan Wiki'
      link: 'https://github.com/yomidevs/yomitan-wiki'
      desc: 'The flagship website of Yomitan.'
      icon: 'i-logos-nuxt-icon saturate-0'

  Past Projects:
    - name: 'Split-n-Share!'
      link: 'https://github.com/Casheeew/split-n-share'
      desc: 'Your solution to group buying.'
      icon: 'i-logos-nuxt-icon saturate-0'
    - name: 'CashewBot'
      link: 'https://github.com/Casheeew/CashewBot'
      desc: 'Discord Bot for learning Chinese.'
      icon: 'i-logos-nuxt-icon saturate-0'
    - name: 'DaruScript'
      link: 'https://github.com/DaruScript/DaruScript'
      desc: 'Rust-inspired programming language for the lazy.'
      icon: 'i-logos-nuxt-icon saturate-0'
    - name: 'Learn Chinese'
      link: 'https://casheeew.github.io/learn-chinese/'
      desc: 'The Learn Chinese blog.'
      icon: 'i-logos-nuxt-icon saturate-0'
    - name: 'Wikipedia CSS'
      link: 'https://github.com/Casheeew/wikipedia-css'
      desc: 'Custom Wikipedia CSS for Japanese and Chinese.'
      icon: 'i-logos-nuxt-icon saturate-0'
    - name: 'Cashew Wiki'
      link: 'https://github.com/Casheeew/cashew-wiki'
      desc: 'My previous personal website.'
      icon: 'i-logos-nuxt-icon saturate-0'

---

<!-- @layout-full-width -->
<ListProjects :projects="frontmatter.projects" />
