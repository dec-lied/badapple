struct RGBPixel<'a>(&'a u8, &'a u8, &'a u8);

impl<'a> RGBPixel<'a>
{
    fn brightness(&'a self) -> u8
    {
        return ((*self.0 as f32 * 0.299f32) + (*self.1 as f32 * 0.587f32) + (*self.2 as f32 * 0.114f32)) as u8;
    }

    fn density_index(&'a self) -> usize // 0-5
    {
        return (self.brightness() as f32 / 3.69565217391f32).round() as usize;
    }

    fn from_u8(arr: &'a [u8; 3]) -> RGBPixel<'a>
    {
        return RGBPixel(&arr[0], &arr[1], &arr[2]);
    }
}

fn main()
{
    if let Ok(available_files) = std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "\\frames"))
    {
        let density: [char; 70] =
        [
            '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 
            'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w', 'm',
            'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 
            'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j', 'f', 't',
            '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']',
            '?', '-', '_', '+', '~', '<', '>', 'i', '!', 'l',
            'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' '
        ];

        for frame in available_files
        {
            if let Ok(file) = frame
            {
                let image= image::open(&file.path()).unwrap().into_rgb8();

                let width: &u32 = &image.dimensions().0;
                let height: &u32 = &image.dimensions().1;
            
                let mut image_string: String = String::with_capacity(((width * height) + height) as usize);
            
                for (index, pixel) in image.pixels().enumerate()
                {
                    if (&index + 1) % *width as usize== 0
                    {
                        image_string.push('\n');
                    }
            
                    let rgb_pixel: RGBPixel = RGBPixel::from_u8(&pixel.0);
            
                    image_string.push(density[rgb_pixel.density_index()]);
                }
            
                println!("{}", image_string);
                std::thread::sleep(std::time::Duration::from_micros(500))
            }
        }
    }
}
