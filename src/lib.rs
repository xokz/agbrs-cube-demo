#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![deny(clippy::all)]

/*
epic 3d cube demo to save bitmap modes from being removed from agbrs
*/

mod drawing;
mod geometry;
mod matrix;

use agb::{
    display::{self},
    fixnum::{num, FixedNum, Num, Vector2D},
    input::{Button, ButtonController},
    rng,
};
use drawing::*;
use geometry::*;
use geometry::*;
use matrix::{DefaultNum, Matrix, MatrixMath};

pub fn run(mut gba: agb::Gba) -> ! {
    // Setup gameboy
    let mut scene = Scene::new();
    let mut vram = gba.display.video.bitmap4();
    let vblank = agb::interrupt::VBlank::get();
    let mut input = ButtonController::new();

    // initialize colors
    for i in 0..256 {
        vram.set_palette_entry(i, (i * 255).try_into().unwrap());
    }

    // just a few different color indexes for ease of use
    let red: u8 = 001;
    let yellow: u8 = 004;
    let green: u8 = 028;
    let blue: u8 = 125;
    let cyan: u8 = 128;
    let white: u8 = 100;
    let black: u8 = 000;
    let inc: DefaultNum = num!(0.01);
    let mut time: DefaultNum = num!(0.0);

    let mut x_rot = Matrix::new(3, 3);
    x_rot.elem[0][0] = num!(1.0);

    let mut y_rot = Matrix::new(3, 3);
    y_rot.elem[1][1] = num!(1.0);

    let mut z_rot = Matrix::new(3, 3);
    z_rot.elem[2][2] = num!(1.0);

    loop {
        // handle controls
        input.update();

        if input.is_pressed(Button::UP) {
            scene.mesh.rot.x += inc;
        }
        if input.is_pressed(Button::DOWN) {
            scene.mesh.rot.x -= inc;
        }
        if input.is_pressed(Button::LEFT) {
            scene.mesh.rot.z += inc;
        }
        if input.is_pressed(Button::RIGHT) {
            scene.mesh.rot.z -= inc;
        }
        if input.is_pressed(Button::L) {
            scene.mesh.rot.y += inc;
        }
        if input.is_pressed(Button::R) {
            scene.mesh.rot.y -= inc;
        }

        time += inc;
        if time > num!(1.0) {
            time = num!(0.0);
        }

        let cos_x = scene.mesh.rot.x.cos();
        let sin_x = scene.mesh.rot.x.sin();
        let cos_y = scene.mesh.rot.y.cos();
        let sin_y = scene.mesh.rot.y.sin();
        let cos_z = scene.mesh.rot.z.cos();
        let sin_z = scene.mesh.rot.z.sin();
        let rot_matrix = Matrix {
            height: 3,
            width: 3,
            // this is the rotation matrix to multiply all the verticies by
            elem: [
                [cos_x * cos_y, sin_x * cos_y, -sin_y, num!(0.0)],
                [
                    cos_x * sin_y * sin_z - sin_x * cos_z,
                    sin_x * sin_y * sin_z + cos_x * cos_z,
                    cos_y * sin_z,
                    num!(0.0),
                ],
                [
                    cos_x * sin_y * cos_z + sin_x * sin_z,
                    sin_x * sin_y * cos_z - cos_x * sin_z,
                    cos_y * cos_z,
                    num!(0.0),
                ],
                [num!(0.0), num!(0.0), num!(0.0), num!(0.0)],
            ],
        };

        vram.clear(black);
        for poly in &scene.mesh.verticies {
            let transformed_tri = [
                Matrix::from_vertex(poly.verticies[0]).mul(rot_matrix),
                Matrix::from_vertex(poly.verticies[1]).mul(rot_matrix),
                Matrix::from_vertex(poly.verticies[2]).mul(rot_matrix),
            ];

            let transformed_tri: Triangle3D<DefaultNum> = [
                Matrix::to_vertex(transformed_tri[0]),
                Matrix::to_vertex(transformed_tri[1]),
                Matrix::to_vertex(transformed_tri[2]),
            ];

            let flat_tri = &to_triangle2D(&transformed_tri);

            let x1 = flat_tri[0].x - flat_tri[1].x;
            let x2 = flat_tri[0].x - flat_tri[2].x;
            let y1 = flat_tri[0].y - flat_tri[1].y;
            let y2 = flat_tri[0].y - flat_tri[2].y;
            // this check simply checks if the triangle is facing the camera via back face culling. If it is, then draw it. This works since a cube is a convex shape.
            if (x1 * y2 - x2 * y1) < num!(0.0) {
                vram.draw_tri(flat_tri, poly.color);
                ///*
                vram.draw_line(flat_tri[0], flat_tri[1], black);
                vram.draw_line(flat_tri[1], flat_tri[2], black);
                vram.draw_line(flat_tri[2], flat_tri[0], black);
                //*/
            }
        }

        vblank.wait_for_vblank();
        vram.flip_page();
    }
}

fn to_triangle2D(tri: &Triangle3D<DefaultNum>) -> Triangle2D<DefaultNum> {
    [
        Vector2D::new(tri[0].x + 120, tri[0].y + 80),
        Vector2D::new(tri[1].x + 120, tri[1].y + 80),
        Vector2D::new(tri[2].x + 120, tri[2].y + 80),
    ]
}
