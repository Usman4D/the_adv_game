extern crate noise;

use bevy::prelude::*;
use noise::*;
use std::collections::HashMap;
pub struct DualContouring {
    origin: IVec3,
    size: i32,
}
impl DualContouring {
    pub fn new(origin: IVec3, size: i32) -> Self {
        Self { origin, size }
    }
    pub fn generate(&self) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
        let xo = self.origin.x;
        let yo = self.origin.y;
        let zo = self.origin.z;

        let mut verts_vec: Vec<[f32; 3]> = Vec::new();
        let mut normals_vec = Vec::new();
        let mut indicies_vec: HashMap<IVec3, u32> = HashMap::new();

        let n = construct_noise();
        let s = self.size;
        for z_v in 0..s {
            for y_v in 0..s {
                for x_v in 0..s {
                    let vertex_normal = find_best_vertex(
                        &n,
                        (x_v + xo) as f32,
                        (y_v + yo) as f32,
                        (z_v + zo) as f32,
                    );
                    if vertex_normal.is_none() {
                        continue;
                    }
                    verts_vec.push(
                        (vertex_normal.unwrap().0 + Vec3::new(x_v as f32, y_v as f32, z_v as f32))
                            .to_array(),
                    );
                    normals_vec.push(vertex_normal.unwrap().1);
                    indicies_vec.insert(
                        IVec3::new(x_v + xo, y_v + yo, z_v + zo),
                        verts_vec.len() as u32 - 1,
                    );
                }
            }
        }

        let mut faces: Vec<u32> = Vec::new();
        for z_v in zo..s + zo {
            for y_v in yo..s + yo {
                for x_v in xo..s + xo {
                    if x_v > xo && y_v > yo {
                        let solid1 = df(&n, x_v as f32, y_v as f32, z_v as f32 + 0f32) > 0.0f32;
                        let solid2 = df(&n, x_v as f32, y_v as f32, z_v as f32 + 1f32) > 0.0f32;

                        if solid1 != solid2 {
                            if !solid2 {
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v - 1, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v - 1, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v - 0, z_v))
                                        .unwrap(),
                                );

                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v - 0, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v - 0, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v - 1, z_v))
                                        .unwrap(),
                                );
                            } else {
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v - 1, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v - 0, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v - 0, z_v))
                                        .unwrap(),
                                );

                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v - 0, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v - 1, z_v))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v - 1, z_v))
                                        .unwrap(),
                                );
                            }
                        }
                    }
                    if x_v > xo && z_v > zo {
                        let solid1 = df(&n, x_v as f32, y_v as f32 + 0f32, z_v as f32) > 0f32;
                        let solid2 = df(&n, x_v as f32, y_v as f32 + 1f32, z_v as f32) > 0f32;

                        if solid1 != solid2 {
                            if !solid1 {
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v, z_v - 0))
                                        .unwrap(),
                                );

                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v, z_v - 1))
                                        .unwrap(),
                                );
                            } else {
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v, z_v - 0))
                                        .unwrap(),
                                );

                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 0, y_v, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v - 1, y_v, z_v - 1))
                                        .unwrap(),
                                );
                            }
                        }
                    }
                    if y_v > yo && z_v > zo {
                        let solid1 = df(&n, x_v as f32 + 0f32, y_v as f32, z_v as f32) > 0f32;
                        let solid2 = df(&n, x_v as f32 + 1f32, y_v as f32, z_v as f32) > 0f32;

                        if solid1 != solid2 {
                            if !solid2 {
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 1, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 0, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 0, z_v - 0))
                                        .unwrap(),
                                );

                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 0, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 1, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 1, z_v - 1))
                                        .unwrap(),
                                );
                            } else {
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 1, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 1, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 0, z_v - 0))
                                        .unwrap(),
                                );

                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 0, z_v - 0))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 0, z_v - 1))
                                        .unwrap(),
                                );
                                faces.push(
                                    *indicies_vec
                                        .get(&IVec3::new(x_v, y_v - 1, z_v - 1))
                                        .unwrap(),
                                );
                            }
                        }
                    }
                }
            }
        }
        return (verts_vec, normals_vec, faces);
    }
}
fn find_best_vertex(
    noise: &Box<dyn NoiseFn<f64, 2>>,
    x: f32,
    y: f32,
    z: f32,
) -> Option<(Vec3, [f32; 3])> {
    let mut points: [[[f32; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

    for z_c in 0..2 {
        for y_c in 0..2 {
            for x_c in 0..2 {
                points[x_c][y_c][z_c] = df(&noise, x + x_c as f32, y + y_c as f32, z + z_c as f32);
            }
        }
    }
    let mut change = false;
    let mut zero_crossings = Vec::new();

    for dy in 0..2 {
        for dx in 0..2 {
            if (points[dx][dy][0] > 0.0) != (points[dx][dy][1] > 0.0) {
                change = true;
                zero_crossings.push(Vec3::new(
                    dx as f32,
                    dy as f32,
                    adapt(points[dx][dy][0], points[dx][dy][1]),
                ));
            }
        }
    }
    for dz in 0..2 {
        for dx in 0..2 {
            if (points[dx][0][dz] > 0.0) != (points[dx][1][dz] > 0.0) {
                change = true;
                zero_crossings.push(Vec3::new(
                    dx as f32,
                    adapt(points[dx][0][dz], points[dx][1][dz]),
                    dz as f32,
                ));
            }
        }
    }
    for dy in 0..2 {
        for dz in 0..2 {
            if (points[0][dy][dz] > 0.0) != (points[1][dy][dz] > 0.0) {
                change = true;
                zero_crossings.push(Vec3::new(
                    adapt(points[0][dy][dz], points[1][dy][dz]),
                    dy as f32,
                    dz as f32,
                ));
            }
        }
    }
    if !change {
        return None;
    }
    let mut avg_point = Vec3::new(0.0, 0.0, 0.0);
    for p in &zero_crossings {
        avg_point += *p;
    }
    avg_point /= zero_crossings.len() as f32;

    let mut normals = Vec::<Vec3>::new();
    for p in &zero_crossings {
        normals.push(normalfunction(&noise, p.x + x, p.y + y, p.z + z, 0.001f32));
    }

    let mut avg_normal = Vec3::new(0.0, 0.0, 0.0);
    for n in normals.iter() {
        avg_normal += *n;
    }
    avg_normal /= normals.len() as f32;
    avg_normal = avg_normal.normalize();

    let normal = normalfunction(
        &noise,
        avg_point.x + x,
        avg_point.y + y,
        avg_point.z + z,
        0.001f32,
    );

    return Some((avg_point, avg_normal.to_array()));
}
fn densityfunction(x: f32, y: f32, z: f32) -> f32 {
    //let perlin = Perlin::new() as NoiseFn>::<[f32;3]>;
    //let radius = 3f32;
    //return radius - f32::powf((x*x+y*y+z*z),0.5);
    //return -y + perlin.get([x, y, z]) ;
    //let perlin = BasicMulti::<Perlin>::new(0)
    //.set_frequency(1.0)
    //.set_persistence(0.5)
    //.set_lacunarity(2.0)
    //.set_octaves(3)
    //.get([x as f64, y as f64, z as f64]);

    // let val = Simplex::new(0).get([x as f64 * 0.010f64, z as f64 * 0.010f64]) * 10.0;

    // NEW TECHNOLOGY FOR TERRAIN
    // let vein = Vein::new(8, 4.0);

    // let final_wood = Turbulence::<_, Perlin>::new(&vein)
    //     .set_seed(2)
    //     .set_frequency(0.1)
    //     .set_power(1.0 / 1.0)
    //     .set_roughness(3);

    // let vein_mult = Multiply::new(&final_wood, Constant::new(0.8));
    //
    // Basic Multifractal noise to use for the wood grain.
    // let terrain_noise = Fbm::<Perlin>::new(0)
    //     .set_frequency(0.3)
    //     // .set_persistence(0.5)
    //     // .set_lacunarity(2.20703125)
    //     .set_octaves(1);
    //
    // let terrain_vein_combined = Add::new(&terrain_noise, &vein);
    //
    // let new_val = terrain_vein_combined.get([x as f64 * 0.10f64, z as f64 * 0.10f64]) * 5.0;

    //return -y + perlin as f32;
    //return -y + val.clamp(1.0, 10.0) as f32;
    //
    // Generate pattern for vein
    let vein = Vein::new(16, 2.0);
    // Finally, perturb the wood texture again to produce the final texture.
    let final_wood = Turbulence::<_, Perlin>::new(vein)
        .set_seed(2)
        .set_frequency(0.05)
        .set_power(4.0 / 1.0)
        .set_roughness(1);

    let vein_mult = Multiply::new(&final_wood, Constant::new(0.8));

    // Basic Multifractal noise to use for the wood grain.
    let terrain_noise = Fbm::<Perlin>::new(0)
        .set_frequency(0.05)
        // .set_persistence(0.5)
        // .set_lacunarity(2.20703125)
        .set_octaves(1);

    let terrain_vein_combined = Add::new(&terrain_noise, &vein_mult);

    let new_val = terrain_vein_combined.get([x as f64, z as f64]) * 5.0;

    return -y + 0.0;
}
fn df(noise: &Box<dyn NoiseFn<f64, 2>>, x: f32, y: f32, z: f32) -> f32 {
    let val = noise.get([x as f64, z as f64]) * 10.0;

    return -y + val as f32;
}
fn construct_noise() -> Box<dyn NoiseFn<f64, 2>> {
    // Generate pattern for vein
    let vein = Vein::new(128, 2.0);
    // Finally, perturb the wood texture again to produce the final texture.
    let final_wood = Turbulence::<_, Perlin>::new(vein)
        .set_seed(2)
        .set_frequency(0.0125)
        .set_power(32.0 / 1.0)
        .set_roughness(2);

    let vein_mult = Multiply::new(final_wood, Constant::new(1.0));

    // Basic Multifractal noise to use for the wood grain.
    let terrain_noise = Fbm::<Perlin>::new(0)
        .set_frequency(0.0125)
        // .set_persistence(0.5)
        // .set_lacunarity(2.20703125)
        .set_octaves(2);

    let terrain_vein_combined = Add::new(terrain_noise, vein_mult);

    Box::new(terrain_vein_combined)
}
fn normalfunction(noise: &Box<dyn NoiseFn<f64, 2>>, x: f32, y: f32, z: f32, d: f32) -> Vec3 {
    let dx = (df(&noise, x + d, y, z) - df(&noise, x - d, y, z)) / -2.0 * d;
    let dy = (df(&noise, x, y + d, z) - df(&noise, x, y - d, z)) / -2.0 * d;
    let dz = (df(&noise, x, y, z + d) - df(&noise, x, y, z - d)) / -2.0 * d;

    return Vec3::new(dx, dy, dz).normalize();
}
fn adapt(v0: f32, v1: f32) -> f32 {
    (-v0) / (v1 - v0)
}
