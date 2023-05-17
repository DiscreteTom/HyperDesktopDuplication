using System;
using System.Runtime.InteropServices;
using UnityEngine;

public class App : MonoBehaviour
{
  [DllImport("Kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
  static extern IntPtr OpenFileMapping(int dwDesiredAccess, bool bInheritHandle, string lpName);
  [DllImport("Kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
  static extern IntPtr MapViewOfFile(IntPtr hFileMappingObject, int dwDesiredAccess, int dwFileOffsetHigh, int dwFileOffsetLow, int dwNumberOfBytesToMap);
  [DllImport("Kernel32.dll", CharSet = CharSet.Auto)]
  static extern bool UnmapViewOfFile(IntPtr pvBaseAddress);
  [DllImport("Kernel32.dll", CharSet = CharSet.Auto)]
  static extern bool CloseHandle(IntPtr handle);
  [DllImport("kernel32.dll")]
  public static extern uint GetLastError();

  IntPtr handle;
  IntPtr address;
  enum State
  {
    ListDisplays,
    ListDisplaysDone,
    CreateCapture,
    CreateCaptureDone,
    TakeCapture,
    TakeCaptureDone,
  }
  State state;
  int bufSize;
  Texture2D texture;
  string filename;
  Grpc.Core.Channel channel;
  Hdd.HDD.HDDClient client;

  void Start()
  {
    channel = new Grpc.Core.Channel("localhost:3030", Grpc.Core.ChannelCredentials.Insecure);
    client = new Hdd.HDD.HDDClient(channel);
    filename = "Global\\HDD" + UnityEngine.Random.Range(10000, 99999).ToString();
    state = State.ListDisplays;
    ListDisplays();
  }

  async void ListDisplays()
  {
    var reply = await client.ListDisplaysAsync(new Hdd.ListDisplaysRequest { });
    state = State.ListDisplaysDone;
    // for (var i = 0; i < reply.Infos.Count; ++i)
    // {
    var i = 0;
    var info = reply.Infos[i];
    var width = info.Right - info.Left;
    var height = info.Bottom - info.Top;
    print($"display {i}: {width}x{height}");

    this.bufSize = width * height * 4;
    texture = new Texture2D(width, height, TextureFormat.BGRA32, false);
    GetComponent<Renderer>().material.mainTexture = texture;
    print("texture created with size: " + width + "x" + height);
    this.transform.localScale = new Vector3(-width / 1000.0f, 1, height / 1000.0f);
    // }
  }

  void Update()
  {
    if (state == State.ListDisplaysDone)
    {
      state = State.CreateCapture;
      CreateCapture(0);
    }
    else if (state == State.CreateCaptureDone)
    {
      this.handle = OpenFileMapping(
        ((0x000F0000) | 0x0001 | 0x0002 | 0x0004 | 0x0008 | 0x0010), // FILE_MAP_ALL_ACCESS
        false,
        filename
      );
      if (handle == IntPtr.Zero)
      {
        Debug.Log("OpenFileMapping() failed");
        Debug.Log(Marshal.GetLastWin32Error());
        return;
      }

      this.address = MapViewOfFile(handle,
        ((0x000F0000) | 0x0001 | 0x0002 | 0x0004 | 0x0008 | 0x0010), // FILE_MAP_ALL_ACCESS
        0, 0, this.bufSize);
      if (address == IntPtr.Zero)
      {
        Debug.Log("MapViewOfFile() failed");
        return;
      }

      state = State.TakeCapture;
      TakeCapture(0);
    }
    else if (state == State.TakeCaptureDone)
    {
      texture.LoadRawTextureData(address, bufSize);
      texture.Apply();

      state = State.TakeCapture;
      TakeCapture(0);
    }
  }

  async void OnDestroy()
  {
    if (address != IntPtr.Zero && !UnmapViewOfFile(address))
    {
      Debug.Log("UnmapViewOfFile() failed");
      Debug.Log(Marshal.GetLastWin32Error());
    }
    if (handle != IntPtr.Zero && !CloseHandle(handle))
    {
      Debug.Log("CloseHandle() failed");
      Debug.Log(Marshal.GetLastWin32Error());
    }
    try
    {
      await client.DeleteCaptureAsync(new Hdd.DeleteCaptureRequest { Id = 0 });
    }
    catch
    {
      print("capture not deleted");
    }
    try
    {
      await channel.ShutdownAsync();
    }
    catch
    {
      print("channel not shutdown");
    }
    print("capture deleted");
  }


  async void CreateCapture(uint n)
  {
    await client.CreateCaptureAsync(new Hdd.CreateCaptureRequest { Id = n, Name = filename });
    state = State.CreateCaptureDone;
  }

  async void TakeCapture(uint n)
  {
    var res = await client.TakeCaptureAsync(new Hdd.TakeCaptureRequest { Id = n });

    if (res.Update)
    {

      state = State.TakeCaptureDone;
    }
    else
    {
      print("old");
      state = State.TakeCapture;
      TakeCapture(0);
    }
  }
}
