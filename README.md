# bevy_mod_speedup

![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=flat-square)
![Crates.io Version](https://img.shields.io/crates/v/bevy_mod_speedup.svg?style=flat-square)
![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?style=flat-square)

bevy_mod_speedup is a collection of ways to increase the performance of your application.

> [!WARNING]  
> This plugin is in a very early stage of development.  
> Expect major changes to the contained features and to the api of existing features.

## Description

> [!NOTE]  
> This repository is open for additional features and the expansion of existing features to other platforms.  
> It is currently limited to the features I use in my personal project and the platforms I have available.

bevy_mod_speedup contains a collection of techniques to make your game (or application) more performant.
This is mostly achieved by interacting with the system the game runs on.
One example is adjusting the energy saving state of the system.
The availability of the provided techniques is dependent on the used platform.

This plugin also offers some `Resources` to check the status of the "speedup".
This allows the application to react to the applied changes or a failure to adjust something.
See this [chapter](#status-resources) for more information.

> [!WARNING]  
> The techniques implemented here are not guaranteed to always improve performance.  
> As such it is recommended to check all used features on their supported platforms.  
> It is also recommended to ensure the application is not dependent on the improvements provided here as this does not replace optimized code.

This repository also includes other tips to improve the performance of your application.
For the other tips look [here](#other-performance-tips).
These tips can not be implemented in a library and need to be done directly in your project.

### Status resources

The resource `SpeedupAdjustments` can be used to check the status of the adjustments.
The features each add a field to the resource that matches the feature name.
These fields are enums with variants describing the current status.

## Features

To be as modular as possible this library has most of its functionality gated behind separate features.  
Most of those features are enable by default via the on-by-default feature `full_speed`.  
You can see the availability of features and their inclusion in `full_speed` in this [table](#platform-dependent-features).

### Platform independent features

The features described here are independent of the used platform.

#### Feature: `egui`

Adds a convenience method to display the `SpeedupAdjustments` with egui.
See the `egui` example for usage.

Run the example with `cargo run --example egui --features="egui"`.

### Platform dependent features

Feature support on different platform:

|                        | in `full_speed` |       Linux        |      Windows       |
| :--------------------- | :-------------: | :----------------: | :----------------: |
| `exclusive_fullscreen` |       no        |  :grey_question:   | :white_check_mark: |
| `power`                |       yes       |  :grey_question:   | :white_check_mark: |
| `priority`             |       yes       | :white_check_mark: | :white_check_mark: |
| `request_fast_gpu`     |       yes       |  :grey_question:   | :white_check_mark: |
| `unattended`           |       no        |  :grey_question:   | :white_check_mark: |

__Legend:__  
:grey_question: = To be evaluated  
:white_square_button: = Not yet implemented  
:white_check_mark: = Implemented  
:negative_squared_cross_mark: = Feature not supported on this platform

> [!NOTE]  
> Features enabled on a platform they do not support do nothing[^1].

[^1]: Beside a log entry in some cases.

#### Feature: `exclusive_fullscreen`

This features tries to fulfill all the conditions for fullscreen exclusivity.
A application that is fullscreen exclusive can bypass the compositor and render slightly faster.

This feature is not enough for fullscreen exclusivity by itself.
It requires further platform dependent adjustments to the application.

TODO: Add information about what is needed to achieve fullscreen exclusivity and how to check that it works.

* __Windows:__ Adjusts the DPI awareness[^2]

[^2]: Can alternatively be set via "Application Manifests".

#### Feature: `power`

Adjusts system settings to disable power saving and/or use high performance mode.

* __Windows:__ Sets the power scheme to `GUID_MIN_POWER_SAVINGS`, restores the original plan on `AppExit`.

#### Feature: `priority`

This tries to adjust the priority of the application.

* __Linux:__ Adjust the priority to -5. Only works with privileges
* __Windows:__ Adjust the priority class to `ABOVE_NORMAL_PRIORITY_CLASS`

#### Feature: `request_fast_gpu`

In hardware setups with integrated and dedicated GPUs it can be necessary to declare that the application wants to use the dedicated GPU.

#### Windows

This is on windows achieved by declaring special exports in the application.
As this can not be done from a library you have to add additional code to your application to make use of this feature.
The following two steps are needed:

* Call the macro `request_fast_gpu!()` in your application
* Add the following code in your `build.rs` to export the added variables.

  ```rust
  println!("cargo:rustc-link-arg=/EXPORT:NvOptimusEnablement");
  println!("cargo:rustc-link-arg=/EXPORT:AmdPowerXpressRequestHighPerformance");
  ```

You can check for the exports with `dumpbin /exports $APPLICATION.exe`.

#### Feature: `unattended`

This feature allows the application to run unattended without limitations.
This means disabling screen-saver, hibernation and similar features.

* __Windows:__ Uses `SetThreadExecutionState`, with `ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED`.

## Installation

Include the library in your project by adding it to your `Cargo.toml`.

```toml
[dependencies]
bevy = "0.13.0"
bevy_mod_speedup = "0.1.0"
```

Then add the `SpeedupPlugin` to your app like shown below.

```rust
use bevy::prelude::*;
use bevy_mod_speedup::SpeedupPlugin;

fn main(){
  App::new()
    .add_plugins(SpeedupPlugin)
    .run();
}
```

## Other performance tips

TODO

## Further reading / references

* <https://anthropicstudios.com/2022/01/13/asking-windows-nicely/>

## Contributing

Contributions are welcome.
For larger changes please open a issue first.

### Your contributions

Unless explicitly stated otherwise, any contribution submitted to this project shall be dual licensed under the [MIT License](LICENSE-MIT) and [Apache License, Version 2.0](LICENSE-APACHE), without any additional terms or conditions.

## License

All code in this repository is dual-licensed under either:

* [MIT License](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)

## Bevy compatibility

|   bevy | bevy_mod_speedup |
| -----: | ---------------: |
| 0.13.1 |    0.1.0 - 0.2.0 |
