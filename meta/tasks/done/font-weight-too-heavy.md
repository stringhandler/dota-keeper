# Bug: Fonts are too heavy/dark, hard to read

## Symptom

Text throughout the app (especially on the Analysis page) feels too bold/heavy and is hard to read.

## Solution

Lighten the font weight globally, or at minimum on the Analysis page. Likely a CSS `font-weight` value that needs to drop one step (e.g. 500 → 400, or 700 → 500).
