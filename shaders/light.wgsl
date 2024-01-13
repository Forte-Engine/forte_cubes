struct Light {
    position: vec3<f32>,
    range: f32,
    color: vec3<f32>,
    exponent: f32,
    direction: vec3<f32>, 
    cutoff: f32
}
@group(2) @binding(0)
var<storage, read_write> lights: array<Light>;
@group(2) @binding(1)
var<uniform> num_lights: u32;
@group(2) @binding(2)
var<uniform> ambient_light: vec3<f32>;

fn calculate_lights(view_pos: vec3<f32>, position: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    var light_color = vec3<f32>(0.0, 0.0, 0.0);
    for (var i = 0u; i < num_lights; i += 1u) {
        // calculate basic light values
        let light = lights[i];
        let delta = light.position - position;
        let distance = length(delta);
        if (distance > light.range) { continue; }

        // do cutoff checks if necessary
        if (light.cutoff < 100.0 && dot(light.direction, normalize(position - light.position)) < light.cutoff) { continue; }

        // get some directions to make our lives easy
        let light_dir = normalize(delta);
        let view_dir = normalize(view_pos - position);
        let half_dir = normalize(view_dir + light_dir);
        let reflect_dir = reflect(-light_dir, normal);

        // calculate the lights "strength"
        var light_strength = 1.0;
        if (light.exponent != 0.0) { light_strength = 1.0 / (pow(distance, light.exponent) + 1.0); }

        // calculate the diffuse component
        let diffuse_strength = max(dot(normal, light_dir), 0.0);
        let diffuse_color = light.color * diffuse_strength;

        // calcualte the specular component
        let specular_strength = pow(max(dot(normal, half_dir), 0.0), 32.0);
        let specular_color = specular_strength * light.color;

        // calculate and append final light color
        light_color += (diffuse_color + specular_color) * light_strength;
    }
    return light_color + ambient_light;
}