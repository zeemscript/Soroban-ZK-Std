# Contributing to Soroban-ZK-Std Documentation

Thank you for your interest in improving the Soroban-ZK-Std documentation! The documentation is built with Next.js and uses MDX (Markdown + JSX), which allows us to embed React components right inside standard markdown files.

This step-by-step guide will walk you through exactly how to add or update documentation.

---

## 🚀 Step 1: Run the Docs Locally

Before you make any changes, you'll want to run the site locally so you can preview your edits in real-time.

1. Ensure you have `Node.js` and `pnpm` installed.
2. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```
3. Install dependencies:
   ```bash
   pnpm install
   ```
4. Start the development server:
   ```bash
   pnpm dev
   ```
5. Open [http://localhost:3000](http://localhost:3000) in your browser. The documentation lives under `/docs`.

---

## 📂 Step 2: Understand the Architecture

All documentation content is stored in the `frontend/content/docs/` directory.

To keep things perfectly organized, files are grouped into categorical subfolders:
* `getting-started/`: Overviews, tutorials, and onboarding.
* `core-concepts/`: Architecture, guides, and FAQs.
* `math-and-cryptography/`: ZK math, cryptographic primitives.
* `integration-and-verification/`: Verifier contract guides.
* `reference/`: Glossaries and API references.

> **Important:** The URL routing is completely flat! Even if you place a file deep in a subfolder (e.g., `content/docs/math-and-cryptography/fri.mdx`), the URL will still be a clean `/docs/fri`.

---

## ✍️ Step 3: Create or Edit a File

To add a new page:
1. Pick the correct subfolder in `frontend/content/docs/`.
2. Create a new `.mdx` file (e.g., `my-new-guide.mdx`).
3. Write your content.

### Using Special Components
Because we use MDX, you have access to powerful custom formatting tools:

**Code Blocks:**
Standard markdown code blocks are automatically intercepted by our Shiki engine for syntax highlighting and line numbers.
\`\`\`rust
pub fn verify() -> bool {
    true
}
\`\`\`

**Math Equations:**
Wrap mathematical equations in `$$` (for blocks) or `$` (for inline). Be sure to keep `$$` block formulas entirely on one single line to prevent parsing errors.
\`\`\`text
$$ \sum_{i=0}^n x_i $$
\`\`\`

**Callouts / Alerts:**
You can use our custom Alert component for warnings or tips.
\`\`\`tsx
<Alert variant="info" title="Note">
  This is an important piece of information.
</Alert>
\`\`\`

---

## 🧭 Step 4: Add the Page to the Sidebar

Just creating an `.mdx` file doesn't automatically put it on the website's sidebar navigation.

To make your page visible to users:
1. Open `frontend/lib/navigation.ts`.
2. Find the relevant categorical section.
3. Add a new entry to the `children` array. The `href` MUST match your file's name (without the `.mdx` extension).

For example, if your file is named `my-new-guide.mdx`:
```ts
{
  title: "Getting Started",
  children: [
    { title: "Introduction", href: "/docs" },
    { title: "My New Guide", href: "/docs/my-new-guide" }, // <-- Add this!
  ],
}
```

---

## ✅ Step 5: Test and Submit

1. **Test your changes:** Verify your new page loads perfectly at `http://localhost:3000/docs/my-new-guide`. Check the sidebar, make sure links aren't broken, and ensure your markdown renders without errors.
2. **Commit:** Commit your changes.
3. **Pull Request:** Open a PR against the main repository! Your new documentation will be automatically compiled and statically generated on the next production build.
