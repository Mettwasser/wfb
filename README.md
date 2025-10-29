<!-- Backend Resources -->

[socketioxide]: https://docs.rs/socketioxide/0.18.0/socketioxide/index.html
[axum]: https://docs.rs/axum/0.8.6/axum/index.html

<!-- Frontend Resources -->

[socket.io-client]: https://www.npmjs.com/package/socket.io-client
[SvelteKit]: https://svelte.dev/docs/kit/introduction
[Skeleton UI]: https://v3.skeleton.dev/

<!-- Markdown -->

# wfb (WIP)

`W`ar`f`rame `B`ingo.

A special type of bingo where users are provided `cards` by the host, which are used for crafting your own Bingo-Board!

The host can then choose cards based on in-game events. First player to get 5 cards in a row right wins.

# Tech Stack

| Comment   | Backend        | Frontend                    |
| --------- | -------------- | --------------------------- |
| Socket.IO | [socketioxide] | [socket.io-client]          |
| Framework | [axum]         | [SvelteKit] + [Skeleton UI] |

\+ utility crates/packages :)

# Running locally

## Prerequisites

- rust (cargo)
- npm

## Backend

```
cargo run
```

## Frontend

```
npm run dev
```
