use std::ptr;

use windows::Win32::Graphics::{
  Direct3D11::{ID3D11Device, ID3D11DeviceContext},
  Dxgi::{IDXGIOutput1, IDXGIOutputDuplication, DXGI_OUTPUT_DESC},
};

pub struct DuplicateOutput {
  device: Box<&'static ID3D11Device>,
  device_context: Box<&'static ID3D11DeviceContext>,
  output: IDXGIOutput1,
  output_duplication: IDXGIOutputDuplication,
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
