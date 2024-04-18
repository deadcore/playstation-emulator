struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    input: VertexInput
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(
        ((input.position.x / 1024.0) * 2.0) - 1.0,
        1.0 - (input.position.y / 512.0) * 2.0,
        0.0,
        1.0
    );
    output.color = input.color;
    return output;
}

struct FragmentInput {
    @location(0) color: vec3<f32>,
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 0.5);
}