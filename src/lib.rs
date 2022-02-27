use {
    ascii::AsciiStr,
    std::fmt::{self, Write},
};

const LINE_FEED: u8 = 10;
const SPACE: u8 = 32;
const ZERO: u8 = 48;
const UPPERCASE_A: u8 = 65;
const LOWERCASE_A: u8 = 97;

#[derive(Copy, Clone)]
pub struct Buffer {
    points: [[u16; 2]; 60],
    width: u16,
    height: u16,
    len: usize,
}

macro_rules! shrink_to_fit {
    ($self:ident, $dimension:expr, $width_or_height:ident) => {{
        let points = &mut $self.points[..$self.len];
        unsafe {
            if let Some(leftmost) = points
                .iter()
                .min_by_key(|p| p.get_unchecked($dimension))
                .cloned()
            {
                let offset = leftmost.get_unchecked($dimension);
                for i in 0..$self.len {
                    *points.get_unchecked_mut(i).get_unchecked_mut($dimension) -= offset;
                }
                $self.$width_or_height -= offset;
            }
        }
    }};
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            points: [[0; 2]; 60],
            width: 0,
            height: 0,
            len: 0,
        }
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Self {
        let mut new = Self::new();
        new.set_from_str(string);
        new
    }

    pub fn from_points(iter: impl IntoIterator<Item = [u16; 2]>) -> Self {
        let mut new = Self::new();
        new.set_from_points(iter);
        new
    }

    pub fn points(&self) -> &[[u16; 2]] {
        &self.points[..self.len]
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn set_from_str(&mut self, string: &str) {
        self.len = 0;
        self.width = 0;
        self.height = 0;
        for (y, line) in string.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    self.set((c as u8 - ZERO) as _, x as _, y as _);
                } else if c.is_ascii_uppercase() {
                    self.set((c as u8 - UPPERCASE_A + 10) as _, x as _, y as _);
                } else if c.is_ascii_lowercase() {
                    self.set((c as u8 - LOWERCASE_A + 35) as _, x as _, y as _);
                }
            }
        }
        shrink_to_fit!(self, 0, width);
        shrink_to_fit!(self, 1, height);
    }

    pub fn set_from_points(&mut self, iter: impl IntoIterator<Item = [u16; 2]>) {
        self.len = 0;
        self.width = 0;
        self.height = 0;
        let mut src_iter = iter.into_iter();
        for i in 0..60 {
            if let Some(src) = src_iter.next() {
                self.set(i, src[0], src[1]);
            } else {
                break;
            }
        }
        shrink_to_fit!(self, 0, width);
        shrink_to_fit!(self, 1, height);
    }

    fn set(&mut self, index: usize, x: u16, y: u16) {
        if x >= self.width {
            self.width = x + 1;
        }
        if y >= self.height {
            self.height = y + 1;
        }
        unsafe { *self.points.get_unchecked_mut(index as usize) = [x, y] };
        self.len += 1;
    }
}

#[derive(Default)]
pub struct Writer(Vec<u8>);

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn write(&mut self, dest: &mut impl Write, src: &Buffer) -> fmt::Result {
        let line = src.width as usize + 1;
        self.clear_and_resize(line, (line * src.height as usize).saturating_sub(1));
        if src.len < 10 {
            self.set_each(&src.points[..src.len], line, ZERO);
        } else {
            self.set_each(&src.points[..10], line, ZERO);
            if src.len < 35 {
                self.set_each(&src.points[10..src.len], line, UPPERCASE_A)
            } else {
                self.set_each(&src.points[10..src.len], line, UPPERCASE_A);
                self.set_each(&src.points[35..src.len], line, LOWERCASE_A)
            }
        }
        let string = unsafe { AsciiStr::from_ascii_unchecked(&self.0) };
        dest.write_str(string.as_str())
    }

    fn clear_and_resize(&mut self, line: usize, len: usize) {
        if len > self.0.len() {
            self.0.fill(SPACE);
            self.0.resize(len, SPACE);
        } else {
            self.0.truncate(len);
            self.0.fill(SPACE);
        }
        for i in (line.saturating_sub(1)..len).step_by(line) {
           
            unsafe { *self.0.get_unchecked_mut(i) = LINE_FEED }
        }
    }

    fn set_each(&mut self, points: &[[u16; 2]], line: usize, mut start_ascii: u8) {
        for point in points {
            unsafe {
                let x = *point.get_unchecked(0) as usize;
                let y = *point.get_unchecked(1) as usize;
                let index = x + y * line;
                *self.0.get_unchecked_mut(index) = start_ascii;
            }
            start_ascii += 1;
        }
    }
}
