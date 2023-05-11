use std::ptr;

use windows::{
  core::ComInterface,
  Win32::Graphics::{
    Direct3D11::{
      ID3D11Device, ID3D11DeviceContext, ID3D11Resource, ID3D11Texture2D, D3D11_BIND_FLAG,
      D3D11_CPU_ACCESS_READ, D3D11_RESOURCE_MISC_FLAG, D3D11_USAGE_STAGING,
    },
    Dxgi::{
      IDXGIOutput1, IDXGIOutputDuplication, IDXGISurface1, DXGI_OUTPUT_DESC,
      DXGI_RESOURCE_PRIORITY_MAXIMUM,
    },
  },
};

pub struct DuplicateContext {
  device: ID3D11Device,
  device_context: ID3D11DeviceContext,
  timeout_ms: u32,
  output: IDXGIOutput1,
  output_duplication: IDXGIOutputDuplication,
}

impl DuplicateContext {
  pub fn new(
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    output: IDXGIOutput1,
    output_duplication: IDXGIOutputDuplication,
    timeout_ms: u32,
  ) -> Self {
    Self {
      device,
      device_context,
      timeout_ms,
      output,
      output_duplication,
    }
  }

  pub fn get_desc(&self) -> Box<DXGI_OUTPUT_DESC> {
    unsafe {
      let desc = ptr::null_mut();
      self.output.GetDesc(desc).unwrap();
      Box::from_raw(desc)
    }
  }

  pub fn acquire_next_frame(&self) -> IDXGISurface1 {
    unsafe {
      let resource = ptr::null_mut();
      let frame_info = ptr::null_mut();
      self
        .output_duplication
        .AcquireNextFrame(self.timeout_ms, frame_info, resource)
        .unwrap();

      // let frame_info = Box::from_raw(frame_info);
      let texture = Box::from_raw(resource.cast::<Option<ID3D11Texture2D>>()).unwrap();

      // Configure the description to make the texture readable
      let mut texture_desc = ptr::null_mut();
      texture.GetDesc(texture_desc);
      (*texture_desc).BindFlags = D3D11_BIND_FLAG(0);
      (*texture_desc).CPUAccessFlags = D3D11_CPU_ACCESS_READ;
      (*texture_desc).MiscFlags = D3D11_RESOURCE_MISC_FLAG(0);
      (*texture_desc).Usage = D3D11_USAGE_STAGING; // A resource that supports data transfer (copy) from the GPU to the CPU.

      // copy from GPU to RAM
      let readable_texture = ptr::null_mut();
      self
        .device
        .CreateTexture2D(texture_desc, None, Some(readable_texture))
        .unwrap();
      let readable_texture = Box::from_raw(readable_texture).unwrap();
      // Lower priorities causes stuff to be needlessly copied from gpu to ram,
      // causing huge ram usage on some systems.
      // https://github.com/bryal/dxgcap-rs/blob/208d93368bc64aed783791242410459c878a10fb/src/lib.rs#L225
      readable_texture.SetEvictionPriority(DXGI_RESOURCE_PRIORITY_MAXIMUM.0);
      let readable_surface = readable_texture.cast::<ID3D11Resource>().unwrap();
      self
        .device_context
        .CopyResource(&readable_surface, &texture);
      self.output_duplication.ReleaseFrame().unwrap();

      readable_surface.cast().unwrap()
    }
  }
}
