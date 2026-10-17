//! Narrow DX12 fallback for the four clip planes in this port's shader pack.
//! The pack already exports an identical vec4 copy at location 15. Keep it,
//! and test its interpolated signs in the fragment stage, including depth-only
//! draws. This is fragment clipping, not exact fixed-function/MSAA clipping.
use regex::Regex;

pub fn vertex(source: &str) -> Result<String, String> {
    let fail = || "unsupported clip shader shape; refusing to drop clipping".to_string();
    let main = Regex::new(r"@vertex\s*fn\s+main\([\s\S]*?\)\s*->\s*(\w+)\s*\{").unwrap();
    let ty = main.captures(source).ok_or_else(fail)?[1].to_string();
    let st = Regex::new(&format!(r"struct\s+{}\s*\{{([^}}]*)\}}",regex::escape(&ty))).unwrap();
    let body = st.captures(source).ok_or_else(fail)?;
    let clip = Regex::new(r"@builtin\(clip_distances\)\s*(\w+)\s*:\s*array<f32,\s*4u?>\s*,").unwrap();
    let field = clip.captures(&body[1]).ok_or_else(fail)?[1].to_string();
    let duplicate = Regex::new(r"@location\(15u?\)\s*(\w+)\s*:\s*vec4<f32>\s*,").unwrap();
    let alias = duplicate.captures(&body[1]).ok_or_else(fail)?[1].to_string();
    let members = Regex::new(r"\b(\w+)\s*:").unwrap();
    let fields: Vec<_> = members.captures_iter(&body[1]).map(|c| c[1].to_string()).collect();
    let ci = fields.iter().position(|n|n==&field).ok_or_else(fail)?;
    let ai = fields.iter().position(|n|n==&alias).ok_or_else(fail)?;
    let ret = Regex::new(&format!(r"return\s+{}\(([^;]*)\);",regex::escape(&ty))).unwrap();
    let r = ret.captures(source).ok_or_else(fail)?;
    let mut args: Vec<_> = r[1].split(',').map(str::trim).map(str::to_string).collect();
    if args.len()!=fields.len() || args[ci]!=field || args[ai]!="v" {return Err(fail());}
    let compact: String=source.chars().filter(|c|!c.is_whitespace()).collect();
    let copy=format!("v=vec4<f32>({field}[0u],{field}[1u],{field}[2u],{field}[3u]);");
    if !compact.contains(&copy) {return Err(fail());}
    args.remove(ci);
    let mut text=ret.replace(source,format!("return {ty}({});",args.join(", "))).into_owned();
    text=clip.replace(&text,"").into_owned();
    text=text.replace("enable clip_distances;","");
    if text.contains("@builtin(clip_distances)") {return Err(fail());}
    Ok(text)
}

pub fn fragment(source: &str) -> Result<String,String> {
    let sig=Regex::new(r"(@fragment\s*fn\s+main\s*\()([\s\S]*?)(\)\s*->)").unwrap();
    let caps=sig.captures(source).ok_or("unsupported fragment signature for clip fallback")?;
    let existing=Regex::new(r"@location\(15u?\)\s*(\w+)\s*:\s*vec4<f32>").unwrap();
    let (name,args)=if let Some(c)=existing.captures(&caps[2]) {
        (c[1].to_string(),caps[2].to_string())
    } else {
        if caps[2].contains("@location(15") {return Err("clip varying location 15 has incompatible type".into());}
        let args=caps[2].trim();
        ("mugoat_clip_planes".into(),format!("{args}{}@location(15) mugoat_clip_planes: vec4<f32>",if args.is_empty(){""}else{", "}))
    };
    let text=sig.replace(source,|c:&regex::Captures|format!("{}{args}{}",&c[1],&c[3])).into_owned();
    let start=text.find("@fragment").ok_or("missing fragment entry")?;
    let brace=start+text[start..].find('{').ok_or("missing fragment body")?;
    Ok(format!("{}\n  if (any({name} < vec4<f32>(0.0))) {{ discard; }}\n{}",&text[..brace+1],&text[brace+1..]))
}

pub fn depth_fragment(location:u32)->String {
    format!("@fragment fn main(@location({location}) planes: vec4<f32>) {{ if (any(planes < vec4<f32>(0.0))) {{ discard; }} }}")
}
