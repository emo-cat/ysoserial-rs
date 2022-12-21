use crate::template_impl::get_f_template_impl;

pub fn get_jboss_interceptors1(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [
        172, 237, 0, 5, 115, 114, 0, 52, 111, 114, 103, 46, 106, 98, 111, 115, 115, 46, 105, 110,
        116, 101, 114, 99, 101, 112, 116, 111, 114, 46, 112, 114, 111, 120, 121, 46, 73, 110, 116,
        101, 114, 99, 101, 112, 116, 111, 114, 77, 101, 116, 104, 111, 100, 72, 97, 110, 100, 108,
        101, 114, 135, 58, 74, 247, 113, 194, 131, 202, 3, 0, 5, 76, 0, 17, 105, 110, 116, 101,
        114, 99, 101, 112, 116, 105, 111, 110, 77, 111, 100, 101, 108, 116, 0, 51, 76, 111, 114,
        103, 47, 106, 98, 111, 115, 115, 47, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114,
        47, 115, 112, 105, 47, 109, 111, 100, 101, 108, 47, 73, 110, 116, 101, 114, 99, 101, 112,
        116, 105, 111, 110, 77, 111, 100, 101, 108, 59, 76, 0, 27, 105, 110, 116, 101, 114, 99,
        101, 112, 116, 111, 114, 72, 97, 110, 100, 108, 101, 114, 73, 110, 115, 116, 97, 110, 99,
        101, 115, 116, 0, 15, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59,
        76, 0, 24, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 67, 111, 110, 116, 101, 120,
        116, 70, 97, 99, 116, 111, 114, 121, 116, 0, 60, 76, 111, 114, 103, 47, 106, 98, 111, 115,
        115, 47, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 47, 115, 112, 105, 47, 99,
        111, 110, 116, 101, 120, 116, 47, 73, 110, 118, 111, 99, 97, 116, 105, 111, 110, 67, 111,
        110, 116, 101, 120, 116, 70, 97, 99, 116, 111, 114, 121, 59, 76, 0, 30, 116, 97, 114, 103,
        101, 116, 67, 108, 97, 115, 115, 73, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 77,
        101, 116, 97, 100, 97, 116, 97, 116, 0, 56, 76, 111, 114, 103, 47, 106, 98, 111, 115, 115,
        47, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 47, 115, 112, 105, 47, 109, 101,
        116, 97, 100, 97, 116, 97, 47, 73, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 77,
        101, 116, 97, 100, 97, 116, 97, 59, 76, 0, 14, 116, 97, 114, 103, 101, 116, 73, 110, 115,
        116, 97, 110, 99, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98,
        106, 101, 99, 116, 59, 120, 112, 115, 114, 0, 51, 111, 114, 103, 46, 106, 98, 111, 115,
        115, 46, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 46, 98, 117, 105, 108, 100,
        101, 114, 46, 73, 110, 116, 101, 114, 99, 101, 112, 116, 105, 111, 110, 77, 111, 100, 101,
        108, 73, 109, 112, 108, 212, 148, 225, 80, 226, 184, 219, 32, 2, 0, 5, 76, 0, 15, 97, 108,
        108, 73, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 115, 116, 0, 15, 76, 106, 97,
        118, 97, 47, 117, 116, 105, 108, 47, 83, 101, 116, 59, 76, 0, 18, 103, 108, 111, 98, 97,
        108, 73, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 115, 113, 0, 126, 0, 2, 76, 0,
        17, 105, 110, 116, 101, 114, 99, 101, 112, 116, 101, 100, 69, 110, 116, 105, 116, 121, 113,
        0, 126, 0, 5, 76, 0, 23, 109, 101, 116, 104, 111, 100, 66, 111, 117, 110, 100, 73, 110,
        116, 101, 114, 99, 101, 112, 116, 111, 114, 115, 113, 0, 126, 0, 2, 76, 0, 22, 109, 101,
        116, 104, 111, 100, 115, 73, 103, 110, 111, 114, 105, 110, 103, 71, 108, 111, 98, 97, 108,
        115, 113, 0, 126, 0, 8, 120, 112, 115, 114, 0, 23, 106, 97, 118, 97, 46, 117, 116, 105,
        108, 46, 76, 105, 110, 107, 101, 100, 72, 97, 115, 104, 83, 101, 116, 216, 108, 215, 90,
        149, 221, 42, 30, 2, 0, 0, 120, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46,
        72, 97, 115, 104, 83, 101, 116, 186, 68, 133, 149, 150, 184, 183, 52, 3, 0, 0, 120, 112,
        119, 12, 0, 0, 0, 16, 63, 64, 0, 0, 0, 0, 0, 1, 115, 114, 0, 54, 111, 114, 103, 46, 106,
        98, 111, 115, 115, 46, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 46, 114, 101,
        97, 100, 101, 114, 46, 83, 105, 109, 112, 108, 101, 73, 110, 116, 101, 114, 99, 101, 112,
        116, 111, 114, 77, 101, 116, 97, 100, 97, 116, 97, 0, 4, 110, 38, 50, 205, 240, 139, 2, 0,
        3, 90, 0, 11, 116, 97, 114, 103, 101, 116, 67, 108, 97, 115, 115, 76, 0, 20, 105, 110, 116,
        101, 114, 99, 101, 112, 116, 111, 114, 77, 101, 116, 104, 111, 100, 77, 97, 112, 113, 0,
        126, 0, 2, 76, 0, 20, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 82, 101, 102,
        101, 114, 101, 110, 99, 101, 116, 0, 57, 76, 111, 114, 103, 47, 106, 98, 111, 115, 115, 47,
        105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 47, 115, 112, 105, 47, 109, 101, 116,
        97, 100, 97, 116, 97, 47, 73, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 82, 101,
        102, 101, 114, 101, 110, 99, 101, 59, 120, 112, 1, 115, 114, 0, 17, 106, 97, 118, 97, 46,
        117, 116, 105, 108, 46, 72, 97, 115, 104, 77, 97, 112, 5, 7, 218, 193, 195, 22, 96, 209, 3,
        0, 2, 70, 0, 10, 108, 111, 97, 100, 70, 97, 99, 116, 111, 114, 73, 0, 9, 116, 104, 114,
        101, 115, 104, 111, 108, 100, 120, 112, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0,
        0, 0, 1, 126, 114, 0, 48, 111, 114, 103, 46, 106, 98, 111, 115, 115, 46, 105, 110, 116,
        101, 114, 99, 101, 112, 116, 111, 114, 46, 115, 112, 105, 46, 109, 111, 100, 101, 108, 46,
        73, 110, 116, 101, 114, 99, 101, 112, 116, 105, 111, 110, 84, 121, 112, 101, 0, 0, 0, 0, 0,
        0, 0, 0, 18, 0, 0, 120, 114, 0, 14, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 69, 110,
        117, 109, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 120, 112, 116, 0, 13, 80, 79, 83, 84, 95, 65,
        67, 84, 73, 86, 65, 84, 69, 115, 114, 0, 19, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46,
        65, 114, 114, 97, 121, 76, 105, 115, 116, 120, 129, 210, 29, 153, 199, 97, 157, 3, 0, 1,
        73, 0, 4, 115, 105, 122, 101, 120, 112, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 115, 114, 0, 90,
        111, 114, 103, 46, 106, 98, 111, 115, 115, 46, 105, 110, 116, 101, 114, 99, 101, 112, 116,
        111, 114, 46, 114, 101, 97, 100, 101, 114, 46, 68, 101, 102, 97, 117, 108, 116, 77, 101,
        116, 104, 111, 100, 77, 101, 116, 97, 100, 97, 116, 97, 36, 68, 101, 102, 97, 117, 108,
        116, 77, 101, 116, 104, 111, 100, 77, 101, 116, 97, 100, 97, 116, 97, 83, 101, 114, 105,
        97, 108, 105, 122, 97, 116, 105, 111, 110, 80, 114, 111, 120, 121, 213, 121, 73, 79, 155,
        125, 210, 202, 2, 0, 2, 76, 0, 15, 109, 101, 116, 104, 111, 100, 82, 101, 102, 101, 114,
        101, 110, 99, 101, 116, 0, 47, 76, 111, 114, 103, 47, 106, 98, 111, 115, 115, 47, 105, 110,
        116, 101, 114, 99, 101, 112, 116, 111, 114, 47, 98, 117, 105, 108, 100, 101, 114, 47, 77,
        101, 116, 104, 111, 100, 82, 101, 102, 101, 114, 101, 110, 99, 101, 59, 76, 0, 26, 115,
        117, 112, 112, 111, 114, 116, 101, 100, 73, 110, 116, 101, 114, 99, 101, 112, 116, 105,
        111, 110, 84, 121, 112, 101, 115, 113, 0, 126, 0, 8, 120, 112, 115, 114, 0, 76, 111, 114,
        103, 46, 106, 98, 111, 115, 115, 46, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114,
        46, 98, 117, 105, 108, 100, 101, 114, 46, 77, 101, 116, 104, 111, 100, 82, 101, 102, 101,
        114, 101, 110, 99, 101, 36, 77, 101, 116, 104, 111, 100, 72, 111, 108, 100, 101, 114, 83,
        101, 114, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 80, 114, 111, 120, 121, 212, 106,
        96, 244, 73, 62, 125, 210, 2, 0, 3, 76, 0, 9, 99, 108, 97, 115, 115, 78, 97, 109, 101, 116,
        0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59,
        76, 0, 10, 109, 101, 116, 104, 111, 100, 78, 97, 109, 101, 113, 0, 126, 0, 28, 91, 0, 19,
        112, 97, 114, 97, 109, 101, 116, 101, 114, 67, 108, 97, 115, 115, 78, 97, 109, 101, 115,
        116, 0, 19, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
        103, 59, 120, 112, 116, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97,
        112, 97, 99, 104, 101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114, 110, 97,
        108, 46, 120, 115, 108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112, 108, 97,
        116, 101, 115, 73, 109, 112, 108, 116, 0, 14, 110, 101, 119, 84, 114, 97, 110, 115, 102,
        111, 114, 109, 101, 114, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103,
        46, 83, 116, 114, 105, 110, 103, 59, 173, 210, 86, 231, 233, 29, 123, 71, 2, 0, 0, 120,
        112, 0, 0, 0, 0, 115, 113, 0, 126, 0, 11, 119, 12, 0, 0, 0, 16, 63, 64, 0, 0, 0, 0, 0, 1,
        113, 0, 126, 0, 20, 120, 120, 120, 115, 114, 0, 62, 111, 114, 103, 46, 106, 98, 111, 115,
        115, 46, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114, 46, 114, 101, 97, 100, 101,
        114, 46, 67, 108, 97, 115, 115, 77, 101, 116, 97, 100, 97, 116, 97, 73, 110, 116, 101, 114,
        99, 101, 112, 116, 111, 114, 82, 101, 102, 101, 114, 101, 110, 99, 101, 203, 131, 179, 190,
        249, 59, 48, 254, 2, 0, 1, 76, 0, 13, 99, 108, 97, 115, 115, 77, 101, 116, 97, 100, 97,
        116, 97, 116, 0, 50, 76, 111, 114, 103, 47, 106, 98, 111, 115, 115, 47, 105, 110, 116, 101,
        114, 99, 101, 112, 116, 111, 114, 47, 115, 112, 105, 47, 109, 101, 116, 97, 100, 97, 116,
        97, 47, 67, 108, 97, 115, 115, 77, 101, 116, 97, 100, 97, 116, 97, 59, 120, 112, 115, 114,
        0, 52, 111, 114, 103, 46, 106, 98, 111, 115, 115, 46, 105, 110, 116, 101, 114, 99, 101,
        112, 116, 111, 114, 46, 114, 101, 97, 100, 101, 114, 46, 82, 101, 102, 108, 101, 99, 116,
        105, 118, 101, 67, 108, 97, 115, 115, 77, 101, 116, 97, 100, 97, 116, 97, 227, 3, 133, 63,
        247, 108, 78, 190, 2, 0, 1, 76, 0, 5, 99, 108, 97, 122, 122, 116, 0, 17, 76, 106, 97, 118,
        97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 120, 112, 118, 113, 0, 126, 0,
        16, 120, 115, 113, 0, 126, 0, 16, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0,
        6, 126, 113, 0, 126, 0, 18, 116, 0, 13, 65, 82, 79, 85, 78, 68, 95, 73, 78, 86, 79, 75, 69,
        115, 113, 0, 126, 0, 22, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 113, 0, 126, 0, 15, 120, 126, 113,
        0, 126, 0, 18, 116, 0, 14, 80, 79, 83, 84, 95, 67, 79, 78, 83, 84, 82, 85, 67, 84, 115,
        113, 0, 126, 0, 22, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 113, 0, 126, 0, 15, 120, 113, 0, 126,
        0, 20, 115, 113, 0, 126, 0, 22, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 113, 0, 126, 0, 15, 120,
        126, 113, 0, 126, 0, 18, 116, 0, 11, 80, 82, 69, 95, 68, 69, 83, 84, 82, 79, 89, 115, 113,
        0, 126, 0, 22, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 113, 0, 126, 0, 15, 120, 126, 113, 0, 126,
        0, 18, 116, 0, 14, 65, 82, 79, 85, 78, 68, 95, 84, 73, 77, 69, 79, 85, 84, 115, 113, 0,
        126, 0, 22, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 113, 0, 126, 0, 15, 120, 126, 113, 0, 126, 0,
        18, 116, 0, 13, 80, 82, 69, 95, 80, 65, 83, 83, 73, 86, 65, 84, 69, 115, 113, 0, 126, 0,
        22, 0, 0, 0, 1, 119, 4, 0, 0, 0, 1, 113, 0, 126, 0, 15, 120, 120, 113, 0, 126, 0, 42, 115,
        113, 0, 126, 0, 16, 63, 64, 0, 0, 0, 0, 0, 0, 119, 8, 0, 0, 0, 16, 0, 0, 0, 0, 120, 115,
        113, 0, 126, 0, 11, 119, 12, 0, 0, 0, 16, 63, 64, 0, 0, 0, 0, 0, 0, 120, 115, 113, 0, 126,
        0, 16, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 113, 0, 126, 0, 15, 115,
        114, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97, 112, 97, 99, 104,
        101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114, 110, 97, 108, 46, 120, 115,
        108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112, 108, 97, 116, 101, 115, 73,
        109, 112, 108, 9, 87, 79, 193, 110, 172, 171, 51, 3, 0, 6, 73, 0, 13, 95, 105, 110, 100,
        101, 110, 116, 78, 117, 109, 98, 101, 114, 73, 0, 14, 95, 116, 114, 97, 110, 115, 108, 101,
        116, 73, 110, 100, 101, 120, 91, 0, 10, 95, 98, 121, 116, 101, 99, 111, 100, 101, 115, 116,
        0, 3, 91, 91, 66, 91, 0, 6, 95, 99, 108, 97, 115, 115, 116, 0, 18, 91, 76, 106, 97, 118,
        97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 76, 0, 5, 95, 110, 97, 109, 101,
        113, 0, 126, 0, 28, 76, 0, 17, 95, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101,
        114, 116, 105, 101, 115, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 80,
        114, 111, 112, 101, 114, 116, 105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255, 255, 255, 255,
        117, 114, 0, 3, 91, 91, 66, 75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0, 120, 112, 0, 0, 0,
        1, 117, 114, 0, 2, 91, 66, 172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0, 120, 112,
    ]
    .to_vec();
    let template_impl = get_f_template_impl(cmd);
    let template_impl_len = template_impl.len() as u32;
    result_bytes.extend(template_impl_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([
        112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 120, 115, 114, 0, 59, 111, 114,
        103, 46, 106, 98, 111, 115, 115, 46, 105, 110, 116, 101, 114, 99, 101, 112, 116, 111, 114,
        46, 112, 114, 111, 120, 121, 46, 68, 101, 102, 97, 117, 108, 116, 73, 110, 118, 111, 99,
        97, 116, 105, 111, 110, 67, 111, 110, 116, 101, 120, 116, 70, 97, 99, 116, 111, 114, 121,
        65, 78, 150, 246, 105, 119, 90, 97, 2, 0, 0, 120, 112, 115, 113, 0, 126, 0, 13, 1, 115,
        113, 0, 126, 0, 16, 63, 64, 0, 0, 0, 0, 0, 0, 119, 8, 0, 0, 0, 16, 0, 0, 0, 0, 120, 115,
        113, 0, 126, 0, 36, 113, 0, 126, 0, 41, 115, 113, 0, 126, 0, 16, 63, 64, 0, 0, 0, 0, 0, 12,
        119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 116, 0, 9, 121, 115, 111, 115, 101, 114, 105, 97, 108,
        113, 0, 126, 0, 79, 120, 120,
    ]);
    result_bytes
}
