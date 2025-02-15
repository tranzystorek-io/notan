use super::geometry;
use super::tess::TessMode;
use super::tess::*;
use crate::builder::DrawProcess;
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use lyon::tessellation::*;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_math::Mat3;

pub struct Rectangle {
    colors: [Color; 4],
    pos: (f32, f32),
    size: (f32, f32),
    mode: TessMode,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    rounded_corners: Option<[f32; 4]>,
    corner_tolerance: f32,
    blend_mode: Option<BlendMode>,
}

impl Rectangle {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            colors: [Color::WHITE; 4],
            pos: position,
            size,
            mode: TessMode::Fill,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
            rounded_corners: None,
            corner_tolerance: FillOptions::DEFAULT_TOLERANCE,
            blend_mode: None,
        }
    }

    pub fn corner_radius(&mut self, radius: f32) -> &mut Self {
        self.rounded_corners = Some([radius; 4]);
        self
    }

    pub fn corner_tolerance(&mut self, tolerance: f32) -> &mut Self {
        self.corner_tolerance = tolerance;
        self
    }

    pub fn top_left_radius(&mut self, radius: f32) -> &mut Self {
        let mut corners = self.rounded_corners.unwrap_or([0.0, 0.0, 0.0, 0.0]);
        corners[0] = radius;
        self
    }

    pub fn top_right_radius(&mut self, radius: f32) -> &mut Self {
        let mut corners = self.rounded_corners.unwrap_or([0.0, 0.0, 0.0, 0.0]);
        corners[1] = radius;
        self
    }

    pub fn bottom_left_radius(&mut self, radius: f32) -> &mut Self {
        let mut corners = self.rounded_corners.unwrap_or([0.0, 0.0, 0.0, 0.0]);
        corners[2] = radius;
        self
    }

    pub fn bottom_right_radius(&mut self, radius: f32) -> &mut Self {
        let mut corners = self.rounded_corners.unwrap_or([0.0, 0.0, 0.0, 0.0]);
        corners[3] = radius;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.colors.fill(color);
        self
    }

    pub fn color_vertex(&mut self, a: Color, b: Color, c: Color, d: Color) -> &mut Self {
        self.colors[0] = a;
        self.colors[1] = b;
        self.colors[2] = c;
        self.colors[3] = d;
        self
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut Self {
        self.alpha = alpha;
        self
    }

    pub fn fill(&mut self) -> &mut Self {
        self.mode = TessMode::Fill;
        self
    }

    pub fn stroke(&mut self, width: f32) -> &mut Self {
        self.mode = TessMode::Stroke;
        self.stroke_width = width;
        self
    }

    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.blend_mode = Some(mode);
        self
    }
}

impl DrawTransform for Rectangle {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Rectangle {
    fn draw_process(self, draw: &mut Draw) {
        match self.mode {
            TessMode::Fill => fill(self, draw),
            TessMode::Stroke => stroke(self, draw),
        }
    }
}

fn stroke(quad: Rectangle, draw: &mut Draw) {
    let Rectangle {
        colors: [ca, ..],
        pos: (x, y),
        size: (width, height),
        stroke_width,
        alpha,
        matrix,
        rounded_corners,
        corner_tolerance,
        blend_mode,
        ..
    } = quad;

    let stroke_options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_tolerance(corner_tolerance);
    let color = ca.with_alpha(ca.a * alpha);

    let path = match rounded_corners {
        Some([tl, tr, bl, br]) => geometry::rounded_rect(x, y, width, height, (tl, tr, bl, br)),
        _ => geometry::rectangle(x, y, width, height),
    };

    let (vertices, indices) = stroke_lyon_path(&path, color, &stroke_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
    });
}

fn fill(quad: Rectangle, draw: &mut Draw) {
    let Rectangle {
        colors: [ca, cb, cc, cd],
        pos: (x1, y1),
        size: (width, height),
        alpha,
        matrix,
        rounded_corners,
        corner_tolerance,
        blend_mode,
        ..
    } = quad;

    let mut draw_shape = |vertices: &[f32], indices: &[u32]| {
        draw.add_shape(&ShapeInfo {
            transform: matrix.as_ref(),
            vertices,
            indices,
            blend_mode,
        });
    };

    match rounded_corners {
        Some([tl, tr, bl, br]) => {
            let path = geometry::rounded_rect(x1, y1, width, height, (tl, tr, bl, br));
            let options = FillOptions::default().with_tolerance(corner_tolerance);
            let (vertices, indices) = fill_lyon_path(&path, ca.with_alpha(ca.a * alpha), &options);

            draw_shape(&vertices, &indices);
        }
        _ => {
            let x2 = x1 + width;
            let y2 = y1 + height;

            let indices = [0, 1, 2, 0, 2, 3];

            #[rustfmt::skip]
            let vertices = [
                x1, y1, ca.r, ca.g, ca.b, ca.a * alpha,
                x1, y2, cb.r, cb.g, cb.b, cb.a * alpha,
                x2, y2, cc.r, cc.g, cc.b, cc.a * alpha,
                x2, y1, cd.r, cd.g, cd.b, cd.a * alpha,
            ];

            draw_shape(&vertices, &indices);
        }
    };
}
