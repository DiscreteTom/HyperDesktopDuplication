use std::ptr;

use windows::Win32::Graphics::{
  Direct3D11::{ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D},
  Dxgi::{IDXGIOutput1, IDXGIOutputDuplication, DXGI_OUTPUT_DESC},
};

pub struct DuplicateContext {
  pub device: ID3D11Device,
  pub device_context: ID3D11DeviceContext,
  pub timeout_ms: u32,
  pub output: IDXGIOutput1,
  pub output_duplication: IDXGIOutputDuplication,
}

impl DuplicateContext {
  pub fn get_desc(&self) -> Box<DXGI_OUTPUT_DESC> {
    unsafe {
      let desc = ptr::null_mut();
      self.output.GetDesc(desc).unwrap();
      Box::from_raw(desc)
    }
  }

  // pub fn acquire_next_frame(&self, timeout_ms: u32) {
  //   unsafe {
  //     let mut resource = ptr::null_mut();
  //     let mut frame_info = ptr::null_mut();
  //     self
  //       .output_duplication
  //       .AcquireNextFrame(timeout_ms, frame_info, resource)
  //       .unwrap();
  //     let resource = *resource;
  //     let frame_info = Box::from_raw(frame_info);
  //     let texture = resource.cast::<ID3D11Texture2D>().unwrap();
  //   }
  // }
}
