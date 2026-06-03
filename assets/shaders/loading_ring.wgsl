#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> progress: f32; // 1.0 = full ring, 0.0 = empty
@group(1) @binding(1) var<uniform> color: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 2.0 - 1.0;
    let dist = length(uv);

    let outer = 0.9;
    let inner = 0.8;

    // Smooth edges instead of hard cutoff
    let outer_alpha = smoothstep(outer, outer - 0.05, dist);
    let inner_alpha = smoothstep(inner, inner + 0.05, dist);
    let ring_alpha = outer_alpha * inner_alpha;

    let angle = (atan2(uv.x, uv.y) + 3.14159) / (2.0 * 3.14159);
    if angle > progress {
        return vec4<f32>(0.0);
    }

    return vec4<f32>(color.rgb, color.a * ring_alpha);
}