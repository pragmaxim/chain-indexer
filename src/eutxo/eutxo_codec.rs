use byteorder::{BigEndian, ByteOrder};

type EutxoPkBytes = [u8; 8];

#[derive(Debug, PartialEq, Clone)]
struct EutxoPk {
    pub block_height: u32,
    pub tx_index: u16,
    pub utxo_index: u16,
}

// Implementing From trait for CiUtxoId to UtxoIdBytes conversion
impl From<EutxoPk> for EutxoPkBytes {
    fn from(utxo_id: EutxoPk) -> EutxoPkBytes {
        let mut bytes: EutxoPkBytes = [0u8; 8];
        BigEndian::write_u32(&mut bytes[0..4], utxo_id.block_height);
        BigEndian::write_u16(&mut bytes[4..6], utxo_id.tx_index);
        BigEndian::write_u16(&mut bytes[6..8], utxo_id.utxo_index);
        bytes
    }
}

// Implementing From trait for UtxoIdBytes to CiUtxoId conversion
impl From<EutxoPkBytes> for EutxoPk {
    fn from(bytes: EutxoPkBytes) -> EutxoPk {
        let block_height = BigEndian::read_u32(&bytes[0..4]);
        let tx_index = BigEndian::read_u16(&bytes[4..6]);
        let utxo_index = BigEndian::read_u16(&bytes[6..8]);
        EutxoPk {
            block_height,
            tx_index,
            utxo_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_utxo_id() {
        let utxo_id = EutxoPk {
            block_height: 123456,
            tx_index: 7890,
            utxo_index: 1234,
        };
        let expected_bytes: EutxoPkBytes = [0, 1, 226, 64, 30, 222, 4, 210];
        let encoded: EutxoPkBytes = utxo_id.into();
        assert_eq!(encoded, expected_bytes);
    }

    #[test]
    fn test_decode_utxo_id() {
        let bytes: EutxoPkBytes = [0, 1, 226, 64, 30, 222, 4, 210];
        let expected_utxo_id = EutxoPk {
            block_height: 123456,
            tx_index: 7890,
            utxo_index: 1234,
        };
        let decoded: EutxoPk = bytes.into();
        assert_eq!(decoded, expected_utxo_id);
    }

    #[test]
    fn test_round_trip_conversion() {
        let utxo_id = EutxoPk {
            block_height: 123456,
            tx_index: 7890,
            utxo_index: 1234,
        };
        let encoded: EutxoPkBytes = utxo_id.clone().into();
        let decoded: EutxoPk = encoded.into();
        assert_eq!(utxo_id, decoded);
    }
}