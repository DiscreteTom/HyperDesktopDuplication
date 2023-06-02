# HyperDesktopDuplication

![GitHub release (latest by date)](https://img.shields.io/github/v/release/DiscreteTom/HyperDesktopDuplication?style=flat-square)
![Platform support](https://img.shields.io/badge/platform-windows-blue?style=flat-square)
![Built for Unity3D](https://img.shields.io/badge/Built%20for-Unity3D-lightgrey?style=flat-square)
![license](https://img.shields.io/github/license/DiscreteTom/HyperDesktopDuplication?style=flat-square)

HyperDesktopDuplication is an Unity asset to use the realtime screen capture as `Texture2D` using Windows Desktop Duplication API, shared memory, gRPC and Rust.

## Why this Project?

This project is based on [uDesktopDuplication](https://github.com/hecomi/uDesktopDuplication), which is a great project to capture screen to Unity3D. However, it is not working with a standalone/discrete GPU. See https://github.com/hecomi/uDesktopDuplication/issues/30.

## Usage

First, start a [shremdup](https://github.com/DiscreteTom/shremdup) (v0.1.7+) server with administrator privilege (to use shared memory across processes).

Then, enable `Allow 'unsafe' Code` in the Player Settings, and copy all `Assets` in this repo into your Unity project.

Finally, add the `HDD_Manager` prefab to your scene, and write a driver script. Here is an example:

```cs
using HyperDesktopDuplication;
using UnityEngine;

public class Example : MonoBehaviour {
  const float scale = 100;

  async void Start() {
    // assume we have an HDD_Manager component on the same GameObject
    var manager = this.GetComponent<HDD_Manager>();
    // refresh monitor infos
    await manager.Refresh();

    // get the center position of the primary monitor
    var primaryCenter = Vector3.zero;
    for (int i = 0; i < manager.Monitors.Count; ++i) {
      var info = manager.Monitors[i];
      if (info.IsPrimary) {
        primaryCenter = new Vector3((info.Right - info.Left) / 2 + info.Left, (info.Top - info.Bottom) / 2 + info.Bottom, 0) / scale;
        break;
      }
    }

    // create all monitors
    for (int i = 0; i < manager.Monitors.Count; ++i) {
      var info = manager.Monitors[i];
      // HDD_Manager will use the HDD_Monitor prefab
      var obj = manager.CreateMonitor(i);
      obj.transform.localScale = new Vector3(1 / scale, 1 / scale, 1);
      // place the monitor according to the system settings
      obj.transform.localPosition = new Vector3((info.Right - info.Left) / 2 + info.Left, (info.Top - info.Bottom) / 2 + info.Bottom, 0) / scale - primaryCenter;

      // if you want to destroy a monitor, you can do this:
      // await obj.GetComponent<HDD_Monitor>().DestroyMonitor();
      // or just:
      // Destroy(obj);
    }

    // on destroy, HDD_Manager will destroy all monitors and close the gRPC channel
  }
}
```

## Adopted Optimizations

- Invisible screens won't be updated.
- Update the desktop image and the mouse cursor image separately to reduce the texture update frequency.

## [CHANGELOG](https://github.com/DiscreteTom/HyperDesktopDuplication/blob/main/CHANGELOG.md)
