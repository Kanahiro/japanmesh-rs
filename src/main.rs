use std::env;
use std::fs;
use std::io::BufWriter;
use std::io::Write;

mod calc;
mod scheme;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (meshname, extent) = parse_args(args);

    make_meshes(&meshname, &extent);
}

fn parse_args(args: Vec<String>) -> (String, [f32; 4]) {
    let meshname = &args[1];
    let _extent = &args[2]
        .split(",")
        .map(|v| v.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    let extent: [f32; 4] = [_extent[0], _extent[1], _extent[2], _extent[3]];
    return (meshname.clone(), extent);
}

fn make_meshes(meshname: &str, extent: &[f32; 4]) {
    let meshsize = scheme::get_meshsize(meshname);
    let grid_count = calc::get_grid_count(&meshsize);
    let offsets = calc::get_offsets(&meshsize, &extent);

    let mut f = BufWriter::new(fs::File::create("output.geojsonl").unwrap());
    for x in (0 + offsets[0])..(grid_count[0] - offsets[2]) {
        for y in (0 + offsets[1])..(grid_count[1] - offsets[3]) {
            let left = 122.0 + x as f32 * meshsize[0];
            let right = 122.0 + (x + 1) as f32 * meshsize[0];
            let bottom = 20.0 + y as f32 * meshsize[1];
            let top = 20.0 + (y + 1) as f32 * meshsize[1];
            let polygon = [
                [left, bottom],
                [right, bottom],
                [right, top],
                [left, top],
                [left, bottom],
            ];
            let s = format!("{{\"type\":\"Feature\", \"geometry\":{{\"type\":\"Polygon\",\"coordinates\":[{:?}]}}}}\n", polygon);
            let b = &s.as_bytes();
            f.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(
            parse_args(vec![
                String::from("dummy"),
                String::from("1"),
                String::from("122.0,20.0,123.0,24.0"),
            ]),
            (String::from("1"), [122.0, 20.0, 123.0, 24.0])
        )
    }

    #[test]
    #[should_panic]
    fn panic() {
        parse_args(vec![
            // String::from("dummy"),
            String::from("1"),
            String::from("122.0,20.0,123.0,24.0"),
        ]);
        parse_args(vec![
            String::from("dummy"),
            String::from("1"),
            String::from("122.0,20.0,123.0"),
        ]);
        parse_args(vec![
            String::from("dummy"),
            String::from("1"),
            String::from("122.0,20.0,123.0,a24.0"),
        ]);
    }
}
