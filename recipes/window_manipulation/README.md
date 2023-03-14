# Window Manipulation

## Init Window (init_window.rs)

Window is defined as Entity from bevy 0.10.0. (check [here](https://bevyengine.org/learn/book/migration-guides/0.9-0.10/#windows-as-entities) for more details).<br />
So if you want to change its value, you need to query the window first.<br />
Also, [DefaultPlugin](https://docs.rs/bevy/latest/bevy/prelude/struct.DefaultPlugins.html) will spawn the primary window, so if you only use one window, you don't have to spawn windows.
