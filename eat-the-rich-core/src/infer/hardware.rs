use candle_core::{Device, Result};

pub struct HardwareEnv;

impl HardwareEnv {
    /// Dynamically selects the best available compute device based on the platform.
    /// Prioritizes CUDA (Windows RTX 4080) -> Metal (macOS M-series) -> CPU fallback.
    pub fn get_best_device() -> Result<Device> {
        if candle_core::utils::cuda_is_available() {
            println!("⚡ Hardware Detected: NVIDIA CUDA. Engaging Tensor Cores.");
            Device::new_cuda(0)
        } else if candle_core::utils::metal_is_available() {
            println!("⚡ Hardware Detected: Apple Silicon (Metal). Engaging MPS.");
            Device::new_metal(0)
        } else {
            println!("⚠️ Warning: GPU acceleration not found. Falling back to CPU.");
            Ok(Device::Cpu)
        }
    }
}
