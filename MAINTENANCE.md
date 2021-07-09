# Making a new release

1. If you don't have cargo-release installed run `cargo install cargo-release`
1. To prevent breakage, one should release `spirv` before releasing a new `rspirv` because it has an obvious dependency.
1. For `rspirv` run `cargo release` in the `rspirv` directory, this should follow Semantic Versioning
