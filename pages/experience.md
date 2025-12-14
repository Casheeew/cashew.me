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

  - duration: 12/2025 - 02/2026 (exp.)
    type: work
    title: Software Engineer Intern
    org: BeringLab (Incoming)
    orgLink: https://beringlab.com
    location: Seoul, Republic of Korea
    desc: Develop, test and scale APIs for BeringAI legal translation services.
    tech:
      - Backend Development
      - API Integration
      - Testing
      - OCR
      - AI Translation
      - Python

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
      - Developed an analytics web dashboard using React to visualize digital footprint data insights and deliver machine learning-driven decision making for fraud detection
      - Formulated a decision forest model on digital footprints to catch suspicious activity
      - Closely collaborated with a cross-functional team to bring the system from concept to deployment
    impact: Reduced manual review times by 90%, detecting tens of thousands of suspicious activities per month
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
