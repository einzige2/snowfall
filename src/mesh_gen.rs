use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

pub enum TerrainResolution {
    LOW = 256,
    MEDIUM = 512,
    HIGH = 1024,
}

const SEED: u32 = 42;

fn get_terrain_height(
    perlin: Perlin,
    x: f64,
    z: f64,
    size: f64,
    frequency: f64,
    amplitude: f64,
) -> f64 {
    let fx: f64 = x / size * frequency;
    let fy: f64 = z / size * frequency;

    perlin.get([fx, fy]) * amplitude
}

pub fn generate_terrain(size: f32, resolution: TerrainResolution) -> Mesh {
    let res: u32 = resolution as u32;
    let step: f32 = size as f32 / res as f32;

    // VERTICES

    let perlin: Perlin = Perlin::new(SEED);
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    for z in 0..res {
        for x in 0..res {
            let vx = x as f32 * step;
            let vz = z as f32 * step;
            vertices.push([
                vx,
                get_terrain_height(perlin, vx as f64, vz as f64, size as f64, 4.0, 5.0) as f32,
                vz,
            ]);
        }
    }

    // UVS

    let uvs: Vec<[f32; 2]> = vertices
        .iter()
        .map(|v| [v[0] / size, v[2] / size])
        .collect();

    // TRIANGLES

    let mut triangles: Vec<u32> = Vec::new();

    for z in 0..(res - 1) {
        for x in 0..(res - 1) {
            // each iteration of this loop will be one quad in the overall terrain

            let root = z * res + x;

            // first triangle
            // 1    2
            // ------
            // |   /
            // |  /
            // | /
            // |/
            // 0

            triangles.push(root);
            triangles.push(root + res);
            triangles.push(root + res + 1);

            // second triangle
            //      1
            //     /|
            //    / |
            //   /  |
            //  /   |
            // ------
            // 0    2

            triangles.push(root);
            triangles.push(root + res + 1);
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
