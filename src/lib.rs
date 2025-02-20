//! 💡 **Did You Know?** You can generate your own, offline version of this document by running this
//! command in your own clone of the repository.
//!
//! ```bash
//! cargo doc --lib --no-deps --open
//! ```
//!
//! **Other Tutorial References**
//!
//! - [Git repository for Rusty Sword Arena](https://github.com/CleanCut/rusty_sword_arena).
//! - [How To Learn Rust](https://github.com/CleanCut/rusty_sword_arena/blob/master/HowToLearnRust.md) - a
//!   handy checklist of things to do.
//!
//!
//!
//! # Rusty Sword Arena - Game Design
//!
//! Rusty Sword Arena is a networked, 2D, top-down, arena combat game.  A server implementation and
//! shared library is provided.  Your task is to implement a game client in Rust using the provided
//! shared library and the game design details below.
//!
//! Note that while significant effort has been put into making the server _robust_ and unlikely to
//! crash, there has consciously been very little effort towards security, or pretty much any proper
//! game engine architecture.  In other words, this is a great way to learn Rust and have some fun
//! at the same time, but this in no way pretends to be a substitute for a _real game engine™_ like
//! [amethyst](https://amethyst.rs/) or [ggez](https://ggez.rs/). 😉
//!
//! ## Basic Gameplay
//!
//! The basic idea of the game is you are presented with the top-down view of a person (circle) in
//! an arena (your window) who can swing his weapon (a rusty sword) in a circle.  You can face any
//! direction you like, turning is instant as far as the server is concerned.  You can move in any
//! direction (which is _not_ instant, the server implements some rudimentary movement physics) and
//! attempt to attack other players with your sword.  If another player is within the radius of
//! your sword reach when you attack, they will be hurt and their health will go down.  You will be
//! limited to 50% movement speed while attacking, so try to time your attack to when they will be
//! successful.
//!
//! You start with some health. When you are hit, you lose health.  When the health runs out, you
//! die and the person who killed you gets a point.  After a respawn delay, you respawn with full
//! health.  If you attempt to run from the arena, you are likely to be eaten by a grue.  Being
//! eaten by a grue causes you to lose a point, but there is no point penalty when another player
//! kills you.
//!
//! ## Preparation
//!
//! First, you should follow the
//! [instructions on the readme](https://github.com/cleancut/rusty_sword_arena#preparation---do-this-before-oscon)
//! to install some dependencies.  Then come back here and keep going.
//!
//! ## Creating Your Game Client
//!
//! These are all things we will do together in the tutorial.  Feel free to get ahead of the group
//! if you can!
//!
//! - Pick a name for your client and create a Rust crate.  Hint: use `cargo`
//! - Obtain the player's desired `name` and the `host` to connect to.
//!   - One easy way is to get the name and host from the command-line.
//!   - See [args](https://doc.rust-lang.org/std/env/fn.args.html) for the code part.
//! - Add `rusty_sword_arena` as a dependency in your `Cargo.toml` file.
//! - Create a `ConnectionToServer` using the `host`
//! - Use the `ConnectionToServer` to join the game.
//!   - If the join fails, print out the error message and quit the program.
//!   - Keep the player id that is returned so you can tell which player you are.
//! - Use the `ConnectionToServer` to get a `GameSetting`.  If `GameSettings.version` does not match
//!   the `rusty_sword_arena::VERSION` you are using, then you are using a different version of
//!   rusty_sword_arena than the server, and you should abort the game and update your `Cargo.toml`
//!   to use the same version.
//! - Use the `Audio` struct to play sound effects.  You can use these free placeholder sounds
//!   either [individually](https://github.com/CleanCut/rusty_sword_arena/tree/master/media)
//!   or [zipped up](https://agileperception.com/static/media.zip)
//!   if you like, or [record](https://www.audacityteam.org/) or [create](https://www.bfxr.net/)
//!   your own sounds!  Files can be MP3, WAV, Vorbis or Flac. (Or you can skip sounds altogether,
//!   really).
//! - Create a `Window`
//! - Create a `PlayerInput` to keep track of player input.
//! - IN YOUR MAIN GAME LOOP...
//!     - Gather `GameEvent`s from the `Window`, handle all the possible events, including updating
//!       your `PlayerInput` based on what you see.
//!     - Get all the pending `GameState`s from the server.
//!         - FOR EACH GAME STATE (which represents the state of one frame)
//!         - Process all the `PlayerState`s into some local collection(s) that represent the
//!           client's current view of players.
//!     - For each `PlayerState`, process any new `PlayerEvent`s to play sounds or update
//!     - Loop through your local state storage and draw a frame that represents the latest state
//!       of the players.
//! ...
//!
//! ## Challenges!
//!
//! Here are some things we will NOT do together in the tutorial.  If you are ahead of the class, or
//! want to keep going after the class, here's some challenges you could take on!
//!
//! - Every GameState includes a `HighScores` struct. Why not do something with it?  You could just
//!   print it to the console every once-in-awhile, or do something more interesting.
//! - Your player might sometimes appear underneath other players if they overlap. Make your player
//!   always render on top.
//! - Which player is yours!? Add some visual indicator so you can easily tell your player apart.
//! - **Multiple players from one client** - The server and networking protocol do not prevent a
//!   single client from adding multiple players to the game. Create some way to divide the
//!   keyboard/mouse input up among two or more local players who will play through the same client.
//! - **AI** - Who says a human has to do the playing?  The server is giving you all the information
//!   about all the players's states.  Use that information to develop an AI player who plays by
//!   himself.
//! - **RTS** - Who says you have to control everything little movement?  Combine one or both of the
//!   above challenges to implement a real-time strategy interface, where you direct one (or more)
//!   players   controlled by your client to achieve a goal (move somewhere, attack things, run
//!   away, etc) and then your controlled players autonomously attempt to achieve that goal until
//!   you specify otherwise.
//! - **Better Graphics** - Improve on the graphics.  Either make better use of the provided
//!   shapes to indicate more of what is going on, or dig into the RSA `gfx.rs` module and augment
//!   it with new and better graphical capabilities.  Health indicators sure would be nice.
//! - **Better Sound Effects** - Make some decent sound effects of your own!
//! - **Improve Rusty Sword Arena Itself** -- Fork
//!   [Rusty Sword Arena](https://github.com/CleanCut/rusty_sword_arena).  Add features, or fix
//!   bugs in the server.
//!   - Improve the reference client implementation.
//!   - Port the graphics subsystem from OpenGL to Vulkan.
//!   - Add more graphics primitives (rectangles for healthbars, for example).
//!   - Add the ability to render text.
//!   - Port the network subsystem from ZeroMQ to [nanomsg](https://nanomsg.org/).
//!   - Upgrade the audio subsystem. Add a way to play looping music alongside the sound effects,
//!   or to adjust the volume.  Read up on [rodio](https://github.com/tomaka/rodio), the low-level
//!   library we use to process and play audio.
//!   - Update the documentation to be clearer, more comprehensive, and have more useful links.
//!   - Add support for Game Modes (Teams? Capture the flag?).
//!   - Add multiple weapon types and random weapon drops to pick up.
//!   - Add the ability to Parry (you can try to attack OR parry - a successful parry causes the
//!     attacking player to have a much larger delay than usual before the next attack attempt).
//!   - Add player collision.
//!   - [Create an issue](https://github.com/CleanCut/rusty_sword_arena/issues/new) if you
//!     have ideas you want to discuss, or need help.
//!   - [Create a Pull Request](https://github.com/CleanCut/rusty_sword_arena/compare) if you would
//!     like feedback on your code, or if think your change is ready to contribute back to the main
//!     project.

#![doc(html_favicon_url = "https://agileperception.com/static/img/favicon.ico")]
#![doc(html_logo_url = "https://agileperception.com/static/img/APSwirl200.png")]

/// A module for reading and playing audio files
pub mod audio;
/// Everything in the game module is shared by the server _and_ the client
pub mod game;
/// The graphics module that will be used by your client
pub mod gfx;
/// The networking module that will be used by your client
pub mod net;
/// A timer module for general use
pub mod timer;

/// The current version number. Your client should check this against the version the server sends
/// in `GameSettings`
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
