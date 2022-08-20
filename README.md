# Forge

Template for a Game using the awesome [Bevy engine][bevy] featuring out of the box builds for Windows, Linux, macOS, and Web (Wasm).

# What does this template give you?
* small example ["game"](https://niklasei.github.io/bevy_game_template/) (*warning: biased; e.g., split into a lot of plugins and using `bevy_kira_audio` for sound*)
* easy setup for running the web build using [trunk] (`trunk serve`) 
* run the native version with `cargo run`
* workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow
    * WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes. You might want to consider running the workflow less often or removing some builds from it. **For public repositories the builds are free!**

### Deploy web build to GitHub pages
 1. Trigger the `deploy-github-page` workflow
 2. Activate [GitHub pages](https://pages.github.com/) for your repository
     1. Source from the `gh-pages` branch (created by the just executed action)
 3. After a few minutes your game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Credits

Other licenses and credits can be viewed in the [credits](./credits) directory.

[bevy]: https://bevyengine.org/
[bevy-learn]: https://bevyengine.org/learn/
[bevy-discord]: https://discord.gg/bevy
[nikl-twitter]: https://twitter.com/nikl_me
[firefox-sound-issue]: https://github.com/NiklasEi/bevy_kira_audio/issues/9
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[`wasm-server-runner`]: https://github.com/jakobhellermann/wasm-server-runner
[trunk]: https://trunkrs.dev/
