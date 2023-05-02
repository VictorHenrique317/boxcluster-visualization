import sys
from PIL import Image

def resize_image(image_path, output_path, size):
    original_image = Image.open(image_path)
    width, height = original_image.size
    resized_image = original_image.resize(size)
    resized_image.save(output_path)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python resize_image.py <width> <height>")
        sys.exit()
    image_path = "logo.png"
    new_width = int(sys.argv[1])
    new_height = int(sys.argv[2])
    output_path = f"logo{new_width}x{new_height}.png"
    size = (new_width, new_height)
    resize_image(image_path, output_path, size)
