struct BlackShaderParams
{
    invert: f32,
    time: f32,
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
    let color = textureSample(t, s, in.uv );
    let colorInv = textureSample(t, s, in.uv + vec2(
        sin((in.uv.x * 6.28 + params.time) % 6.28) * 0.1,
        0,
    ));

    let contrast = length(color - textureSample(t, s, in.uv  + vec2(0.01, 0)));

    let inverted = vec4<f32>(
        1.0 - colorInv.r + sin(params.time * 0.2) * 0.5 - contrast * 4,
        1.0 - colorInv.g + sin(params.time * 0.5) * 0.5 - contrast * 4,
        1.0 - colorInv.b + sin(params.time * 2) * 0.5 - contrast * 4,
        colorInv.a
    );

    return mix(color, inverted * 0.6, params.invert);
}