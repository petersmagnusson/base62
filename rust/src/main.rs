mod base62;

fn main() {
    let data = b"Hello, World!";
    let encoded = base62::array_buffer_to_base62(data, "");
    println!("Encoded: {}", encoded);

    match base62::base62_to_array_buffer(&encoded, "") {
        Ok(decoded) => println!("Decoded: {:?}", String::from_utf8(decoded).unwrap()),
        Err(e) => println!("Error decoding: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_buffer_to_base62() {
        let test_cases = vec![
            (vec![229, 249, 55, 36, 154, 19, 199, 251, 228, 200, 180, 30, 74], "GKMXoLWxKIJ9nRvcvA"),
            (vec![230, 74, 40, 40, 202, 187, 186, 98, 246, 218, 86, 0, 214, 220, 187], "BrPC0abXu43ZDOQcgprT9"),
            (vec![184, 18, 214, 143, 66, 141, 218, 172, 133, 82], "EVa0q8AvOp25jU"),
            (vec![38, 85, 227, 221, 135, 81, 235], "AxaEi0ahmD"),
            (vec![169, 154, 142, 36], "DGjU5e"),
            (vec![241, 120], "QFC"),
            (vec![129, 157, 167, 119, 175], "Jxoyu5Z"),
            (vec![33, 253, 92, 67, 78, 200, 186], "ArysuVaSTY"),
            (vec![253, 111, 239, 94, 255, 211, 191], "FQsqiz7S1J"),
            (vec![], ""),
            (vec![9, 215, 175], "ACrx1"),
            (vec![213, 247, 82, 99, 184, 41, 29], "Ebz0T7Bvjf"),
            (vec![216, 116, 150, 56, 17, 237, 75, 133, 218, 204, 191, 22], "BZHmuNXvQ7QD6vPbI"),
            (vec![57, 151, 80, 63, 162, 108, 40, 140, 113, 195, 243], "FmEJc8i3gfA89d5"),
            (vec![222, 14, 24], "A9DxW"),
            (vec![126, 159, 129, 64, 253, 19, 174, 76, 46, 153, 27, 162], "Ay71uYPlpFKTWXd5Y"),
            (vec![167, 64, 75, 68, 135], "MoGF6Bp"),
            (vec![152, 139, 251, 121, 188, 18, 128, 101], "NGALxVBEJVb"),
        ];

        for (buffer, expected) in test_cases {
            let encoded = base62::array_buffer_to_base62(&buffer, "");
            assert_eq!(encoded, expected);
        }
    }

    #[test]
    fn test_base62_to_array_buffer() {
        let test_cases = vec![
            (vec![229, 249, 55, 36, 154, 19, 199, 251, 228, 200, 180, 30, 74], "GKMXoLWxKIJ9nRvcvA"),
            (vec![230, 74, 40, 40, 202, 187, 186, 98, 246, 218, 86, 0, 214, 220, 187], "BrPC0abXu43ZDOQcgprT9"),
            (vec![184, 18, 214, 143, 66, 141, 218, 172, 133, 82], "EVa0q8AvOp25jU"),
            (vec![38, 85, 227, 221, 135, 81, 235], "AxaEi0ahmD"),
            (vec![169, 154, 142, 36], "DGjU5e"),
            (vec![241, 120], "QFC"),
            (vec![129, 157, 167, 119, 175], "Jxoyu5Z"),
            (vec![33, 253, 92, 67, 78, 200, 186], "ArysuVaSTY"),
            (vec![253, 111, 239, 94, 255, 211, 191], "FQsqiz7S1J"),
            (vec![], ""),
            (vec![9, 215, 175], "ACrx1"),
            (vec![213, 247, 82, 99, 184, 41, 29], "Ebz0T7Bvjf"),
            (vec![216, 116, 150, 56, 17, 237, 75, 133, 218, 204, 191, 22], "BZHmuNXvQ7QD6vPbI"),
            (vec![57, 151, 80, 63, 162, 108, 40, 140, 113, 195, 243], "FmEJc8i3gfA89d5"),
            (vec![222, 14, 24], "A9DxW"),
            (vec![126, 159, 129, 64, 253, 19, 174, 76, 46, 153, 27, 162], "Ay71uYPlpFKTWXd5Y"),
            (vec![167, 64, 75, 68, 135], "MoGF6Bp"),
            (vec![152, 139, 251, 121, 188, 18, 128, 101], "NGALxVBEJVb"),
        ];

        for (expected, base62_string) in test_cases {
            let decoded = base62::base62_to_array_buffer(base62_string, "").unwrap();
            assert_eq!(decoded, expected);
        }
    }
}
