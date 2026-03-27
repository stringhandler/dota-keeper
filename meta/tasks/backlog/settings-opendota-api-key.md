# Settings: OpenDota API Key Input

## Description

Allow the user to enter and save their personal OpenDota API key in the settings page. OpenDota allows unauthenticated requests but rate-limits them — providing an API key raises the limit significantly.

## Acceptance Criteria

- A text input field is shown in the Settings page (API / Data section) for the OpenDota API key
- The key is saved to the app's settings (persisted in SQLite alongside other settings)
- Saved key is used as the `api_key` query parameter on all OpenDota API requests
- If no key is set, requests are made without the parameter (existing behaviour)
- The input should mask the key (password-style) with a show/hide toggle
- A "Save" action confirms the change with a toast notification
- A "Clear" option removes the stored key

## Notes

- OpenDota API key can be obtained from https://www.opendota.com/api-keys
- Key should be stored in the settings table, not hardcoded or in a plain config file
