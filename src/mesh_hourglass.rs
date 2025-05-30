//! Mesh-based hourglass implementation with composable parts.

use crate::components::{Hourglass, SandSplash, SandSplashConfig};
use crate::curves::{generate_sand_outline, BulbStyle, HourglassShapeBuilder, NeckStyle, SandBulb};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::AlphaMode2d,
};
use earcutr::earcut;

/// Configuration for the hourglass body (the glass part)
#[derive(Clone, Debug)]
pub struct HourglassMeshBodyConfig {
    pub total_height: f32,
    pub bulb_style: BulbStyle,
    pub neck_style: NeckStyle,
    pub color: Color,
}

impl Default for HourglassMeshBodyConfig {
    fn default() -> Self {
        Self {
            total_height: 200.0,
            bulb_style: BulbStyle::default(),
            neck_style: NeckStyle::default(),
            color: Color::srgba(0.85, 0.95, 1.0, 0.2), // Light blue glass with transparency
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
            color: Color::srgb(0.6, 0.4, 0.2), // Wood brown color
        }
    }
}

/// Configuration for the sand inside the hourglass
#[derive(Clone, Debug)]
pub struct HourglassMeshSandConfig {
    pub color: Color,
    pub fill_percent: f32, // 0.0 to 1.0, how full the top bulb is
    pub wall_offset: f32,  // Distance in pixels from glass walls
}

impl Default for HourglassMeshSandConfig {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.9, 0.8, 0.6), // Sand color
            fill_percent: 1.0,                 // Start with full top bulb
            wall_offset: 8.0,                  // 8 pixels from glass walls
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

/// Type alias for the complex sand entities query to reduce type complexity
type SandEntitiesQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static HourglassMeshSand,
        Option<&'static mut Mesh2d>,
        Option<&'static MeshMaterial2d<ColorMaterial>>,
    ),
>;

/// Type alias for the complex mesh hourglass query to reduce type complexity
type MeshHourglassQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Hourglass, &'static mut HourglassMeshSandState),
    (With<HourglassMesh>, Changed<Hourglass>),
>;

/// Builder for creating a mesh-based hourglass
#[derive(Default)]
pub struct HourglassMeshBuilder {
    transform: Transform,
    body_config: Option<HourglassMeshBodyConfig>,
    plates_config: Option<HourglassMeshPlatesConfig>,
    sand_config: Option<HourglassMeshSandConfig>,
    sand_splash_config: Option<SandSplashConfig>,
    timing: Option<f32>,
    flip_duration: Option<f32>,
    auto_flip: Option<bool>,
}

impl HourglassMeshBuilder {
    /// Creates a new hourglass builder with the specified transform
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            body_config: None,
            plates_config: None,
            sand_config: None,
            sand_splash_config: None,
            timing: None,
            flip_duration: None,
            auto_flip: None,
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

    /// Adds automatic timing to the hourglass with the specified duration in seconds
    pub fn with_timing(mut self, duration: f32) -> Self {
        self.timing = Some(duration);
        self
    }

    /// Sets the flip animation duration
    pub fn with_flip_duration(mut self, duration: f32) -> Self {
        self.flip_duration = Some(duration);
        self
    }

    /// Sets whether the hourglass should auto-flip when empty
    pub fn with_auto_flip(mut self, auto_flip: bool) -> Self {
        self.auto_flip = Some(auto_flip);
        self
    }

    /// Adds sand splash configuration to the hourglass
    pub fn with_sand_splash(mut self, config: SandSplashConfig) -> Self {
        self.sand_splash_config = Some(config);
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
        let mut entity_commands = commands.spawn((HourglassMesh, self.transform));

        // Add automatic timing component if specified
        if let Some(duration) = self.timing {
            let mut hourglass = Hourglass::new(duration);

            // Apply flip configuration
            if let Some(flip_duration) = self.flip_duration {
                hourglass.flip_duration = flip_duration;
            }
            if let Some(auto_flip) = self.auto_flip {
                hourglass.auto_flip_when_empty = auto_flip;
            }

            entity_commands.insert(hourglass);
        }

        // Add sand splash if configured
        if let Some(sand_splash_config) = self.sand_splash_config {
            entity_commands.insert(SandSplash::new(sand_splash_config));
        }

        let hourglass_entity = entity_commands.id();

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

    /// Spawns just the hourglass body using the new curve system
    fn spawn_body(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        config: &HourglassMeshBodyConfig,
    ) -> Entity {
        // Create the hourglass shape builder from the config
        let shape_builder = HourglassShapeBuilder {
            total_height: config.total_height,
            bulb_style: config.bulb_style.clone(),
            neck_style: config.neck_style.clone(),
        };

        // Generate the hourglass outline using the composable curve system
        let outline_points = shape_builder.generate_outline();

        // Convert outline points to the format expected by mesh creation
        let points: Vec<[f32; 2]> = outline_points;

        // Create mesh from the generated points
        let mesh =
            Self::create_mesh_from_points(points).expect("Failed to create hourglass body mesh");

        // Create glass material with transparency
        let glass_material = materials.add(ColorMaterial {
            color: config.color,
            alpha_mode: AlphaMode2d::Blend,
            ..default()
        });

        commands
            .spawn((
                HourglassMeshBody,
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(glass_material),
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
                Transform::from_xyz(0.0, half_total_height + config.height / 2.0, 0.0),
            ))
            .id();

        // Spawn bottom plate
        let bottom_plate = commands
            .spawn((
                HourglassMeshPlate::Bottom,
                Mesh2d(plate_mesh_handle),
                MeshMaterial2d(plate_material),
                Transform::from_xyz(0.0, -half_total_height - config.height / 2.0, 0.0),
            ))
            .id();

        (top_plate, bottom_plate)
    }

    /// Spawns the sand inside the hourglass using the new curve system
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

        // Generate the hourglass outline first (this will be used as a base for sand generation)
        let shape_builder = HourglassShapeBuilder {
            total_height: body_config.total_height,
            bulb_style: body_config.bulb_style.clone(),
            neck_style: body_config.neck_style.clone(),
        };

        let hourglass_outline =
            shape_builder.generate_outline_with_wall_offset(sand_config.wall_offset);

        // Generate top sand mesh using the new curve system
        let half_height = body_config.total_height / 2.0;
        let top_points = generate_sand_outline(
            &hourglass_outline,
            sand_config.fill_percent,
            sand_config.wall_offset,
            SandBulb::Top,
            body_config.neck_style.height(),
            -half_height,
            half_height,
        );

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

        // Generate bottom sand mesh using the new curve system
        let bottom_points = generate_sand_outline(
            &hourglass_outline,
            sand_config.fill_percent,
            sand_config.wall_offset,
            SandBulb::Bottom,
            body_config.neck_style.height(),
            -half_height,
            half_height,
        );

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

    /// Create a mesh from a set of 2D points
    fn create_mesh_from_points(points: Vec<[f32; 2]>) -> Option<Mesh> {
        if points.is_empty() {
            return None;
        }

        let num_vertices = points.len();
        let points_3d = points.iter().map(|p| [p[0], p[1], 0.0]).collect::<Vec<_>>();

        let coords: Vec<f32> = points.iter().flat_map(|p| vec![p[0], p[1]]).collect();
        let hole_indices: Vec<usize> = Vec::new();

        match earcut(&coords, &hole_indices, 2) {
            Ok(triangles) => {
                let indices: Vec<u32> = triangles.into_iter().map(|i| i as u32).collect();

                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
                mesh.insert_indices(Indices::U32(indices));
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; num_vertices]);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; num_vertices]);

                Some(mesh)
            }
            Err(_) => None,
        }
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

/// System to update sand meshes when fill percentage changes using the new curve system
pub fn update_mesh_hourglass_sand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut sand_query: Query<(Entity, &mut HourglassMeshSandState), With<HourglassMesh>>,
    children_query: Query<&Children>,
    mut sand_entities_query: SandEntitiesQuery,
) {
    for (hourglass_entity, mut sand_state) in sand_query.iter_mut() {
        if !sand_state.needs_update {
            continue;
        }

        sand_state.needs_update = false;

        // Generate the hourglass outline for sand calculations
        let shape_builder = HourglassShapeBuilder {
            total_height: sand_state.body_config.total_height,
            bulb_style: sand_state.body_config.bulb_style.clone(),
            neck_style: sand_state.body_config.neck_style.clone(),
        };

        let hourglass_outline =
            shape_builder.generate_outline_with_wall_offset(sand_state.sand_config.wall_offset);

        // Find sand child entities
        if let Ok(children) = children_query.get(hourglass_entity) {
            for child in children.iter() {
                if let Ok((entity, sand_type, mesh_handle_opt, material_opt)) =
                    sand_entities_query.get_mut(child)
                {
                    match sand_type {
                        HourglassMeshSand::TopBulb => {
                            let half_height = sand_state.body_config.total_height / 2.0;
                            let points = generate_sand_outline(
                                &hourglass_outline,
                                sand_state.sand_config.fill_percent,
                                sand_state.sand_config.wall_offset,
                                SandBulb::Top,
                                sand_state.body_config.neck_style.height(),
                                -half_height,
                                half_height,
                            );

                            if let Some(new_mesh) =
                                HourglassMeshBuilder::create_mesh_from_points(points)
                            {
                                let mesh_handle = meshes.add(new_mesh);
                                if let Some(mut existing_mesh) = mesh_handle_opt {
                                    existing_mesh.0 = mesh_handle;
                                } else {
                                    // Add mesh component back if it was removed
                                    let material = if let Some(mat) = material_opt {
                                        mat.clone()
                                    } else {
                                        MeshMaterial2d(materials.add(sand_state.sand_config.color))
                                    };
                                    commands
                                        .entity(entity)
                                        .insert((Mesh2d(mesh_handle), material));
                                }
                            } else {
                                // Empty mesh - remove the mesh component if it exists
                                if mesh_handle_opt.is_some() {
                                    commands.entity(entity).remove::<Mesh2d>();
                                }
                            }
                        }
                        HourglassMeshSand::BottomBulb => {
                            let half_height = sand_state.body_config.total_height / 2.0;
                            let points = generate_sand_outline(
                                &hourglass_outline,
                                sand_state.sand_config.fill_percent,
                                sand_state.sand_config.wall_offset,
                                SandBulb::Bottom,
                                sand_state.body_config.neck_style.height(),
                                -half_height,
                                half_height,
                            );

                            if let Some(new_mesh) =
                                HourglassMeshBuilder::create_mesh_from_points(points)
                            {
                                let mesh_handle = meshes.add(new_mesh);
                                if let Some(mut existing_mesh) = mesh_handle_opt {
                                    existing_mesh.0 = mesh_handle;
                                } else {
                                    // Add mesh component back if it was removed
                                    let material = if let Some(mat) = material_opt {
                                        mat.clone()
                                    } else {
                                        MeshMaterial2d(materials.add(sand_state.sand_config.color))
                                    };
                                    commands
                                        .entity(entity)
                                        .insert((Mesh2d(mesh_handle), material));
                                }
                            } else {
                                // Empty mesh - remove the mesh component if it exists
                                if mesh_handle_opt.is_some() {
                                    commands.entity(entity).remove::<Mesh2d>();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// System to sync Hourglass component state with HourglassMeshSandState
pub fn sync_mesh_hourglass_with_timer(mut mesh_query: MeshHourglassQuery) {
    for (hourglass, mut sand_state) in mesh_query.iter_mut() {
        // Always use upper_chamber for visual top bulb fill - keep it simple
        update_sand_fill_percent(&mut sand_state, hourglass.upper_chamber);
    }
}

/// Spawn a mesh-based hourglass with automatic timing and default configuration
pub fn spawn_mesh_hourglass_with_timer(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    duration: f32,
    position: Vec3,
) -> Entity {
    HourglassMeshBuilder::new(Transform::from_translation(position))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_timing(duration)
        .build(commands, meshes, materials)
}

/// Spawn a mesh-based hourglass with flip configuration
pub fn spawn_mesh_hourglass_with_flip(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    duration: f32,
    position: Vec3,
    flip_duration: f32,
    auto_flip: bool,
) -> Entity {
    HourglassMeshBuilder::new(Transform::from_translation(position))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_timing(duration)
        .with_flip_duration(flip_duration)
        .with_auto_flip(auto_flip)
        .build(commands, meshes, materials)
}

/// Create a hourglass with a specific bulb and neck style
pub fn spawn_styled_mesh_hourglass(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    duration: f32,
    position: Vec3,
    bulb_style: BulbStyle,
    neck_style: NeckStyle,
) -> Entity {
    let body_config = HourglassMeshBodyConfig {
        bulb_style,
        neck_style,
        ..Default::default()
    };

    HourglassMeshBuilder::new(Transform::from_translation(position))
        .with_body(body_config)
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_timing(duration)
        .build(commands, meshes, materials)
}
