use std::collections::HashMap;

pub const MAX_PALETTE_COLORS: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SixelColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_percentages(&self) -> (u8, u8, u8) {
        (
            ((self.r as u32 * 100) / 255) as u8,
            ((self.g as u32 * 100) / 255) as u8,
            ((self.b as u32 * 100) / 255) as u8,
        )
    }

    /// quantization to 6-6-6 color cube (216 colors)
    pub fn quantize(&self) -> Self {
        Self {
            r: (self.r / 51) * 51,
            g: (self.g / 51) * 51,
            b: (self.b / 51) * 51,
        }
    }

    /// quantization to 4-4-4 color cube (64 colors)
    pub fn quantize_64(&self) -> Self {
        Self {
            r: (self.r / 85) * 85,
            g: (self.g / 85) * 85,
            b: (self.b / 85) * 85,
        }
    }
}

pub struct SixelEncoder {
    output: String,
    palette: Vec<SixelColor>,
    color_map: HashMap<SixelColor, usize>,
    width: u32,
    height: u32,
}

impl SixelEncoder {
    pub fn new() -> Self {
        Self {
            output: String::with_capacity(16384),
            palette: Vec::with_capacity(MAX_PALETTE_COLORS),
            color_map: HashMap::with_capacity(MAX_PALETTE_COLORS),
            width: 0,
            height: 0,
        }
    }

    pub fn clear(&mut self) {
        self.output.clear();
        self.palette.clear();
        self.color_map.clear();
        self.width = 0;
        self.height = 0;
    }

    #[allow(dead_code)]
    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn into_output(self) -> String {
        self.output
    }

    fn add_color(&mut self, color: SixelColor) -> usize {
        if let Some(&idx) = self.color_map.get(&color) {
            return idx;
        }

        // If palette is full, quantize and try again
        if self.palette.len() >= MAX_PALETTE_COLORS {
            let quantized = color.quantize_64();
            if let Some(&idx) = self.color_map.get(&quantized) {
                return idx;
            }
            // Palette is truly full, return closest existing color
            return self.find_closest_color(color);
        }

        let idx = self.palette.len();
        self.palette.push(color);
        self.color_map.insert(color, idx);
        idx
    }

    fn find_closest_color(&self, color: SixelColor) -> usize {
        let mut best_idx = 0;
        let mut best_dist = u32::MAX;

        for (idx, &pal_color) in self.palette.iter().enumerate() {
            let dr = (color.r as i32 - pal_color.r as i32).abs() as u32;
            let dg = (color.g as i32 - pal_color.g as i32).abs() as u32;
            let db = (color.b as i32 - pal_color.b as i32).abs() as u32;
            let dist = dr * dr + dg * dg + db * db;
            if dist < best_dist {
                best_dist = dist;
                best_idx = idx;
            }
        }

        best_idx
    }

    fn build_palette(&mut self, pixels: &[(u8, u8, u8, u8)], width: u32, height: u32) -> Vec<Option<usize>> {
        self.width = width;
        self.height = height;

        let mut unique_colors: HashMap<SixelColor, usize> = HashMap::new();

        for &(r, g, b, a) in pixels {
            if a < 128 {
                continue; // Skip transparent pixels
            }
            let color = SixelColor::new(r, g, b);
            *unique_colors.entry(color).or_insert(0) += 1;
        }

        // If too many colors, quantize
        if unique_colors.len() > MAX_PALETTE_COLORS {
            unique_colors.clear();
            for &(r, g, b, a) in pixels {
                if a < 128 {
                    continue;
                }
                let color = SixelColor::new(r, g, b).quantize();
                *unique_colors.entry(color).or_insert(0) += 1;
            }
        }

        if unique_colors.len() > MAX_PALETTE_COLORS {
            unique_colors.clear();
            for &(r, g, b, a) in pixels {
                if a < 128 {
                    continue;
                }
                let color = SixelColor::new(r, g, b).quantize_64();
                *unique_colors.entry(color).or_insert(0) += 1;
            }
        }

        // Build palette from most common colors
        let mut color_counts: Vec<_> = unique_colors.into_iter().collect();
        color_counts.sort_by(|a, b| b.1.cmp(&a.1));

        for (color, _) in color_counts.into_iter().take(MAX_PALETTE_COLORS) {
            self.add_color(color);
        }

        // Second pass: map pixels to palette indices
        let mut pixel_indices = Vec::with_capacity(pixels.len());
        for &(r, g, b, a) in pixels {
            if a < 128 {
                pixel_indices.push(None); // Transparent
            } else {
                let color = SixelColor::new(r, g, b);
                let quantized = if self.color_map.contains_key(&color) {
                    color
                } else {
                    let q = color.quantize();
                    if self.color_map.contains_key(&q) {
                        q
                    } else {
                        color.quantize_64()
                    }
                };

                if let Some(&idx) = self.color_map.get(&quantized) {
                    pixel_indices.push(Some(idx));
                } else {
                    // Fallback: find closest color
                    pixel_indices.push(Some(self.find_closest_color(color)));
                }
            }
        }

        pixel_indices
    }

    fn write_header(&mut self) {
        // DCS P1 ; P2 ; P3 q
        // P1 = pixel aspect ratio (0 = let terminal decide)
        // P2 = background select (0 = device default, 1 = don't clear, 2 = set to color 0)
        // P3 = horizontal grid size (0 = no grid)
        self.output.push_str("\x1bPq");
    }

    /// Write the palette definitions
    fn write_palette(&mut self) {
        let palette_data: Vec<_> = self.palette.iter().map(|c| c.to_percentages()).collect();
        for (idx, (r, g, b)) in palette_data.into_iter().enumerate() {
            // #Pc;Pu;Px;Py;Pz - define color Pc using color system Pu (2=RGB)
            self.output.push('#');
            self.write_number(idx);
            self.output.push_str(";2;");
            self.write_number(r as usize);
            self.output.push(';');
            self.write_number(g as usize);
            self.output.push(';');
            self.write_number(b as usize);
        }
    }

    fn write_number(&mut self, n: usize) {
        if n == 0 {
            self.output.push('0');
            return;
        }
        let mut buf = [0u8; 20];
        let mut i = 20;
        let mut n = n;
        while n > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        self.output.push_str(unsafe { std::str::from_utf8_unchecked(&buf[i..]) });
    }

    fn write_footer(&mut self) {
        self.output.push_str("\x1b\\");
    }

    /// Encode pixel data as sixel graphics
    ///
    /// # Arguments
    /// * `pixels` - RGBA pixel data (r, g, b, a) for each pixel
    /// * `width` - Image width in pixels
    /// * `height` - Image height in pixels
    ///
    /// # Returns
    /// The sixel-encoded string
    pub fn encode(&mut self, pixels: &[(u8, u8, u8, u8)], width: u32, height: u32) -> &str {
        if width == 0 || height == 0 || pixels.is_empty() {
            return "";
        }

        self.clear();

        // Build palette and get pixel indices
        let pixel_indices = self.build_palette(pixels, width, height);

        // Write sixel header
        self.write_header();

        // Write palette definitions
        self.write_palette();

        // Encode the image data
        self.encode_image_data(&pixel_indices);

        // Write footer
        self.write_footer();

        &self.output
    }

    fn encode_image_data(&mut self, pixel_indices: &[Option<usize>]) {
        let width = self.width as usize;
        let height = self.height as usize;
        let num_colors = self.palette.len();

        // Process 6 rows at a time (one sixel band)
        let mut band = 0;
        while band * 6 < height {
            let band_start_y = band * 6;
            let band_end_y = (band_start_y + 6).min(height);

            // For each color in palette
            for color_idx in 0..num_colors {
                let mut has_pixels = false;
                let mut sixel_row: Vec<u8> = vec![0; width];

                // Build sixel row for this color
                for x in 0..width {
                    let mut sixel_value: u8 = 0;
                    for dy in 0..(band_end_y - band_start_y) {
                        let y = band_start_y + dy;
                        let pixel_idx = y * width + x;
                        if let Some(Some(pix_color)) = pixel_indices.get(pixel_idx) {
                            if *pix_color == color_idx {
                                sixel_value |= 1 << dy;
                                has_pixels = true;
                            }
                        }
                    }
                    sixel_row[x] = sixel_value;
                }

                // Only output if this color has pixels in this band
                if has_pixels {
                    // Select color
                    self.output.push('#');
                    self.write_number(color_idx);

                    // Output sixel data with RLE compression
                    self.write_sixel_row_rle(&sixel_row);

                    // Carriage return to start of band
                    self.output.push('$');
                }
            }

            // Move to next sixel band
            self.output.push('-');
            band += 1;
        }
    }

    /// Write a sixel row with run-length encoding
    fn write_sixel_row_rle(&mut self, row: &[u8]) {
        if row.is_empty() {
            return;
        }

        let mut i = 0;
        while i < row.len() {
            let current = row[i];
            let sixel_char = (current + 0x3F) as char; // Convert to sixel character (? = 0)

            let mut count = 1;
            while i + count < row.len() && row[i + count] == current {
                count += 1;
            }

            if count >= 3 {
                // Use RLE: !<count><char>
                self.output.push('!');
                self.write_number(count);
                self.output.push(sixel_char);
            } else {
                // Output characters directly
                for _ in 0..count {
                    self.output.push(sixel_char);
                }
            }

            i += count;
        }
    }

    #[allow(dead_code)]
    pub fn encode_rgba(&mut self, buffer: &[u8], width: u32, height: u32) -> &str {
        if buffer.len() < (width * height * 4) as usize {
            return "";
        }

        let pixels: Vec<(u8, u8, u8, u8)> = buffer.chunks(4).map(|c| (c[0], c[1], c[2], c[3])).collect();

        self.encode(&pixels, width, height)
    }

    #[allow(dead_code)]
    pub fn encode_rgb(&mut self, buffer: &[u8], width: u32, height: u32) -> &str {
        if buffer.len() < (width * height * 3) as usize {
            return "";
        }

        let pixels: Vec<(u8, u8, u8, u8)> = buffer.chunks(3).map(|c| (c[0], c[1], c[2], 255)).collect();

        self.encode(&pixels, width, height)
    }
}

impl Default for SixelEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sixel_color_percentages() {
        let color = SixelColor::new(255, 127, 0);
        let (r, g, b) = color.to_percentages();
        assert_eq!(r, 100);
        assert_eq!(g, 49);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_sixel_color_quantize() {
        let color = SixelColor::new(100, 150, 200);
        let quantized = color.quantize();
        // 100/51 = 1, 1*51 = 51
        // 150/51 = 2, 2*51 = 102
        // 200/51 = 3, 3*51 = 153
        assert_eq!(quantized.r, 51);
        assert_eq!(quantized.g, 102);
        assert_eq!(quantized.b, 153);
    }

    #[test]
    fn test_sixel_encode_simple() {
        let mut encoder = SixelEncoder::new();

        let pixels = vec![(255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255), (255, 0, 0, 255)];

        let output = encoder.encode(&pixels, 2, 2);

        assert!(output.starts_with("\x1bPq"));
        assert!(output.ends_with("\x1b\\"));
        assert!(output.contains("#0;2;"));
    }

    #[test]
    fn test_sixel_encode_with_transparency() {
        let mut encoder = SixelEncoder::new();

        let pixels = vec![(255, 0, 0, 255), (0, 0, 0, 0), (0, 255, 0, 255), (0, 0, 255, 255)];

        let output = encoder.encode(&pixels, 2, 2);

        assert!(output.starts_with("\x1bPq"));
        assert!(output.ends_with("\x1b\\"));
    }

    #[test]
    fn test_sixel_rle() {
        let mut encoder = SixelEncoder::new();

        let pixels: Vec<_> = (0..10).map(|_| (128, 128, 128, 255)).collect();

        let output = encoder.encode(&pixels, 10, 1);

        assert!(output.contains("!"));
    }
}
