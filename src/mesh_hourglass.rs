//! Mesh-based hourglass implementation with composable parts.

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use earcutr::earcut;

/// Configuration for the hourglass body (the glass part)
#[derive(Clone, Debug)]
pub struct HourglassMeshBodyConfig {
    pub total_height: f32,
    pub bulb_radius: f32,
    pub bulb_width_factor: f32,
    pub bulb_height_factor: f32,
    pub bulb_curve_resolution: usize,
    pub neck_width: f32,
    pub neck_height: f32,
    pub neck_curve_resolution: usize,
    pub color: Color,
}

impl Default for HourglassMeshBodyConfig {
    fn default() -> Self {
        Self {
            total_height: 200.0,
            bulb_radius: 100.0,
            bulb_width_factor: 0.75,
            bulb_height_factor: 1.0,
            bulb_curve_resolution: 20,
            neck_width: 12.0,
            neck_height: 7.0,
            neck_curve_resolution: 5,
            color: Color::srgb(0.8, 0.5, 0.3),
        }
    }
}

/// Configuration for the plates at the top and bottom of the hourglass
#[derive(Clone, Debug)]
pub struct HourglassMeshPlatesConfig {
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Default for HourglassMeshPlatesConfig {
    fn default() -> Self {
        Self {
            width: 165.0,
            height: 10.0,
            color: Color::srgb(0.0, 0.0, 0.0),
        }
    }
}

/// Configuration for the sand inside the hourglass
#[derive(Clone, Debug)]
pub struct HourglassMeshSandConfig {
    pub color: Color,
    pub fill_percent: f32,      // 0.0 to 1.0, how full the top bulb is
    pub scale_factor: f32,      // How much smaller than the glass (0.0 to 1.0)
    pub neck_scale_factor: f32, // How much smaller than the neck (0.0 to 1.0)
}

impl Default for HourglassMeshSandConfig {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.9, 0.8, 0.6), // Sand color
            fill_percent: 1.0,                 // Start with full top bulb
            scale_factor: 0.95,                // Sand is 95% of glass size
            neck_scale_factor: 0.35,           // Sand is 35% of neck size
        }
    }
}

/// Marker component for the mesh hourglass container entity
#[derive(Component)]
pub struct HourglassMesh;

/// Marker component for the hourglass body
#[derive(Component)]
pub struct HourglassMeshBody;

/// Marker component for the hourglass plates
#[derive(Component)]
pub enum HourglassMeshPlate {
    Top,
    Bottom,
}

/// Marker component for the hourglass sand
#[derive(Component)]
pub enum HourglassMeshSand {
    TopBulb,
    BottomBulb,
}

/// Component to track sand state for animations
#[derive(Component, Debug, Clone)]
pub struct HourglassMeshSandState {
    pub fill_percent: f32,
    pub body_config: HourglassMeshBodyConfig,
    pub sand_config: HourglassMeshSandConfig,
    /// Flag to track if the sand needs to be regenerated
    pub needs_update: bool,
}

/// Builder for creating a mesh-based hourglass
#[derive(Default)]
pub struct HourglassMeshBuilder {
    transform: Transform,
    body_config: Option<HourglassMeshBodyConfig>,
    plates_config: Option<HourglassMeshPlatesConfig>,
    sand_config: Option<HourglassMeshSandConfig>,
}

impl HourglassMeshBuilder {
    /// Creates a new hourglass builder with the specified transform
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            body_config: None,
            plates_config: None,
            sand_config: None,
        }
    }

    /// Adds a body configuration to the hourglass
    pub fn with_body(mut self, config: HourglassMeshBodyConfig) -> Self {
        self.body_config = Some(config);
        self
    }

    /// Adds plates configuration to the hourglass
    pub fn with_plates(mut self, config: HourglassMeshPlatesConfig) -> Self {
        self.plates_config = Some(config);
        self
    }

    /// Adds sand configuration to the hourglass
    pub fn with_sand(mut self, config: HourglassMeshSandConfig) -> Self {
        self.sand_config = Some(config);
        self
    }

    /// Builds the hourglass entity and all its configured components
    pub fn build(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        // Create parent entity for the hourglass
        let hourglass_entity = commands.spawn((HourglassMesh, self.transform)).id();

        // Add body if configured
        if let Some(body_config) = &self.body_config {
            let body_entity = self.spawn_body(commands, meshes, materials, body_config);
            commands.entity(hourglass_entity).add_child(body_entity);
        }

        // Add plates if configured
        if let Some(plates_config) = &self.plates_config {
            let (top_plate, bottom_plate) =
                self.spawn_plates(commands, meshes, materials, plates_config);
            commands
                .entity(hourglass_entity)
                .add_child(top_plate)
                .add_child(bottom_plate);
        }

        // Add sand if configured
        if let Some(sand_config) = &self.sand_config {
            if let Some(body_config) = &self.body_config {
                let (top_sand, bottom_sand) =
                    self.spawn_sand(commands, meshes, materials, body_config, sand_config);
                commands
                    .entity(hourglass_entity)
                    .add_child(top_sand)
                    .add_child(bottom_sand);

                // Add sand state component for animation support
                commands
                    .entity(hourglass_entity)
                    .insert(HourglassMeshSandState {
                        fill_percent: sand_config.fill_percent,
                        body_config: body_config.clone(),
                        sand_config: sand_config.clone(),
                        needs_update: false,
                    });
            }
        }

        hourglass_entity
    }

    /// Spawns just the hourglass body
    fn spawn_body(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        config: &HourglassMeshBodyConfig,
    ) -> Entity {
        let half_height = config.total_height / 2.0;
        let neck_half_width = config.neck_width / 2.0;
        let neck_half_height = config.neck_height / 2.0;

        // Calculate effective bulb dimensions
        let bulb_width = config.bulb_radius * config.bulb_width_factor;
        let bulb_height = config.bulb_radius * config.bulb_height_factor;

        let mut points: Vec<[f32; 2]> = Vec::new();

        // Bottom quarter circle left side
        for i in 0..=config.bulb_curve_resolution {
            let theta =
                std::f32::consts::PI / 2.0 * (i as f32 / config.bulb_curve_resolution as f32);
            let x = (-bulb_width * theta.cos()).min(-neck_half_width);
            let y = -neck_half_height - half_height + bulb_height * theta.sin();
            points.push([x, y]);
        }

        // Upward left neck curve
        for i in 1..=config.neck_curve_resolution {
            let theta = std::f32::consts::PI * (i as f32 / config.neck_curve_resolution as f32);

            // Calculate a smooth arc that connects the bottom and top bulbs
            let x = -neck_half_width + (neck_half_width * 0.2 * theta.sin()); // Curve inward
            let y = -neck_half_height
                + config.neck_height * (i as f32 / config.neck_curve_resolution as f32);

            points.push([x, y]);
        }

        // Top quarter circle left side
        for i in (0..=config.bulb_curve_resolution).rev() {
            let theta =
                std::f32::consts::PI / 2.0 * (i as f32 / config.bulb_curve_resolution as f32);
            let x = (-bulb_width * theta.cos()).min(-neck_half_width);
            let y = neck_half_height + half_height - bulb_height * theta.sin();
            points.push([x, y]);
        }

        // Cut across the top
        points.push([neck_half_width + bulb_width, neck_half_height + half_height]);

        // Top quarter circle right side
        for i in 0..=config.bulb_curve_resolution {
            let theta =
                std::f32::consts::PI / 2.0 * (i as f32 / config.bulb_curve_resolution as f32);
            let x = (bulb_width * theta.cos()).max(neck_half_width);
            let y = neck_half_height + half_height - bulb_height * theta.sin();
            points.push([x, y]);
        }

        // Downward right neck curve
        for i in 1..=config.neck_curve_resolution {
            let theta = std::f32::consts::PI * (i as f32 / config.neck_curve_resolution as f32);

            // Calculate a smooth arc that connects the top and bottom bulbs
            let x = neck_half_width - (neck_half_width * 0.2 * theta.sin()); // Curve inward
            let y = neck_half_height
                - config.neck_height * (i as f32 / config.neck_curve_resolution as f32);

            points.push([x, y]);
        }

        // Bottom quarter circle right side
        for i in (0..=config.bulb_curve_resolution).rev() {
            let theta =
                std::f32::consts::PI / 2.0 * (i as f32 / config.bulb_curve_resolution as f32);
            let x = (bulb_width * theta.cos()).max(neck_half_width);
            let y = -neck_half_height - half_height + bulb_height * theta.sin();
            points.push([x, y]);
        }

        // Flatten to [x, y, 0.0]
        let num_vertices = points.len();
        let points_3d = points.iter().map(|p| [p[0], p[1], 0.0]).collect::<Vec<_>>();

        let coords: Vec<f32> = points.iter().flat_map(|p| vec![p[0], p[1]]).collect();
        let hole_indices: Vec<usize> = Vec::new();
        let indices: Vec<u32> = earcut(&coords, &hole_indices, 2)
            .unwrap()
            .into_iter()
            .map(|i| i as u32)
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);

        let normals = vec![[0.0, 0.0, 1.0]; num_vertices];
        let uvs = vec![[0.0, 0.0]; num_vertices];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        commands
            .spawn((
                HourglassMeshBody,
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(config.color)),
            ))
            .id()
    }

    /// Spawns the top and bottom plates
    fn spawn_plates(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        config: &HourglassMeshPlatesConfig,
    ) -> (Entity, Entity) {
        // Create plate mesh (simple rectangle)
        let mut plate_mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());

        // Rectangle vertices (centered at origin)
        let half_width = config.width / 2.0;
        let half_height = config.height / 2.0;
        let points_3d = vec![
            [-half_width, -half_height, 0.0], // bottom left
            [half_width, -half_height, 0.0],  // bottom right
            [half_width, half_height, 0.0],   // top right
            [-half_width, half_height, 0.0],  // top left
        ];

        // Indices for two triangles making up the rectangle
        let indices = vec![0, 1, 2, 0, 2, 3];

        plate_mesh.insert_indices(Indices::U32(indices));
        plate_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);
        plate_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4]);
        plate_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 4]);

        // Add the mesh to assets
        let plate_mesh_handle = meshes.add(plate_mesh);
        let plate_material = materials.add(config.color);

        // Get the total height from body config or use a default
        let total_height = self
            .body_config
            .as_ref()
            .map(|cfg| cfg.total_height)
            .unwrap_or(200.0);

        let half_total_height = total_height / 2.0;

        // Spawn top plate
        let top_plate = commands
            .spawn((
                HourglassMeshPlate::Top,
                Mesh2d(plate_mesh_handle.clone()),
                MeshMaterial2d(plate_material.clone()),
                Transform::from_xyz(0.0, half_total_height + config.height - 1.0, 0.0),
            ))
            .id();

        // Spawn bottom plate
        let bottom_plate = commands
            .spawn((
                HourglassMeshPlate::Bottom,
                Mesh2d(plate_mesh_handle),
                MeshMaterial2d(plate_material),
                Transform::from_xyz(0.0, -half_total_height - config.height + 1.0, 0.0),
            ))
            .id();

        (top_plate, bottom_plate)
    }

    /// Spawns the sand inside the hourglass
    fn spawn_sand(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        body_config: &HourglassMeshBodyConfig,
        sand_config: &HourglassMeshSandConfig,
    ) -> (Entity, Entity) {
        // Create material for sand
        let sand_material = materials.add(sand_config.color);

        // Generate top sand mesh
        let top_points = Self::generate_top_sand_points(body_config, sand_config);
        let top_sand_entity = if let Some(mesh) = Self::create_mesh_from_points(top_points) {
            commands
                .spawn((
                    HourglassMeshSand::TopBulb,
                    Mesh2d(meshes.add(mesh)),
                    MeshMaterial2d(sand_material.clone()),
                    Transform::from_xyz(0.0, 0.0, 0.1), // Slightly in front of body
                ))
                .id()
        } else {
            // Empty top bulb
            commands
                .spawn((
                    HourglassMeshSand::TopBulb,
                    Transform::from_xyz(0.0, 0.0, 0.1),
                ))
                .id()
        };

        // Generate bottom sand mesh
        let bottom_points = Self::generate_bottom_sand_points(body_config, sand_config);
        let bottom_sand_entity = if let Some(mesh) = Self::create_mesh_from_points(bottom_points) {
            commands
                .spawn((
                    HourglassMeshSand::BottomBulb,
                    Mesh2d(meshes.add(mesh)),
                    MeshMaterial2d(sand_material),
                    Transform::from_xyz(0.0, 0.0, 0.1), // Slightly in front of body
                ))
                .id()
        } else {
            // Empty bottom bulb
            commands
                .spawn((
                    HourglassMeshSand::BottomBulb,
                    Transform::from_xyz(0.0, 0.0, 0.1),
                ))
                .id()
        };

        (top_sand_entity, bottom_sand_entity)
    }
}

/// Helper functions for sand mesh generation
impl HourglassMeshBuilder {
    /// Generate top bulb sand mesh points
    fn generate_top_sand_points(
        body_config: &HourglassMeshBodyConfig,
        sand_config: &HourglassMeshSandConfig,
    ) -> Vec<[f32; 2]> {
        let half_height = body_config.total_height / 2.0;
        let neck_half_width = body_config.neck_width / 2.0 * sand_config.scale_factor;
        let neck_half_height = body_config.neck_height / 2.0;

        // Calculate effective bulb dimensions with sand scaling
        let bulb_width =
            body_config.bulb_radius * body_config.bulb_width_factor * sand_config.scale_factor;
        let bulb_height =
            body_config.bulb_radius * body_config.bulb_height_factor * sand_config.scale_factor;

        let mut top_points: Vec<[f32; 2]> = Vec::new();

        // Calculate the fill line for the top bulb
        let top_bulb_base_y = neck_half_height;
        let top_bulb_top_y = neck_half_height + half_height;
        let fill_line_y =
            top_bulb_base_y + (top_bulb_top_y - top_bulb_base_y) * sand_config.fill_percent;

        if sand_config.fill_percent > 0.0 {
            // Start from the neck on the left side
            top_points.push([
                -neck_half_width * sand_config.neck_scale_factor,
                -body_config.bulb_radius - neck_half_height,
            ]);

            // Left side of top bulb (up to fill line)
            for i in (0..=body_config.bulb_curve_resolution).rev() {
                let theta = std::f32::consts::PI / 2.0
                    * (i as f32 / body_config.bulb_curve_resolution as f32);
                let x = (-bulb_width * theta.cos())
                    .min(-neck_half_width * sand_config.neck_scale_factor);
                let y = neck_half_height + half_height - bulb_height * theta.sin();

                if y <= fill_line_y {
                    top_points.push([x, y]);
                } else {
                    // Calculate intersection with fill line
                    let prev_i = i + 1;
                    if prev_i <= body_config.bulb_curve_resolution {
                        let prev_theta = std::f32::consts::PI / 2.0
                            * (prev_i as f32 / body_config.bulb_curve_resolution as f32);
                        let prev_y =
                            neck_half_height + half_height - bulb_height * prev_theta.sin();
                        if prev_y <= fill_line_y {
                            // Interpolate x position at fill line
                            let t = (fill_line_y - prev_y) / (y - prev_y);
                            let x_at_fill = x * t
                                + (-bulb_width * prev_theta.cos()).min(-neck_half_width)
                                    * (1.0 - t);
                            top_points.push([x_at_fill, fill_line_y]);
                        }
                    }
                    break;
                }
            }

            // Calculate right side x at fill line
            let mut right_x_at_fill = neck_half_width;
            for i in (0..=body_config.bulb_curve_resolution).rev() {
                let theta = std::f32::consts::PI / 2.0
                    * (i as f32 / body_config.bulb_curve_resolution as f32);
                let x =
                    (bulb_width * theta.cos()).max(neck_half_width * sand_config.neck_scale_factor);
                let y = neck_half_height + half_height - bulb_height * theta.sin();

                if y <= fill_line_y {
                    right_x_at_fill = x;
                } else {
                    // Calculate intersection
                    let prev_i = i + 1;
                    if prev_i <= body_config.bulb_curve_resolution {
                        let prev_theta = std::f32::consts::PI / 2.0
                            * (prev_i as f32 / body_config.bulb_curve_resolution as f32);
                        let prev_y =
                            neck_half_height + half_height - bulb_height * prev_theta.sin();
                        if prev_y <= fill_line_y {
                            let t = (fill_line_y - prev_y) / (y - prev_y);
                            right_x_at_fill = x * t
                                + (bulb_width * prev_theta.cos()).max(neck_half_width) * (1.0 - t);
                        }
                    }
                    break;
                }
            }

            top_points.push([right_x_at_fill, fill_line_y]);

            // Right side of top bulb (down from fill line)
            for i in 0..=body_config.bulb_curve_resolution {
                let theta = std::f32::consts::PI / 2.0
                    * (i as f32 / body_config.bulb_curve_resolution as f32);
                let x =
                    (bulb_width * theta.cos()).max(neck_half_width * sand_config.neck_scale_factor);
                let y = neck_half_height + half_height - bulb_height * theta.sin();

                if y <= fill_line_y {
                    top_points.push([x, y]);
                }
            }

            // Close at the neck on the right side
            top_points.push([
                neck_half_width * sand_config.neck_scale_factor,
                -body_config.bulb_radius - neck_half_height,
            ]);
        }

        top_points
    }

    /// Generate bottom bulb sand mesh points
    fn generate_bottom_sand_points(
        body_config: &HourglassMeshBodyConfig,
        sand_config: &HourglassMeshSandConfig,
    ) -> Vec<[f32; 2]> {
        let half_height = body_config.total_height / 2.0;
        let neck_half_width = body_config.neck_width / 2.0 * sand_config.scale_factor;
        let neck_half_height = body_config.neck_height / 2.0;

        // Calculate effective bulb dimensions with sand scaling
        let bulb_width =
            body_config.bulb_radius * body_config.bulb_width_factor * sand_config.scale_factor;
        let bulb_height =
            body_config.bulb_radius * body_config.bulb_height_factor * sand_config.scale_factor;

        let mut bottom_points: Vec<[f32; 2]> = Vec::new();

        // Calculate the fill line for the bottom bulb
        let bottom_fill_percent = 1.0 - sand_config.fill_percent;
        let bottom_bulb_base_y = -neck_half_height - half_height;
        let bottom_bulb_top_y = -neck_half_height;
        let bottom_fill_line_y =
            bottom_bulb_base_y + (bottom_bulb_top_y - bottom_bulb_base_y) * bottom_fill_percent;

        if bottom_fill_percent > 0.0 {
            // Start from bottom left
            for i in 0..=body_config.bulb_curve_resolution {
                let theta = std::f32::consts::PI / 2.0
                    * (i as f32 / body_config.bulb_curve_resolution as f32);
                let x = (-bulb_width * theta.cos()).min(-neck_half_width);
                let y = -neck_half_height - half_height + bulb_height * theta.sin();

                if y <= bottom_fill_line_y {
                    bottom_points.push([x, y]);
                } else {
                    // Calculate intersection with fill line
                    if i > 0 {
                        let prev_theta = std::f32::consts::PI / 2.0
                            * ((i - 1) as f32 / body_config.bulb_curve_resolution as f32);
                        let prev_x = (-bulb_width * prev_theta.cos()).min(-neck_half_width);
                        let prev_y =
                            -neck_half_height - half_height + bulb_height * prev_theta.sin();
                        if prev_y <= bottom_fill_line_y {
                            let t = (bottom_fill_line_y - prev_y) / (y - prev_y);
                            let x_at_fill = prev_x * (1.0 - t) + x * t;
                            bottom_points.push([x_at_fill, bottom_fill_line_y]);
                        }
                    }
                    break;
                }
            }

            // Add fill line across
            let mut right_x_at_fill = neck_half_width;
            for i in 0..=body_config.bulb_curve_resolution {
                let theta = std::f32::consts::PI / 2.0
                    * (i as f32 / body_config.bulb_curve_resolution as f32);
                let x = (bulb_width * theta.cos()).max(neck_half_width);
                let y = -neck_half_height - half_height + bulb_height * theta.sin();

                if y > bottom_fill_line_y {
                    if i > 0 {
                        let prev_theta = std::f32::consts::PI / 2.0
                            * ((i - 1) as f32 / body_config.bulb_curve_resolution as f32);
                        let prev_x = (bulb_width * prev_theta.cos()).max(neck_half_width);
                        let prev_y =
                            -neck_half_height - half_height + bulb_height * prev_theta.sin();
                        if prev_y <= bottom_fill_line_y {
                            let t = (bottom_fill_line_y - prev_y) / (y - prev_y);
                            right_x_at_fill = prev_x * (1.0 - t) + x * t;
                        }
                    }
                    break;
                }
            }

            bottom_points.push([right_x_at_fill, bottom_fill_line_y]);

            // Right side down to bottom
            for i in (0..=body_config.bulb_curve_resolution).rev() {
                let theta = std::f32::consts::PI / 2.0
                    * (i as f32 / body_config.bulb_curve_resolution as f32);
                let x = (bulb_width * theta.cos()).max(neck_half_width);
                let y = -neck_half_height - half_height + bulb_height * theta.sin();

                if y <= bottom_fill_line_y {
                    bottom_points.push([x, y]);
                }
            }
        }

        bottom_points
    }

    /// Create a mesh from a set of 2D points
    fn create_mesh_from_points(points: Vec<[f32; 2]>) -> Option<Mesh> {
        if points.is_empty() {
            return None;
        }

        let num_vertices = points.len();
        let points_3d = points.iter().map(|p| [p[0], p[1], 0.0]).collect::<Vec<_>>();

        let coords: Vec<f32> = points.iter().flat_map(|p| vec![p[0], p[1]]).collect();
        let hole_indices: Vec<usize> = Vec::new();
        let indices: Vec<u32> = earcut(&coords, &hole_indices, 2)
            .ok()?
            .into_iter()
            .map(|i| i as u32)
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; num_vertices]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; num_vertices]);

        Some(mesh)
    }
}

/// Update sand fill percentage
pub fn update_sand_fill_percent(sand_state: &mut HourglassMeshSandState, new_fill_percent: f32) {
    let clamped_fill_percent = new_fill_percent.clamp(0.0, 1.0);
    if (sand_state.fill_percent - clamped_fill_percent).abs() > f32::EPSILON {
        sand_state.fill_percent = clamped_fill_percent;
        sand_state.sand_config.fill_percent = clamped_fill_percent;
        sand_state.needs_update = true;
    }
}

/// System to update sand meshes when fill percentage changes
pub fn update_mesh_hourglass_sand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sand_query: Query<(Entity, &mut HourglassMeshSandState), With<HourglassMesh>>,
    children_query: Query<&Children>,
    mut sand_entities_query: Query<(Entity, &HourglassMeshSand, &mut Mesh2d)>,
) {
    for (hourglass_entity, mut sand_state) in sand_query.iter_mut() {
        if !sand_state.needs_update {
            continue;
        }

        sand_state.needs_update = false;

        // Find sand child entities
        if let Ok(children) = children_query.get(hourglass_entity) {
            for child in children.iter() {
                if let Ok((entity, sand_type, mut mesh_handle)) = sand_entities_query.get_mut(child)
                {
                    match sand_type {
                        HourglassMeshSand::TopBulb => {
                            let points = HourglassMeshBuilder::generate_top_sand_points(
                                &sand_state.body_config,
                                &sand_state.sand_config,
                            );
                            if let Some(new_mesh) =
                                HourglassMeshBuilder::create_mesh_from_points(points)
                            {
                                mesh_handle.0 = meshes.add(new_mesh);
                            } else {
                                // Empty mesh - remove the mesh component
                                commands.entity(entity).remove::<Mesh2d>();
                            }
                        }
                        HourglassMeshSand::BottomBulb => {
                            let points = HourglassMeshBuilder::generate_bottom_sand_points(
                                &sand_state.body_config,
                                &sand_state.sand_config,
                            );
                            if let Some(new_mesh) =
                                HourglassMeshBuilder::create_mesh_from_points(points)
                            {
                                mesh_handle.0 = meshes.add(new_mesh);
                            } else {
                                // Empty mesh - remove the mesh component
                                commands.entity(entity).remove::<Mesh2d>();
                            }
                        }
                    }
                }
            }
        }
    }
}
