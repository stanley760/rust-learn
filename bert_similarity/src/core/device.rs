// Device management for BERT model inference and training

use crate::utils::AppError;
use candle_core::Device;

/// Helper function to determine the best available device
///
/// # Arguments
/// * `device_str` - Device identifier ("auto", "cuda", "metal", "cpu")
///
/// # Returns
/// * `Result<Device>` - The selected device
///
/// # Examples
/// ```ignore
/// use bert_similarity::core::get_device;
///
/// // Auto-detect the best available device
/// let device = get_device("auto")?;
///
/// // Force CPU usage
/// let device = get_device("cpu")?;
///
/// // Force CUDA usage (returns error if CUDA unavailable)
/// let device = get_device("cuda")?;
/// ```
pub fn get_device(device_str: &str) -> Result<Device, AppError> {
    match device_str.to_lowercase().as_str() {
        "auto" => {
            if candle_core::utils::cuda_is_available() {
                tracing::info!("CUDA is available, using CUDA device");
                Device::new_cuda(0).map_err(|e| {
                    AppError::ModelError(format!("Failed to create CUDA device: {}", e))
                })
            } else if candle_core::utils::metal_is_available() {
                tracing::info!("Metal is available, using Metal device");
                Device::new_metal(0).map_err(|e| {
                    AppError::ModelError(format!("Failed to create Metal device: {}", e))
                })
            } else {
                tracing::info!("No GPU available, using CPU device");
                Ok(Device::Cpu)
            }
        }
        "cuda" => {
            if !candle_core::utils::cuda_is_available() {
                return Err(AppError::ModelError(
                    "CUDA device requested but CUDA is not available".to_string(),
                ));
            }
            tracing::info!("Using CUDA device");
            Device::new_cuda(0)
                .map_err(|e| AppError::ModelError(format!("Failed to create CUDA device: {}", e)))
        }
        "metal" => {
            if !candle_core::utils::metal_is_available() {
                return Err(AppError::ModelError(
                    "Metal device requested but Metal is not available".to_string(),
                ));
            }
            tracing::info!("Using Metal device");
            Device::new_metal(0)
                .map_err(|e| AppError::ModelError(format!("Failed to create Metal device: {}", e)))
        }
        "cpu" => {
            tracing::info!("Using CPU device");
            Ok(Device::Cpu)
        }
        _ => Err(AppError::ModelError(format!(
            "Invalid device: {}. Must be one of: auto, cuda, metal, cpu",
            device_str
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_device_cpu() {
        let device = get_device("cpu").unwrap();
        assert!(matches!(device, Device::Cpu));
    }

    #[test]
    fn test_get_device_auto_fallback_to_cpu() {
        let device = get_device("auto").unwrap();
        assert!(matches!(
            device,
            Device::Cpu | Device::Cuda(_) | Device::Metal(_)
        ));
    }

    #[test]
    fn test_get_device_invalid() {
        let result = get_device("invalid_device");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    }

    #[test]
    fn test_get_device_case_insensitive() {
        let device1 = get_device("CPU").unwrap();
        let device2 = get_device("cpu").unwrap();
        let device3 = get_device("Cpu").unwrap();

        assert!(matches!(device1, Device::Cpu));
        assert!(matches!(device2, Device::Cpu));
        assert!(matches!(device3, Device::Cpu));
    }
}
