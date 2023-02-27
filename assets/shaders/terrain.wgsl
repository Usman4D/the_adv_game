#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct VertexOutput{
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
};

struct TerrainMat {
  color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> uniform_data: TerrainMat;

@group(1) @binding(1)
var color_texture: texture_2d<f32>;
@group(1) @binding(2)
var color_tex_sampler: sampler;

@group(1) @binding(3)
var normalmap_texture: texture_2d<f32>;
@group(1) @binding(4)
var normalmap_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
  var output_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  output_color = output_color * textureSample(color_texture, color_tex_sampler, in.uv);
  output_color = output_color * uniform_data.color;

  var N = normalize(in.world_normal);
  var V = normalize(view.world_position.xyz -  in.world_position.xyz);
  var light = vec3(1.0, 0.7, 1.0);

  let NdotV = max(dot(N, light), 0.0001);
  /* return vec4(vec3(NdotV),1.0); */
  return output_color * vec4(vec3(NdotV), 1.0);
}
