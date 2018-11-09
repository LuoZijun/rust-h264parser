
// SPS
#[derive(Debug, Clone, Copy)]
pub struct SequenceParameterSetFlag(u8);

impl SequenceParameterSetFlag {
    pub fn set0(&self) -> bool {
        (self.0 & 0b1000_0000) == 1
    }
    
    pub fn set1(&self) -> bool {
        (self.0 & 0b0100_0000) == 1
    }

    pub fn set2(&self) -> bool {
        (self.0 & 0b0010_0000) == 1
    }

    pub fn set3(&self) -> bool {
        (self.0 & 0b0001_0000) == 1
    }

    pub fn set4(&self) -> bool {
        (self.0 & 0b0000_1000) == 1
    }

    pub fn set5(&self) -> bool {
        (self.0 & 0b0000_0100) == 1
    }
}

impl From<u8> for SequenceParameterSetFlag {
    fn from(value: u8) -> SequenceParameterSetFlag {
        SequenceParameterSetFlag(value)
    }
}

impl Into<u8> for SequenceParameterSetFlag {
    fn into(self) -> u8 {
        self.0
    }
}

// Page 43
// 7.3.2.1.1 Sequence parameter set data syntax
// SPS
#[derive(Debug, Clone, Copy)]
pub struct SequenceParameterSet {
    profile_idc: u8,
    flag: SequenceParameterSetFlag,
    level_idc: u8,
    // seq_parameter_set_id: 
}



