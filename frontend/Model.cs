using System;

[Serializable]
public struct DisplaysInfo
{
  public DisplayInfo[] displays;
}

[Serializable]
public struct DisplayInfo
{
  public int bottom;
  public int top;
  public int left;
  public int right;
  public string name;
  public int rotation;
}