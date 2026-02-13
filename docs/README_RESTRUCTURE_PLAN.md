# README Restructure Plan

## Goal

Make the root `README.md` short, scannable, and task-oriented, then move deep content into focused docs so users can find the right path quickly.

## Current Problems

- Too many responsibilities in one file (project overview, provisioning tutorial, Home Assistant setup, API usage, device matrix, references, legal notes).
- Setup section is very long and mixes provisioning details with server install and Home Assistant paths.
- Heading style is inconsistent (`### 6b) ...`, all-caps section names, mixed imperative tone).
- Some wording and step numbering are ambiguous (for example “created in Step 3” inside Step 3 context).
- The root doc is hard to scan for users with different goals:
  - “I want Home Assistant add-on”
  - “I want standalone server”
  - “I want protocol details / DOP2 / remote start”

## Target Information Architecture

### Keep in root `README.md`

- Project summary (what it is, what it is not).
- Quickstart decision tree:
  - Home Assistant add-on path
  - Standalone/local server path
- Minimal install/run links:
  - “Standalone setup guide”
  - “Home Assistant add-on guide”
  - “API usage”
  - “Compatibility”
- High-level disclaimer and license pointer.

### Move to dedicated docs

- `docs/standalone-setup.md`
  - Device reset
  - WiFi onboarding
  - Key provisioning
  - Server config
  - Install and test
- `docs/home-assistant.md`
  - Add-on repo setup
  - Add-on options schema examples
  - Legacy file compatibility mode
  - Troubleshooting add-on build/runtime issues
- `docs/api-usage.md`
  - Endpoint overview
  - Querying and DOP2 operations
  - Remote start behavior and caveats
- `docs/compatibility.md`
  - Device compatibility table
  - Notes on confidence levels / report quality
- `docs/references.md`
  - External links and community threads
- `docs/disclaimer.md`
  - Full legal/risk wording

## Proposed New Root README Outline

1. `# Miele REST Server`
2. One-paragraph summary
3. “Choose your setup” section with two paths
4. Quick links to main docs
5. Minimal API example (single request)
6. Compatibility link
7. License/disclaimer short notice

## Content Split Mapping (Old -> New)

- `## Scope` -> root README summary.
- `## Setup` and substeps -> `docs/standalone-setup.md`.
- `### 6) Optional -- Home Assistant integration` -> `docs/home-assistant.md`.
- `### 6b) Optional -- Home Assistant add-on repository` -> `docs/home-assistant.md`.
- `## QUERYING AND SETTING DEVICE INFORMATION` -> `docs/api-usage.md`.
- `## USING REMOTE START` -> `docs/api-usage.md`.
- `## TODO` -> move to GitHub Issues (or keep tiny note at root linking to issues).
- `## COMPATIBILITY` -> `docs/compatibility.md`.
- `## FURTHER READING` -> `docs/references.md`.
- `## LICENSE AND DISCLAIMER` -> short in root + full text in `docs/disclaimer.md`.

## Style and Formatting Rules

- Use sentence-case headings.
- Keep section titles action-oriented (for example “Set up with Home Assistant”).
- Keep root README under ~150 lines.
- Use consistent code block language tags.
- Prefer short paragraphs and numbered procedures for workflows.
- Remove all-caps headings.
- Replace ambiguous terms with explicit paths/commands.
- Keep one canonical config example per flow and link to others.

## Migration Steps

1. Create the new docs files listed above.
2. Move existing content with minimal wording changes first.
3. Rewrite root `README.md` to the new concise structure.
4. Normalize heading style and cross-links.
5. Validate all file links and command snippets.
6. Remove duplicated explanations between root and docs.
7. Add “last validated” notes where behavior changes often (especially Home Assistant add-on flow).

## Acceptance Criteria

- Root README can be read in under 3 minutes.
- A new user can pick setup path in under 30 seconds.
- Every deep topic has exactly one primary document.
- No broken internal links.
- No duplicated full procedures between root and sub-docs.

## Follow-up (Optional)

- Add a docs index page (`docs/README.md`) to make navigation easier.
- Add screenshots for Home Assistant add-on setup.
- Add a troubleshooting matrix for common add-on build errors.
