using UnityEngine;

namespace HyperDesktopDuplication {
  public class App : MonoBehaviour {
    async void Start() {
      var manager = this.GetComponent<HDD_Manager>();
      await manager.Refresh();
      manager.CreateMonitor(0);
    }
  }
}
