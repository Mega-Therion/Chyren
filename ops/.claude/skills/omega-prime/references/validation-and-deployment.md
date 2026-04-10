# Validation and Deployment

## Validation Order

1. Build integrity
2. Lint and static checks
3. Unit and integration tests
4. Eval or regression suites
5. Live smoke test
6. Deployment script generation only after the above pass

## Finalization Pattern

- Use `repo-finalize` logic for broad repository checks.
- Use `one-block-builder` logic when a single reproducible command is needed for setup, validation, recovery, or deploy.
- Keep deployment scripts idempotent.
- Prefer exact commands with explicit paths and environment variables.

## Acceptance Threshold

- Treat partial success as insufficient unless the objective explicitly allows it.
- Treat a silent fallback as a failure until the observable output proves otherwise.
- Treat live deployment as unverified until smoke tests pass against the deployed URL.

