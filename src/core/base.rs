use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{assets::AppAssets, core::ElementList};

const HALF_SIZE_DEFAULT: Vec3 = Vec3::new(15000. * 0.5, 300. * 0.5, 10000. * 0.5);
const MIN_DISTANCE: f32 = 300.;

const VERTEX_PER_NODE: u32 = 6;
const INDEX_PER_TRIANGLE: usize = 3;

#[derive(Component, Clone, Copy)]
pub enum BaseShape {
    Rectangle,
    L,
    N,
}

impl BaseShape {
    fn create(&self, meshes: ResMut<Assets<Mesh>>) -> Base {
        // Calculate half size (mm)
        let x = HALF_SIZE_DEFAULT.x;
        let z = HALF_SIZE_DEFAULT.z;

        match self {
            BaseShape::Rectangle => Base::builder()
                .start_point(-x, -z)
                .move_x_to(x)
                .move_z_to(z)
                .move_x_to(-x)
                .build(meshes),
            BaseShape::L => Base::builder()
                .start_point(-x, -z)
                .move_z_to(z)
                .move_x_to(0.)
                .move_z_to(0.)
                .move_x_to(x)
                .move_z_to(-z)
                .build(meshes),
            BaseShape::N => {
                // Calculate special points
                let p0 = x / 3.;
                let p1 = z / 3.;

                Base::builder()
                    .start_point(-x, -z)
                    .move_x_to(x)
                    .move_z_to(z)
                    .move_x_to(p0)
                    .move_z_to(p1)
                    .move_x_to(-p0)
                    .move_z_to(z)
                    .move_x_to(-x)
                    .build(meshes)
            }
        }
    }
}

#[derive(Event)]
pub struct OnSpawnBase(pub BaseShape);

pub fn on_spawn_base(
    on_spawn_base: On<OnSpawnBase>,
    meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut elements: ResMut<ElementList>,
    assets: Res<AppAssets>,
) {
    let base = on_spawn_base.0.create(meshes);

    let id = commands
        .spawn((
            Name::from("Base"),
            Mesh3d(base.mesh.clone()),
            MeshMaterial3d(assets.materials.matcaps.gray.clone()),
            Transform::from_xyz(0.0, 150., 0.0),
        ))
        .id();

    commands.insert_resource(base);

    elements.list.push((id, String::from("Base")));
}

#[inline]
const fn min_distance(distance: f32) -> f32 {
    if distance.abs() < MIN_DISTANCE {
        distance.signum() * MIN_DISTANCE
    } else {
        distance
    }
}

/// Holds the normals of the base mesh, each node has 6 vertex and each of
/// these vertices has its own normal.
///
/// See [BaseMeshIndices] to for more information about the order of normals.
struct BaseMeshNormals(Vec<[f32; 3]>);

impl BaseMeshNormals {
    /// Creates a new [BaseMeshNormals] with the given capacity
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Calculates and adds the calculated normals to its inner vector
    ///
    /// The given positions are from 3 connected nodes
    /// either in left to right (prev - current - next)
    /// or in right to left (next -  current - prev) order.
    fn add_from_positions(
        &mut self,
        position0: Position,
        position1: Position,
        position2: Position,
    ) {
        let v_01 = position1 - position0;
        let normal_a = if v_01.x == 0. {
            [v_01.z.signum() * 1., 0., 0.]
        } else {
            [0., 0., v_01.x.signum() * 1.]
        };

        let v_12 = position2 - position1;
        let normal_b = if v_12.x == 0. {
            [v_12.z.signum() * 1., 0., 0.]
        } else {
            [0., 0., v_12.x.signum() * 1.]
        };

        self.0.extend_from_slice(&[
            [0., 1., 0.],  // top
            normal_a,      // top side a
            normal_b,      // top side b
            normal_a,      // bottom side a
            normal_b,      // bottom side b
            [0., -1., 0.], // bottom
        ]);
    }
}

/// Holds the indices of the base mesh, the top and bottom face each has
/// node count - 2 of triangles. While each side built by a triangle pair.
///
/// The associated offset constants helps indentify the vertex indices,
/// see the figure below which depicts 2 neighbouring sides.
/// (Also includes offset of the top vertex which is 0)
///
/// ```text
///     
///        TOP
///         0
///    -----+-----    
///        2|1
///      B  |  A    
///        3|4
///    -----+-----
///         5
///      BOTTOM
///
/// ```
struct BaseMeshIndices(Vec<u32>);

impl BaseMeshIndices {
    const OFFSET_TOP_SIDE_A: u32 = 1;
    const OFFSET_TOP_SIDE_B: u32 = 2;
    const OFFSET_BOTTOM_SIDE_A: u32 = 3;
    const OFFSET_BOTTOM_SIDE_B: u32 = 4;
    const OFFSET_BOTTOM: u32 = 5;

    /// Creates a new [BaseMeshIndices] with the given capacity
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Calculates and adds the top triangle based on the given indices
    ///
    /// index0, index1, index2 are the indices of 3 nodes
    fn add_top_triangle(&mut self, index0: usize, index1: usize, index2: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE,
            index1 as u32 * VERTEX_PER_NODE,
            index2 as u32 * VERTEX_PER_NODE,
        ]);
    }

    /// Calculates and adds the bottom triangle based on the given indices
    ///
    /// index0, index1, index2 are the indices of 3 nodes
    fn add_bottom_triangle(&mut self, index0: usize, index1: usize, index2: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM,
            index2 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM,
        ]);
    }

    /// Calculates and adds the bottom triangle based on the given indices
    ///
    /// index0, index1 are the indices of 2 connected nodes
    fn add_side_triangle_pair(&mut self, index0: usize, index1: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM_SIDE_A,
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_TOP_SIDE_A,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM_SIDE_B,
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_TOP_SIDE_A,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_TOP_SIDE_B,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM_SIDE_B,
        ]);
    }
}

#[derive(Resource)]
struct Base {
    nodes: NodeList,
    convex_turn: Turn,
    mesh: Handle<Mesh>,
}

impl Base {
    fn builder() -> BaseBuilder<Empty> {
        BaseBuilder::<Empty> { geometry: Empty }
    }
}

/// Describes the geometry state of the base builder
///
/// Geometry   | Node count
/// ---------- | -----------
/// [Empty]    | 0
/// [Point]    | 1
/// [Line]     | 2
/// [Triangle] | 3
/// [Polygon]  | >=4
///
/// The [Geometry] traits helps to type safely build a base with only
/// 90 degree angles
trait Geometry {}

struct BaseBuilder<G: Geometry> {
    geometry: G,
}

struct Empty;

impl Geometry for Empty {}

impl BaseBuilder<Empty> {
    /// Adds the starting [Node] to the base builder
    fn start_point(self, x: f32, z: f32) -> BaseBuilder<Point> {
        BaseBuilder {
            geometry: Point {
                node: Position::new(x, z),
            },
        }
    }
}

struct Point {
    node: Position,
}

impl Geometry for Point {}

impl BaseBuilder<Point> {
    /// Places a new [Node] at the given x, but with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_to(self, x: f32) -> BaseBuilder<Line<X>> {
        let x = self.geometry.node.x + min_distance(x - self.geometry.node.x);
        let z = self.geometry.node.z;

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }

    /// Places a new [Node] moving in x direction with the given distance, but
    /// with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_with(self, distance: f32) -> BaseBuilder<Line<X>> {
        let x = self.geometry.node.x + min_distance(distance);
        let z = self.geometry.node.z;

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }

    /// Places a new [Node] at the given z, but with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_to(self, z: f32) -> BaseBuilder<Line<Z>> {
        let x = self.geometry.node.x;
        let z = self.geometry.node.z + min_distance(z - self.geometry.node.z);

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }

    /// Places a new [Node] moving in z direction with the given distance, but
    /// with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_with(self, distance: f32) -> BaseBuilder<Line<Z>> {
        let x = self.geometry.node.x;
        let z = self.geometry.node.z + min_distance(distance);

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }
}

/// Each [Geometry] after [Point] also holds the previous move's
/// direction. This allows only alternating between [X] and [Z] moves.
trait PrevMove {}

struct X;
impl PrevMove for X {}

struct Z;
impl PrevMove for Z {}

struct Line<P: PrevMove> {
    nodes: NodeList,
    prev_move: std::marker::PhantomData<P>,
}

impl<P: PrevMove> Line<P> {
    /// Create a new [Line] from the given [Node] positions
    fn from_position_pair(position0: Position, position1: Position) -> Self {
        let mut nodes = NodeList::with_capacity(2);

        nodes.push(Node::new(position0, 1, 1));
        nodes.push(Node::new(position1, 0, 0));

        Self {
            nodes,
            prev_move: std::marker::PhantomData,
        }
    }
}

impl<P: PrevMove> Geometry for Line<P> {}

impl BaseBuilder<Line<X>> {
    /// Places a new [Node] at the given z, but with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_to(self, z: f32) -> BaseBuilder<Triangle<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z
            + min_distance(z - self.geometry.nodes[middle].position.z);

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }

    /// Places a new [Node] moving in z direction with the given distance, but
    /// with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_with(self, distance: f32) -> BaseBuilder<Triangle<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z + min_distance(distance);

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }
}

impl BaseBuilder<Line<Z>> {
    /// Places a new [Node] at the given x, but with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_to(self, x: f32) -> BaseBuilder<Triangle<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x
            + min_distance(x - self.geometry.nodes[middle].position.x);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }

    /// Places a new [Node] moving in x direction with the given distance, but
    /// with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_with(self, distance: f32) -> BaseBuilder<Triangle<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x + min_distance(distance);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }
}

struct Triangle<P: PrevMove> {
    nodes: NodeList,
    prev_move: std::marker::PhantomData<P>,
}

impl<To: PrevMove> Triangle<To> {
    /// Create a new [Triangle] from the [Line] and the node position
    ///
    /// It also calculates the [Turn] of the middle node
    fn from_line<From: PrevMove>(line: Line<From>, position: Position) -> Self {
        let mut nodes = line.nodes;

        nodes.push(Node::new(position, 1, 0));
        nodes[1].connection.next = 2;
        nodes[0].connection.prev = 2;

        let p_1 = &nodes[0].position;
        let p_2 = &nodes[1].position;
        let p_3 = &nodes[2].position;

        let v_12 = *p_2 - *p_1;
        let v_13 = *p_3 - *p_1;

        if v_12.cross(v_13) > 0. {
            nodes[1].turn = Turn::Clockwise;
        } else {
            nodes[1].turn = Turn::CounterClockwise;
        }

        Self {
            nodes,
            prev_move: std::marker::PhantomData,
        }
    }
}

impl<P: PrevMove> Geometry for Triangle<P> {}

impl BaseBuilder<Triangle<X>> {
    /// Places a new [Node] at the given z, but with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_to(self, z: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z
            + min_distance(z - self.geometry.nodes[middle].position.z);

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }

    /// Places a new [Node] moving in z direction with the given distance, but
    /// with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_with(self, distance: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z + min_distance(distance);

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }
}

impl BaseBuilder<Triangle<Z>> {
    /// Places a new [Node] at the given x, but with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_to(self, x: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x
            + min_distance(x - self.geometry.nodes[middle].position.x);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }

    /// Places a new [Node] moving in x direction with the given distance, but
    /// with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_with(self, distance: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x + min_distance(distance);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }
}

struct Polygon<P: PrevMove> {
    nodes: NodeList,
    cw_list: std::collections::VecDeque<usize>,
    ccw_list: std::collections::VecDeque<usize>,
    prev_move: std::marker::PhantomData<P>,
}

impl<To: PrevMove> Polygon<To> {
    /// Create a new [Polygon] from the [Triangle] and the [Node] [Position]
    ///
    /// It also calculates the [Turn] of the third [Node]
    ///
    /// It also saves the indices of the second and third [Node] in the correct
    /// cw or ccw list.
    fn from_triangle<From: PrevMove>(triangle: Triangle<From>, position: Position) -> Self {
        let mut polygon = Self {
            nodes: triangle.nodes,
            cw_list: std::collections::VecDeque::new(),
            ccw_list: std::collections::VecDeque::new(),
            prev_move: std::marker::PhantomData,
        };

        polygon.nodes.push(Node::new(position, 2, 0));
        polygon.nodes[2].connection.next = 3;
        polygon.nodes[0].connection.prev = 3;

        if polygon.nodes[1].turn == Turn::Clockwise {
            polygon.cw_list.push_back(1);
        } else {
            polygon.ccw_list.push_back(1);
        }

        polygon.calc_node_turn(2);

        polygon
    }
}

impl<P: PrevMove> Polygon<P> {
    /// Adds the given [Node] positions to the [NodeList]
    ///
    /// Updates the connections of the [NodeList]
    ///
    /// Calculates the [Node]'s [Turn] and saves it in the right cw/ccw list.
    fn insert_node(&mut self, position: Position) {
        let new_i = self.nodes.len();

        self.nodes.push(Node::new(position, new_i - 1, 0));
        self.nodes[new_i - 1].connection.next = new_i;
        self.nodes[0].connection.prev = new_i;

        self.calc_node_turn(self.nodes[new_i].connection.prev);
    }

    /// Calculates the [Turn] of the [Node] with the given index and
    /// saves it if it was the first time it was calculated.
    fn calc_node_turn(&mut self, index: usize) -> Turn {
        let p_1 = &self.nodes[self.nodes[index].connection.prev].position;
        let p_2 = &self.nodes[index].position;
        let p_3 = &self.nodes[self.nodes[index].connection.next].position;

        let v_12 = *p_2 - *p_1;
        let v_13 = *p_3 - *p_1;

        if v_12.cross(v_13) > 0. {
            if self.nodes[index].turn == Turn::Undefined {
                self.cw_list.push_back(index);
            }
            self.nodes[index].turn = Turn::Clockwise
        } else {
            if self.nodes[index].turn == Turn::Undefined {
                self.ccw_list.push_back(index);
            }
            self.nodes[index].turn = Turn::CounterClockwise
        }

        self.nodes[index].turn
    }
}

impl<P: PrevMove> Geometry for Polygon<P> {}

impl BaseBuilder<Polygon<X>> {
    /// Places a new [Node] at the given z, but with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_to(mut self, z: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z
            + min_distance(z - self.geometry.nodes[middle].position.z);

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }

    /// Places a new [Node] moving in z direction with the given distance, but
    /// with the previous [Node]'s x value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_z_with(mut self, distance: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z + min_distance(distance);

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }
}

impl BaseBuilder<Polygon<Z>> {
    /// Places a new [Node] at the given x, but with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_to(mut self, x: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x
            + min_distance(x - self.geometry.nodes[middle].position.x);
        let z = self.geometry.nodes[middle].position.z;

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }

    /// Places a new [Node] moving in x direction with the given distance, but
    /// with the previous [Node]'s z value.
    ///
    /// There is a minimum distance [MIN_DISTANCE], if the new position is
    /// below this value, the new point will be moved further away to reach
    /// the minimum distance
    fn move_x_with(mut self, distance: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x + min_distance(distance);
        let z = self.geometry.nodes[middle].position.z;

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }
}

impl<P: PrevMove> BaseBuilder<Polygon<P>> {
    /// Calculates the mesh data (vertices, normals, indices) of the base
    /// using ear clipping algorithm to triangulate the top and bottom faces
    fn build(mut self, mut meshes: ResMut<Assets<Mesh>>) -> Base {
        let y = HALF_SIZE_DEFAULT.y;
        let nodes_len = self.geometry.nodes.len();
        let mut nodes_temp = NodeList(self.geometry.nodes.clone());

        let mut mesh_vertices = Vec::with_capacity(nodes_len * VERTEX_PER_NODE as usize);
        let mut mesh_normals = BaseMeshNormals::with_capacity(nodes_len * VERTEX_PER_NODE as usize);
        let mut mesh_indices = BaseMeshIndices::with_capacity(
            2 * (nodes_len - 2) * INDEX_PER_TRIANGLE // top + bottom
            + 2 * nodes_len * INDEX_PER_TRIANGLE, // sides
        );

        // Caclulate turn of first/last node
        self.geometry.calc_node_turn(0);
        self.geometry.calc_node_turn(nodes_len - 1);

        // If there are more cw turns than ccw, then the nodes are connected
        // in clockwise order. In this case each cw turn is convex, while
        // counter clockwise turns start out as reflexes.
        let (convex_turn, mut convex_list) =
            if self.geometry.cw_list.len() > self.geometry.ccw_list.len() {
                (Turn::Clockwise, self.geometry.cw_list)
            } else {
                (Turn::CounterClockwise, self.geometry.ccw_list)
            };

        // Each node point represent 8 vertex
        // Top (Y+) vertices building up 4 triangles (top + 3 side)
        // Bottom (Y-) vertices building up 4 triangles (bottom + 3 side)
        for (
            curr_i,
            Node {
                position: Position { x, z },
                connection,
                ..
            },
        ) in self.geometry.nodes.iter().enumerate()
        {
            mesh_vertices.extend_from_slice(&[
                [*x, y, *z],  // top
                [*x, y, *z],  // top side a
                [*x, y, *z],  // top side b
                [*x, -y, *z], // bottom side a
                [*x, -y, *z], // bottom side b
                [*x, -y, *z], // bottom
            ]);

            let prev_i = connection.prev;
            let next_i = connection.next;

            if convex_turn == Turn::Clockwise {
                mesh_indices.add_side_triangle_pair(curr_i, next_i);
                mesh_normals.add_from_positions(
                    nodes_temp[next_i].position,
                    nodes_temp[curr_i].position,
                    nodes_temp[prev_i].position,
                );
            } else {
                mesh_indices.add_side_triangle_pair(next_i, curr_i);
                mesh_normals.add_from_positions(
                    nodes_temp[prev_i].position,
                    nodes_temp[curr_i].position,
                    nodes_temp[next_i].position,
                );
            }
        }

        let (mut triangle_empty, mut n);
        let (mut start, mut end, mut count) = (0, nodes_len, nodes_len);
        while count > 3 {
            let Some(curr_i) = convex_list.pop_front() else {
                n = start;
                for _ in 0..count {
                    nodes_temp.is_turn_changed(n);
                    n = nodes_temp[n].connection.next;
                }

                continue;
            };

            let prev_i = nodes_temp[curr_i].connection.prev;
            let next_i = nodes_temp[curr_i].connection.next;

            let p_1 = nodes_temp[prev_i].position;
            let p_2 = nodes_temp[curr_i].position;
            let p_3 = nodes_temp[next_i].position;

            let v_12 = p_2 - p_1;
            let v_23 = p_3 - p_2;
            let v_31 = p_1 - p_3;

            triangle_empty = true;
            n = start;
            for _ in 0..count {
                if n == prev_i || n == curr_i || n == next_i {
                    n = nodes_temp[n].connection.next;
                    continue;
                }

                let p_n = nodes_temp[n].position;

                let v_1n = p_n - p_1;
                let v_2n = p_n - p_2;
                let v_3n = p_n - p_3;

                // Check n-th node position is inside the triangle defined by p_1-3
                if convex_turn == Turn::Clockwise {
                    if v_12.cross(v_1n) >= 0. && v_23.cross(v_2n) >= 0. && v_31.cross(v_3n) >= 0. {
                        triangle_empty = false;
                        break;
                    }
                } else {
                    if v_12.cross(v_1n) <= 0. && v_23.cross(v_2n) <= 0. && v_31.cross(v_3n) <= 0. {
                        triangle_empty = false;
                        break;
                    }
                }

                n = nodes_temp[n].connection.next;
            }

            if triangle_empty {
                if convex_turn == Turn::Clockwise {
                    mesh_indices.add_top_triangle(prev_i, next_i, curr_i);
                    mesh_indices.add_bottom_triangle(prev_i, curr_i, next_i);
                } else {
                    mesh_indices.add_top_triangle(prev_i, curr_i, next_i);
                    mesh_indices.add_bottom_triangle(prev_i, next_i, curr_i);
                }

                nodes_temp[prev_i].connection.next = nodes_temp[curr_i].connection.next;
                nodes_temp[next_i].connection.prev = nodes_temp[curr_i].connection.prev;

                if nodes_temp[prev_i].turn != convex_turn && nodes_temp.is_turn_changed(prev_i) {
                    convex_list.push_back(prev_i);
                }

                if nodes_temp[next_i].turn != convex_turn && nodes_temp.is_turn_changed(next_i) {
                    convex_list.push_back(next_i);
                }

                if curr_i == start {
                    start = nodes_temp[curr_i].connection.next;
                } else if curr_i == end {
                    end = nodes_temp[curr_i].connection.prev;
                }

                count -= 1;
            } else {
                convex_list.push_back(curr_i);
            }
        }

        let start_prev_i = nodes_temp[start].connection.prev;
        let start_next_i = nodes_temp[start].connection.next;

        if convex_turn == Turn::Clockwise {
            mesh_indices.add_top_triangle(start, start_prev_i, start_next_i);
            mesh_indices.add_bottom_triangle(start, start_next_i, start_prev_i);
        } else {
            mesh_indices.add_top_triangle(start, start_next_i, start_prev_i);
            mesh_indices.add_bottom_triangle(start, start_prev_i, start_next_i);
        }

        Base {
            nodes: self.geometry.nodes,
            convex_turn,
            mesh: meshes.add(
                Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_vertices)
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals.0)
                .with_inserted_indices(Indices::U32(mesh_indices.0)),
            ),
        }
    }
}

/// Describes the [Node]'s connection turn
///
/// ```text
/// Clockwise        Counter Clockwise
/// (Right)          (Left)
///
/// 0--->1           1<---0
///      |           |
///      v           v
///      2           2
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    Undefined,
    Clockwise,
    CounterClockwise,
}

impl std::ops::Not for Turn {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Turn::Undefined => Turn::Undefined,
            Turn::Clockwise => Turn::CounterClockwise,
            Turn::CounterClockwise => Turn::Clockwise,
        }
    }
}

#[derive(Deref, DerefMut)]
struct NodeList(Vec<Node>);

impl NodeList {
    /// Creates a new [NodeList] with the given capacity
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Returns true if the [Turn] of the [Node] at the given index changed
    ///
    /// If it changed then it also updates the [Node] with the new [Turn]
    fn is_turn_changed(&mut self, index: usize) -> bool {
        let p_1 = &self[self[index].connection.prev].position;
        let p_2 = &self[index].position;
        let p_3 = &self[self[index].connection.next].position;

        let v_12 = *p_2 - *p_1;
        let v_13 = *p_3 - *p_1;

        let cross = v_12.cross(v_13);

        if cross > 0. && self[index].turn == Turn::CounterClockwise {
            self[index].turn = Turn::Clockwise;
            return true;
        }

        if cross < 0. && self[index].turn == Turn::Clockwise {
            self[index].turn = Turn::CounterClockwise;
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    connection: Connection,
    position: Position,
    turn: Turn,
}

impl Node {
    /// Creates a new [Node] at the given [Position], with [Turn::Undefined]
    /// and sets the [Connection] to the proveded prev/next indices.
    fn new(position: Position, prev: usize, next: usize) -> Self {
        Self {
            connection: Connection { prev, next },
            position,
            turn: Turn::Undefined,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Connection {
    prev: usize,
    next: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: f32,
    z: f32,
}

impl Position {
    /// Creates a new [Position] at given x and z
    const fn new(x: f32, z: f32) -> Self {
        Self { x, z }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
            ..self
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z,
            ..self
        }
    }
}

impl std::ops::Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            z: self.z * rhs.z,
            ..self
        }
    }
}

impl std::ops::Div for Position {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            z: self.z / rhs.z,
            ..self
        }
    }
}

trait CrossProduct<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl CrossProduct for Position {
    type Output = f32;

    fn cross(self, rhs: Self) -> Self::Output {
        self.x * rhs.z - self.z * rhs.x
    }
}

#[test]
fn test_base_builder() {
    let x = 10000.;
    let z = 1234.;

    // -x,-z      x, -z
    // 0----------3
    // |          |
    // |     x    |
    // |          |
    // 1----------2
    // -x,z       x, z

    let base_ccw = Base::builder()
        .start_point(-x, -z)
        .move_z_to(z)
        .move_x_to(x)
        .move_z_to(-z);

    assert_eq!(
        base_ccw
            .geometry
            .nodes
            .iter()
            .map(|node| node.position)
            .collect::<Vec<Position>>(),
        vec![
            Position::new(-x, -z),
            Position::new(-x, z),
            Position::new(x, z),
            Position::new(x, -z),
        ]
    );
    assert_eq!(base_ccw.geometry.cw_list.len(), 0);
    assert_eq!(base_ccw.geometry.ccw_list.len(), 2);

    let x = MIN_DISTANCE * 0.5;
    let z = MIN_DISTANCE * 0.5;

    // -x,-z      x, -z
    // 0----------1
    // |          |
    // |     x    |
    // |          |
    // 3----------2
    // -x,z       x, z

    // Since the move distance is less than MIN_DISTANCE it should correct
    // the new node to have at least the minimum distance
    let mut base_cw = Base::builder()
        .start_point(-x, -z)
        .move_x_to(0.)
        .move_z_with(z)
        .move_x_with(-x);

    base_cw.geometry.calc_node_turn(0);
    base_cw.geometry.calc_node_turn(3);

    assert_eq!(
        base_cw
            .geometry
            .nodes
            .iter()
            .map(|node| node.position)
            .collect::<Vec<Position>>(),
        vec![
            Position::new(-x, -z),
            Position::new(x, -z),
            Position::new(x, z),
            Position::new(-x, z),
        ]
    );
    assert_eq!(base_cw.geometry.cw_list.len(), 4);
    assert_eq!(base_cw.geometry.ccw_list.len(), 0);
}
