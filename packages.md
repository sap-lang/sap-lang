# packages

the packages ecosystem of sap-lang is divided into 4 parts: `prelude`, `batteries`, `defacto`, `community`.

how to choose the package?

1. check the `prelude` first, if the package is in the `prelude`, use it. scripting language should not waste time on the elementary building blocks.
2. if it is not in the `prelude` but you think it is quite elementary in your domain, check the `batteries`.
3. if it is not in the `batteries`, check the `defacto` which is widely used in the sap-lang community.
4. otherwise, search the package in the `community`, if still not found, you can contribute to the `community`, we will be very happy to see that.

## prelude
prelude is shipped with the language, it is the standard library of the language, it contains the most elementary building blocks of the language, and nearly all programs will use it.

like
- `java.lang` in java
- `prelude` in haskell
- `std` in rust

## battery
batteries are list of packages third party developers contributed elementary building blocks of some 
specific domain, but selected by the sap-lang team.

like
- `futures` in rust
- `lodash` in nodejs
- `numpy` in python

it is defaultly included when creating a new project.

you can config `batteries = false` in the `manifest.sap` to exclude the batteries.

## defacto
is the list of packages that are widely used in the sap-lang community, it is not shipped with the language, but selected by the community.

like
- `serde` `tokio` in rust
- `express` in nodejs

## community
is the list of packages that are contributed by the community, it is not shipped with the language, everyone can contribute to it.