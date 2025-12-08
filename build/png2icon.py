#!/usr/bin/env python3

import sys
import argparse
import struct

def error_exit(message, code):
    print(message, file=sys.stderr)
    sys.exit(code)

try:
    import png
except ImportError:
    error_exit("You need pypng to run this script.\n  Use `pip3 install pypng`", 1)

try:
    import lz4.block
except ImportError:
    error_exit("You need lz4 to run this script.\n  Use `pip3 install lz4`", 1)


def main():
    # Parse args.
    parser = argparse.ArgumentParser()
    parser.add_argument("input")
    parser.add_argument("output")
    args = parser.parse_args()
    
    with open(args.input, 'rb') as input_file:
        r = png.Reader(file=input_file)

        width, height, rows, infos = r.asRGBA8()
        
        if width != 55 or height != 56:
            error_exit("Icon must be 55x56 !", 2)
        
        output = bytearray(width * height * 2)
        colors = list(rows)
        
        for y in range(height):
            for x in range(width):
                
                red   = colors[y][x * 4 + 0] / 255
                green = colors[y][x * 4 + 1] / 255
                blue  = colors[y][x * 4 + 2] / 255
                alpha = colors[y][x * 4 + 3] / 255
                
                Bred   = red   * alpha + 1 * (1 - alpha)
                Bgreen = green * alpha + 1 * (1 - alpha)
                Bblue  = blue  * alpha + 1 * (1 - alpha)
                
                Ired   = int(Bred   * 0xFF)
                Igreen = int(Bgreen * 0xFF)
                Iblue  = int(Bblue  * 0xFF)
                
                rgb565value = (Ired >> 3) << 11 | (Igreen >> 2) << 5 | (Iblue >> 3)
                
                i = (y * width + x) * 2
                struct.pack_into("<H", output, i, rgb565value)

        compressed = lz4.block.compress(output, mode="high_compression", store_size=False, return_bytearray=True)
        
        with open(args.output, 'wb') as output_file:
            output_file.write(compressed)

if __name__ == "__main__":
    main()

