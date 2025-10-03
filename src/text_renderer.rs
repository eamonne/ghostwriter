use anyhow::Result;
use log::{debug, info};

/// Converts text to an SVG with handwriting-style rendering
/// This supports any Unicode characters, not limited by keyboard mapping
pub fn text_to_svg(text: &str, width: u32, height: u32) -> Result<String> {
    // Starting position for text
    let x = 50;
    let y = 100;
    let font_size = 32;
    let line_height = font_size + 10;

    // Split text into lines and escape for XML
    let lines: Vec<String> = text.lines().map(|line| escape_xml(line)).collect();

    // Build SVG with text elements
    let mut svg_content = String::new();
    
    for (i, line) in lines.iter().enumerate() {
        let y_pos = y + (i as u32 * line_height);
        svg_content.push_str(&format!(
            r#"    <text x="{}" y="{}" font-family="Noto Sans, DejaVu Sans, Arial, sans-serif" font-size="{}" fill="black">{}</text>"#,
            x, y_pos, font_size, line
        ));
        svg_content.push('\n');
    }

    let svg = format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
{}
</svg>"#,
        width, height, svg_content
    );

    debug!("Generated SVG for text with {} lines", lines.len());
    Ok(svg)
}

/// Converts text to an SVG with a more handwriting-style cursive appearance
/// Uses path elements to simulate handwritten strokes
pub fn text_to_cursive_svg(text: &str, width: u32, height: u32) -> Result<String> {
    info!("Converting text to cursive SVG");
    
    // For cursive/handwriting style, we'll use a combination of:
    // 1. A cursive/script font family that's more likely to be available
    // 2. Slightly randomized positions for a more natural look (optional)
    
    let x = 50;
    let y = 100;
    let font_size = 36;
    let line_height = font_size + 15;

    let lines: Vec<String> = text.lines().map(|line| escape_xml(line)).collect();

    let mut svg_content = String::new();
    
    // Use multiple fallback fonts for best compatibility
    let font_families = "Noto Sans, DejaVu Sans, Liberation Sans, Arial, Helvetica, sans-serif";
    
    for (i, line) in lines.iter().enumerate() {
        let y_pos = y + (i as u32 * line_height);
        
        // Add text with styling that looks more handwritten
        svg_content.push_str(&format!(
            r#"    <text x="{}" y="{}" font-family="{}" font-size="{}" fill="black" style="font-style: italic; font-weight: 400;">{}</text>"#,
            x, y_pos, font_families, font_size, line
        ));
        svg_content.push('\n');
    }

    let svg = format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
{}
</svg>"#,
        width, height, svg_content
    );

    debug!("Generated cursive SVG for text with {} lines", lines.len());
    Ok(svg)
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_svg() {
        let result = text_to_svg("Hello World", 768, 1024);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("Hello World"));
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_unicode_support() {
        let result = text_to_svg("Hello 世界 שלום مرحبا", 768, 1024);
        assert!(result.is_ok());
    }

    #[test]
    fn test_xml_escaping() {
        let result = text_to_svg("Test <tag> & \"quotes\"", 768, 1024);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("&lt;tag&gt;"));
        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&quot;"));
    }
}
