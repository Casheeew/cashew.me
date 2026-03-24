---
title: Experience - Cashew
display: Experience
description: Places where I've worked at.
wrapperClass: 'text-center'
art: dots
experience:
  - duration: Present - 02/2027
    type: education
    title: B.S. in Computer Science
    org: Korea Advanced Institute of Science and Technology (KAIST)
    orgLink: https://www.kaist.ac.kr
    location: Daejeon, Republic of Korea
    desc: Major GPA 94.3/100 (3.77/4) · Overall 91.7/100 (3.67/4)
    research:
      lab: Human Centered Interactive Technologies Lab
      duration: 06/2025 - 10/2025  # Add your specific research duration here
      labLink: https://hcitech.org
      advisor: Prof. Sang Ho Yoon
      advisorLink: https://sanghoy.com
      project: Dance Motion Simplification via Diffusion Models
      details:
        - Full paper submitted to CHI'26
        - Researched novel interaction techniques for entertainment computing
        - Explored diffusion models and generative AI to make dance more accessible
    tech:
      - HCI Research
      - Diffusion Models
      - Python
      - Deep Learning
      - Machine Learning

  - duration: 10/2025 - 02/2026
    type: work
    title: Backend / Systems Engineer Intern
    org: Bering Lab
    orgLink: https://beringlab.com
    location: Seoul, Republic of Korea
    desc: Core Product Team
    highlights:
      - Built and maintained cloud backend services in **Rust** on **Amazon EKS** for **high-volume text & document processing and translation** used by global enterprise clients.
      - Built document parsers for OOXML (DOCX, XLSX, PPTX) with **improved segmentation and detection** of non-translatable text, **enabling efficient streaming processing** for file translations with full layout preservation.
      - Reduced system overhead by **10× during traffic spikes** by designing adaptive request batching for file and text.
      - Deployed and managed the system in Cloud and air-gapped On-Premise environments, ensuring reliable execution under strict infrastructure constraints.
    tech:
      - Rust
      - Amazon EKS
      - Kubernetes
      - Backend Development
      - Document Processing
      - API Integration

  - duration: 02/2025 - 10/2025
    type: work
    title: Software Engineer Freelancer
    org: AiGlow
    orgLink: https://www.linkedin.com/company/aiglowedu/
    location: New York City, United States (Remote)
    highlights:
      - Built a scalable online examination platform with Java, Spring Boot, Docker, and Firestore, supporting multi-tenant access and high-throughput data handling and real-time student analytics
      - Integrated a RAG LLM tutoring assistant with Weaviate to analyze performance, identify learning patterns, and deliver customized feedback
      - Used React, Chakra UI and Axios to build intuitive and modular role-based user interfaces for students and teachers
    impact: Serving 1000+ students with 80% boost in engagement and satisfaction
    tech:
      - Java
      - Spring Boot
      - Docker
      - Firestore
      - RAG
      - LLM
      - Weaviate
      - React
      - Chakra UI

  - duration: 12/2023 - 02/2024
    type: work
    title: Software Engineer Intern
    org: VPBank
    orgLink: https://www.vpbank.com.vn/en
    location: Hanoi, Vietnam
    desc: Digital Footprint Team
    highlights:
      - Built a **full-stack fraud detection monitoring web app** in **Spring Boot** and **React**, enabling automated logging of digital footprints on millions of transactions for a major bank in Vietnam, **cutting review times by 90%**.
      - Delivered the production-ready analytics app with automated decision-making within only 8 weeks in a team of 3.
    tech:
      - React
      - Machine Learning
      - Decision Forest
      - Data Analytics
      - Fraud Detection

  - duration: 02/2022 - 12/2023
    type: education
    title: Undergraduate Researcher
    org: Neuro-Machine Augmented Intelligence Lab
    orgLink: http://nmail.kaist.ac.kr
    location: Daejeon, Republic of Korea
    desc: School of Computing, KAIST
    highlights:
      - Restoration of audio using Attention Mechanism in Deep Learning
      - Facial Emotion Recognition targeting on Complex Emotions using Deep Learning
    tech:
      - Deep Learning
      - Attention Mechanism
      - Audio Processing
      - Computer Vision
      - Emotion Recognition
---

<!-- @layout-full-width -->
<ListExperience :experience="frontmatter.experience" />
