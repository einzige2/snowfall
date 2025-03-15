use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use noise::{Fbm, NoiseFn, Perlin};
use std::cmp::Ordering;

pub enum TerrainResolution {
    LOW = 256,
    MEDIUM = 512,
    HIGH = 1024,
}

#[derive(Event)]
pub enum TerrainEvent {
    Generate,
    SetResolution(TerrainResolution),
    SetSize(f64),
    SetFrequency(f64),
    SetAmplitude(f64),
}

const NOISE_SEED: u32 = 42;
const NOISE_FREQUENCY: f64 = 4.0;
const NOISE_AMPLITUDE: f64 = 24.0;

fn get_terrain_height(
    noise: &Fbm<Perlin>,
    x: f64,
    z: f64,
    size: f64,
    frequency: f64,
    amplitude: f64,
) -> f64 {
    let fx: f64 = x / size * frequency;
    let fy: f64 = z / size * frequency;

    noise.get([fx, fy]) * amplitude
}

pub fn generate_terrain(size: f32, resolution: TerrainResolution) -> Mesh {
    let res: usize = resolution as usize;
    let step: f32 = size as f32 / res as f32;

    // VERTICES

    // set up noise
    let mut noise: Fbm<Perlin> = Fbm::new(NOISE_SEED);
    noise.octaves = 3;

    let mut potential_start_points: Vec<usize> = Vec::new();
    let start_point_threshold = 0.999 * (NOISE_AMPLITUDE / 2.0) as f32;

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    for z in 0..res {
        for x in 0..res {
            let vx = x as f32 * step;
            let vz = z as f32 * step;
            let vy = get_terrain_height(
                &noise,
                vx as f64,
                vz as f64,
                size as f64,
                NOISE_FREQUENCY,
                NOISE_AMPLITUDE,
            ) as f32;
            vertices.push([vx, vy, vz]);

            if vy >= start_point_threshold
                && (potential_start_points.len() == 0
                    || !potential_start_points.iter().any(|psp| {
                        Vec3::from_array(*vertices.get(*psp).unwrap())
                            .distance(Vec3::from_array([vx, vy, vz]))
                            < 4.0
                    }))
            {
                potential_start_points.push(z * res + x);
            }
        }
    }

    // sort the potential start points
    // potential_start_points.sort_by(|a, b| start_point_sort(&vertices[*a], &vertices[*b]));

    // println!(
    //     "Got {} potential start points!",
    //     potential_start_points.len()
    // );

    // define_ridge_path(&mut vertices, &potential_start_points);

    // UVS

    let uvs: Vec<[f32; 2]> = vertices
        .iter()
        .map(|v| [v[0] / size, v[2] / size])
        .collect();

    // TRIANGLES

    let mut triangles: Vec<u32> = Vec::new();

    let res_32 = res as u32;
    for z in 0..(res_32 - 1) {
        for x in 0..(res_32 - 1) {
            // each iteration of this loop will be one quad in the overall terrain

            let root: u32 = z * res_32 + x;

            // first triangle
            // 1    2
            // ------
            // |   /
            // |  /
            // | /
            // |/
            // 0

            triangles.push(root);
            triangles.push(root + res_32);
            triangles.push(root + res_32 + 1);

            // second triangle
            //      1
            //     /|
            //    / |
            //   /  |
            //  /   |
            // ------
            // 0    2

            triangles.push(root);
            triangles.push(root + res_32 + 1);
            triangles.push(root + 1);
        }
    }

    let mut terrain = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(triangles));

    // auto calculate the normals
    terrain.compute_smooth_normals();

    terrain
}

fn define_ridge_path(vs: &mut Vec<[f32; 3]>, hp: &Vec<usize>) {
    let mut prev_point_idx = 0;
    let mut next_point_idx = 1;

    let mut path_vertices: Vec<usize> = Vec::new();
    while next_point_idx < (*hp).len() {
        let prev_point = vs[(*hp)[prev_point_idx]];
        let next_point = vs[(*hp)[next_point_idx]];

        let potential_path_points = vs
            .iter()
            .enumerate()
            .filter(|(_, c)| potential_path_filter(&prev_point, &next_point, c))
            .map(|(i, v)| {
                (
                    i,
                    Vec3 {
                        x: v[0],
                        y: v[1],
                        z: v[2],
                    },
                )
            });

        let next_point_vec = Vec3::from_array(next_point);
        let prev_point_vec = Vec3::from_array(prev_point);

        let path_points = potential_path_points
            .filter(|(_, v)| path_filter(v, &prev_point_vec, &next_point_vec))
            .map(|(i, _)| i);
        println!(
            "{} path points between {:?} and {:?}",
            path_points.clone().count(),
            &prev_point_vec,
            &next_point_vec
        );
        path_vertices.extend(path_points);

        prev_point_idx += 1;
        next_point_idx += 1;
    }

    path_vertices.sort_by(|a, b| b.cmp(a));

    println!("Got {} ridge path points.", path_vertices.len());

    let mut next_path_point = path_vertices.pop().unwrap();
    for n in 0..vs.len() {
        if n != next_path_point {
            vs[n][1] = 0.0;
            continue;
        }

        let maybe_next_path_point = path_vertices.pop();
        if maybe_next_path_point.is_none() {
            break;
        }
        next_path_point = maybe_next_path_point.unwrap();
    }
}

// If the distance from the mesh origin to the new point is between the origin distance
// from the previous point and the next path point, could be a path point
fn potential_path_filter(prev: &[f32; 3], next: &[f32; 3], curr: &[f32; 3]) -> bool {
    let pd = (prev[0] * prev[0] + prev[2] + prev[2]).sqrt();
    let nd = (next[0] * next[0] + next[2] + next[2]).sqrt();
    let cd = (curr[0] * curr[0] + curr[2] + curr[2]).sqrt();

    return cd <= nd && cd >= pd;
}

// Now determine how close to the line between previous and next point marker
// the current vertex falls
fn path_filter(v: &Vec3, p: &Vec3, n: &Vec3) -> bool {
    let cv = (v - p).normalize();
    let nv = (n - p).normalize();
    cv.dot(nv) > 0.66
}

fn start_point_sort(l: &[f32; 3], r: &[f32; 3]) -> Ordering {
    let ld = (l[0] * l[0] + l[2] * l[2]).sqrt();
    let rd = (r[0] * r[0] + r[2] * r[2]).sqrt();

    if ld < rd {
        Ordering::Less
    } else if ld == rd {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}
