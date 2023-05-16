using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.Networking;

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

  void Start()
  {
    filename = "HDD" + UnityEngine.Random.Range(10000, 99999).ToString();
    state = State.ListDisplays;
    StartCoroutine(ListDisplays());
  }

  void Update()
  {
    if (state == State.ListDisplaysDone)
    {
      state = State.CreateCapture;
      StartCoroutine(CreateCapture(0));
    }
    else if (state == State.CreateCaptureDone)
    {
      this.handle = OpenFileMapping(
        ((0x000F0000) | 0x0001 | 0x0002 | 0x0004 | 0x0008 | 0x0010), // FILE_MAP_ALL_ACCESS
        false,
        "Global\\" + filename
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
      StartCoroutine(TakeCapture(0));
    }
    else if (state == State.TakeCaptureDone)
    {
      texture.LoadRawTextureData(address, bufSize);
      texture.Apply();

      state = State.TakeCapture;
      StartCoroutine(TakeCapture(0));
    }
  }

  void OnDestroy()
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
    UnityWebRequest www = UnityWebRequest.Delete("http://localhost:3030/captures/0");
    www.SendWebRequest();
    print("capture deleted");
  }

  IEnumerator ListDisplays()
  {
    UnityWebRequest www = UnityWebRequest.Get("http://localhost:3030/displays");
    yield return www.SendWebRequest();

    if (www.result != UnityWebRequest.Result.Success)
    {
      Debug.Log(www.error);
    }
    else
    {
      // Show results as text
      print(www.downloadHandler.text);
      var info = JsonUtility.FromJson<DisplaysInfo>(www.downloadHandler.text);
      int width = info.displays[0].right - info.displays[0].left;
      int height = info.displays[0].bottom - info.displays[0].top;
      this.bufSize = width * height * 4;
      texture = new Texture2D(width, height, TextureFormat.BGRA32, false);
      GetComponent<Renderer>().material.mainTexture = texture;
      print("texture created with size: " + width + "x" + height);
      this.transform.localScale = new Vector3(-width / 1000.0f, 1, height / 1000.0f);
    }

    state = State.ListDisplaysDone;
  }

  IEnumerator CreateCapture(int n)
  {
    UnityWebRequest www = UnityWebRequest.Put("http://localhost:3030/captures/0/Global%5C" + filename, "");
    yield return www.SendWebRequest();

    if (www.result != UnityWebRequest.Result.Success)
    {
      Debug.Log(www.error);
    }
    else
    {
      // Show results as text
      print(www.downloadHandler.text);
    }

    state = State.CreateCaptureDone;
  }

  IEnumerator TakeCapture(int n)
  {
    UnityWebRequest www = UnityWebRequest.Post("http://localhost:3030/captures/0", "");
    yield return www.SendWebRequest();

    if (www.result != UnityWebRequest.Result.Success)
    {
      Debug.Log(www.error);
    }
    else
    {
      // Show results as text
      // print(www.downloadHandler.text);
      if (www.downloadHandler.text == "\"old\"")
      {
        print("old");
        state = State.TakeCapture;
        StartCoroutine(TakeCapture(0));
      }
      else if (www.downloadHandler.text == "\"new\"")
      {
        state = State.TakeCaptureDone;
      }
      else
      {
        Debug.Log("unknown response: " + www.downloadHandler.text);
      }
    }
  }
}
