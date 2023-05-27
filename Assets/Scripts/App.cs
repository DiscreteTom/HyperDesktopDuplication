using UnityEngine;

namespace HyperDesktopDuplication {
  public class App : MonoBehaviour {
    async void Start() {
      var manager = this.GetComponent<HDD_Manager>();
      await manager.Refresh();

      var primaryCenter = Vector3.zero;
      for (int i = 0; i < manager.Monitors.Count; ++i) {
        var info = manager.Monitors[i];
        if (info.IsPrimary) {
          primaryCenter = new Vector3((info.Right - info.Left) / 2 + info.Left, (info.Top - info.Bottom) / 2 + info.Bottom, 0) / 100;
          break;
        }
      }

      for (int i = 0; i < manager.Monitors.Count; ++i) {
        var info = manager.Monitors[i];
        print($"display {i}: left={info.Left}, right={info.Right}, top={info.Top}, bottom={info.Bottom}");
        var obj = manager.CreateMonitor(i);
        obj.transform.localPosition = new Vector3((info.Right - info.Left) / 2 + info.Left, (info.Top - info.Bottom) / 2 + info.Bottom, 0) / 100 - primaryCenter;
      }
    }
  }
}
