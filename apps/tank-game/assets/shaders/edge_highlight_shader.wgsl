#import bevy_pbr::forward_io::VertexOutput

const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 0.5);
const COLOR_GREENER: vec4<f32> = vec4<f32>(0.0, 1.0, 0.0, 1.0);

//@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
//@group(2) @binding(2) var material_color_sampler: sampler;
//
//@fragment
//fn fragment(
//    mesh: VertexOutput,
//) -> @location(0) vec4<f32> {
//    return material_color * textureSample(material_color_texture, material_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
//}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let distance = length(mesh.uv * 2.0 - vec2<f32>(1.0, 1.0));
    let edge_width = 0.01;
    let alpha = 1.0 - smoothstep(0.49 - edge_width, 0.49, distance) + smoothstep(0.51, 0.51 + edge_width, distance);
    return vec4<f32>(COLOR_GREENER.rgb, material_color.a * alpha) * COLOR_MULTIPLIER;
}