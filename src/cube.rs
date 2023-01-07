//! Module for creating and interacting
//! with the orientation of a square 1
//! cube in order to produce a scramble.

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
enum Color {
    White = 0,
    Yellow,
    Blue = 3,
    Green,
    Red = 6,
    Orange,
}

/// Returns true if the combination 
/// of colors is possible on the
/// cube. (Opposite colors, ex. yellow
/// and white, cannot be on the same piece)
fn possible(c1: u8, c2: u8) -> bool {
    c1.abs_diff(c2) != 1
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct EdgeColor {
    colors: (Color, Color),
}

impl EdgeColor {
    
    /// Returns true if the edge created
    /// contains a possible combination
    /// of colors
    pub fn possible(&self) -> bool {
        possible(self.colors.0 as u8, self.colors.1 as u8)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct CornerColor {
    colors: (Color, Color, Color),
}

impl CornerColor {

    /// Function returns true if the
    /// combination of colors is possible
    /// for the corner
    pub fn possible(&self) -> bool {
        possible(self.colors.0 as u8, self.colors.1 as u8)
            && possible(self.colors.1 as u8, self.colors.2 as u8)
            && possible(self.colors.0 as u8, self.colors.2 as u8)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Piece {
    Edge(EdgeColor) = 1,
    Corner(CornerColor),
}

const COLOR_ORDER: [Color; 4] = [
    Color::Green,
    Color::Orange,
    Color::Blue,
    Color::Red,
];

#[derive(Debug)]
pub struct SqOne {
    top: Vec<Option<Piece>>,
    top_offset: i8,
    bottom: Vec<Option<Piece>>,
    bottom_offset: i8,
    middle: bool,
}

/// Returns the value of a % n
/// where the result is always
/// positive. This is different
/// than the default Rust behavior.
///
/// Ex:     -1 % 4 == -1, abs_mod(-1, 4) == 3
fn abs_mod(a: i8, n: i8) -> u8 {
    let mut a = a;
    while a < 0 {
        a += n;
    }
    (a % n) as u8
}

impl SqOne {

    /// Produces a Square One
    /// configuration with the
    /// default colors and pieces
    /// of a solved Square One.
    pub fn new() -> SqOne {
        
        // Produce the top layer
        // and bottom layer color
        // configuration.
        let top = SqOne::gen_layer(true);
        let bottom = SqOne::gen_layer(false);

        SqOne {
            top,
            top_offset: 0,
            bottom,
            bottom_offset: 0,
            middle: false,
        }
    }

    /// Creates a vector containing the
    /// corner and edge color orientation
    /// of the layer of the cube. If the
    /// is_top_layer is set, the top color
    /// will be white. Otherwise, it will
    /// be yellow.
    fn gen_layer(is_top_layer: bool) -> Vec<Option<Piece>> {
        
        // Set the top color of
        // the layer
        let top_color = if is_top_layer {
            Color::White
        } else {
            Color::Yellow
        };

        let mut layer = Vec::with_capacity(12);
        for i in 0usize..4 {

            // Create the i-th corner
            // piece
            let corner = Piece::Corner(
                CornerColor {
                    colors: (
                        COLOR_ORDER[abs_mod(i as i8 - 1, 4) as usize],
                        COLOR_ORDER[i],
                        top_color,
                    ),
                }
            );

            // Create the i-th edge piece
            let edge = Piece::Edge(
                EdgeColor {
                    colors: (
                        COLOR_ORDER[i],
                        top_color,
                    ),
                }
            );

            // Add the corner
            // and edge
            layer.push(Some(corner));
            layer.push(None);
            layer.push(Some(edge));
        }

        layer
    }

    /// Returns true if the layer
    /// slice specified can be
    /// flipped.
    fn can_flip_layer(layer: &[Option<Piece>], offset: i8) -> bool {
  
        // If the front or back piece is
        // partway through a corner piece,
        // then the layer cannot be flipped.
        if let Some(Piece::Corner(_)) = layer[abs_mod(5 - offset, 12) as usize] {
            return false;
        } else if let Some(Piece::Corner(_)) = layer[abs_mod(11 - offset, 12) as usize] {
            return false;
        }

        true
    }

    /// Returns true if the current
    /// configuration of the top and
    /// bottom layer is able to
    /// be flipped.
    fn can_flip(&self) -> bool {
        SqOne::can_flip_layer(&self.top[..], self.top_offset)
            && SqOne::can_flip_layer(&self.bottom[..], self.bottom_offset)
    }

    /// When provided a layer of the
    /// Square One, it will consume the
    /// later half of the layer and
    /// return the properly reversed
    /// vector.
    fn get_reverse(layer: &Vec<Option<Piece>>, offset: i8) -> Vec<Option<Piece>> {

        let mut layer = layer.clone();

        // Initialize a new vector
        // to store the reverse of
        // half of the provided layer.
        // Iter will be used to iterate
        // across the layer, and end
        // is the index after all 
        // pieces that should be reversed.
        let mut reverse: Vec<Option<Piece>> = vec![];
        let mut iter = abs_mod(11 - offset, 12);
        let end = abs_mod(5 - offset, 12);

        loop {

            // If the end is reached,
            // return the reversed vector
            if iter == end {
                return reverse;
            }

            match &layer[iter as usize] {

                // If the value is Some, just
                // add that value to the vector
                Some(_) => reverse.push(layer[iter as usize].take()),

                // If it is None, then this
                // is in the middle of a corner
                // piece. This and the next need
                // to be added in reverse order.
                None => {
                    reverse.push(layer[abs_mod(iter as i8 - 1, 12) as usize].take());
                    reverse.push(None);
                    iter = abs_mod(iter as i8 - 1, 12);
                },
            }

            // Decrement the index mod
            // the size of the layer
            iter = abs_mod(iter as i8 - 1, 12);
        }
    }

    /// Flips the top and bottom
    /// layer of the cube with
    /// the alignment and piece/color
    /// orientation.
    fn flip(&mut self) {
        if !self.can_flip() {
            return;
        }

        // Get the reverse of half
        // of the bottom and top layers.
        let mut top_reverse = SqOne::get_reverse(&mut self.top, self.top_offset);
        let mut bottom_reverse = SqOne::get_reverse(&mut self.bottom, self.bottom_offset);
        let mut index = 0;

        // Assign the flipped half
        // into the opposite layer.
        for i in 6..12 {
            self.top[abs_mod(i - self.top_offset, 12) as usize] = bottom_reverse[index].take();
            self.bottom[abs_mod(i - self.bottom_offset, 12) as usize] = top_reverse[index].take();
            index += 1;
        }

        self.middle = !self.middle;
    }

    /// Twists the cube by the provided
    /// offset. This follows the standard
    /// Square One notation.
    fn twist(&mut self, top_offset: i8, bottom_offset: i8) {
        let top_sum = self.top_offset + top_offset;
        let bot_sum = self.bottom_offset - bottom_offset;
        self.top_offset = abs_mod(top_sum + 5, 12) as i8 - 5;
        self.bottom_offset = abs_mod(bot_sum + 5, 12) as i8 - 5;
    }

    /// Generates a random offset value
    /// for the given layer.
    fn rand_layer_offset(layer: &[Option<Piece>], offset: i8) -> i8 {
        use rand::{thread_rng, Rng};

        // Loop through until an offset
        // that would allow this layer
        // to flip is found.
        let mut rng = rand::thread_rng();
        let r = loop {
            let rnum = rng.gen_range(-5..6 + 1);
            if SqOne::can_flip_layer(layer, rnum + offset) {
                break rnum;
            }
        };

        if r == -6 {
            6
        } else {
            r
        }
    }

    /// Scrambles the cube using NUM_FLIPS
    /// twists/flips, and then returns the
    /// list of twists to get the scramble.
    pub fn scramble(&mut self) -> Vec<(i8, i8)> {
        const NUM_FLIPS: u32 = 20;
        let mut twists = vec![];

        // Generate NUM_FLIPS random
        // twists to scramble the cube.
        for _ in 0..NUM_FLIPS {

            // Get a random top and 
            // bottom layer offset to
            // turn the cube by. Both
            // the top and bottom cannot
            // be equal to 0.
            let top_layer_offset = Self::rand_layer_offset(&self.top[..], self.top_offset);
            let mut bot_layer_offset = Self::rand_layer_offset(&self.bottom[..], self.bottom_offset);
            if top_layer_offset == 0 {
                while bot_layer_offset == 0 {
                    bot_layer_offset = Self::rand_layer_offset(&self.bottom[..], self.bottom_offset);
                }
            }

            // Twist the cube by the random
            // top and bottom offsets, and
            // flip the cube for the next
            // random offset.
            self.twist(top_layer_offset, -bot_layer_offset);
            self.flip();

            // Add the twist offset values
            // to the list to return.
            twists.push((top_layer_offset, -bot_layer_offset));
        }

        twists
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Creates a new Square One
    // and asserts that the cube
    // is in a solved configuration.
    #[test]
    fn create_solved_cube() {
        let test_cube = SqOne::new();
        assert_eq!(test_cube.top_offset, 0);
        assert_eq!(test_cube.bottom_offset, 0);
        assert_eq!(test_cube.middle, false);
    }

    #[test]
    fn flip_solved_cube() {
        let mut test_cube = SqOne::new();
        test_cube.flip();
    }

    #[test]
    fn twist_cube() {
        let mut test_cube = SqOne::new();
        test_cube.twist(4, -1);
        test_cube.flip();
        test_cube.twist(-3, 6);
        test_cube.flip();
    }

    #[test]
    fn scramble_cube() {
        let mut test_cube = SqOne::new();
        let scramble = test_cube.scramble();
        dbg!(scramble);
    }
}
