#[cfg(test)]
mod tests {
    use test_env_log::test;
    use crate::CommandParser;
    use log::debug;
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    #[test]
    fn test_parser_bytes() {
        let mut command_fired = false;
        let on_command = |_bytes: &[u8]| {
            command_fired = true;
        };
        let mut parser = CommandParser::new(on_command);
        let length_bytes = (4 as u32).to_le_bytes();
        let bytes = &[8, 0, 0, 8];

        parser.parse(&length_bytes);
        parser.parse(bytes);
        assert_eq!(command_fired, true);
    }

    #[test]
    fn stress_test() {
        let mut rng = Pcg64::seed_from_u64(80085);
        for _ in 0..100 {
            let length: u8 = rng.gen_range(10..255);
            let mut command_fired = false;
            let length_bytes = (length as u32).to_le_bytes();
            let mut bytes = vec![0u8; length.into()];
            rng.fill(&mut bytes[..]);
            debug!("Pushing length_bytes: {} and bytes: {}", hex::encode(&length_bytes), hex::encode(&bytes));
            let on_command = |command_bytes: &[u8]| {
                assert_eq!(bytes, command_bytes);
                command_fired = true;
            };
            let mut parser = CommandParser::new(on_command);
            parser.parse(&length_bytes);
            parser.parse(&bytes);
            assert_eq!(command_fired, true);
        }
    }
}