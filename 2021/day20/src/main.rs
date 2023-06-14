use std::collections::HashSet;
use std::fmt::Write;
use std::io::BufRead;

#[derive(Debug)]
enum Error {
    ArgumentMissing,
    FileMissing(String),
    InvalidInputFormat,
    InvalidAlgorithmLength(usize),
    InvalidCharacterInAlgorithm(char),
    FileReadError,
}

trait ToError<T,E> {
    fn check(self, error: E) -> Result<T,E>;
}

impl<T,E> ToError<T,E> for Option<T> {
    fn check(self, error: E) -> Result<T,E> {
        self.ok_or(error)
    }
}

impl<T,E,_E> ToError<T,E> for Result<T,_E> {
    fn check(self, error: E) -> Result<T,E> {
        self.ok().check(error)
    }
}

struct EnhanceableImage {
    algorithm: HashSet<usize>,
    pixels: HashSet<(isize,isize)>,
    dark_mode: bool,
}

struct BoundingBox {
    left: isize,
    top: isize,
    right: isize,
    bottom: isize,
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox{left:0, top:0, right:0, bottom:0}
    }
}

impl BoundingBox {
    fn with_margin(self, margin: isize) -> BoundingBox {
        BoundingBox {
            left: self.left - margin,
            top: self.top + margin,
            right: self.right + margin,
            bottom: self.bottom - margin,
        }
    }
}

trait PixelMap {
    fn get_bounding_box(&self) -> BoundingBox;
}

impl PixelMap for HashSet<(isize,isize)> {
    fn get_bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::default();
        for &(x,y) in self.iter() {
            if x < bb.left {
                bb.left = x;
            }
            else if x > bb.right {
                bb.right = x;
            }
            if y < bb.bottom {
                bb.bottom = y;
            } else if y > bb.top {
                bb.top = y;
            }
        }
        bb
    }
}

impl EnhanceableImage {

    fn is_pixel_lit(&self, x: isize, y: isize) -> bool {
        self.pixels.contains(&(x,y)) == self.dark_mode
    }

    fn is_enhanced_pixel_lit(&self, x: isize, y: isize) -> bool {
        let mut index: usize = 0;
        for dy in [1,0,-1] {
            for dx in [-1,0,1] {
                index <<= 1;
                if self.is_pixel_lit(x+dx, y+dy) {
                    index |= 1;
                }
            }
        }
        self.algorithm.contains(&index)
    }

    fn enhance(&mut self) {
        let bbox = self.pixels.get_bounding_box().with_margin(1);
        let mut pixels = HashSet::new();
        let dark_mode = !self.is_enhanced_pixel_lit(bbox.top + 1, bbox.right + 1);

        for y in (bbox.bottom..=bbox.top).rev() {
            for x in bbox.left..=bbox.right {
                if dark_mode == self.is_enhanced_pixel_lit(x, y) {
                    pixels.insert((x,y));
                }
            }
        }

        self.pixels = pixels;
        self.dark_mode = dark_mode;
    }

    fn read(file_name: String) -> Result<Self,Error> {
        let file = std::fs::File::open(&file_name)
            .check(Error::FileMissing(file_name))?;
        let lines: Result<Vec<String>,Error> = std::io::BufReader::new(file).lines()
            .map(|l| l.check(Error::FileReadError)).collect();
        let mut lines = lines?;
        lines.reverse();
        let program = lines.pop()
            .check(Error::InvalidInputFormat)?;
        if program.len() != 0b1_000_000_000 {
            return Err(Error::InvalidAlgorithmLength(program.len()));
        }

        fn parse<T>(c: char, t: T) -> Option<Result<T,Error>> {
            match c {
                '#' => Some(Ok(t)),
                '.' => None,
                 _  => Some(Err(Error::InvalidCharacterInAlgorithm(c)))
            }
        }

        let algorithm: Result<HashSet<_>,Error> = program
            .chars().enumerate().filter_map(|(k,c)| parse(c, k)).collect();
        let algorithm = algorithm?;

        let pixels: Result<HashSet<_>,Error> = lines
            .iter()
            .skip_while(|s| s.len() == 0)
            .take_while(|s| s.len() != 0)
            .zip(0..)
            .flat_map(|(row,y)| {
                row.chars().zip(0..).filter_map(move |(c,x)| parse(c, (x,y)))
            }).collect();
        let pixels = pixels?;
        Ok(EnhanceableImage {algorithm, pixels, dark_mode: true })
    }

}

impl std::fmt::Display for EnhanceableImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bbox = self.pixels.get_bounding_box();
        for y in (bbox.bottom..=bbox.top).rev() {
            for x in bbox.left..=bbox.right {
                f.write_char(if self.is_pixel_lit(x,y) {'#'} else {'.'})?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}


fn main_or_error() -> Result<(),Error> {
    let mut img = EnhanceableImage::read(
        std::env::args().nth(1).check(Error::ArgumentMissing)?)?;
    
    img.enhance();
    img.enhance();
    
    println!("pixels lit round 1: {}", img.pixels.len());
    
    for _ in 1..=(50-2) {
        img.enhance();
    }
    
    println!("pixels lit round 2: {}", img.pixels.len());

    Ok(())
}

fn main() {
    if let Err(e) = main_or_error() {
        println!("error: {:?}", e);
    }
}
