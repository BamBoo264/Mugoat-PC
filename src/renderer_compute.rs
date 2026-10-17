// DX12/FXC repair for three pinned 16x16 compute shaders. Depth retains its
// original reduction and gets an explicit single-writer predicate. Luminance
// retains the same tree reduction, normalization and bindings, but expresses
// lane indices and conditions as integers/bools rather than float registers.
pub fn patch(hash: &str, source: &str) -> Result<String, String> {
    let depth = matches!(hash, "23a7a3c1c0d38f46");
    let luminance = matches!(hash, "7a1e45a48ffaaf08" | "9b8e0c6c85e065da");
    if !depth && !luminance { return Ok(source.to_owned()); }
    if !source.contains("@workgroup_size(16u, 16u, 1u)") || !source.contains("@builtin(local_invocation_id) vThreadIDInGroup") {
        return Err("unexpected compute lane layout".into());
    }
    let mut text=source.to_owned();
    let guard=if depth {"if ((bitcast<u32>(v_7) != 0u)) {"} else {"if ((bitcast<u32>(r0.x) != 0u)) {"};
    if text.matches(guard).count()!=1 {return Err("unexpected final writer guard".into());}
    text=text.replace(guard,"if (any(vThreadIDInGroup != vec3<u32>(0u))) {");
    if luminance {
        // Translated float registers also carry integer lane IDs and NaN masks.
        // FXC compiled those but GPU readback returned zero. Express the same
        // 256-lane tree with integer indices and boolean predicates throughout.
        let start=text.find("@compute @workgroup_size").ok_or("missing compute entry")?;
        text.truncate(start);
        text.push_str(r#"@compute @workgroup_size(16u, 16u, 1u)
fn main(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_id) local: vec3<u32>) {
    let lane = local.y * 16u + local.x;
    let pos = group.xy * 16u + local.xy;
    let size = vec2<u32>(max(cb12_12.tint_symbol[0u].zw, vec2<f32>(0.0)));
    var value = 0.0;
    if (all(pos < size)) {
"#);
        if hash == "9b8e0c6c85e065da" {
            text.push_str(r#"        value = dot(textureLoad(t3, vec2<i32>(pos), 0).xyz, vec3<f32>(0.21250000596046447754, 0.71539998054504394531, 0.0720999985933303833));
        if ((bitcast<u32>(value) & 0x7fffffffu) >= 0x7f800000u) { value = 0.0; }
"#);
        } else {
            text.push_str("        value = textureLoad(t2, vec2<i32>(pos), 0).x;\n");
        }
        text.push_str(r#"    }
    g0[lane] = bitcast<u32>(value);
    workgroupBarrier();
    for (var stride = 128u; stride > 0u; stride >>= 1u) {
        if (lane < stride) {
            g0[lane] = bitcast<u32>(bitcast<f32>(g0[lane]) + bitcast<f32>(g0[lane + stride]));
        }
        workgroupBarrier();
    }
    if (lane == 0u) {
        var result = bitcast<f32>(g0[0u]);
"#);
        if hash == "9b8e0c6c85e065da" {
            text.push_str("        result *= 1.0 / (cb12_12.tint_symbol[0u].w * cb12_12.tint_symbol[0u].z);\n");
        }
        text.push_str("        textureStore(u1, vec2<i32>(group.xy), vec4<f32>(result));\n    }\n}\n");
    }
    Ok(text)
}
