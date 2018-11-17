
use std::fmt;


// A.2 Profiles ( Page 306 )
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Baseline,
    Main,
    Extended,
    High,
    ProgressiveHigh,
    ConstrainedHigh,
    // A.2.5 High 10 profile (Page 309)
    High10,
    // A.2.5.1 Progressive High 10 profile
    ProgressiveHigh10,
    // A.2.6 High 4:2:2 profile ( page 310 )
    High422,
    // High 4:4:4 Predictive
    High444Predictive,
    // High 10 Intra
    High10Intra,
    // High 4:2:2 Intra
    High422Intra,
    // High 4:4:4 Intra profile
    High444Intra,
    // CAVLC 4:4:4 Intra profile
    CAVLC444Intra,
}


// A.3 Levels ( 312 )
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    L1,
    L1B,
    L1_1,
    L1_2,
    L1_3,
    L2,
    L2_1,
    L2_2,
    L3,
    L3_1,
    L3_2,
    L4,
    L4_1,
    L4_2,
    L5,
    L5_1,
    L5_2,
    L6,
    L6_1,
    L6_2,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Level::*;

        match *self {
            L1 => write!(f, "1"),
            L1B => write!(f, "1b"),
            L1_1 => write!(f, "1.1"),
            L1_2 => write!(f, "1.2"),
            L1_3 => write!(f, "1.3"),
            L2 => write!(f, "2"),
            L2_1 => write!(f, "2.1"),
            L2_2 => write!(f, "2.2"),
            L3 => write!(f, "3"),
            L3_1 => write!(f, "3.1"),
            L3_2 => write!(f, "3.2"),
            L4 => write!(f, "4"),
            L4_1 => write!(f, "4.1"),
            L4_2 => write!(f, "4.2"),
            L5 => write!(f, "5"),
            L5_1 => write!(f, "5.1"),
            L5_2 => write!(f, "5.2"),
            L6 => write!(f, "6"),
            L6_1 => write!(f, "6.1"),
            L6_2 => write!(f, "6.2"),
        }
    }
}


// SPS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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


// Syntax: 7.3.2.1.1 ( Page 64 )
// Semantic: 7.4.2.1 ( Page 94 )
// VUI syntax: Annex E ( Page 414 )
// 
// SPS
#[derive(Debug, Clone)]
pub struct SequenceParameterSet {
    profile_idc: u8,
    flag: SequenceParameterSetFlag,
    level_idc: u8,
    seq_parameter_set_id: u32,             // ue(v)

    chroma_format_idc: Option<u32>,        // ue(v)
    separate_colour_plane_flag: Option<u8>,
    bit_depth_luma_minus8: Option<u32>,    // ue(v)
    bit_depth_chroma_minus8: Option<u32>,  // ue(v)
    qpprime_y_zero_transform_bypass_flag: Option<bool>,
    seq_scaling_matrix_present_flag: Option<bool>,
    seq_scaling_list_present_flag: Option<Vec<bool>>,
    // TODO:
    // scaling_list()

    log2_max_frame_num_minus4: u32,        // ue(v)
    pic_order_cnt_type: u32,               // ue(v)
    log2_max_pic_order_cnt_lsb_minus4: Option<u32>,     // ue(v)
    delta_pic_order_always_zero_flag: Option<bool>,
    offset_for_non_ref_pic: Option<i32>,                // se(v)
    offset_for_top_to_bottom_field: Option<i32>,        // se(v)
    num_ref_frames_in_pic_order_cnt_cycle: Option<u32>, // ue(v)
    offset_for_ref_frame: Option<Vec<i32>>, // se(v)

    max_num_ref_frames: u32,                // ue(v)
    gaps_in_frame_num_value_allowed_flag: bool,
    pic_width_in_mbs_minus1: u32,           // ue(v)
    pic_height_in_map_units_minus1: u32,    // ue(v)
    frame_mbs_only_flag: bool,
    mb_adaptive_frame_field_flag: Option<bool>,
    direct_8x8_inference_flag: bool,
    frame_cropping_flag: bool,

    frame_crop_left_offset: Option<u32>,     // ue(v)
    frame_crop_right_offset: Option<u32>,    // ue(v)
    frame_crop_top_offset: Option<u32>,      // ue(v)
    frame_crop_bottom_offset: Option<u32>,   // ue(v)

    vui_parameters_present_flag: bool,

    // TODO: 
    // vui_parameters()

}



