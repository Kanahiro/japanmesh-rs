type MeshInfo = (&'static str, u8);
type MeshSize = [f32; 2];

const MESH_SCHEMS: [MeshInfo; 6] = [("0", 1), ("1", 8), ("2", 10), ("3", 2), ("4", 2), ("5", 2)];
const BASE_MESHSIZE: MeshSize = [1.0, 2.0 / 3.0];

fn get_meshinfo(meshname: &str) -> MeshInfo {
    match meshname {
        "1" => return MESH_SCHEMS[0],
        "2" => return MESH_SCHEMS[1],
        "3" => return MESH_SCHEMS[2],
        "4" => return MESH_SCHEMS[3],
        "5" => return MESH_SCHEMS[4],
        "6" => return MESH_SCHEMS[5],
        _ => panic!("undefinde meshname"),
    }
}

pub fn get_meshsize(meshname: &str) -> MeshSize {
    let meshinfo = get_meshinfo(meshname);
    if meshinfo.0 == "0" {
        return BASE_MESHSIZE;
    };
    let parent: &str = &meshinfo.0;
    let parent_size = get_meshsize(parent);
    return [
        parent_size[0] / meshinfo.1 as f32,
        parent_size[1] / meshinfo.1 as f32,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(get_meshsize("1"), [1.0, 2.0 / 3.0]);
        assert_eq!(get_meshsize("2"), [1.0 / 8.0, 1.0 / 12.0]);
        assert_eq!(get_meshsize("3"), [1.0 / 80.0, 1.0 / 120.0]);
        assert_eq!(get_meshsize("4"), [1.0 / 160.0, 1.0 / 240.0]);
        assert_eq!(get_meshsize("5"), [1.0 / 320.0, 1.0 / 480.0]);
        assert_eq!(get_meshsize("6"), [1.0 / 640.0, 1.0 / 960.0]);
    }

    #[test]
    #[should_panic]
    fn panic() {
        get_meshsize("undefined");
    }
}
