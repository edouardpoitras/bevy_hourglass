//! Composable curve generation system for hourglass shapes.

use std::f32::consts::PI;

/// A 2D point
pub type Point2D = [f32; 2];

/// Trait for generating curve segments
pub trait CurveGenerator: Send + Sync {
    /// Generate points along the curve with the specified resolution
    fn generate_points(&self, resolution: usize) -> Vec<Point2D>;

    /// Get the start point of the curve
    fn start_point(&self) -> Point2D;

    /// Get the end point of the curve
    fn end_point(&self) -> Point2D;
}

/// Configuration for a circular arc curve
#[derive(Debug, Clone)]
pub struct CircularArc {
    pub center: Point2D,
    pub radius: f32,
    pub start_angle: f32, // in radians
    pub end_angle: f32,   // in radians
    pub clockwise: bool,
}

impl CircularArc {
    /// Create a new circular arc
    pub fn new(
        center: Point2D,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        clockwise: bool,
    ) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
            clockwise,
        }
    }

    /// Create a quarter circle arc (90 degrees)
    pub fn quarter_circle(center: Point2D, radius: f32, quadrant: CircleQuadrant) -> Self {
        let (start_angle, end_angle) = match quadrant {
            CircleQuadrant::TopRight => (0.0, PI / 2.0),
            CircleQuadrant::TopLeft => (PI / 2.0, PI),
            CircleQuadrant::BottomLeft => (PI, 3.0 * PI / 2.0),
            CircleQuadrant::BottomRight => (3.0 * PI / 2.0, 2.0 * PI),
        };

        Self::new(center, radius, start_angle, end_angle, false)
    }
}

impl CurveGenerator for CircularArc {
    fn generate_points(&self, resolution: usize) -> Vec<Point2D> {
        if resolution == 0 {
            return vec![self.start_point(), self.end_point()];
        }

        let mut points = Vec::with_capacity(resolution + 1);
        let angle_diff = if self.clockwise {
            if self.end_angle <= self.start_angle {
                self.end_angle + 2.0 * PI - self.start_angle
            } else {
                self.end_angle - self.start_angle
            }
        } else if self.end_angle >= self.start_angle {
            self.end_angle - self.start_angle
        } else {
            self.end_angle + 2.0 * PI - self.start_angle
        };

        for i in 0..=resolution {
            let t = i as f32 / resolution as f32;
            let angle = if self.clockwise {
                self.start_angle - t * angle_diff
            } else {
                self.start_angle + t * angle_diff
            };

            let x = self.center[0] + self.radius * angle.cos();
            let y = self.center[1] + self.radius * angle.sin();
            points.push([x, y]);
        }

        points
    }

    fn start_point(&self) -> Point2D {
        [
            self.center[0] + self.radius * self.start_angle.cos(),
            self.center[1] + self.radius * self.start_angle.sin(),
        ]
    }

    fn end_point(&self) -> Point2D {
        [
            self.center[0] + self.radius * self.end_angle.cos(),
            self.center[1] + self.radius * self.end_angle.sin(),
        ]
    }
}

/// Quadrants for quarter circle generation
#[derive(Debug, Clone, Copy)]
pub enum CircleQuadrant {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}

/// Configuration for a smooth transition curve between two points
#[derive(Debug, Clone)]
pub struct SmoothTransition {
    pub start: Point2D,
    pub end: Point2D,
    pub curvature: f32, // 0.0 = straight line, 1.0 = maximum curve
    pub curve_direction: CurveDirection,
}

impl SmoothTransition {
    /// Create a new smooth transition curve
    pub fn new(
        start: Point2D,
        end: Point2D,
        curvature: f32,
        curve_direction: CurveDirection,
    ) -> Self {
        Self {
            start,
            end,
            curvature: curvature.max(0.0), // Allow values > 1.0 for more extreme curves
            curve_direction,
        }
    }

    /// Create a straight line (no curvature)
    pub fn straight_line(start: Point2D, end: Point2D) -> Self {
        Self::new(start, end, 0.0, CurveDirection::None)
    }
}

impl CurveGenerator for SmoothTransition {
    fn generate_points(&self, resolution: usize) -> Vec<Point2D> {
        if resolution == 0 {
            return vec![self.start, self.end];
        }

        let mut points = Vec::with_capacity(resolution + 1);

        for i in 0..=resolution {
            let t = i as f32 / resolution as f32;

            if self.curvature == 0.0 {
                // Straight line interpolation
                let x = self.start[0] * (1.0 - t) + self.end[0] * t;
                let y = self.start[1] * (1.0 - t) + self.end[1] * t;
                points.push([x, y]);
            } else {
                // Curved interpolation using sine wave for smooth transitions
                let curve_offset = match self.curve_direction {
                    CurveDirection::None => 0.0,
                    CurveDirection::Inward => -self.curvature * (t * PI).sin(),
                    CurveDirection::Outward => self.curvature * (t * PI).sin(),
                };

                // Base linear interpolation
                let base_x = self.start[0] * (1.0 - t) + self.end[0] * t;
                let base_y = self.start[1] * (1.0 - t) + self.end[1] * t;

                // Apply curve offset perpendicular to the line direction
                let dx = self.end[0] - self.start[0];
                let dy = self.end[1] - self.start[1];
                let length = (dx * dx + dy * dy).sqrt();

                if length > 0.0 {
                    // Perpendicular direction (rotated 90 degrees)
                    let perp_x = -dy / length;
                    let perp_y = dx / length;

                    let x = base_x + perp_x * curve_offset * length * 0.1; // Scale factor for reasonable curve
                    let y = base_y + perp_y * curve_offset * length * 0.1;
                    points.push([x, y]);
                } else {
                    points.push([base_x, base_y]);
                }
            }
        }

        points
    }

    fn start_point(&self) -> Point2D {
        self.start
    }

    fn end_point(&self) -> Point2D {
        self.end
    }
}

/// Direction of curve bending
#[derive(Debug, Clone, Copy)]
pub enum CurveDirection {
    None,
    Inward,
    Outward,
}

/// A composite curve made up of multiple curve segments
pub struct CompositeCurve {
    pub segments: Vec<Box<dyn CurveGenerator>>,
}

impl CompositeCurve {
    /// Create a new composite curve
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Add a curve segment to the composite
    pub fn add_segment(mut self, segment: Box<dyn CurveGenerator>) -> Self {
        self.segments.push(segment);
        self
    }

    /// Add a circular arc segment
    pub fn add_arc(self, arc: CircularArc) -> Self {
        self.add_segment(Box::new(arc))
    }

    /// Add a smooth transition segment
    pub fn add_transition(self, transition: SmoothTransition) -> Self {
        self.add_segment(Box::new(transition))
    }
}

impl CurveGenerator for CompositeCurve {
    fn generate_points(&self, resolution: usize) -> Vec<Point2D> {
        if self.segments.is_empty() {
            return Vec::new();
        }

        let mut all_points = Vec::new();
        let segment_resolution = resolution / self.segments.len().max(1);

        for (i, segment) in self.segments.iter().enumerate() {
            let mut segment_points = segment.generate_points(segment_resolution);

            // Skip the first point of subsequent segments to avoid duplication
            if i > 0 && !segment_points.is_empty() {
                segment_points.remove(0);
            }

            all_points.extend(segment_points);
        }

        all_points
    }

    fn start_point(&self) -> Point2D {
        self.segments
            .first()
            .map(|s| s.start_point())
            .unwrap_or([0.0, 0.0])
    }

    fn end_point(&self) -> Point2D {
        self.segments
            .last()
            .map(|s| s.end_point())
            .unwrap_or([0.0, 0.0])
    }
}

impl Default for CompositeCurve {
    fn default() -> Self {
        Self::new()
    }
}

/// Different styles for hourglass bulbs
#[derive(Debug, Clone)]
pub enum BulbStyle {
    /// Circular bulbs with adjustable curvature
    Circular {
        curvature: f32,
        width_factor: f32,
        curve_resolution: usize,
    },
    /// Straight-sided bulbs (triangular shape)
    Straight { width_factor: f32 },
}

impl BulbStyle {
    /// Get the width factor for this bulb style
    pub fn width_factor(&self) -> f32 {
        match self {
            BulbStyle::Circular { width_factor, .. } => *width_factor,
            BulbStyle::Straight { width_factor } => *width_factor,
        }
    }

    /// Get the curve resolution for this bulb style
    pub fn curve_resolution(&self) -> usize {
        match self {
            BulbStyle::Circular {
                curve_resolution, ..
            } => *curve_resolution,
            BulbStyle::Straight { .. } => 2, // Minimal resolution for straight lines
        }
    }
}

impl Default for BulbStyle {
    fn default() -> Self {
        Self::Circular {
            curvature: 1.0,
            width_factor: 0.75,
            curve_resolution: 20,
        }
    }
}

/// Different styles for hourglass necks
#[derive(Debug, Clone)]
pub enum NeckStyle {
    /// Straight neck
    Straight { width: f32, height: f32 },
    /// Curved neck that bends inward
    Curved {
        curvature: f32,
        width: f32,
        height: f32,
        curve_resolution: usize,
    },
}

impl NeckStyle {
    /// Get the width for this neck style
    pub fn width(&self) -> f32 {
        match self {
            NeckStyle::Straight { width, .. } => width.max(3.0),
            NeckStyle::Curved { width, .. } => width.max(3.0),
        }
    }

    /// Get the width for this neck style with wall offset constraint
    /// Ensures neck width is always at least 2 * wall_offset + minimum_gap
    pub fn width_with_wall_offset(&self, wall_offset: f32) -> f32 {
        let minimum_gap = 2.0; // Minimum gap between sand on left and right sides of neck
        let minimum_width = 2.0 * wall_offset + minimum_gap;
        self.width().max(minimum_width)
    }

    /// Get the height for this neck style
    pub fn height(&self) -> f32 {
        match self {
            NeckStyle::Straight { height, .. } => *height,
            NeckStyle::Curved { height, .. } => *height,
        }
    }

    /// Get the curve resolution for this neck style
    pub fn curve_resolution(&self) -> usize {
        match self {
            NeckStyle::Straight { .. } => 2, // Minimal resolution for straight lines
            NeckStyle::Curved {
                curve_resolution, ..
            } => *curve_resolution,
        }
    }
}

impl Default for NeckStyle {
    fn default() -> Self {
        Self::Curved {
            curvature: 0.2,
            width: 12.0,
            height: 8.0,
            curve_resolution: 5,
        }
    }
}

/// Builder for creating hourglass shape outlines using curves
pub struct HourglassShapeBuilder {
    pub total_height: f32,
    pub bulb_style: BulbStyle,
    pub neck_style: NeckStyle,
}

impl HourglassShapeBuilder {
    /// Create a new hourglass shape builder
    pub fn new() -> Self {
        Self {
            total_height: 200.0,
            bulb_style: BulbStyle::default(),
            neck_style: NeckStyle::default(),
        }
    }

    /// Set the bulb style
    pub fn with_bulb_style(mut self, style: BulbStyle) -> Self {
        self.bulb_style = style;
        self
    }

    /// Set the neck style
    pub fn with_neck_style(mut self, style: NeckStyle) -> Self {
        self.neck_style = style;
        self
    }

    /// Generate the complete hourglass outline
    pub fn generate_outline(&self) -> Vec<Point2D> {
        self.generate_outline_with_wall_offset(0.0)
    }

    /// Generate the complete hourglass outline with wall offset constraint for sand generation
    pub fn generate_outline_with_wall_offset(&self, wall_offset: f32) -> Vec<Point2D> {
        let half_height = self.total_height / 2.0;
        let neck_width = if wall_offset > 0.0 {
            self.neck_style.width_with_wall_offset(wall_offset)
        } else {
            self.neck_style.width()
        };
        let neck_half_width = neck_width / 2.0;
        let neck_half_height = self.neck_style.height() / 2.0;

        // Calculate bulb dimensions based on total_height as the authoritative dimension
        let bulb_height = (self.total_height - self.neck_style.height()) / 2.0;
        let bulb_width = bulb_height * self.bulb_style.width_factor();

        let mut outline = Vec::new();

        // Bottom bulb (left side, bottom to neck)
        let bottom_bulb_left = self.create_bulb_curve(
            [-bulb_width, -half_height],
            [-neck_half_width, -neck_half_height],
            BulbSection::BottomLeft,
        );
        outline.extend(bottom_bulb_left.generate_points(self.bulb_style.curve_resolution()));

        // Left neck curve (bottom to top)
        let left_neck = self.create_neck_curve(
            [-neck_half_width, -neck_half_height],
            [-neck_half_width, neck_half_height],
            NeckSection::Left,
        );
        let mut left_neck_points = left_neck.generate_points(self.neck_style.curve_resolution());
        if !left_neck_points.is_empty() {
            left_neck_points.remove(0); // Remove duplicate point
        }
        outline.extend(left_neck_points);

        // Top bulb (left side, neck to top)
        let top_bulb_left = self.create_bulb_curve(
            [-neck_half_width, neck_half_height],
            [-bulb_width, half_height],
            BulbSection::TopLeft,
        );
        let mut top_bulb_left_points =
            top_bulb_left.generate_points(self.bulb_style.curve_resolution());
        if !top_bulb_left_points.is_empty() {
            top_bulb_left_points.remove(0);
        }
        outline.extend(top_bulb_left_points);

        // TODO: Allow for curved top cap
        // Top cap
        outline.push([bulb_width, half_height]);

        // Top bulb (right side, top to neck)
        let top_bulb_right = self.create_bulb_curve(
            [bulb_width, half_height],
            [neck_half_width, neck_half_height],
            BulbSection::TopRight,
        );
        let mut top_bulb_right_points =
            top_bulb_right.generate_points(self.bulb_style.curve_resolution());
        if !top_bulb_right_points.is_empty() {
            top_bulb_right_points.remove(0);
        }
        outline.extend(top_bulb_right_points);

        // Right neck curve (top to bottom)
        let right_neck = self.create_neck_curve(
            [neck_half_width, neck_half_height],
            [neck_half_width, -neck_half_height],
            NeckSection::Right,
        );
        let mut right_neck_points = right_neck.generate_points(self.neck_style.curve_resolution());
        if !right_neck_points.is_empty() {
            right_neck_points.remove(0);
        }
        outline.extend(right_neck_points);

        // Bottom bulb (right side, neck to bottom)
        let bottom_bulb_right = self.create_bulb_curve(
            [neck_half_width, -neck_half_height],
            [bulb_width, -half_height],
            BulbSection::BottomRight,
        );
        let mut bottom_bulb_right_points =
            bottom_bulb_right.generate_points(self.bulb_style.curve_resolution());
        if !bottom_bulb_right_points.is_empty() {
            bottom_bulb_right_points.remove(0);
        }
        outline.extend(bottom_bulb_right_points);

        // TODO: Allow for curved bottom cap
        outline
    }

    /// Create a bulb curve based on the bulb style
    fn create_bulb_curve(
        &self,
        start: Point2D,
        end: Point2D,
        section: BulbSection,
    ) -> Box<dyn CurveGenerator> {
        match &self.bulb_style {
            BulbStyle::Circular { curvature, .. } => {
                let curve_direction = match section {
                    BulbSection::BottomLeft | BulbSection::TopLeft => CurveDirection::Outward,
                    BulbSection::BottomRight | BulbSection::TopRight => CurveDirection::Outward,
                };

                Box::new(SmoothTransition::new(
                    start,
                    end,
                    *curvature,
                    curve_direction,
                ))
            }
            BulbStyle::Straight { .. } => Box::new(SmoothTransition::straight_line(start, end)),
        }
    }

    /// Create a neck curve based on the neck style
    fn create_neck_curve(
        &self,
        start: Point2D,
        end: Point2D,
        _section: NeckSection,
    ) -> Box<dyn CurveGenerator> {
        match &self.neck_style {
            NeckStyle::Straight { .. } => Box::new(SmoothTransition::straight_line(start, end)),
            NeckStyle::Curved { curvature, .. } => Box::new(SmoothTransition::new(
                start,
                end,
                *curvature,
                CurveDirection::Inward,
            )),
        }
    }
}

impl Default for HourglassShapeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Sections of the bulb for curve generation
#[derive(Debug, Clone, Copy)]
enum BulbSection {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Sections of the neck for curve generation
#[derive(Debug, Clone, Copy)]
enum NeckSection {
    Left,
    Right,
}

/// Generate sand shape points using the same curve system with smooth fill line interpolation
#[allow(clippy::too_many_arguments)]
pub fn generate_sand_outline(
    hourglass_outline: &[Point2D],
    fill_percent: f32,
    wall_offset: f32,
    bulb: SandBulb,
    neck_height: f32,
    min_y: f32,
    max_y: f32,
    bottom_mound_factor: f32,
) -> Vec<Point2D> {
    if hourglass_outline.is_empty() {
        return Vec::new();
    }

    // The hourglass is always centered at y=0
    let center_y = 0.0;

    // Calculate neck boundaries to prevent bottom sand from entering neck area
    let neck_bottom = center_y - (neck_height / 2.0);

    // Calculate fill line based on which bulb and fill percentage
    let fill_line = match bulb {
        SandBulb::Top => {
            // For top bulb: fill_percent represents how much of the top bulb is filled
            // 1.0 = full top bulb (fill_line at max_y), 0.0 = empty top bulb (fill_line at center_y)
            center_y + (fill_percent * (max_y - center_y))
        }
        SandBulb::Bottom => {
            // For bottom bulb: fill based on how much sand has drained from top (1.0 - fill_percent)
            // When fill_percent = 0.0 (empty top), bottom should be full (fill_line at neck_bottom)
            // When fill_percent = 1.0 (full top), bottom should be empty (fill_line at min_y)
            // IMPORTANT: Bottom sand should never go above neck_bottom to prevent entering neck area
            let max_bottom_fill = neck_bottom;
            min_y + ((1.0 - fill_percent) * (max_bottom_fill - min_y))
        }
    };

    // Generate points with smooth fill line interpolation
    let filtered_points = match bulb {
        SandBulb::Bottom => generate_outline_with_mounded_fill_line(
            hourglass_outline,
            fill_line,
            center_y,
            bottom_mound_factor,
            fill_percent,
        ),
        SandBulb::Top => {
            generate_outline_with_fill_line(hourglass_outline, fill_line, bulb, center_y)
        }
    };

    if filtered_points.is_empty() {
        return Vec::new();
    }

    let mut sand_points = Vec::new();
    let neck_region_height = neck_height / 2.0; // Check points within half the neck height of center

    // Apply offsetting with special handling at the neck
    for point in filtered_points {
        let mut offset_to_use = wall_offset;

        // Check if this point is in the neck region
        if (point[1] - center_y).abs() <= neck_region_height {
            // For neck region points, ensure we don't cross the center
            // Calculate what the offset point would be
            let potential_offset_x = if point[0] >= 0.0 {
                point[0] - wall_offset
            } else {
                point[0] + wall_offset
            };

            // Check if this would cross the center line (with 1 pixel minimum gap)
            if point[0] >= 0.0 && potential_offset_x <= 0.5 {
                // Right side would cross to left - limit offset
                offset_to_use = (point[0] - 0.5).max(0.0);
            } else if point[0] < 0.0 && potential_offset_x >= -0.5 {
                // Left side would cross to right - limit offset
                offset_to_use = (-point[0] - 0.5).max(0.0);
            }
        }

        let offset_point = if point[0] >= 0.0 {
            // Right side of hourglass - move left (inward)
            [point[0] - offset_to_use, point[1]]
        } else {
            // Left side of hourglass - move right (inward)
            [point[0] + offset_to_use, point[1]]
        };
        sand_points.push(offset_point);
    }

    // For top bulb, add falling sand stream from neck to bottom only when sand is still flowing
    // fill_percent > 0.0 means there's still sand in the top bulb and it's flowing
    if matches!(bulb, SandBulb::Top) && !sand_points.is_empty() && fill_percent > 0.0 {
        // The last point should be the right neck point
        // The first point should be the left neck point
        let left_neck_x = sand_points.first().unwrap()[0];
        let right_neck_x = sand_points.last().unwrap()[0];

        // Add points extending from neck to bottom of glass
        sand_points.push([right_neck_x, min_y]);
        sand_points.push([left_neck_x, min_y]);
    }

    sand_points
}

/// Generate outline points with smooth fill line interpolation
fn generate_outline_with_fill_line(
    hourglass_outline: &[Point2D],
    fill_line: f32,
    bulb: SandBulb,
    center_y: f32,
) -> Vec<Point2D> {
    let mut result_points = Vec::new();
    let mut fill_line_intersections = Vec::new();

    // Process each segment of the outline
    for i in 0..hourglass_outline.len() {
        let current_point = hourglass_outline[i];
        let next_point = hourglass_outline[(i + 1) % hourglass_outline.len()];

        // Check if current point should be included based on bulb and fill level
        let current_included = match bulb {
            SandBulb::Top => current_point[1] >= center_y && current_point[1] <= fill_line,
            SandBulb::Bottom => current_point[1] <= center_y && current_point[1] <= fill_line,
        };

        let next_included = match bulb {
            SandBulb::Top => next_point[1] >= center_y && next_point[1] <= fill_line,
            SandBulb::Bottom => next_point[1] <= center_y && next_point[1] <= fill_line,
        };

        // Add current point if it should be included
        if current_included {
            result_points.push(current_point);
        }

        // Check if the segment crosses the fill line
        let segment_crosses_fill_line = (current_point[1] <= fill_line
            && next_point[1] > fill_line)
            || (current_point[1] > fill_line && next_point[1] <= fill_line);

        if segment_crosses_fill_line {
            // Calculate intersection point with fill line
            if let Some(intersection) =
                calculate_line_intersection(current_point, next_point, fill_line)
            {
                // Check if this intersection should be included based on bulb type
                let intersection_valid = match bulb {
                    SandBulb::Top => intersection[1] >= center_y,
                    SandBulb::Bottom => intersection[1] <= center_y,
                };

                if intersection_valid {
                    // Store intersection for later addition
                    fill_line_intersections.push(intersection);

                    // Add intersection point if transitioning between included/not-included states
                    if current_included != next_included {
                        result_points.push(intersection);
                    }
                }
            }
        }
    }

    // Add fill line intersections to close the shape for partial fills
    if !fill_line_intersections.is_empty() {
        // Sort intersections by x-coordinate
        fill_line_intersections.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        // Add intersections in the appropriate order based on bulb type
        match bulb {
            SandBulb::Top => {
                // For top bulb, add intersections in reverse order to close the shape properly
                for intersection in fill_line_intersections.iter().rev() {
                    if !result_points.contains(intersection) {
                        result_points.push(*intersection);
                    }
                }
            }
            SandBulb::Bottom => {
                // For bottom bulb, add intersections in forward order
                for intersection in fill_line_intersections.iter() {
                    if !result_points.contains(intersection) {
                        result_points.push(*intersection);
                    }
                }
            }
        }
    }

    result_points
}

/// Calculate intersection point between a line segment and a horizontal line
fn calculate_line_intersection(p1: Point2D, p2: Point2D, y_line: f32) -> Option<Point2D> {
    // Check if the segment is horizontal (parallel to the fill line)
    if (p1[1] - p2[1]).abs() < f32::EPSILON {
        return None;
    }

    // Check if y_line is within the segment's y range
    let min_y = p1[1].min(p2[1]);
    let max_y = p1[1].max(p2[1]);

    if y_line < min_y || y_line > max_y {
        return None;
    }

    // Calculate intersection using linear interpolation
    let t = (y_line - p1[1]) / (p2[1] - p1[1]);
    let x_intersection = p1[0] + t * (p2[0] - p1[0]);

    Some([x_intersection, y_line])
}

/// Generate outline points with mounded fill line for bottom bulb only
fn generate_outline_with_mounded_fill_line(
    hourglass_outline: &[Point2D],
    base_fill_line: f32,
    center_y: f32,
    bottom_mound_factor: f32,
    fill_percent: f32,
) -> Vec<Point2D> {
    if bottom_mound_factor == 0.0 {
        // No mound - use regular flat fill line
        return generate_outline_with_fill_line(
            hourglass_outline,
            base_fill_line,
            SandBulb::Bottom,
            center_y,
        );
    }

    let mut result_points = Vec::new();
    let mut fill_line_intersections = Vec::new();

    // Calculate mound parameters
    // Mound is most pronounced when there's little sand (high fill_percent in top bulb)
    // As sand accumulates in bottom, mound flattens out
    let mound_strength = bottom_mound_factor * fill_percent; // More mound when top is fuller

    // Find the width of the bulb at the base fill line to determine mound extent
    let mut left_x = 0.0;
    let mut right_x = 0.0;

    // Find intersections with the base fill line to determine sand width
    for i in 0..hourglass_outline.len() {
        let current_point = hourglass_outline[i];
        let next_point = hourglass_outline[(i + 1) % hourglass_outline.len()];

        if let Some(intersection) =
            calculate_line_intersection(current_point, next_point, base_fill_line)
        {
            if intersection[0] < 0.0 {
                left_x = intersection[0];
            } else {
                right_x = intersection[0];
            }
        }
    }

    let sand_width = right_x - left_x;
    if sand_width <= 0.0 {
        return generate_outline_with_fill_line(
            hourglass_outline,
            base_fill_line,
            SandBulb::Bottom,
            center_y,
        );
    }

    // Calculate mounded fill line closure
    let mounded_fill_line = |x: f32| -> f32 {
        if sand_width <= 0.0 {
            return base_fill_line;
        }

        // Normalize x position within sand width (-1.0 to 1.0)
        let normalized_x = (x - (left_x + right_x) * 0.5) / (sand_width * 0.5);
        let normalized_x = normalized_x.clamp(-1.0, 1.0);

        // Create parabolic mound: highest at center (x=0), zero at edges
        let mound_height = mound_strength * sand_width * 0.1 * (1.0 - normalized_x * normalized_x);
        base_fill_line + mound_height
    };

    // Process each segment of the outline
    for i in 0..hourglass_outline.len() {
        let current_point = hourglass_outline[i];
        let next_point = hourglass_outline[(i + 1) % hourglass_outline.len()];

        // Check if current point should be included - for bottom bulb only
        let current_mounded_fill = mounded_fill_line(current_point[0]);
        let current_included =
            current_point[1] <= center_y && current_point[1] <= current_mounded_fill;

        let next_mounded_fill = mounded_fill_line(next_point[0]);
        let next_included = next_point[1] <= center_y && next_point[1] <= next_mounded_fill;

        // Add current point if it should be included
        if current_included {
            result_points.push(current_point);
        }

        // For segments that cross the mounded fill line, we need to find intersections
        // This is more complex than the flat case since our fill line is curved
        if current_included != next_included {
            // Approximate intersection by sampling along the segment
            let samples = 10;
            for j in 1..samples {
                let t = j as f32 / samples as f32;
                let sample_x = current_point[0] * (1.0 - t) + next_point[0] * t;
                let sample_y = current_point[1] * (1.0 - t) + next_point[1] * t;
                let sample_mounded_fill = mounded_fill_line(sample_x);

                let sample_included = sample_y <= center_y && sample_y <= sample_mounded_fill;

                if sample_included != current_included {
                    // Found approximate intersection
                    fill_line_intersections.push([sample_x, sample_mounded_fill]);
                    result_points.push([sample_x, sample_mounded_fill]);
                    break;
                }
            }
        }
    }

    // Add mounded fill line points to close the shape
    if !fill_line_intersections.is_empty() {
        // Sort intersections by x-coordinate
        fill_line_intersections.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        // Generate additional points along the mounded fill line for smooth curve
        if fill_line_intersections.len() >= 2 {
            let leftmost = fill_line_intersections[0];
            let rightmost = fill_line_intersections[fill_line_intersections.len() - 1];

            // Add points along the mounded curve between intersections
            let curve_samples = 20;
            let mut curve_points = Vec::new();

            for i in 0..=curve_samples {
                let t = i as f32 / curve_samples as f32;
                let x = leftmost[0] * (1.0 - t) + rightmost[0] * t;
                let y = mounded_fill_line(x);
                curve_points.push([x, y]);
            }

            // For bottom bulb, add curve points in forward order
            result_points.extend(&curve_points);
        }
    }

    result_points
}

/// Which bulb to generate sand for
#[derive(Debug, Clone, Copy)]
pub enum SandBulb {
    Top,
    Bottom,
}
