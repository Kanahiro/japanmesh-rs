const MESH_EXTENT: [f32; 4] = [122.0, 20.0, 154.0, 46.0];

pub fn get_offsets(size: &[f32; 2], extent: &[f32; 4]) -> [u32; 4] {
    let x_start_offset = ((extent[0] - MESH_EXTENT[0]) / size[0]) as u32;
    let y_start_offset = ((extent[1] - MESH_EXTENT[1]) / size[1]) as u32;
    let x_end_offset = ((MESH_EXTENT[2] - extent[2]) / size[0]) as u32;
    let y_end_offset = ((MESH_EXTENT[3] - extent[3]) / size[1]) as u32;
    return [x_start_offset, y_start_offset, x_end_offset, y_end_offset];
}

pub fn get_grid_count(size: &[f32; 2]) -> [u32; 2] {
    return [
        ((MESH_EXTENT[2] - MESH_EXTENT[0]) / size[0]) as u32,
        ((MESH_EXTENT[3] - MESH_EXTENT[1]) / size[1]) as u32,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(
            get_offsets(&[1.0, 2.0 / 3.0], &[122.0, 20.0, 154.0, 46.0]),
            [0, 0, 0, 0]
        );
        assert_eq!(
            get_offsets(&[1.0, 2.0 / 3.0], &[123.0, 21.0, 153.0, 45.0]),
            [1, 1, 1, 1]
        );
        assert_eq!(
            get_offsets(&[1.0, 2.0 / 3.0], &[122.0, 20.0, 123.0, 21.0]),
            [0, 0, 31, 37]
        );
        assert_eq!(
            get_offsets(&[1.0 / 8.0, 1.0 / 12.0], &[122.125, 20.0, 153.875, 46.0]),
            [1, 0, 1, 0]
        );
    }
}
