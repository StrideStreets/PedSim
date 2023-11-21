use image::{io::Reader, ImageError, ImageFormat, Luma};
use ndarray::Array2;

pub fn read_raster(filepath: String) -> Result<Array2<u8>, ImageError> {
    match Reader::open(filepath)?.with_guessed_format() {
        Ok(mut buffered_reader) => {
            //buffered_reader.set_format(ImageFormat::Tiff);
            match buffered_reader.decode() {
                Ok(img) => {
                    let (width, height) = (img.width(), img.height());
                    let mut binary_pixel_matrix =
                        Array2::<u8>::default((width as usize, height as usize));
                    img.into_luma8()
                        .enumerate_pixels()
                        .for_each(|(col, row, p)| match p {
                            Luma([0]) => {
                                binary_pixel_matrix[[row as usize, col as usize]] = 0;
                            }
                            _ => {
                                binary_pixel_matrix[[row as usize, col as usize]] = 0;
                            }
                        });

                    return Ok(binary_pixel_matrix);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            return Err(ImageError::IoError(e));
        }
    }
}
