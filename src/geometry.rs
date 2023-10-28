use agb::fixnum::{num, FixedNum, Number, Vector2D};
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use crate::matrix::DefaultNum;

const ZERO: DefaultNum = agb::fixnum::Num::from_raw(0);
#[derive(Clone, Copy, Debug)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
pub type Triangle2D<T> = [Vector2D<T>; 3];
pub type Triangle3D<T> = [Vector3D<T>; 3];
pub struct Polygon {
    pub color: u8,
    pub verticies: Triangle3D<DefaultNum>,
}

pub struct Mesh {
    pub verticies: Vec<Polygon>,
    pub pos: Vector3D<DefaultNum>,
    pub rot: Vector3D<DefaultNum>,
}

pub struct Camera {
    pub pos: Vector3D<DefaultNum>,
    pub rot: Vector3D<DefaultNum>,
}
pub struct Scene {
    pub camera: Camera,
    pub mesh: Mesh,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera {
                pos: Vector3D {
                    x: ZERO,
                    y: ZERO,
                    z: ZERO,
                },
                rot: Vector3D {
                    x: ZERO,
                    y: ZERO,
                    z: ZERO,
                },
            },
            mesh: Mesh {
                // cube model, stolen from some github.
                verticies: vec![
                    Polygon {
                        color: 001,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 001,
                        verticies: [
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 004,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 004,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 028,
                        verticies: [
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 028,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 125,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 125,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 128,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 128,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 100,
                        verticies: [
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(-24.0),
                                z: num!(24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                        ],
                    },
                    Polygon {
                        color: 100,
                        verticies: [
                            Vector3D {
                                x: num!(24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(24.0),
                                y: num!(-24.0),
                                z: num!(-24.0),
                            },
                            Vector3D {
                                x: num!(-24.0),
                                y: num!(24.0),
                                z: num!(-24.0),
                            },
                        ],
                    },
                ],
                pos: Vector3D {
                    x: num!(0.0),
                    y: num!(0.0),
                    z: num!(-32.0),
                },
                rot: Vector3D {
                    x: num!(0.0),
                    y: num!(0.0),
                    z: num!(0.0),
                },
            },
        }
    }
}

/*
                    [
                        Vector3D {x: num!(-24.0),  y: num!(-24.0),  z: num!(24.0)},
                        Vector3D {x: num!(-24.0), y: num!(-24.0),  z: num!(-24.0)},
                        Vector3D {x: num!(24.0),   y: num!(-24.0), z: num!(-24.0)},
                    ],
                    [
                        Vector3D {x: num!(-24.0),  y: num!(-24.0),  z: num!(24.0)},
                        Vector3D {x: num!(24.0), y: num!(-24.0),  z: num!(24.0)},
                        Vector3D {x: num!(24.0),   y: num!(-24.0), z: num!(-24.0)},
                    ],

                    [
                        Vector3D {x: num!(-24.0),  y: num!(24.0),  z: num!(24.0)},
                        Vector3D {x: num!(-24.0), y: num!(24.0),  z: num!(-24.0)},
                        Vector3D {x: num!(24.0),   y: num!(24.0), z: num!(-24.0)},
                    ],
                    [
                        Vector3D {x: num!(-24.0),  y: num!(24.0),  z: num!(24.0)},
                        Vector3D {x: num!(24.0), y: num!(24.0),  z: num!(24.0)},
                        Vector3D {x: num!(24.0),   y: num!(24.0), z: num!(-24.0)},
                    ],
*/
