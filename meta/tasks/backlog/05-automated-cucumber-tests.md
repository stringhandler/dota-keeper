# Task: Automated Cucumber-Based Tests

## Goal

Implement automated end-to-end tests using Cucumber (or a compatible BDD framework) based on the use case specifications in `meta/spec/`.

## Background

Use case specifications have been written for all currently implemented features. These use Gherkin syntax (`Given`/`When`/`Then`) and are stored in `meta/spec/` organised by feature area.

The goal is to convert these into runnable automated tests that verify the application behaves as specified.

## Scope

All use cases in `meta/spec/` should eventually be covered:

| Area         | Use Cases           |
|--------------|---------------------|
| Auth         | UC-001 to UC-003    |
| Matches      | UC-010 to UC-015    |
| Goals        | UC-020 to UC-025    |
| Dashboard    | UC-030 to UC-032    |
| Analysis     | UC-040 to UC-042    |
| Challenges   | UC-050 to UC-055    |
| Settings     | UC-060 to UC-063    |

## Considerations

- **Framework**: Cucumber.js is the preferred framework given the existing JavaScript/Svelte frontend. Alternatively, WebdriverIO or Playwright with Cucumber integration could be used.
- **Tauri testing**: Tauri supports WebDriver via `tauri-driver`. This would allow full end-to-end testing of the desktop app, including backend commands.
- **Mocking**: OpenDota API calls should be mocked/stubbed in tests to avoid real network dependency.
- **Test data**: A seeded in-memory or test SQLite database should be used to provide consistent test data.
- **CI**: Tests should be runnable in a CI pipeline (GitHub Actions already exists for this project).

## References

- Use case specs: `meta/spec/README.md`
- Tauri WebDriver docs: https://tauri.app/develop/tests/webdriver/
- Cucumber.js: https://cucumber.io/docs/installation/javascript/

## Acceptance Criteria

- [ ] Test framework is configured and integrated with the existing build system
- [ ] At least the Auth and Goals use cases have runnable Cucumber feature files
- [ ] Tests can be run with a single command (e.g., `npm test` or `bun test`)
- [ ] Tests run in CI via GitHub Actions
- [ ] Mocking strategy for OpenDota API is documented
