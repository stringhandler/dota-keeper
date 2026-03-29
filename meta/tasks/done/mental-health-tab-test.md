# Test Guide — Mental Health Tab

## Setup
- Ensure the app is built and running
- Enable Mental Health tracking: Settings → Mental Wellbeing → On

## Steps

### 1. Navigation
- Confirm "Mind" tab (heart icon) appears in the bottom nav bar
- Tap it — confirm you land on `/mental-health`

### 2. Disabled state
- Go to Settings, toggle tracking Off
- Go back to Mind tab
- Confirm you see the "Mood tracking is off" card with a Settings link

### 3. Assessment card (no data yet)
- Enable tracking, navigate to Mind
- If no check-ins yet: confirm "Complete more check-ins..." message with dot indicators

### 4. Assessment card (with data)
- Complete 3+ mood check-ins (play matches, dismiss or submit the check-in card)
- Revisit Mind tab
- Confirm energy and calm progress bars appear with 7-day averages
- Confirm trend arrow shows ↗ / → / ↘ depending on recent scores

### 5. Warning badge
- If tilt score > 55 (3+ loss streak + declining mood): confirm ⚠ badge appears

### 6. Suggestion card
- After a loss streak: confirm a suggestion card appears with appropriate text
- After 5+ games in a day: confirm fatigue suggestion with "Take a Break" button

### 7. Check-in history
- Confirm each completed check-in shows: date, hero icon, W/L pill, energy emoji, calm emoji
- Confirm skipped check-ins show "Skipped" label with reduced opacity

### 8. Empty history
- If no check-ins: confirm empty state message shown

### 9. Sidebar (desktop)
- On desktop: confirm "Mind" is also listed in the left sidebar nav (requires manual addition if sidebar is not yet updated — sidebar currently requires manual update to include /mental-health link)
