// ue(v)：无符号整数指数哥伦布码编码的语法元素，左位在先。
// se(v)：有符号整数指数哥伦布码编码的语法元素，左位在先。
// u(n)：n位无符号整数。在语法表中，如果n是‘v’，其比特数由其它语法元素值确定。解析过程由函
// 数read_bits(n)的返回值规定，该返回值用最高有效位在前的二进制表示。


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



