mod duplicate_context;

use self::duplicate_context::DuplicateContext;
use windows::core::ComInterface;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_9_1};
use windows::Win32::Graphics::Direct3D11::{
  D3D11CreateDevice, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1, IDXGIOutput1};

pub struct Manager {
  dup_ctx: Vec<DuplicateContext>,
  timeout_ms: u32,
}

impl Manager {
  pub fn default() -> Result<Manager, &'static str> {
    Manager::new(300)
  }

  pub fn new(timeout_ms: u32) -> Result<Manager, &'static str> {
    let mut manager = Manager {
      dup_ctx: Vec::new(),
      timeout_ms,
    };
    match manager.refresh() {
      Ok(_) => Ok(manager),
      Err(_) => Err("Failed to acquire output duplication"),
    }
  }

  pub fn refresh(&mut self) -> Result<(), ()> {
    self.dup_ctx.clear();

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
        let device = Box::from_raw(device.unwrap()).unwrap();
        let device_context = Box::from_raw(device_context.unwrap()).unwrap();

        // create duplication output for each output
        for output in outputs {
          let output = output.cast::<IDXGIOutput1>().unwrap();
          let output_duplication = output.DuplicateOutput(&device).unwrap();
          self.dup_ctx.push(DuplicateContext {
            device: device.clone(),
            device_context: device_context.clone(),
            output,
            timeout_ms: self.timeout_ms,
            output_duplication,
          })
        }
      }
      Ok(())
    }
  }
}
