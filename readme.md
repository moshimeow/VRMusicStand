## Template for a basic stereokit-rust program

### Download the source project then this template:
* git clone https://github.com/mvvvv/StereoKit-rust/
* git clone https://github.com/mvvvv/stereokit-template/

First, check that you can launch the Stereokit-rust demos as described here https://github.com/mvvvv/StereoKit-rust/blob/master/README.md

Then, go to the Stereokit-template project and transform it to your project :
- by renaming the name, package and labels in Cargo.toml, 
- and removing the .git in order to create yours,

### Run your project on your PC's headset :
* Make sure you have [OpenXR installed](https://www.khronos.org/openxr/) with an active runtine.
* Launch: `cargo run`

### Run your project on your PC using the [simulator](https://stereokit.net/Pages/Guides/Using-The-Simulator.html) 
* Launch: `cargo run  -- --test`

If you're using VsCode you'll see two launchers in launch.json to debug the project.


## Run the project on your Android headset:
* Launch: `cargo apk run`

## Build the release versions of your project:
* Desktop : `cargo build --release`
* Android : `cargo apk build --release`

Binaries and APK archives are produced under ./target/release

## Compile shaders
If you want to create your own shaders, you'll need the binary `compile_sks` of the stereokit-rust project and so you have to 'install' the project: 
* `cargo install --path <path to git directory of Stereokit-rust>`

`compile_sks` calls the stereokit binary `skshaderc` using the following configuration:
* The shaders (*.hlsl files) must be created inside the shaders_src directory inside the root directory of your project. 
* The result (*.hlsl.sks files) will be produced inside the assets/shaders directory inside the root directory of your project.

To compile the *.hlsl files, go to the root directory of your project then launch `cargo compile_sks`

## Troubleshooting
Submit bugs on the [Issues tab](https://github.com/mvvvv/StereoKit-rust/issues), and ask questions in the [Discussions tab](https://github.com/mvvvv/StereoKit-rust/discussions)!

The project <https://github.com/StereoKit/StereoKit/> will give you many useful links (Discord/Twitter/Blog)
