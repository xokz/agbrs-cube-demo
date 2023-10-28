#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![deny(clippy::all)]

use agb::{
    display::bitmap4::Page,
    fixnum::{num, FixedNum, Num, Vector2D},
};

use crate::geometry::*;
use crate::matrix::*;

pub trait Draw {
    fn draw_line(&mut self, a: Vector2D<DefaultNum>, b: Vector2D<DefaultNum>, color: u8);
    fn draw_tri(&mut self, tri: &Triangle2D<DefaultNum>, color: u8);
}

impl Draw for agb::display::bitmap4::Bitmap4<'_> {
    fn draw_line(&mut self, a: Vector2D<DefaultNum>, b: Vector2D<DefaultNum>, color: u8) {
        let a = a.trunc();
        let b = b.trunc();
        let mut x = a.x;
        let mut y = a.y;
        let w = b.x - a.x;
        let h = b.y - a.y;
        let dx1 = {
            if w < 0 {
                -1
            } else if w > 0 {
                1
            } else {
                0
            }
        };
        let dy1 = {
            if h < 0 {
                -1
            } else if h > 0 {
                1
            } else {
                0
            }
        };
        let mut dx2 = dx1;
        let mut dy2 = 0;
        let mut longest = w.abs();
        let mut shortest = h.abs();
        if longest <= shortest {
            longest = h.abs();
            shortest = w.abs();
            if h < 0 {
                dy2 = -1;
            } else if h > 0 {
                dy2 = 1;
            }
            dx2 = 0;
        }
        let mut numerator = longest >> 1;
        for i in 0..longest {
            self.draw_point(x as i32, y as i32, color);
            numerator += shortest;
            if numerator >= longest {
                numerator -= longest;
                x += dx1;
                y += dy1;
            } else {
                x += dx2;
                y += dy2;
            }
        }
    }
    fn draw_tri(&mut self, tri: &Triangle2D<DefaultNum>, color: u8) {
        // Uses a slightly modified version of the bresenham fill method
        static mut X_BOUNDS: [usize; 160] = [0; 160];
        let tri = [tri[0].trunc(), tri[1].trunc(), tri[2].trunc()];

        // find point that is inbetween the other verticies on the y-axis
        let midpoint_index: usize = {
            if tri[0].y > tri[1].y {
                if tri[1].y > tri[2].y {
                    1
                } else if tri[0].y > tri[2].y {
                    2
                } else {
                    0
                }
            } else {
                if tri[0].y > tri[2].y {
                    0
                } else if tri[1].y > tri[2].y {
                    2
                } else {
                    1
                }
            }
        };

        // this edge is the tallest
        let outside_indicies = {
            match midpoint_index {
                0 => (1, 2),
                1 => (0, 2),
                _ => (0, 1),
            }
        };

        let edges = [
            (tri[midpoint_index], tri[outside_indicies.0]),
            (tri[midpoint_index], tri[outside_indicies.1]),
            (tri[outside_indicies.0], tri[outside_indicies.1]),
        ];

        for edge in 0..=2 {
            let mut x = edges[edge].0.x;
            let mut y = edges[edge].0.y;
            let w = edges[edge].1.x - edges[edge].0.x;
            let h = edges[edge].1.y - edges[edge].0.y;
            let dx1 = {
                if w < 0 {
                    -1
                } else if w > 0 {
                    1
                } else {
                    0
                }
            };
            let dy1 = {
                if h < 0 {
                    -1
                } else if h > 0 {
                    1
                } else {
                    0
                }
            };
            let mut dx2 = dx1;
            let mut dy2 = 0;
            let mut longest = w.abs();
            let mut shortest = h.abs();
            if longest <= shortest {
                longest = h.abs();
                shortest = w.abs();
                if h < 0 {
                    dy2 = -1;
                } else if h > 0 {
                    dy2 = 1;
                }
                dx2 = 0;
            }
            let mut numerator = longest >> 1;

            if edge < 2 {
                // get all furthest points on the x-axis that the two shortest edges of the triangle reach.
                for _ in 0..=longest {
                    if y >= 0 && y <= 159 {
                        unsafe {
                            X_BOUNDS[y as usize] = x.clamp(0, 239) as usize;
                        }
                    }
                    numerator += shortest;
                    if numerator >= longest {
                        numerator -= longest;
                        x += dx1;
                        y += dy1;
                    } else {
                        x += dx2;
                        y += dy2;
                    }
                }
            } else {
                // if all the bounds are set, draw the triangle. this is the last step.
                let mut prev_y: i32 = -1;
                for i in 0..longest {
                    // if the y-axis has changed, draw a horizontal line up to the furthest point on the x-axis provided by the two shortest edges.
                    if prev_y != y && (y >= 0 && y <= 159) {
                        {
                            let (start, end) = {
                                unsafe {
                                    if x > X_BOUNDS[y as usize] as i32 {
                                        (X_BOUNDS[y as usize] as i32, x)
                                    } else {
                                        (x, X_BOUNDS[y as usize] as i32)
                                    }
                                }
                            };
                            self.draw_point(start, y, color);
                            for x in start..end {
                                if x & 1 == 0 && (x >= 0 && x <= 239) && (y >= 0 && y <= 159) {
                                    self.draw_wide_point(x, y, color);
                                }
                            }
                            self.draw_point(end, y, color);
                        }
                        prev_y = y;
                    }
                    numerator += shortest;
                    if numerator >= longest {
                        numerator -= longest;
                        x += dx1;
                        y += dy1;
                    } else {
                        x += dx2;
                        y += dy2;
                    }
                }
            }
        }
    }
}
