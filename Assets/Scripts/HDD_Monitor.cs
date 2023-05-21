using System;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using UnityEngine;

namespace HyperDesktopDuplication {
  public class HDD_Monitor : MonoBehaviour {
    [DllImport("Kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    static extern IntPtr OpenFileMapping(int dwDesiredAccess, bool bInheritHandle, string lpName);
    [DllImport("Kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    static extern IntPtr CreateFileMappingA(int hFile, IntPtr lpAttributes, int flProtect, int dwMaxSizeHi, int dwMaxSizeLow, string lpName);
    [DllImport("Kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    static extern IntPtr MapViewOfFile(IntPtr hFileMappingObject, int dwDesiredAccess, int dwFileOffsetHigh, int dwFileOffsetLow, int dwNumberOfBytesToMap);
    [DllImport("Kernel32.dll", CharSet = CharSet.Auto)]
    static extern bool UnmapViewOfFile(IntPtr pvBaseAddress);
    [DllImport("Kernel32.dll", CharSet = CharSet.Auto)]
    static extern bool CloseHandle(IntPtr handle);

    enum State {
      Idle,
      CreateCapture,
      TakeCapture,
      TakeCaptureDone,
    }

    State state = State.Idle;
    int id = 0;
    Shremdup.Shremdup.ShremdupClient client;
    IntPtr handle;
    IntPtr address;
    int bufSize;
    Texture2D texture;
    string filename; // name of shared memory

    public void Setup(Shremdup.Shremdup.ShremdupClient client, int id, int width, int height, int pixel_width, int pixel_height, string filenamePrefix) {
      this.client = client;
      this.id = id;
      this.filename = $"{filenamePrefix}-{id}";

      var desktopRenderer = this.transform.Find("DesktopRenderer");

      this.bufSize = pixel_width * pixel_height * 4; // 4 for BGRA32
      texture = new Texture2D(pixel_width, pixel_height, TextureFormat.BGRA32, false);
      desktopRenderer.GetComponent<Renderer>().material.mainTexture = texture;
      Logger.Log($"display {this.id}: texture created with size: {pixel_width}x{pixel_height}");
      desktopRenderer.transform.localScale = new Vector3(-width / 1000.0f, 1, height / 1000.0f); // resize to a proper size

      this.CreateCapture();
    }

    async void CreateCapture() {
      this.state = State.CreateCapture;

      await client.CreateCaptureAsync(new Shremdup.CreateCaptureRequest { Id = (uint)this.id, Name = filename, Open = false });

      this.handle = OpenFileMapping(
        ((0x000F0000) | 0x0001 | 0x0002 | 0x0004 | 0x0008 | 0x0010), // FILE_MAP_ALL_ACCESS
        false,
        filename
      );
      if (handle == IntPtr.Zero) {
        Logger.Log($"display {this.id}: OpenFileMapping() failed: {Marshal.GetLastWin32Error()}");
        return;
      }

      this.address = MapViewOfFile(handle,
        ((0x000F0000) | 0x0001 | 0x0002 | 0x0004 | 0x0008 | 0x0010), // FILE_MAP_ALL_ACCESS
        0, 0, this.bufSize);
      if (address == IntPtr.Zero) {
        Logger.Log($"display {this.id}: MapViewOfFile() failed");
        return;
      }

      this.TakeCapture();
    }

    async void TakeCapture() {
      this.state = State.TakeCapture;

      try {
        var res = await client.TakeCaptureAsync(new Shremdup.TakeCaptureRequest { Id = (uint)this.id });

        if (res.DesktopUpdated) {
          // load from shared memory
          texture.LoadRawTextureData(address, bufSize);
          texture.Apply();
        }
      } catch (Exception e) {
        Logger.Log($"display {this.id}: TakeCapture failed: {e}");
      }

      this.state = State.TakeCaptureDone;
    }

    void Update() {
      // call take capture in Update to control the request interval
      if (this.state == State.TakeCaptureDone) this.TakeCapture();
    }

    public async Task DestroyMonitor() {
      // first, set to idle to prevent further updates
      this.state = State.Idle;

      // close shared memory file
      if (address != IntPtr.Zero && !UnmapViewOfFile(address)) {
        Logger.Log($"display {this.id}: UnmapViewOfFile() failed");
        Logger.Log(Marshal.GetLastWin32Error());
      }
      if (handle != IntPtr.Zero && !CloseHandle(handle)) {
        Logger.Log($"display {this.id}: CloseHandle() failed");
        Logger.Log(Marshal.GetLastWin32Error());
      }

      // stop server capture
      try {
        await client.DeleteCaptureAsync(new Shremdup.DeleteCaptureRequest { Id = 0 });
        Logger.Log($"display {this.id}: capture deleted");
      } catch (Exception e) {
        Logger.Log($"display {this.id}: delete capture failed: {e}");
      }
    }
  }
}
