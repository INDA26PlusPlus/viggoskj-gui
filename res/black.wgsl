struct BlackShaderParams
{
    invert: f32,
}

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};


@group(3) @binding(0)
var<uniform> params: BlackShaderParams;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t, s, in.uv);

    let inverted = vec4<f32>(
        1.0 - color.r,
        1.0 - color.g,
        1.0 - color.b,
        color.a
    );

    return mix(color, inverted * 0.6, params.invert);
}