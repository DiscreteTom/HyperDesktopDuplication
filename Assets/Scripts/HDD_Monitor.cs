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
    Transform mouse;
    Material mouseMaterial;
    Shremdup.DisplayInfo info;
    DesktopRenderer desktopRenderer;

    public int pixelWidth => (int)this.info.PixelWidth;
    public int pixelHeight => (int)this.info.PixelHeight;
    public int rotation => this.info.Rotation;
    public bool portrait => this.rotation == 2 || this.rotation == 4;
    public bool landscape => this.rotation == 1 || this.rotation == 3;
    public int width => this.portrait ? this.info.Bottom - this.info.Top : this.info.Right - this.info.Left;
    public int height => this.portrait ? this.info.Right - this.info.Left : this.info.Bottom - this.info.Top;

    public void Setup(Shremdup.Shremdup.ShremdupClient client, int id, Shremdup.DisplayInfo info, string filenamePrefix) {
      this.client = client;
      this.id = id;
      this.filename = $"{filenamePrefix}-{id}";
      this.info = info;

      this.mouse = this.transform.Find("MouseRenderer");
      this.mouseMaterial = this.mouse.GetComponent<Renderer>().material;
      this.desktopRenderer = this.GetComponentInChildren<DesktopRenderer>();

      this.bufSize = this.pixelWidth * this.pixelHeight * 4; // 4 for BGRA32
      this.texture = new Texture2D(this.portrait ? this.pixelHeight : this.pixelWidth, this.portrait ? this.pixelWidth : this.pixelHeight, TextureFormat.BGRA32, false);
      this.desktopRenderer.GetComponent<Renderer>().material.mainTexture = this.texture;
      Logger.Log($"display {this.id}: texture created with size: {this.pixelWidth}x{this.pixelHeight}");
      this.desktopRenderer.transform.localScale = new Vector3(this.width, this.height, 1); // resize to a proper size
      this.desktopRenderer.transform.localRotation = Quaternion.Euler(0, 0, this.rotation * 90 - 90); // rotate to a proper angle

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

        if (res.PointerPosition != null) {
          // update mouse position
          var pos = res.PointerPosition;
          if (pos.Visible) {
            mouse.gameObject.SetActive(true);
          } else {
            mouse.gameObject.SetActive(false);
          }
          // set z to -0.001 to make sure it's in front of the desktop
          // TODO: use width instead of pixel_width?
          // e.g. x = (-this.pixel_width / 2 + pos.X) * this.width / this.pixel_width
          mouse.localPosition = new Vector3(-this.pixelWidth / 2 + pos.X + this.mouse.transform.localScale.x / 2, -this.pixelHeight / 2 + pos.Y + this.mouse.transform.localScale.y / 2, -0.001f);
        }

        if (res.PointerShape != null) {
          // update mouse shape
          var shape = res.PointerShape;
          switch (shape.ShapeType) {
            case 1: {
                // DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME
                // TODO
                break;
              }
            case 2: {
                // DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR
                var cursorTexture = new Texture2D((int)shape.Width, (int)shape.Height, TextureFormat.ARGB32, false);
                cursorTexture.SetPixelData(shape.Data.ToByteArray(), 0);
                cursorTexture.Apply();
                this.mouseMaterial.mainTexture = cursorTexture;
                this.mouse.transform.localScale = new Vector3(shape.Width, shape.Height, 1);
                break;
              }
            case 4: {
                // DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR
                // TODO
                break;
              }
          }
        }
      } catch (Exception e) {
        Logger.Log($"display {this.id}: TakeCapture failed: {e}");
      }

      this.state = State.TakeCaptureDone;
    }

    void Update() {
      // call take capture in Update to control the request interval
      if (this.state == State.TakeCaptureDone && this.desktopRenderer.visible) this.TakeCapture();
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
        await client.DeleteCaptureAsync(new Shremdup.DeleteCaptureRequest { Id = (uint)this.id });
        Logger.Log($"display {this.id}: capture deleted");
      } catch (Exception e) {
        Logger.Log($"display {this.id}: delete capture failed: {e}");
      }
    }
  }
}
