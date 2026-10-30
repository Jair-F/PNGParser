
## Parsing IDAT

PNG Pixel Reconstruction for minifb Rendering

To take your decompressed idat.bytes and turn them into a format that minifb can draw, you need to bridge a structural gap: idat.bytes is a flat sequence of filtered bytes, but minifb expects a 1D vector of u32 pixel values, where each u32 represents a complete 0xFFRRGGBB pixel.

Step 1: Mapping the Scanline Layout

Before writing loops, you need to understand how the decompressed data is structured in memory. The zlib-inflated bytes look like a repeating pattern of rows:
text
[Filter Byte 0] [ ... Bytes for Row 0 ... ] [Filter Byte 1] [ ... Bytes for Row 1 ... ] ...
Use code with caution.
To slice into this buffer correctly, calculate your widths:
• bpp (bytes per pixel): You already have this from your function! (e.g., 3 for RGB, 4 for RGBA).
• stride: The number of data bytes in a single row, which is width * bpp.
• Total scanline width: The total width of a row including the leading filter byte is 1 + stride.

Step 2: Setting up the Destination Buffers

You will want to allocate a destination buffer to hold your clean, unfiltered raw pixels. Since you are drawing to a window, you also want a separate buffer for minifb:
1. reconstructed_bytes: A Vec<u8> initialized to zero. Its size will be height * stride. This holds the un-filtered byte sequence, which you need because later rows refer back to previously reconstructed bytes.
2. buffer_u32: A Vec<u32> initialized to zero with a size of width * height. This will be what you pass to window.update_with_buffer().

Step 3: Reversing the Filters (The Core Logic)

You will loop through your image row by row (0..height). For every row, you extract the filter byte, and then process the next stride number of bytes.
To prevent Rust from panicking with an overflow error when you add raw pixel values together, use wrapping_add() instead of the + operator. In PNG specifications, all mathematical unfiltering is done modulo 256. u8::wrapping_add does exactly that.
Here is what your math loop needs to track for each byte at index i in the current row:
• Filter 0 (None): The output byte is exactly the input byte.
• Filter 1 (Sub): Look at the byte to the left in the current row. If i >= bpp, get the reconstructed byte at current_row_start + i - bpp. If it's the very first pixel in the row, the left value is 0.
• Filter 2 (Up): Look at the byte directly above in the previous row. If row > 0, get the reconstructed byte at current_row_start - stride + i. If it's the very first row, the above value is 0.
• Filter 3 (Average): Get both the Left and Above values (using the same boundary rules as above). The formula is input_byte.wrapping_add(((left as u16 + above as u16) / 2) as u8).
• Filter 4 (Paeth): Get the Left, Above, and Upper-Left (diagonal) values. Pass them to a helper function that implements the Paeth logic to find the closest neighbor, and then add it to your input byte.

Step 4: Converting Bytes to u32 for minifb

Once a row (or the whole image) is completely reconstructed into your reconstructed_bytes vector, you need to repack those separate color chunks into whole u32 tokens.
minifb expects pixels formatted as 0x00RRGGBB or 0xFFRRGGBB. You can build this using bitwise shift operators (<<) and bitwise OR operators (|).
If you are parsing a standard 8-bit RGB image (bpp = 3), you will loop through your reconstructed byte array 3 bytes at a time:
1. Grab r = bytes[idx], g = bytes[idx + 1], and b = bytes[idx + 2].
2. Convert them to u32 types: let r32 = r as u32;
3. Combine them: let pixel = (r32 << 16) | (g32 << 8) | b32;
4. Push or assign that pixel into your buffer_u32.
If your PNG is RGBA (bpp = 4), you just read 4 bytes at a time instead and shift the Alpha channel (a as u32 << 24) if you want to preserve transparency values.
Would you like to explore how the Paeth Predictor selects the left, above, or upper-left pixel using its distance formulas before you write it?
