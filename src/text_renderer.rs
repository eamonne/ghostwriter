use anyhow::Result;
use log::{debug, info};
use resvg::usvg::{self, fontdb, Options, Tree};
use std::sync::Arc;

/// Represents a single stroke (line segment) for drawing
#[derive(Debug, Clone)]
pub struct Stroke {
    pub points: Vec<(f32, f32)>,
}

/// Converts text to vector strokes that can be drawn efficiently
/// This approach is much faster than bitmap rendering
pub fn text_to_strokes(text: &str, width: u32, height: u32) -> Result<Vec<Stroke>> {
    info!("Converting text to vector strokes");
    
    // Create SVG with text elements
    let svg_str = text_to_svg(text, width, height)?;
    
    // Parse SVG and convert to paths
    let mut opt = Options::default();
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    opt.fontdb = Arc::new(fontdb);
    
    let tree = Tree::from_str(&svg_str, &opt)?;
    
    // Convert tree to strokes
    let strokes = extract_strokes_from_tree(&tree)?;
    
    debug!("Generated {} strokes from text", strokes.len());
    Ok(strokes)
}

/// Extract strokes from a parsed SVG tree
fn extract_strokes_from_tree(tree: &Tree) -> Result<Vec<Stroke>> {
    let mut strokes = Vec::new();
    
    // Recursively traverse the tree and extract path data
    extract_strokes_from_node(tree.root(), &mut strokes);
    
    Ok(strokes)
}

/// Recursively extract strokes from a node and its children
fn extract_strokes_from_node(node: &usvg::Group, strokes: &mut Vec<Stroke>) {
    for child in node.children() {
        match child {
            usvg::Node::Path(path_node) => {
                // Convert the path to strokes
                let path_strokes = path_to_strokes(path_node.data());
                strokes.extend(path_strokes);
            }
            usvg::Node::Group(group) => {
                // Recursively process group children
                extract_strokes_from_node(group, strokes);
            }
            _ => {
                // Ignore other node types (image, text, etc.)
            }
        }
    }
}

/// Convert a path to a series of strokes
fn path_to_strokes(path: &usvg::tiny_skia_path::Path) -> Vec<Stroke> {
    let mut strokes = Vec::new();
    let mut current_stroke = Vec::new();
    
    for segment in path.segments() {
        match segment {
            usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                // MoveTo starts a new stroke
                if !current_stroke.is_empty() {
                    strokes.push(Stroke {
                        points: current_stroke.clone(),
                    });
                    current_stroke.clear();
                }
                current_stroke.push((p.x, p.y));
            }
            usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                current_stroke.push((p.x, p.y));
            }
            usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2) => {
                // Approximate quadratic bezier with line segments
                if let Some(&last_point) = current_stroke.last() {
                    let segments = approximate_quad_bezier(
                        last_point,
                        (p1.x, p1.y),
                        (p2.x, p2.y),
                        10,
                    );
                    current_stroke.extend(segments);
                }
            }
            usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => {
                // Approximate cubic bezier with line segments
                if let Some(&last_point) = current_stroke.last() {
                    let segments = approximate_cubic_bezier(
                        last_point,
                        (p1.x, p1.y),
                        (p2.x, p2.y),
                        (p3.x, p3.y),
                        10,
                    );
                    current_stroke.extend(segments);
                }
            }
            usvg::tiny_skia_path::PathSegment::Close => {
                // Close the path by connecting to the first point
                if let Some(&first_point) = current_stroke.first() {
                    current_stroke.push(first_point);
                }
                if !current_stroke.is_empty() {
                    strokes.push(Stroke {
                        points: current_stroke.clone(),
                    });
                    current_stroke.clear();
                }
            }
        }
    }
    
    // Add any remaining stroke
    if !current_stroke.is_empty() {
        strokes.push(Stroke {
            points: current_stroke,
        });
    }
    
    strokes
}

/// Approximate a quadratic bezier curve with line segments
fn approximate_quad_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    segments: usize,
) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    
    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        
        let x = mt2 * p0.0 + 2.0 * mt * t * p1.0 + t2 * p2.0;
        let y = mt2 * p0.1 + 2.0 * mt * t * p1.1 + t2 * p2.1;
        
        points.push((x, y));
    }
    
    points
}

/// Approximate a cubic bezier curve with line segments
fn approximate_cubic_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    segments: usize,
) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    
    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        let x = mt3 * p0.0 + 3.0 * mt2 * t * p1.0 + 3.0 * mt * t2 * p2.0 + t3 * p3.0;
        let y = mt3 * p0.1 + 3.0 * mt2 * t * p1.1 + 3.0 * mt * t2 * p2.1 + t3 * p3.1;
        
        points.push((x, y));
    }
    
    points
}

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
