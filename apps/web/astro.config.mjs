import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  output: "static",
  integrations: [
    starlight({
      title: "khaos",
      description: "Learn how physical noise becomes cryptographic entropy.",
      customCss: ["./src/styles/custom.css"],
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/theobiabo/khaos" }
      ],
      sidebar: [
        {
          label: "Learn khaos",
          items: [{ autogenerate: { directory: "." } }]
        }
      ]
    })
  ]
});
