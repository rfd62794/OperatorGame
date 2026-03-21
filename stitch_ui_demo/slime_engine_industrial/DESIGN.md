# Design System Specification: Industrial Bioluminescence

## 1. Overview & Creative North Star
**Creative North Star: "The Synthetic Laboratory"**

This design system rejects the "flat" gaming trends of the last decade in favor of a tactical, high-fidelity industrial aesthetic. We are moving away from generic sci-fi into a space where heavy machinery meets volatile biology. The experience should feel like a high-contrast monitoring terminal—utilitarian, dense with data, yet punctuated by the vibrant, "leaking" neon colors of the slimes.

The system breaks the "template" look through **intentional density**. We do not fear a crowded UI; we embrace a "dashboard" feel where every pixel conveys status. We use monochromatic surfaces to ground the experience, allowing the "gooey" accent colors to act as vital status indicators that pop against the brutalist industrial backdrop.

---

## 2. Colors: Tonal Depth & Slime Accents
The palette is divided into "The Machine" (neutral surfaces) and "The Bio-Matter" (vibrant accents).

### The Machine (Neutrals)
- **Base Background:** `#0e0e13` (Surface). This is our void.
- **Surface Tiers:** Use `surface_container_low` (`#131318`) for large layout sections and `surface_container_highest` (`#25252c`) for interactive module headers.

### The Bio-Matter (Accents)
- **Primary (Growth/Success):** `primary` (`#69fea5`). Use for leveling up, positive growth, and primary actions.
- **Secondary (Warning/Heat):** `secondary` (`#ff8844`). Use for volatile states or energy consumption.
- **Tertiary (Fluid/Utility):** `tertiary` (`#6a9cff`). Use for hydration, cooling, or secondary systems.
- **Error (Destruction):** `error` (`#ff716c`). Use for critical failures or containment breaches.

### The "No-Line" Rule
Prohibit 1px solid borders for sectioning. Use background shifts to define boundaries. A `surface_container_high` module should sit on a `surface` background to create a "panel" effect. If a separation is needed within a panel, use a `1.5` (0.3rem) spacing gap to let the background bleed through as a "negative space divider."

### Signature Textures
Apply a subtle linear gradient to main CTAs transitioning from `primary` to `primary_container`. This mimics the internal glow of a liquid-filled vial rather than a flat plastic button.

---

## 3. Typography: Tactical Readability
Typography is the primary vehicle for the "Industrial" feel. We pair high-character display fonts with functional body text.

- **The Display Scale:** Use **Space Grotesk** for all headers (`headline-lg` to `headline-sm`). Its wide apertures and geometric construction feel like stenciled machinery labels.
- **The Data Scale:** Use **Inter** (or a system Monospace fallback) for all stats and numerical values. For `body-md` and `body-sm`, ensure a tracking of `0.05em` to improve legibility against dark backgrounds.
- **Visual Hierarchy:** Headers should be all-caps when used as section labels to reinforce the "Terminal" aesthetic. Data points (the "Stats") should use the `primary` or `secondary` color tokens to distinguish them from descriptive labels.

---

## 4. Elevation & Depth: Tonal Layering
We do not use drop shadows to indicate height; we use **Tonal Layering** to indicate "containment."

- **The Layering Principle:** 
    1. **Level 0 (Floor):** `surface` (`#0e0e13`).
    2. **Level 1 (Panel):** `surface_container_low` (`#131318`).
    3. **Level 2 (Active Module):** `surface_container_high` (`#1f1f26`).
- **Glassmorphism:** For floating HUD elements or pop-up modals, use `surface_container` at 80% opacity with a `12px` backdrop blur. This creates a "frosted laboratory glass" effect that keeps the player immersed in the garden behind the UI.
- **Ghost Borders:** If a high-density list requires containment, use the `outline_variant` at 15% opacity. This creates a "hairline" feel that defines the edge without cluttering the high-contrast environment.

---

## 5. Components: Industrial Primitives

### Buttons (Status-Based)
- **Primary (Success):** Background: `primary`. Text: `on_primary`. High-contrast, bold, all-caps.
- **Destructive:** Background: `error_container`. Text: `on_error_container`. Use for "Reset" or "Cull Slime" actions.
- **Warning:** Background: `secondary`. Text: `on_secondary`. Use for "Low Resources" or "Confirm Upgrade."
- **Shape:** Use the `sm` (0.125rem) or `none` roundedness scale. Rounded corners feel too "soft" for an industrial garden; sharp or nearly-sharp corners feel tactical.

### Data Chips
Use `surface_container_highest` for the chip background with a `tertiary` text color. These are for "Slime Types" or "Environmental Tags." No borders—only background color shifts.

### Input Fields
Industrial terminals use "Underline" or "Bracket" styles. Instead of a full box, use a bottom border of 2px `outline_variant`. When focused, transition the border color to `primary`.

### Cards & Progress Bars
- **Cards:** Forbid divider lines. Use `10` (2.25rem) spacing to separate card content.
- **Progress Bars (The "Slime Gauge"):** The track should be `surface_container_highest`. The fill should be a gradient of the specific slime color (e.g., `green` to `primary_container`). Use a "segmented" look (repeating transparent gaps) to make it look like a physical LED readout.

### Contextual Components
- **The "Bio-Monitor" List:** A high-density vertical list where each item uses a `surface_container_low` background. Use a 2px vertical "accent strip" on the left edge of the list item using the `primary` or `error` token to indicate health status.

---

## 6. Do's and Don'ts

### Do:
- **Do** embrace high-density layouts. The player should feel like a scientist managing a complex system.
- **Do** use `spaceGrotesk` in all-caps for labels like "PRESSURE," "VITALITY," or "SLIME DENSITY."
- **Do** use the `2` (0.4rem) spacing token for tight, data-heavy clusters.

### Don't:
- **Don't** use soft, large-radius corners (`xl` or `full`) unless it is for a literal slime icon. The UI should remain rigid.
- **Don't** use standard 1px gray borders. They look like "web templates." Use background tier shifts instead.
- **Don't** use "off-white" for text. Use `on_surface` (`#f8f5fd`) for maximum contrast against the deep black background. Readability is paramount in a stat-heavy game.