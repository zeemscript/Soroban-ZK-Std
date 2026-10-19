"use client";

import React, { useEffect, useState } from "react";

interface TocItem {
  id: string;
  text: string;
  level: number;
}

export function TocSidebar() {
  const [items, setItems] = useState<TocItem[]>([]);
  const [activeId, setActiveId] = useState<string>("");

  useEffect(() => {
    // Wait a brief moment to ensure MDX has rendered
    const timeoutId = setTimeout(() => {
      const elements = Array.from(document.querySelectorAll("main h2, main h3"));
      const headings = elements
        .map((elem) => ({
          id: elem.id,
          text: elem.textContent || "",
          level: Number(elem.tagName.charAt(1)),
        }))
        .filter((h) => h.id);

      setItems(headings);

      const observer = new IntersectionObserver(
        (entries) => {
          const visibleEntries = entries.filter((entry) => entry.isIntersecting);
          if (visibleEntries.length > 0) {
            // Find the first visible one
            setActiveId(visibleEntries[0].target.id);
          }
        },
        { rootMargin: "-100px 0px -66% 0px" }
      );

      elements.forEach((elem) => observer.observe(elem));

      return () => observer.disconnect();
    }, 100);

    return () => clearTimeout(timeoutId);
  }, []);

  if (items.length === 0) return null;

  return (
    <aside className="hidden xl:block w-64 shrink-0 pr-8 py-10">
      <div className="sticky top-24">
        <div className="flex items-center gap-2 text-sm font-semibold text-neutral-900 dark:text-neutral-100 mb-4">
          <svg
            className="w-4 h-4 text-neutral-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M4 6h16M4 12h16M4 18h7"
            />
          </svg>
          On this page
        </div>
        <ul className="relative border-l border-neutral-200 dark:border-neutral-800 text-[13px] max-h-[calc(100vh-10rem)] overflow-y-auto pr-2 pb-10">
          {items.map((item) => (
            <li key={item.id} className="relative">
              <a
                href={`#${item.id}`}
                className={`block py-1.5 -ml-[1px] border-l-2 transition-colors duration-200 leading-snug ${
                  activeId === item.id
                    ? "border-blue-500 text-blue-600 dark:text-blue-400 font-medium"
                    : "border-transparent text-neutral-500 dark:text-neutral-400 hover:text-black dark:hover:text-white"
                } ${item.level === 3 ? "pl-8" : "pl-4"}`}
              >
                {item.text}
              </a>
            </li>
          ))}
        </ul>
      </div>
    </aside>
  );
}
