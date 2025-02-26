use anyhow::Result;
use image::{GrayImage, RgbaImage};
use log::{debug, info};
use std::fs::File;
use std::io::Write;
use std::io::{Read, Seek};
use std::process;

use base64::{engine::general_purpose, Engine as _};
use image::ImageEncoder;

use crate::device::DeviceModel;

// Output dimensions remain the same for both devices
const OUTPUT_WIDTH: u32 = 768;
const OUTPUT_HEIGHT: u32 = 1024;

pub struct Screenshot {
    data: Vec<u8>,
    device: DeviceModel,
}

impl Screenshot {
    pub fn new() -> Result<Screenshot> {
        let device = DeviceModel::detect();
        info!("Detected device: {}", device.name());
        let screenshot_data = Self::take_screenshot(&device)?;
        Ok(Screenshot {
            data: screenshot_data,
            device,
        })
    }

    fn take_screenshot(device: &DeviceModel) -> Result<Vec<u8>> {
        // Find xochitl's process
        let pid = Self::find_xochitl_pid()?;

        // Find framebuffer location in memory
        let skip_bytes = Self::find_framebuffer_address(&pid, device)?;

        // Read the framebuffer data
        let screenshot_data = Self::read_framebuffer(&pid, skip_bytes, device)?;

        // Process the image data (transpose, color correction, etc.)
        let processed_data = Self::process_image(screenshot_data, device)?;

        Ok(processed_data)
    }

    fn find_xochitl_pid() -> Result<String> {
        let output = process::Command::new("pidof").arg("xochitl").output()?;
        let pids = String::from_utf8(output.stdout)?;
        for pid in pids.split_whitespace() {
            let has_fb = process::Command::new("grep")
                .args(&["-C1", "/dev/fb0", &format!("/proc/{}/maps", pid)])
                .output()?;
            if !has_fb.stdout.is_empty() {
                return Ok(pid.to_string());
            }
        }
        anyhow::bail!("No xochitl process with /dev/fb0 found")
    }

    fn find_framebuffer_address(pid: &str, device: &DeviceModel) -> Result<u64> {
        match device {
            DeviceModel::RemarkablePaperPro => {
                // For RMPP (arm64), we need to use the approach from pointer_arm64.go
                let start_address = Self::get_memory_range(pid)?;
                let frame_pointer = Self::calculate_frame_pointer(pid, start_address, device)?;
                Ok(frame_pointer)
            },
            _ => {
                // Original RM2 approach
                let output = process::Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "grep -C1 '/dev/fb0' /proc/{}/maps | tail -n1 | sed 's/-.*$//'",
                        pid
                    ))
                    .output()?;
                let address_hex = String::from_utf8(output.stdout)?.trim().to_string();
                let address = u64::from_str_radix(&address_hex, 16)?;
                Ok(address + 7)
            }
        }
    }

    // Get memory range for RMPP based on goMarkableStream/pointer_arm64.go
    fn get_memory_range(pid: &str) -> Result<u64> {
        let maps_file_path = format!("/proc/{}/maps", pid);
        let maps_content = std::fs::read_to_string(&maps_file_path)?;
        
        let mut memory_range = String::new();
        for line in maps_content.lines() {
            if line.contains("/dev/dri/card0") {
                memory_range = line.to_string();
            }
        }
        
        if memory_range.is_empty() {
            anyhow::bail!("No mapping found for /dev/dri/card0");
        }
        
        let fields: Vec<&str> = memory_range.split_whitespace().collect();
        let range_field = fields[0];
        let start_end: Vec<&str> = range_field.split('-').collect();
        
        if start_end.len() != 2 {
            anyhow::bail!("Invalid memory range format");
        }
        
        let end = u64::from_str_radix(start_end[1], 16)?;
        Ok(end)
    }
    
    // Calculate frame pointer for RMPP based on goMarkableStream/pointer_arm64.go
    fn calculate_frame_pointer(pid: &str, start_address: u64, device: &DeviceModel) -> Result<u64> {
        let mem_file_path = format!("/proc/{}/mem", pid);
        let mut file = std::fs::File::open(mem_file_path)?;
        
        let screen_size_bytes = device.screen_width() as usize * device.screen_height() as usize * device.bytes_per_pixel();
        
        let mut offset: u64 = 0;
        let mut length: usize = 2;
        
        while length < screen_size_bytes {
            offset += (length - 2) as u64;
            
            file.seek(std::io::SeekFrom::Start(start_address + offset + 8))?;
            let mut header = [0u8; 8];
            file.read_exact(&mut header)?;
            
            length = (header[0] as usize) | 
                     ((header[1] as usize) << 8) | 
                     ((header[2] as usize) << 16) | 
                     ((header[3] as usize) << 24);
        }
        
        Ok(start_address + offset)
    }

    fn read_framebuffer(pid: &str, skip_bytes: u64, device: &DeviceModel) -> Result<Vec<u8>> {
        let window_bytes = device.screen_width() as usize * device.screen_height() as usize * device.bytes_per_pixel();
        let mut buffer = vec![0u8; window_bytes];
        let mut file = std::fs::File::open(format!("/proc/{}/mem", pid))?;
        file.seek(std::io::SeekFrom::Start(skip_bytes))?;
        file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn process_image(data: Vec<u8>, device: &DeviceModel) -> Result<Vec<u8>> {
        // Encode the raw data to PNG
        let png_data = Self::encode_png(&data, device)?;

        // Resize the PNG to OUTPUT_WIDTH x OUTPUT_HEIGHT
        let img = image::load_from_memory(&png_data)?;
        let resized_img = img.resize(
            OUTPUT_WIDTH,
            OUTPUT_HEIGHT,
            image::imageops::FilterType::Lanczos3,
        );

        // Encode the resized image back to PNG
        let mut resized_png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut resized_png_data);
        
        // Handle different color types based on device
        match device {
            DeviceModel::RemarkablePaperPro => {
                encoder.write_image(
                    resized_img.as_luma8().unwrap().as_raw(),
                    OUTPUT_WIDTH,
                    OUTPUT_HEIGHT,
                    image::ExtendedColorType::L8,
                )?;
            },
            _ => {
                encoder.write_image(
                    resized_img.as_luma8().unwrap().as_raw(),
                    OUTPUT_WIDTH,
                    OUTPUT_HEIGHT,
                    image::ExtendedColorType::L8,
                )?;
            }
        }

        Ok(resized_png_data)
    }

    fn encode_png(raw_data: &[u8], device: &DeviceModel) -> Result<Vec<u8>> {
        match device {
            DeviceModel::RemarkablePaperPro => {
                // RMPP uses 32-bit RGBA format
                Self::encode_png_rmpp(raw_data, device)
            },
            _ => {
                // RM2 uses 16-bit grayscale
                Self::encode_png_rm2(raw_data, device)
            }
        }
    }
    
    fn encode_png_rm2(raw_data: &[u8], device: &DeviceModel) -> Result<Vec<u8>> {
        let raw_u8: Vec<u8> = raw_data
            .chunks_exact(2)
            .map(|chunk| u8::from_le_bytes([chunk[1]]))
            .collect();

        let width = device.screen_width();
        let height = device.screen_height();
        let mut processed = vec![0u8; (width * height) as usize];

        for y in 0..height {
            for x in 0..width {
                let src_idx = (height - 1 - y) + (width - 1 - x) * height;
                let dst_idx = y * width + x;
                processed[dst_idx as usize] = Self::apply_curves(raw_u8[src_idx as usize]);
            }
        }

        let img = GrayImage::from_raw(width, height, processed)
            .ok_or_else(|| anyhow::anyhow!("Failed to create image from raw data"))?;

        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.write_image(
            img.as_raw(),
            width,
            height,
            image::ExtendedColorType::L8,
        )?;

        Ok(png_data)
    }
    
    fn encode_png_rmpp(raw_data: &[u8], device: &DeviceModel) -> Result<Vec<u8>> {
        // RMPP uses 32-bit RGBA format, but we'll convert to grayscale
        let width = device.screen_width();
        let height = device.screen_height();
        
        // Extract grayscale from RGBA data (using average of RGB)
        let mut processed = vec![0u8; (width * height) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let pixel_idx = ((y * width + x) * 4) as usize;
                
                // Get RGB values (skip alpha)
                let r = raw_data[pixel_idx] as u16;
                let g = raw_data[pixel_idx + 1] as u16;
                let b = raw_data[pixel_idx + 2] as u16;
                
                // Convert to grayscale using average
                let gray = ((r + g + b) / 3) as u8;
                
                // Apply curves and store
                processed[(y * width + x) as usize] = Self::apply_curves(gray);
            }
        }
        
        let img = GrayImage::from_raw(width, height, processed)
            .ok_or_else(|| anyhow::anyhow!("Failed to create image from raw data"))?;

        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.write_image(
            img.as_raw(),
            width,
            height,
            image::ExtendedColorType::L8,
        )?;

        Ok(png_data)
    }

    fn apply_curves(value: u8) -> u8 {
        let normalized = value as f32 / 255.0;
        let adjusted = if normalized < 0.045 {
            0.0
        } else if normalized < 0.06 {
            (normalized - 0.045) / (0.06 - 0.045)
        } else {
            1.0
        };
        (adjusted * 255.0) as u8
    }

    pub fn save_image(&self, filename: &str) -> Result<()> {
        let mut png_file = File::create(filename)?;
        png_file.write_all(&self.data)?;
        debug!("PNG image saved to {}", filename);
        Ok(())
    }

    pub fn base64(&self) -> Result<String> {
        let base64_image = general_purpose::STANDARD.encode(&self.data);
        Ok(base64_image)
    }
}
