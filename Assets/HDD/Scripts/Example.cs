using UnityEngine;

namespace HyperDesktopDuplication {
  public class Example : MonoBehaviour {
    public int frameRate = 120;
    const float scale = 100;

    async void Start() {
      // set the application's frame rate
      Application.targetFrameRate = this.frameRate;

      // assume we have an HDD_Manager component on the same GameObject
      var manager = this.GetComponent<HDD_Manager>();
      manager.frameRate = this.frameRate;
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
      }

      // on destroy, HDD_Manager will destroy all monitors and close the gRPC channel
    }
  }
}