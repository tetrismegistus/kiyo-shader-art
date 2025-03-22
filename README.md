# Kiyo template

This repository acts as a template to easily get started using [Kiyo](https://github.com/angelocarly/kiyo).  
It is a Rust project setup with Kiyo as a dependency, and contains the necessary code to run a simple shader.

Use the top-right button "Use this template" to get started.

Once cloned you can run kiyo using `cargo run`. Make sure to have the [Vulkan SDK](https://vulkan.lunarg.com) installed.

## Versions
You can set the version of Kiyo by editing `Cargo.toml` and specifying a specific release version published on [crates.io](https://crates.io/crates/kiyo/).
```toml
kiyo = "0.0.6"
```

Or use the latest git commit:
```toml
kiyo = { git = "https://github.com/angelocarly/kiyo" }
```
If you want to pull the latest git version, you would use `cargo update`

Or use a specific git commit:
```toml
kiyo = { git = "https://github.com/angelocarly/kiyo", rev = "8eb08954530a9a947b644828062d7d03a10218b" }
```

Please note that updating your kiyo version might cause some code to break. Refer back to the original template repository to always get a working example.

## Extra notes
I recommend to rename your project and rewrite this `README.md`! Describing your intentions not only makes it clear for others to understand your project, but also for yourself. 
