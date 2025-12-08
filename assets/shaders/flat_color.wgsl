struct FlatColorMat {
    color: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: FlatColorMat;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return material.color;
}