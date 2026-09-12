import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  output: "static",
  integrations: [
    starlight({
      title: "Khaos",
      description: "Architecture and build notes for the Khaos entropy workspace.",
      customCss: ["./src/styles/custom.css"],
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/theobiabo/khaos" }
      ],
      sidebar: [
        {
          label: "Documentation",
          items: [{ autogenerate: { directory: "." } }]
        }
      ]
    })
  ]
});
