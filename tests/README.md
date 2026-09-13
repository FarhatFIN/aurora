# Test data inventory

The canonical inventory with sources is §8.9 of the master specification.
Directories are created by the milestone that first fills them (empty
directories are not committed):

| Directory | First filled |
|---|---|
| `golden/` | M2 (DOM dumps) |
| `layout/cases/` | M4 (fragment-tree geometry) |
| `pixel/` | M5 (golden PNGs) |
| `vendor/` | M2 (html5lib), M3 (css-parsing-tests), M9 (test262), M10 (WPT) |
| `corpus/` | M2 (html), M8 (images) |
| `security/` | M12 (adversarial page set) |
| `bench/pages/` | M13 (reference pages incl. showcase) |
| `fixtures/fonts/` | M7 (OFL reference fonts) |

Test tiers and harness rules live in Part 8 of the master specification.
