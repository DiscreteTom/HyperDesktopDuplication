using UnityEngine;

namespace HyperDesktopDuplication {
  public class Example : MonoBehaviour {
    const float scale = 100;

    async void Start() {
      var manager = this.GetComponent<HDD_Manager>();
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
        print($"display {i}: left={info.Left}, right={info.Right}, top={info.Top}, bottom={info.Bottom}");
        var obj = manager.CreateMonitor(i);
        obj.transform.localScale = new Vector3(1 / scale, 1 / scale, 1);
        obj.transform.localPosition = new Vector3((info.Right - info.Left) / 2 + info.Left, (info.Top - info.Bottom) / 2 + info.Bottom, 0) / scale - primaryCenter;
      }
    }
  }
}
