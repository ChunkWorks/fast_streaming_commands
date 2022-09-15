#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use crate::struct_to_packet_u16;

    #[derive(Serialize, Deserialize)]
    pub struct TestStruct {
        pub command: String,
        pub parameters: Vec<Vec<u8>>,
    }

    #[test]
    fn test_command_packets() {
        let command_packet = TestStruct {
            command: "login".to_string(),
            parameters: vec![
                vec![0, 1, 8, 0, 0, 8, 5]
            ]
        };
        let u16_packet = struct_to_packet_u16(&command_packet);
        let known_bytes = hex::decode("0f00056c6f67696e010700010800000805").unwrap();
        assert_eq!(u16_packet, known_bytes);
    }
}
