use std::ptr;

use windows::Win32::Graphics::{
  Direct3D11::{ID3D11Device, ID3D11DeviceContext},
  Dxgi::{IDXGIOutput1, IDXGIOutputDuplication, DXGI_OUTPUT_DESC},
};

pub struct DuplicateOutput {
  pub device: ID3D11Device,
  pub device_context: ID3D11DeviceContext,
  pub timeout_ms: u32,
  pub output: IDXGIOutput1,
  pub output_duplication: IDXGIOutputDuplication,
}

impl DuplicateOutput {
  pub fn get_desc(&self) -> Box<DXGI_OUTPUT_DESC> {
    unsafe {
      let desc = ptr::null_mut();
      self.output.GetDesc(desc).unwrap();
      Box::from_raw(desc)
    }
  }
}
