# Releasing

Publishing runs from `release.yml`. It is driven by a tag and it authenticates
with a short-lived crates.io token obtained over OIDC. There is no
`CARGO_REGISTRY_TOKEN` secret in this repository.

## The version rule

The tag is `v` plus the version in `Cargo.toml`. Tag `v0.3.1` publishes the
crate only if `Cargo.toml` says `version = "0.3.1"`. The `version` job fails the
run otherwise, so a tag can never publish a version that differs from what is in
the tree.

Bump the version, commit, push, then tag that commit:

```sh
cargo publish --dry-run
git tag v0.3.1
git push origin main
git push origin v0.3.1
```

## First publish of a new crate

crates.io trusted publishing cannot create a brand-new crate. The OIDC exchange
needs a crate that exists and a trusted publisher configured on it, so a cold
start over OIDC is rejected by crates.io. Do not try to build one.

The first publish is manual, with an API token:

1. Create a crates.io API token scoped to `publish-new` and to the crate's
   publish scope if the crate already has a scope entry. Keep it out of the
   repository and out of shell history where possible.
2. From a clean checkout of the commit that will become the release, run
   `cargo publish` with the token supplied through the environment. Do not
   publish from a working tree with uncommitted changes.
3. Add the trusted publisher on crates.io, in the crate's settings:
   owner `neotales`, repository name, workflow filename `release.yml`.
   Leave the environment empty. The publish job does not set one. If you set
   an environment here, the job must set the same name, or crates.io rejects
   the token.
4. Revoke the API token once an OIDC release has succeeded.

The release run for that first version fails, because the version it would
publish is already uploaded. The tag is provenance for the first version, not
its publish trigger. Every later version publishes from its tag.

If the crate name is taken on crates.io by someone else, stop. A rename is a
different repository setup, not a flag.

`cargo package` includes every tracked file, so `.github/` and `RELEASE.md`
ship inside the published crate unless the manifest excludes them. That is
harmless. Exclude them if package size matters. The README style guide lives
at `~/work/rs/README.STYLE.md` and is not part of this crate.

## Later releases

1. Bump the version in `Cargo.toml` and commit it. `Cargo.lock` is committed on
   purpose, so run `cargo build` and commit the updated lockfile with it. The
   lockfile is what the scheduled audit checks.
2. Push the commit.
3. Tag it `vX.Y.Z` and push the tag.
4. The workflow runs the checks, then `cargo publish --dry-run`, then publishes
   with a token minted for that run.

A manual run, `workflow_dispatch`, runs the checks and the dry run only. It
never publishes, whatever branch it is started from.

After the token-based first release is done, switch the crate to Trusted
Publishing Only mode on crates.io. That rejects token publishes everywhere, so
a leaked token cannot publish this crate later.

## What is not allowed

- `pull_request_target` and `workflow_run` are not used anywhere, and must not
  be added. Both can run untrusted code with publishing credentials in scope.
- No long-lived crates.io token in GitHub secrets for this workflow. The job
  reads `id-token: write` and nothing else.
- No publishing from a branch. The tag check exists so the artifact has a name.
- No republishing a version. crates.io rejects it. If a publish fails after the
  upload was accepted, bump the patch version and release again.

## Failure modes

| Symptom | Cause |
| --- | --- |
| `version` job: tag does not match `vX.Y.Z` | `Cargo.toml` version and the tag disagree. Delete the tag, fix, re-tag. |
| Publish job: OIDC exchange fails | Crate does not exist yet, or the trusted publisher does not match this repository, `release.yml`, and an empty environment. |
| Publish job: version already uploaded | The version was published before, usually by the manual first release. Bump the version. |
| Publish job: `cargo publish` rejects the token | Trusted Publishing Only mode is on and the crate has no trusted publisher configured. |
