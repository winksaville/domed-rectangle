use csgrs::csg::CSG;

fn main() {
    let width_dome: f64 = 10.0;
    let length_dome: f64 = 20.0;
    let height_base: f64 = 0.0;
    let segments: i32 = 32;

    // Create dome
    let dome_radius = width_dome / 2.0;
    let dome: CSG<()> = CSG::circle_with_flat(dome_radius, segments as usize, height_base, None);
    let dome = dome.extrude(length_dome);

    if !dome.is_manifold() {
        println!("The domed_cube is not a manifold");
    }

    // Write the result as an ASCII STL:
    let name = format!(
        "dome.width-{:0.3}_length-{:0.3}_height-base-{:0.3}_segments-{}",
        width_dome, length_dome, height_base, segments
    );

    let stl = dome.to_stl_ascii(&name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
