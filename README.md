# HyperDesktopDuplication

HyperDesktopDuplication is an Unity asset to use the realtime screen capture as `Texture2D` using Windows Desktop Duplication API, shared memory, gRPC and Rust.

## Why this Project?

This project is based on [uDesktopDuplication](https://github.com/hecomi/uDesktopDuplication), which is a great project to capture screen to Unity3D. However, it is not working with a standalone/discrete GPU. See https://github.com/hecomi/uDesktopDuplication/issues/30.

## Usage

First, start a [shremdup](https://github.com/DiscreteTom/shremdup) server with administrator privilege (to use shared memory across processes).

Then, copy all assets in this project into your Unity project.

Finally, add the `HDD_Manager` prefab to your scene, and write a driver script. You can find the example in [`App.cs`](https://github.com/DiscreteTom/HyperDesktopDuplication/blob/main/Assets/Scripts/App.cs).
