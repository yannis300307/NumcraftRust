// In src/renderer.rs

#[cfg(target_os = "none")]
use alloc::format; // Needed for string formatting like FPS display

#[cfg(target_os = "none")]
use alloc::vec::Vec; // Needed for dynamic arrays like triangles_to_render

use nalgebra::{Matrix4, Perspective3, Vector2, Vector3, Vector4}; // Linear algebra types for 3D math

use core::{cmp::Ordering, f32, mem::swap}; // Core utilities for comparisons, float numbers, and swapping values

use crate::{
    camera::Camera, // The camera controlling the view
    constants::{ // Global constants and specific modules
        get_quad_color_from_texture_id, // Function to get color from texture ID
        rendering::*, // All rendering-related constants (e.g., SCREEN_WIDTH)
        world, // World module constants (e.g., CHUNK_SIZE)
    },
    eadk::{self, Color, Rect, display}, // EADK library types and display functions
    mesh::{Quad, Triangle, Triangle2D}, // Mesh geometry definitions
    player::Player, // The player character data
    world::World, // The game world data
};

// Display Inventory Collor 
use crate::constants::{UI_BLACK, UI_LIGHT_GREY, UI_RED};
// For Item Constant
use crate::constants::{BlockType, rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, QuadDir};

// Screen dimensions as floating-point numbers
const SCREEN_WIDTHF: f32 = SCREEN_WIDTH as f32;
const SCREEN_HEIGHTF: f32 = SCREEN_HEIGHT as f32;

// Dimensions for screen tiling, used for rendering in smaller segments
const SCREEN_TILE_WIDTH: usize = SCREEN_WIDTH.div_ceil(SCREEN_TILE_SUBDIVISION);
const SCREEN_TILE_HEIGHT: usize = SCREEN_HEIGHT.div_ceil(SCREEN_TILE_SUBDIVISION);
const SCREEN_TILE_WIDTHF: f32 = SCREEN_TILE_WIDTH as f32;
const SCREEN_TILE_HEIGHTF: f32 = SCREEN_TILE_HEIGHT as f32;

const HALF_SCREEN_TILE_WIDTHF: f32 = SCREEN_TILE_WIDTHF / 2.0;
const HALF_SCREEN_TILE_HEIGHTF: f32 = SCREEN_TILE_HEIGHTF / 2.0;

// 3D projection parameters
const ASPECT_RATIO: f32 = SCREEN_WIDTHF / SCREEN_HEIGHTF;
const ZNEAR: f32 = 1.0; // Near clipping plane distance
const ZFAR: f32 = 1000.0; // Far clipping plane distance

// Chunk size as a signed integer
const CHUNK_SIZE_I: isize = world::CHUNK_SIZE as isize;

// Font and crosshair image data (loaded from binary files)
static FONT_DATA: &[u8] = include_bytes!("../target/font.bin");
const FONT_WIDTH: usize = 1045;
const FONT_HEIGHT: usize = 15;

static CROSS_DATA: &[u8] = include_bytes!("../target/cross.bin");
const CROSS_WIDTH: usize = 14;
const CROSS_HEIGHT: usize = 14;

const FONT_CHAR_WIDTH: usize = 11;
static FONT_ORDER: &str = "!\"_$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^+`abcdefghijklmnopqrstuvwxyz{|}~€";


// Size of inventory hotbar
pub const INVENTORY_HOTBAR_SIZE: usize = 9;

// Fills a 2D triangle on the internal framebuffer
fn fill_triangle(
    mut t0: Vector2<isize>,
    mut t1: Vector2<isize>,
    mut t2: Vector2<isize>,
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color,
) {
    // Sort triangle points by Y-coordinate for proper filling
    if t0.y > t1.y { swap(&mut t0, &mut t1); }
    if t0.y > t2.y { swap(&mut t0, &mut t2); }
    if t1.y > t2.y { swap(&mut t1, &mut t2); }

    let triangle_height = t2.y - t0.y;

    // Iterate over each row of the triangle
    'height_iter: for i in 0..triangle_height {
        let second_half = i > (t1.y - t0.y) || (t1.y == t0.y);
        let segment_height = if second_half { t2.y - t1.y } else { t1.y - t0.y };

        let alpha = i as f32 / triangle_height as f32;
        let beta = if second_half { (i as f32 - (t1.y - t0.y) as f32) / segment_height as f32 } else { i as f32 / segment_height as f32 };

        // Calculate X-coordinates of the left and right edges for the current row
        let mut a = t0.x as f32 + ((t2 - t0).x as f32 * alpha);
        let mut b = if second_half { t1.x as f32 + ((t2 - t1).x as f32 * beta) } else { t0.x as f32 + ((t1 - t0).x as f32 * beta) };

        if a > b { swap(&mut a, &mut b); } // Ensure 'a' is always left of 'b'

        let y = t0.y + i;
        // Skip rows outside the screen bounds
        if y < 0 { continue 'height_iter; }
        if y >= SCREEN_TILE_HEIGHT as isize { break 'height_iter; }

        if (b as usize) < 1 { continue; } // Prevent drawing very thin lines causing bugs

        // Draw horizontal line segment
        for j in (a as usize)..=(b as usize) {
            if j >= SCREEN_TILE_WIDTH { continue 'height_iter; } // Skip pixels outside screen
            frame_buffer[j + y as usize * SCREEN_TILE_WIDTH] = color;
        }
    }
}

// Draws a 2D line using Bresenham's algorithm
fn draw_line(
    pos1: (isize, isize),
    pos2: (isize, isize),
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color,
) {
    for point in bresenham::Bresenham::new(pos1, pos2) {
        // Draw pixel if within screen bounds
        if point.0 >= 0
            && point.0 < SCREEN_TILE_WIDTH as isize
            && point.1 >= 0
            && point.1 < SCREEN_TILE_HEIGHT as isize
        {
            frame_buffer[(point.0 + point.1 * SCREEN_TILE_WIDTH as isize) as usize] = color;
        }
    }
}

// Draws a 2D triangle (either filled or as a wireframe marker)
fn draw_2d_triangle(
    tri: &Triangle2D,
    frame_buffer: &mut [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    if tri.texture_id == 255 {
        // Special case: draw a red wireframe marker for blocks
        draw_line(
            (tri.p1.x as isize, tri.p1.y as isize),
            (tri.p2.x as isize, tri.p2.y as isize),
            frame_buffer,
            Color::from_components(0b11111, 0b0, 0b0), // Red
        );
        draw_line(
            (tri.p2.x as isize, tri.p2.y as isize),
            (tri.p3.x as isize, tri.p3.y as isize),
            frame_buffer,
            Color::from_components(0b11111, 0b0, 0b0), // Red
        );
    } else {
        // Draw a filled triangle with its textured color and lighting applied
        fill_triangle(
            Vector2::new(tri.p1.x as isize, tri.p1.y as isize),
            Vector2::new(tri.p2.x as isize, tri.p2.y as isize),
            Vector2::new(tri.p3.x as isize, tri.p3.y as isize),
            frame_buffer,
            get_quad_color_from_texture_id(tri.texture_id).apply_light(tri.light * 17),
        );
    }
}

// Creates a view matrix (camera transformation) looking from a position to a target
fn matrix_point_at(pos: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
    let new_forward = (target - pos).normalize(); // Direction from camera to target
    let new_up = (up - new_forward * up.dot(&new_forward)).normalize(); // Orthogonal up vector
    let new_right = new_up.cross(&new_forward); // Right vector, orthogonal to up and forward

    // Construct the 4x4 rotation and translation matrix
    Matrix4::new(
        new_right.x, new_up.x, new_forward.x, pos.x,
        new_right.y, new_up.y, new_forward.y, pos.y,
        new_right.z, new_up.z, new_forward.z, pos.z,
        0.0, 0.0, 0.0, 1.0,
    )
}

// Calculates the intersection point of a line segment with a 3D plane
fn vector_intersect_plane(
    plane_p: &Vector3<f32>, // A point on the plane
    plane_n: &Vector3<f32>, // The normal vector of the plane
    line_start: &Vector3<f32>, // Start point of the line
    line_end: &Vector3<f32>, // End point of the line
) -> Vector3<f32> {
    let plane_n = plane_n.normalize();
    let plane_d = -plane_n.dot(plane_p); // Plane equation constant
    let ad = line_start.dot(&plane_n); // Dot product for start point
    let bd = line_end.dot(&plane_n); // Dot product for end point
    let t = (-plane_d - ad) / (bd - ad); // Parameter 't' along the line
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    line_start + line_to_intersect // Intersection point
}

// Calculates the intersection point of a line segment with another 2D line
fn vector_intersect_line(
    line_p: &Vector2<f32>, // A point on the clipping line
    line_n: &Vector2<f32>, // The normal vector of the clipping line
    line_start: &Vector2<f32>, // Start point of the segment to intersect
    line_end: &Vector2<f32>, // End point of the segment to intersect
) -> Vector2<i16> {
    let line_n = line_n.normalize();
    let line_d = -line_n.dot(line_p);
    let ad = line_start.dot(&line_n);
    let bd = line_end.dot(&line_n);
    let t = (-line_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    let coords = line_start + line_to_intersect;
    coords.map(|x| x as i16) // Convert to i16 coordinates
}

// Clips a 2D triangle against a 2D line (returns up to two new triangles)
fn triangle_clip_against_line(
    line_p: &Vector2<f32>, // Point on the clipping line
    line_n: &Vector2<f32>, // Normal of the clipping line
    in_tri: &Triangle2D, // Input triangle
) -> (Option<Triangle2D>, Option<Triangle2D>) {
    let line_n = line_n.normalize();
    let dist = |p: Vector2<f32>| line_n.x * p.x + line_n.y * p.y - line_n.dot(line_p);

    let binding = Default::default();
    let mut inside_points: [&Vector2<f32>; 3] = [&binding; 3];
    let mut n_inside_point_count = 0;
    let binding = Default::default();
    let mut outside_points: [&Vector2<f32>; 3] = [&binding; 3];
    let mut n_outside_point_count = 0;

    // Convert triangle points to f32 for distance calculation
    let p1 = in_tri.p1.map(|x| x as f32);
    let p2 = in_tri.p2.map(|x| x as f32);
    let p3 = in_tri.p3.map(|x| x as f32);

    // Classify each point as inside or outside the clipping line
    let d0 = dist(p1);
    let d1 = dist(p2);
    let d2 = dist(p3);

    if d0 >= 0.0 { inside_points[n_inside_point_count] = &p1; n_inside_point_count += 1; } else { outside_points[n_outside_point_count] = &p1; n_outside_point_count += 1; }
    if d1 >= 0.0 { inside_points[n_inside_point_count] = &p2; n_inside_point_count += 1; } else { outside_points[n_outside_point_count] = &p2; n_outside_point_count += 1; }
    if d2 >= 0.0 { inside_points[n_inside_point_count] = &p3; n_inside_point_count += 1; } else { outside_points[n_outside_point_count] = &p3; n_outside_point_count += 1; }

    // Handle different clipping cases
    if n_inside_point_count == 0 { return (None, None); } // Triangle completely outside
    if n_inside_point_count == 3 { return (Some(*in_tri), None); } // Triangle completely inside

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        // One point inside, two outside: results in one new triangle
        let out_tri = Triangle2D {
            p1: inside_points[0].map(|x| x as i16),
            p2: vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]),
            p3: vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[1]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        // Two points inside, one outside: results in two new triangles (a quad)
        let out_tri1 = Triangle2D {
            p1: inside_points[0].map(|x| x as i16),
            p2: inside_points[1].map(|x| x as i16),
            p3: vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        let out_tri2 = Triangle2D {
            p1: inside_points[1].map(|x| x as i16),
            p2: out_tri1.p3, // Reuse intersection point
            p3: vector_intersect_line(line_p, &line_n, inside_points[1], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None) // Fallback, should not be reached with valid triangle inputs
}

// Clips a 3D triangle against a 3D plane (returns up to two new triangles)
fn triangle_clip_against_plane(
    plane_p: &Vector3<f32>, // Point on the clipping plane
    plane_n: &Vector3<f32>, // Normal of the clipping plane
    in_tri: &Triangle, // Input triangle
) -> (Option<Triangle>, Option<Triangle>) {
    let plane_n = plane_n.normalize();
    let dist = |p: Vector3<f32>| { plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - plane_n.dot(plane_p) };

    let binding = Default::default();
    let mut inside_points: [&Vector3<f32>; 3] = [&binding; 3];
    let mut n_inside_point_count = 0;
    let binding = Default::default();
    let mut outside_points: [&Vector3<f32>; 3] = [&binding; 3];
    let mut n_outside_point_count = 0;

    // Classify each point as inside or outside the clipping plane
    let d0 = dist(in_tri.p1);
    let d1 = dist(in_tri.p2);
    let d2 = dist(in_tri.p3);

    if d0 >= 0.0 { inside_points[n_inside_point_count] = &in_tri.p1; n_inside_point_count += 1; } else { outside_points[n_outside_point_count] = &in_tri.p1; n_outside_point_count += 1; }
    if d1 >= 0.0 { inside_points[n_inside_point_count] = &in_tri.p2; n_inside_point_count += 1; } else { outside_points[n_outside_point_count] = &in_tri.p2; n_outside_point_count += 1; }
    if d2 >= 0.0 { inside_points[n_inside_point_count] = &in_tri.p3; n_inside_point_count += 1; } else { outside_points[n_outside_point_count] = &in_tri.p3; n_outside_point_count += 1; }

    // Handle different clipping cases
    if n_inside_point_count == 0 { return (None, None); } // Triangle completely outside
    if n_inside_point_count == 3 { return (Some(*in_tri), None); } // Triangle completely inside

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        // One point inside, two outside: results in one new triangle
        let out_tri = Triangle {
            p1: *inside_points[0],
            p2: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]),
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[1]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        // Two points inside, one outside: results in two new triangles (a quad)
        let out_tri1 = Triangle {
            p1: *inside_points[0],
            p2: *inside_points[1],
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[0], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        let out_tri2 = Triangle {
            p1: *inside_points[1],
            p2: out_tri1.p3, // Reuse intersection point
            p3: vector_intersect_plane(plane_p, &plane_n, inside_points[1], outside_points[0]),
            texture_id: in_tri.texture_id,
            light: in_tri.light,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None) // Fallback
}

// Manages the projection matrix for 3D to 2D transformation
struct MathTools {
    projection_matrix: Perspective3<f32>,
}

impl MathTools {
    pub fn new() -> Self {
        MathTools {
            projection_matrix: Perspective3::new(ASPECT_RATIO, FOV, ZNEAR, ZFAR),
        }
    }
}

// The main renderer responsible for drawing the game world and UI
pub struct Renderer {
    pub camera: Camera, // The active camera
    math_tools: MathTools, // Projection and other math utilities
    triangles_to_render: Vec<Triangle2D>, // Dynamic list of 2D triangles to draw
    tile_frame_buffer: [Color; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT], // Off-screen buffer for rendering small screen tiles
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            camera: Camera::new(),
            math_tools: MathTools::new(),
            triangles_to_render: Vec::new(),
            tile_frame_buffer: [Color { rgb565: 0 }; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT], // Initialize with black color
        }
    }

    // Projects a 3D point into 2D screen coordinates
    fn project_point(&self, point: Vector3<f32>) -> Vector2<f32> {
        self.math_tools
            .projection_matrix
            .project_vector(&point)
            .xy()
            * -1.0 // Invert Y-axis if necessary for screen coordinates
    }

    // Fills the current tile's framebuffer with a solid color
    fn clear_screen(&mut self, color: eadk::Color) {
        self.tile_frame_buffer.fill(color);
    }

    // Calculates the view matrix from the camera's position and orientation
    fn get_mat_view(&self) -> Matrix4<f32> {
        let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0); // World up direction
        let target: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0); // Initial forward direction
        let look_dir = self.camera.get_rotation_matrix() * target.to_homogeneous(); // Apply camera rotation to get actual look direction
        let target = self.camera.get_pos() + look_dir.xyz(); // Calculate target point in world space

        let mat_camera = matrix_point_at(self.camera.get_pos(), &target, &up); // Create matrix looking at target
        mat_camera.try_inverse().unwrap() // Inverse to get view matrix (world to camera space)
    }

    // Adds a 3D triangle to the list of triangles to be processed and rendered
    fn add_3d_triangle_to_render(&mut self, tri: Triangle, mat_view: &Matrix4<f32>) {
        let mut tri = tri;
        let camera_ray = tri.p1 - self.camera.get_pos();

        // Perform back-face culling: only render triangles facing the camera
        if tri.get_normal().dot(&camera_ray) < 0.0 {
            // Transform triangle vertices from world space to camera space
            tri.p1 = (mat_view * Vector4::new(tri.p1.x, tri.p1.y, tri.p1.z, 1.0)).xyz();
            tri.p2 = (mat_view * Vector4::new(tri.p2.x, tri.p2.y, tri.p2.z, 1.0)).xyz();
            tri.p3 = (mat_view * Vector4::new(tri.p3.x, tri.p3.y, tri.p3.z, 1.0)).xyz();

            // Clip the 3D triangle against the near clipping plane
            let clipped_triangles = triangle_clip_against_plane(
                &Vector3::new(0.0, 0.0, 0.1), // Point on the near plane
                &Vector3::new(0.0, 0.0, 1.0), // Normal of the near plane (positive Z-axis)
                &tri,
            );

            // Function to project a 3D triangle to 2D and clip it against screen edges
            let mut project_and_add = |to_project: Triangle| {
                // Project 3D triangle vertices to 2D screen coordinates
                let projected_triangle = Triangle2D {
                    p1: ((self.project_point(to_project.p1) + Vector2::new(1., 1.))
                        .component_mul(&Vector2::new(HALF_SCREEN_TILE_WIDTHF, HALF_SCREEN_TILE_HEIGHTF)))
                        .map(|x| x as i16),
                    p2: ((self.project_point(to_project.p2) + Vector2::new(1., 1.))
                        .component_mul(&Vector2::new(HALF_SCREEN_TILE_WIDTHF, HALF_SCREEN_TILE_HEIGHTF)))
                        .map(|x| x as i16),
                    p3: ((self.project_point(to_project.p3) + Vector2::new(1., 1.))
                        .component_mul(&Vector2::new(HALF_SCREEN_TILE_WIDTHF, HALF_SCREEN_TILE_HEIGHTF)))
                        .map(|x| x as i16),
                    texture_id: to_project.texture_id,
                    light: to_project.light,
                };

                // Use a deque to manage triangles during 2D clipping against screen edges
                let mut clip_buffer: heapless::Deque<Triangle2D, 16> = heapless::Deque::new();
                clip_buffer.push_back(projected_triangle).unwrap(); // Add the projected triangle
                let mut new_tris = 1; // Count of triangles in buffer to process

                // Helper closure to clip triangles against a given line
                let mut clip_triangle = |line_p, line_n| {
                    while new_tris > 0 { // Process all triangles currently in buffer
                        let test = clip_buffer.pop_front().unwrap(); // Get one triangle
                        new_tris -= 1;
                        let clipped = triangle_clip_against_line(&line_p, &line_n, &test); // Clip it
                        if let Some(clipped_tri) = clipped.0 { clip_buffer.push_back(clipped_tri).unwrap(); } // Add resulting triangles back
                        if let Some(clipped_tri) = clipped.1 { clip_buffer.push_back(clipped_tri).unwrap(); }
                    }
                    new_tris = clip_buffer.len(); // Update count for next clipping stage
                };

                // Clip against all 4 screen edges (top, bottom, left, right)
                clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)); // Top edge
                clip_triangle(Vector2::new(0.0, SCREEN_HEIGHTF), Vector2::new(0.0, -1.0)); // Bottom edge
                clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)); // Left edge
                clip_triangle(Vector2::new(SCREEN_WIDTHF - 1.0, 0.0), Vector2::new(-1.0, 0.0)); // Right edge

                // Add all final clipped 2D triangles to the renderer's main list
                for tri in clip_buffer {
                    self.triangles_to_render.push(tri);
                }
            };

            // Add resulting triangles from 3D clipping to the pipeline
            if let Some(clipped) = clipped_triangles.0 { project_and_add(clipped) }
            if let Some(clipped) = clipped_triangles.1 { project_and_add(clipped) }
        }
    }

    // Draws all 2D triangles to the internal framebuffer for a specific screen tile
    fn draw_triangles(&mut self, tile_x: usize, tile_y: usize) {
        for tri in self.triangles_to_render.iter_mut() {
            let mut tri_copy = *tri;
            // Adjust triangle coordinates to be relative to the current tile's origin
            tri_copy.p1.x -= (SCREEN_TILE_WIDTH * tile_x) as i16;
            tri_copy.p1.y -= (SCREEN_TILE_HEIGHT * tile_y) as i16;
            tri_copy.p2.x -= (SCREEN_TILE_WIDTH * tile_x) as i16;
            tri_copy.p2.y -= (SCREEN_TILE_HEIGHT * tile_y) as i16;
            tri_copy.p3.x -= (SCREEN_TILE_WIDTH * tile_x) as i16;
            tri_copy.p3.y -= (SCREEN_TILE_HEIGHT * tile_y) as i16;

            draw_2d_triangle(&tri_copy, &mut self.tile_frame_buffer); // Draw the adjusted triangle
        }
    }

    // Draws a text string to the internal tile framebuffer
    fn draw_string(&mut self, text: &str, pos: &Vector2<usize>) {
        let mut text_cursor: usize = 0;
        for char_code_point in text.chars() { // Iterate over each character
            let font_index = FONT_ORDER.chars().position(|c| c == char_code_point); // Find character's position in font map

            if let Some(font_index) = font_index {
                let font_pixel_start_x = font_index * FONT_CHAR_WIDTH; // Calculate starting X-pixel for this character in the font data

                for x_offset in 0..FONT_CHAR_WIDTH {
                    for y_offset in 0..FONT_HEIGHT {
                        let font_data_index = (font_pixel_start_x + x_offset) + y_offset * FONT_WIDTH;
                        if font_data_index >= FONT_DATA.len() { continue; } // Safety check for font data bounds
                        let pixel_value = FONT_DATA[font_data_index]; // Get pixel value from font data

                        // Create color from pixel value (assuming grayscale font where value indicates brightness)
                        let rgb565 = Color::from_components(pixel_value as u16, pixel_value as u16, pixel_value as u16);

                        let pix_x = pos.x + x_offset + text_cursor; // Final X position on tile
                        let pix_y = pos.y + y_offset; // Final Y position on tile

                        // Draw pixel if within current tile's bounds
                        if pix_x < SCREEN_TILE_WIDTH && pix_y < SCREEN_TILE_HEIGHT {
                            let frame_buffer_index = pix_x + pix_y * SCREEN_TILE_WIDTH;
                            if frame_buffer_index < self.tile_frame_buffer.len() {
                                self.tile_frame_buffer[frame_buffer_index] = rgb565;
                            }
                        }
                    }
                }
            }
            text_cursor += FONT_CHAR_WIDTH; // Advance cursor for next character
        }
    }

    // Adds a 3D quad (which is made of two triangles) to the render list
    fn add_quad_to_render(
        &mut self,
        quad: &Quad,
        mat_view: &Matrix4<f32>,
        chunk_pos: Vector3<isize>,
    ) {
        let quad_triangles = quad.get_triangles(chunk_pos); // Get the two triangles forming the quad
        self.add_3d_triangle_to_render(quad_triangles.0, mat_view); // Add first triangle
        self.add_3d_triangle_to_render(quad_triangles.1, mat_view); // Add second triangle
    }

    // Draws an image by inverting background colors where the image pixel is 0
    pub fn draw_image_negate(
        &mut self,
        image: &[u8], // Image data
        image_size: Vector2<isize>, // Image dimensions
        pos: Vector2<isize>, // Position to draw image
    ) {
        for y in 0..image_size.y {
            let current_screen_y = pos.y + y;
            if current_screen_y < 0 || current_screen_y >= SCREEN_TILE_HEIGHT as isize { continue; }
            for x in 0..image_size.x {
                let current_screen_x = pos.x + x;
                if current_screen_x < 0 || current_screen_x >= SCREEN_TILE_WIDTH as isize { continue; }

                let image_pixel_index = (x + image_size.x * y) as usize;
                if image_pixel_index >= image.len() { continue; } // Safety check for image data bounds
                let pixel = image[image_pixel_index];

                if pixel == 0 { // If image pixel is 0 (e.g., black or transparent part)
                    let frame_buff_index = (current_screen_x + current_screen_y * SCREEN_TILE_WIDTH as isize) as usize;
                    if frame_buff_index < self.tile_frame_buffer.len() {
                        let components = self.tile_frame_buffer[frame_buff_index].get_components(); // Get existing pixel color
                        // Invert the color components
                        let inverted_color = Color::from_components(
                            0b11111 - components.0,    // 5-bit Red max (31)
                            0b111111 - components.1,   // 6-bit Green max (63)
                            0b11111 - components.2,    // 5-bit Blue max (31)
                        );
                        self.tile_frame_buffer[frame_buff_index] = inverted_color; // Apply inverted color
                    }
                }
            }
        }
    }

    // Main rendering update method, orchestrates drawing the world and UI
    pub fn update(&mut self, world: &World, player: &Player, fps_count: f32) {
        self.triangles_to_render.clear(); // Clear the list of triangles from the previous frame

        let mat_view = self.get_mat_view(); // Get the current view matrix from the camera

        // --- 3D World Rendering Pipeline ---
        // Iterate through world chunks, sorted by distance to the camera
        for chunk in world.get_chunks_sorted_by_distance(*self.camera.get_pos()) {
            let mut quads = chunk.get_mesh().get_reference_vec(); // Get quads (blocks) in the chunk

            // Sort quads by distance to camera (for basic depth sorting)
            quads.sort_by(|a, b| -> Ordering {
                let a_pos = a.get_pos().map(|x| x as isize) + chunk.get_pos() * CHUNK_SIZE_I;
                let b_pos = b.get_pos().map(|x| x as isize) + chunk.get_pos() * CHUNK_SIZE_I;
                let avec = Vector3::new(a_pos.x as f32 + 0.5, a_pos.y as f32 + 0.5, a_pos.z as f32 + 0.5);
                let bvec = Vector3::new(b_pos.x as f32 + 0.5, b_pos.y as f32 + 0.5, b_pos.z as f32 + 0.5);

                bvec.metric_distance(self.camera.get_pos()) // Farther objects first
                    .total_cmp(&avec.metric_distance(self.camera.get_pos()))
            });
            // Add each quad's triangles to the renderer's list
            for quad in quads {
                self.add_quad_to_render(quad, &mat_view, *chunk.get_pos());
            }
        }

        // Add the player's block marker (crosshair for placing/breaking) to the render list
        let block_marker = player.get_block_marker();
        for quad in block_marker.0.get_reference_vec() {
            self.add_quad_to_render(quad, &mat_view, block_marker.1);
        }

        // --- Screen Tiling and Final Display ---
        // Calculate global crosshair start coordinates once
        let cross_global_x_start = (SCREEN_WIDTH as isize - CROSS_WIDTH as isize) / 2;
        let cross_global_y_start = (SCREEN_HEIGHT as isize - CROSS_HEIGHT as isize) / 2;

        // Render the screen in smaller tiles to manage memory/performance
        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                // Clear the current tile's internal framebuffer with a sky-like color
                self.clear_screen(Color::from_components(0b01110, 0b110110, 0b11111)); // Light blue/cyan

                // Draw all accumulated 3D triangles relevant to this tile
                self.draw_triangles(x, y);

                // Draw debug strings (FPS, Triangle count, Player position) only on the top-left tile
                if x == 0 && y == 0 {
                    self.draw_string(format!("FPS:{fps_count:.2}").as_str(), &Vector2::new(10, 10));
                    self.draw_string(format!("Tris:{}", self.triangles_to_render.len()).as_str(), &Vector2::new(10, 30));
                    self.draw_string(
                        format!(
                            "{:.1},{:.1},{:.1}",
                            self.camera.get_pos().x,
                            self.camera.get_pos().y,
                            self.camera.get_pos().z
                        )
                        .as_str(),
                        &Vector2::new(10, 50),
                    );
                }

                // Calculate the crosshair position relative to the current tile's origin
                // and draw it on this tile. `draw_image_negate` will handle clipping.
                let target_x_for_tile = cross_global_x_start - (SCREEN_TILE_WIDTH * x) as isize;
                let target_y_for_tile = cross_global_y_start - (SCREEN_TILE_HEIGHT * y) as isize;
                self.draw_image_negate(
                    CROSS_DATA,
                    Vector2::new(CROSS_WIDTH as isize, CROSS_HEIGHT as isize),
                    Vector2::new(target_x_for_tile, target_y_for_tile),
                );

                // Push the rendered tile from the internal framebuffer to the actual display hardware
                display::push_rect(
                    Rect {
                        x: (SCREEN_TILE_WIDTH * x) as u16,
                        y: (SCREEN_TILE_HEIGHT * y) as u16,
                        width: SCREEN_TILE_WIDTH as u16,
                        height: SCREEN_TILE_HEIGHT as u16,
                    },
                    &self.tile_frame_buffer,
                );
            }
        }

        // --- UI Overlay Rendering ---
        // Render player's hotbar
        let slot_size: u16 = 20; // Size of each inventory slot square
        let padding: u16 = 2; // Padding between slots
        let border_thickness: u16 = 1; // Border for slots

        // Calculate total width and starting X position to center the hotbar
        let total_width = (slot_size + padding) * (INVENTORY_HOTBAR_SIZE as u16) - padding;
        let start_x = (SCREEN_WIDTH as u16 - total_width) / 2;
        let start_y = SCREEN_HEIGHT as u16 - slot_size - 5;

        // Iterate through inventory slots and draw each one
        for (i, &block_type) in player.inventory.slots.iter().enumerate() {
            let x = start_x + (i as u16) * (slot_size + padding);
            let y = start_y;

            // Draw background for the slot
            display::push_rect_uniform(
                Rect { x, y, width: slot_size, height: slot_size },
                UI_LIGHT_GREY
            );

            // Draw block icon if the slot is not empty
            if block_type != BlockType::Air {
                let texture_id = block_type.get_texture_id(QuadDir::Top); // Get texture ID for the block
                let block_color = get_quad_color_from_texture_id(texture_id); // Get color from texture

                // Draw a smaller rectangle inside for the block color
                display::push_rect_uniform(
                    Rect {
                        x: x + border_thickness,
                        y: y + border_thickness,
                        width: slot_size - 2 * border_thickness,
                        height: slot_size - 2 * border_thickness,
                    },
                    block_color,
                );
            }

            // Draw selection border if this slot is currently selected
            if i == player.inventory.selected_slot_index {
                display::push_rect_uniform(Rect { x, y, width: slot_size, height: slot_size }, UI_RED); // Red border for selected slot
            } else {
                display::push_rect_uniform(Rect { x, y, width: slot_size, height: slot_size }, UI_BLACK); // Black border for unselected slots
            }
        }

        // player.inventory.draw(); No use 
        // player.draw_debug_message(); For Debugging, not used in final rendering
    }
}
