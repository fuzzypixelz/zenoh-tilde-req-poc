# Zenoh "Tilde" version requirement (e.g. `~4.4`) proof of concept

From [Dependency Resolution](https://doc.rust-lang.org/cargo/reference/resolver.html):

> Avoid overly narrow version requirements if possible. For example, if you
> specify a tilde requirement like `bar="~1.3"`, and another package specifies a
> requirement of `bar="1.4"`, this will fail to resolve, even though minor
> releases should be compatible.

Running `cargo +nightly -Z bindeps tree` fails with:

```text
error: failed to select a version for `clap`.
    ... required by package `zenohd v0.11.0-dev (/path/to/zenoh-tilde-req-poc/zenoh/zenohd)`
    ... which satisfies path dependency `zenohd` (locked to 0.11.0-dev) of package `zenoh-tilde-req-poc v0.1.0 (/path/to/zenoh-tilde-req-poc)`
versions that meet the requirements `~4.4` are: 4.4.18, 4.4.17, 4.4.16, 4.4.15, 4.4.14, 4.4.13, 4.4.12, 4.4.11, 4.4.10, 4.4.9, 4.4.8, 4.4.7, 4.4.6, 4.4.5, 4.4.4, 4.4.3, 4.4.2, 4.4.1, 4.4.0

all possible versions conflict with previously selected packages.

  previously selected package `clap v4.5.1`
    ... which satisfies dependency `clap = "^4.5.1"` of package `zenoh-tilde-req-poc v0.1.0 (/path/to/zenoh-tilde-req-poc)`

failed to select a version for `clap` which could resolve this conflict
```

This is because zenohd's clap version is set to `~4.4`, while this crate
specifies `4.5.1`. This illustrates that pinning the minor (or patch) of
dependency version can easily cause the Cargo resolver to fail in user crates.
