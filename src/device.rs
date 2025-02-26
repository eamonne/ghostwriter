use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceModel {
    Remarkable2,
    RemarkablePaperPro,
    Unknown,
}

impl DeviceModel {
    pub fn detect() -> Self {
        // Check for Paper Pro specific files or characteristics
        // This is a simplified detection method - you may need to adjust based on actual device differences
        if Path::new("/sys/devices/platform/30a40000.i2c/i2c-0/0-0038/input/input2").exists() {
            DeviceModel::RemarkablePaperPro
        } else {
            DeviceModel::Remarkable2
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            DeviceModel::Remarkable2 => "Remarkable2",
            DeviceModel::RemarkablePaperPro => "RemarkablePaperPro",
            DeviceModel::Unknown => "Unknown",
        }
    }
    
    pub fn screen_width(&self) -> u32 {
        match self {
            DeviceModel::Remarkable2 => 1872,
            DeviceModel::RemarkablePaperPro => 1624,
            DeviceModel::Unknown => 1872, // Default to RM2
        }
    }
    
    pub fn screen_height(&self) -> u32 {
        match self {
            DeviceModel::Remarkable2 => 1404,
            DeviceModel::RemarkablePaperPro => 2154,
            DeviceModel::Unknown => 1404, // Default to RM2
        }
    }
    
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            DeviceModel::Remarkable2 => 2,
            DeviceModel::RemarkablePaperPro => 4,
            DeviceModel::Unknown => 2, // Default to RM2
        }
    }
    
    pub fn pen_input_device(&self) -> &str {
        match self {
            DeviceModel::Remarkable2 => "/dev/input/event1",
            DeviceModel::RemarkablePaperPro => "/dev/input/event2",
            DeviceModel::Unknown => "/dev/input/event1", // Default to RM2
        }
    }
    
    pub fn touch_input_device(&self) -> &str {
        match self {
            DeviceModel::Remarkable2 => "/dev/input/event2",
            DeviceModel::RemarkablePaperPro => "/dev/input/event3",
            DeviceModel::Unknown => "/dev/input/event2", // Default to RM2
        }
    }
    
    pub fn max_x_value(&self) -> i32 {
        match self {
            DeviceModel::Remarkable2 => 15725,
            DeviceModel::RemarkablePaperPro => 11180,
            DeviceModel::Unknown => 15725, // Default to RM2
        }
    }
    
    pub fn max_y_value(&self) -> i32 {
        match self {
            DeviceModel::Remarkable2 => 20966,
            DeviceModel::RemarkablePaperPro => 15340,
            DeviceModel::Unknown => 20966, // Default to RM2
        }
    }
}
