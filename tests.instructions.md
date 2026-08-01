---
applyTo: "tests/**/*.rs"
---

# Test Policy: Classical TDD (Kent Beck Style)

This instruction set defines how tests under tests/ must be designed and reviewed.
Focus on Classical TDD for system unit tests, not mockist/London style.

## 1) Core stance

- Treat the SUT as a behavior unit and verify outcomes from public behavior.
- Prefer real collaborators and in-memory data over mocks/stubs whenever practical.
- Use test doubles only when a dependency is slow, non-deterministic, external, or hard to construct.
- Do not verify internal call choreography as the primary assertion style.

## 2) Red-Green-Refactor workflow

- Start from a failing test that states one concrete behavior.
- Implement the smallest production change to make it pass.
- Refactor test and production code while keeping all tests green.
- Keep cycles short; one behavior per iteration.

## 3) Test scope and granularity

- Unit tests should exercise a coherent behavior slice, not private helper details.
- Favor state-based assertions (returned value, state transition, emitted result).
- Interaction assertions are allowed only when behavior cannot be observed through state.
- Keep fixture setup minimal and explicit.

## 4) Quality bar for system unit tests

- Deterministic: no random/flaky behavior.
- Isolated: no shared mutable global state across tests.
- Fast: run locally in seconds.
- Readable: Arrange-Act-Assert structure is obvious.
- Intentional naming: test names express expected behavior.

## 5) Practical guidance for this repository

- Prefer pure domain logic tests in tests/ over UI-render-detail checks.
- When logic can be tested without browser/runtime wiring, test the logic directly.
- Keep assertions focused on externally visible scoring/selection outcomes.
- Add regression tests for every bug fix before changing production code.

## 6) Anti-patterns to avoid

- Over-mocking internal collaborators just to assert method-call order.
- Asserting implementation details that block refactoring.
- Large end-to-end style tests for behavior that can be covered by focused unit tests.
- Multiple unrelated expectations in a single test case.
