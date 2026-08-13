use crate::{
    data::{VertexCordinate, VertexPosition},
    render::vertex::{Size, Topology, VertexShape}
};
use super::{Color, Position};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Triangle {
    pub color: Color,
    pub points: [Position; 3]
}

impl VertexShape for Triangle {
    fn color(&self) -> Color {
        self.color.clone()
    }

    fn positions(&self, extent:&Size) -> Vec<VertexPosition> {
        self.points.iter()
            .map(|p|p.format_ref(extent))
            .collect()
    }

    fn topology(&self) -> Topology {
        Topology::TRIANGLE_LIST
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Rectangle {
    pub color: Color,
    pub pos: Position,
    pub size: Size
}

impl VertexShape for Rectangle {
    fn color(&self) -> Color {
        self.color.clone()
    }

    fn positions(&self, size:&Size) -> Vec<VertexPosition> {
        let start = self.pos.format_coord(size);
        vec![
            start.to_position(size),
            start.add(&VertexCordinate { x: 0.0, y: self.size.height })
                .to_position(size),
            start.add(&VertexCordinate { x: self.size.width, y: 0.0 })
                .to_position(size),
            start.add(&VertexCordinate { x: self.size.width, y: self.size.height })
                .to_position(size)
        ]
    }

    fn topology(&self) -> Topology {
        Topology::TRIANGLE_STRIP
    }
}

const CIRCLE_SEGMENTS:usize = 50;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Circle {
    pub color: Color,
    pub pos: Position,
    pub radius: f32
}

impl VertexShape for Circle {
    fn color(&self) -> Color {
        self.color.clone()
    }

    fn topology(&self) -> Topology {
        Topology::TRIANGLE_STRIP
    }

    fn positions(&self, extent:&Size) -> Vec<VertexPosition> {
        let mut vertcies = Vec::with_capacity(CIRCLE_SEGMENTS + 2);
        let offset = self.pos.format_coord(extent).add(&VertexCordinate { x: self.radius, y: self.radius });

        vertcies.push(self.pos.format_ref(extent));
        for i in 0..=CIRCLE_SEGMENTS {
            let theta = (i as f32) / (CIRCLE_SEGMENTS as f32) * 2.0 *  std::f32::consts::PI;
            vertcies.push(
                offset.mul(&VertexCordinate { x: theta.cos(), y: theta.sin() })
                    .to_position(extent)
            );
        }

        vertcies
    }
}