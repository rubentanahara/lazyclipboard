# Critique: 05 Screen · Main Window · Search

**Screen:** the main window with a query ("spacing") applied. Job: find items across groups and act on one. Seen: header reads "Search results, 3 matches across 3 groups", the search field, results with group tags and ages, the sidebar with no group selected, the detail pane still populated.

## Walkthrough

Turning the header into a results summary tells the user the view changed (heuristic 1 holds), and disabling the group's rename and delete buttons in this mode avoids acting on the wrong thing. Group tags in each row answer where each result lives.

**Important (UX):** there is no way out of results except deleting the query. Add a clear (`×`) control in the field and an `esc` behaviour, and a "Back to Design refs" link in the header so the user can return to where they were.

**Important (UX):** the sidebar shows every group unselected, so the user loses their place. Highlight all matching groups or show match counts next to each group (3 in "Design refs", 1 in "Links to read").

**Important (UI):** the search field is 220px and sits beside the title; a search-driven view deserves the wide field. Give the field the full header width in this mode and move the title below it.

**Important (UX):** matched text is not highlighted in the rows or the preview.

**Important (UI, accessibility):** selected result is tint-only.

**Nit (UI):** the detail pane keeps its "Copy / Delete" buttons; keep, but show which group the selected result belongs to as a link ("Design refs →"), since the pane no longer implies it.

## Start here
1. Clear control and a way back.
2. Show match counts in the sidebar.
3. Highlight the matched term.
