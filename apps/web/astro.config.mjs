import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  output: "static",
  markdown: {
    shikiConfig: {
      theme: "github-dark-default",
      wrap: true
    }
  },
  integrations: [
    starlight({
      title: "khaos",
      description: "Learn how physical noise becomes cryptographic entropy.",
      customCss: ["./src/styles/custom.css"],
      social: [{ icon: "github", label: "GitHub", href: "https://github.com/theobiabo/khaos" }],
      sidebar: [
        {
          label: "Overview",
          items: [
            { label: "Introduction", slug: "" },
            { label: "Getting started", slug: "getting_started" },
            { label: "How it works", slug: "architecture" }
          ]
        },
        {
          label: "Use khaos",
          items: [
            { label: "TypeScript", slug: "node_bindings" },
            { label: "Rust", slug: "rust_usage" }
          ]
        },
        {
          label: "Reference",
          items: [
            { label: "API reference", slug: "api_reference" },
            { label: "Security model", slug: "security" },
            { label: "Development", slug: "development" }
          ]
        }
      ]
    })
  ]
});
