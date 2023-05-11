use std::ptr;

use windows::{
  core::ComInterface,
  Win32::Graphics::{
    Direct3D11::{
      ID3D11Device, ID3D11DeviceContext, ID3D11Resource, ID3D11Texture2D, D3D11_BIND_FLAG,
      D3D11_CPU_ACCESS_READ, D3D11_RESOURCE_MISC_FLAG, D3D11_SUBRESOURCE_DATA,
      D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
    },
    Dxgi::{
      IDXGIOutput1, IDXGIOutputDuplication, IDXGIResource, IDXGISurface1, DXGI_MAP_READ,
      DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTPUT_DESC, DXGI_RESOURCE_PRIORITY_MAXIMUM,
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

  pub fn get_desc(&self) -> DXGI_OUTPUT_DESC {
    unsafe {
      let desc = DXGI_OUTPUT_DESC::default();
      self.output.GetDesc(&desc as *const _ as *mut _).unwrap();
      desc
    }
  }

  pub fn acquire_next_frame(&self) -> IDXGISurface1 {
    unsafe {
      let frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
      let resource: Option<IDXGIResource> = None.clone();
      self
        .output_duplication
        .AcquireNextFrame(
          self.timeout_ms,
          &frame_info as *const _ as *mut _,
          &resource as *const _ as *mut _,
        )
        .unwrap();

      let resource = resource.unwrap();
      let texture = (&resource as *const IDXGIResource).cast::<ID3D11Texture2D>();

      // Configure the description to make the texture readable
      let mut texture_desc = D3D11_TEXTURE2D_DESC::default();
      (*texture).GetDesc(&mut texture_desc as *mut _);
      texture_desc.BindFlags = D3D11_BIND_FLAG::default();
      texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
      texture_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();
      texture_desc.Usage = D3D11_USAGE_STAGING; // A resource that supports data transfer (copy) from the GPU to the CPU.

      // copy a readable version of the texture in GPU
      let readable_texture: Option<ID3D11Texture2D> = None.clone();
      self
        .device
        .CreateTexture2D(
          &texture_desc,
          None,
          Some(&readable_texture as *const _ as *mut _),
        )
        .unwrap();
      let readable_texture = readable_texture.unwrap();
      // Lower priorities causes stuff to be needlessly copied from gpu to ram,
      // causing huge ram usage on some systems.
      // https://github.com/bryal/dxgcap-rs/blob/208d93368bc64aed783791242410459c878a10fb/src/lib.rs#L225
      readable_texture.SetEvictionPriority(DXGI_RESOURCE_PRIORITY_MAXIMUM.0);
      let readable_surface = readable_texture.cast::<ID3D11Resource>().unwrap();
      self
        .device_context
        .CopyResource(&readable_surface, &*texture);
      self.output_duplication.ReleaseFrame().unwrap();

      readable_surface.cast().unwrap()
    }
  }

  pub fn capture_frame(&self, dest: *mut u8, size: usize) {
    unsafe {
      let frame = &self.acquire_next_frame();
      let mapped_surface = ptr::null_mut();
      frame.Map(mapped_surface, DXGI_MAP_READ).unwrap();
      let mapped_surface = Box::from_raw(mapped_surface);

      ptr::copy_nonoverlapping(mapped_surface.pBits, dest, size);

      frame.Unmap().unwrap();
    }
  }
}
