use std::ptr;

use windows::core::ComInterface;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_9_1};
use windows::Win32::Graphics::Direct3D11::{
  D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::{
  CreateDXGIFactory1, IDXGIFactory1, IDXGIOutput1, IDXGIOutputDuplication, DXGI_OUTPUT_DESC,
};

struct DuplicateOutput {
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

pub struct Manager {
  duplicated_output: Vec<DuplicateOutput>,
}

impl Manager {
  pub fn new() -> Result<Manager, &'static str> {
    let mut manager = Manager {
      duplicated_output: Vec::new(),
    };
    match manager.acquire_output_duplication() {
      Ok(_) => Ok(manager),
      Err(_) => Err("Failed to acquire output duplication"),
    }
  }

  fn acquire_output_duplication(&mut self) -> Result<(), ()> {
    unsafe {
      let factory = CreateDXGIFactory1::<IDXGIFactory1>().unwrap();
      let mut adapter_outputs = Vec::new();

      // collect adapters and outputs
      for adapter_index in 0.. {
        let adapter = match factory.EnumAdapters1(adapter_index) {
          Ok(adapter) => adapter,
          Err(_) => break,
        };
        let mut outputs = Vec::new();
        for output_index in 0.. {
          match adapter.EnumOutputs(output_index) {
            Err(_) => break,
            Ok(output) => outputs.push(output),
          }
        }
        if outputs.len() > 0 {
          adapter_outputs.push((adapter, outputs))
        }
      }
      if adapter_outputs.len() == 0 {
        panic!();
      }

      // prepare device and output
      for (adapter, outputs) in adapter_outputs {
        let device = None;
        let device_context = None;
        let mut feature_level = D3D_FEATURE_LEVEL_9_1;

        // create device for each adapter
        D3D11CreateDevice(
          &adapter,
          D3D_DRIVER_TYPE_UNKNOWN,
          None,
          D3D11_CREATE_DEVICE_FLAG(0),
          None,
          D3D11_SDK_VERSION,
          device,
          Some(&mut feature_level),
          device_context,
        )
        .unwrap();
        let device = (*device.unwrap()).as_ref().unwrap();
        let device_context = (*device_context.unwrap()).as_ref().unwrap();

        // create duplication output for each output
        for output in outputs {
          let output = output.cast::<IDXGIOutput1>().unwrap();
          let output_duplication = output.DuplicateOutput(device).unwrap();
          self.duplicated_output.push(DuplicateOutput {
            device: Box::new(device),
            device_context: Box::new(device_context),
            output,
            output_duplication,
          })
        }
      }
      Ok(())
    }
  }
}
