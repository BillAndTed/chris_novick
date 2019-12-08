# DSS Exercise #1
I enjoyed this exercise quite a lot.  It was an opportunity to expose myself to OpenGL development in Rust and reacquaint myself with OpenGL development generally.  While I wasn't able to include all of the features I had planned, I'm (mostly) happy with the result.

## Build Instructions
1. Download and install [Rust](https://www.rust-lang.org/tools/install) _or_ update to the latest
```
rustup update
```

2. Clone the repository and run
```
git clone https://github.com/TheRealBluesun/disney_interview1
cd disney_interview1
cargo run --release
```

## Known Issues
1. All HTTP requests are synchronous, causing UI delay when loading resources.  
This is a big one.  This causes a black window on startup for some time, and a noticeable delay when changing dates.  This is most noticeable the first time a date is selected, and the first time the program is run, as there are no cached images and all must be downloaded.
2. Fullscreen works in some environments and not others and is disabled for now


## Missing Features (ran out of time)
1. No loading window during synchronous fetching
2. Toggleable fullscreen (and fullscreen in all environments)
3. Using the Enter key to display details of the selected game
4. Displaying cached/preloading previous and next dates (decreased gamma image carousels above and below navigable center)
5. Querying for next available game when the selected day has none (I'm sure there's a way to do this)

## Screenshot
![Alt text](images/screenshot.png?raw=true "Screenshot")
