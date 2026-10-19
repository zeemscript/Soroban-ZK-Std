# Documentation Content Directory

This directory contains the Markdown (`.md` and `.mdx`) source files that generate the Soroban-ZK-Std documentation site. 

## 📂 Folder Structure

To keep the documentation clean and maintainable, all files are strictly organized into subdirectories that correspond to the main navigation categories of the site.

When contributing new documentation, **do not place files in the root of this directory.** Please place your `.md` or `.mdx` file into the appropriate categorical subfolder:

```text
content/docs/
├── getting-started/
│   # Onboarding, overviews, and tutorials (e.g. why-soroban.mdx)
├── core-concepts/
│   # High-level architecture, upgrading guides, and FAQs
├── math-and-cryptography/
│   # Deep dives into ZK math, hash functions, and protocols (e.g. fri.md, bulletproofs.mdx)
├── integration-and-verification/
│   # Practical guides for verifying proofs in Soroban contracts
└── reference/
    # Glossaries, API references, and specifications
```

## 🔗 Routing Architecture

Under the hood, the Next.js router (`app/docs/[slug]`) is designed with **flat URL routing** but **nested file resolution**. 

What this means for you:
1. You can organize files into as many deep subfolders as you want inside this directory to keep things tidy.
2. The site's URL structure will always remain completely flat (e.g. `localhost:3000/docs/fri`), regardless of how deeply nested the file is (`math-and-cryptography/fri.md`).
3. You do **not** need to update existing internal markdown links when moving a file into a subfolder. The backend automatically scans the entire folder tree recursively to match the URL slug to the correct file!

## 📝 Navigation Sidebar

Creating a file here does not automatically add it to the website's sidebar. To make a new document visible in the site navigation, you must manually register its URL slug in the centralized navigation configuration file located at:
👉 `frontend/lib/navigation.ts`
