use crate::rbsp::RawByteSequencePayload;

use std::fmt;
use std::convert::TryFrom;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaluKind {
    Unspecified(u8),
    Reserved(u8),
    /// Coded slice of a non-IDR picture
    CodedSliceNonIdr, // CodedSliceOfANonIDRPicture,
    /// Coded slice data partition A
    CodedSliceDataPartitionA,
    /// Coded slice data partition B
    CodedSliceDataPartitionB,
    /// Coded slice data partition C
    CodedSliceDataPartitionC,
    /// Coded slice of an IDR picture
    CodedSliceIdr, // CodedSliceOfAnIDRPicture,
    /// Supplemental enhancement information (SEI)
    SupplementalEnhancementInformation,
    /// Sequence parameter set (SPS)
    SequenceParameterSet,
    /// Picture parameter set (PPS)
    PictureParameterSet,
    /// Access unit delimiter
    AccessUnitDelimiter,
    /// End of sequence
    EndOfSequence,
    /// End of stream
    EndOfStream,
    /// Filler data
    FillerData,
    /// Sequence parameter set extension
    SequenceParameterSetExtension,
    /// Prefix NAL unit
    PrefixNALUnit,
    /// Subset sequence parameter set
    SubsetSequenceParameterSet,
    /// Depth parameter set
    DepthParameterSet,
    /// Coded slice of an auxiliary coded picture without partitioning
    CodedSliceOfAnAuxiliaryCodedPictureWithoutPartitioning,
    /// Coded slice extension
    CodedSliceExtension,
    /// Coded slice extension for a depth view component or a 3D-AVC texture view component
    CodedSliceExtensionForADepthViewComponentOrA3DAVCTextureViewComponent,
}

impl TryFrom<u8> for NaluKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NaluKind::Unspecified(0)),
            1 => Ok(NaluKind::CodedSliceNonIdr),
            2 => Ok(NaluKind::CodedSliceDataPartitionA),
            3 => Ok(NaluKind::CodedSliceDataPartitionB),
            4 => Ok(NaluKind::CodedSliceDataPartitionC),
            5 => Ok(NaluKind::CodedSliceIdr),
            6 => Ok(NaluKind::SupplementalEnhancementInformation),
            7 => Ok(NaluKind::SequenceParameterSet),
            8 => Ok(NaluKind::PictureParameterSet),
            9 => Ok(NaluKind::AccessUnitDelimiter),
            10 => Ok(NaluKind::EndOfSequence),
            11 => Ok(NaluKind::EndOfStream),
            12 => Ok(NaluKind::FillerData),
            13 => Ok(NaluKind::SequenceParameterSetExtension),
            14 => Ok(NaluKind::PrefixNALUnit),
            15 => Ok(NaluKind::SubsetSequenceParameterSet),
            16 => Ok(NaluKind::DepthParameterSet),
            n @ 17 ... 18 => Ok(NaluKind::Reserved(n)),
            19 => Ok(NaluKind::CodedSliceOfAnAuxiliaryCodedPictureWithoutPartitioning),
            20 => Ok(NaluKind::CodedSliceExtension),
            21 => Ok(NaluKind::CodedSliceExtensionForADepthViewComponentOrA3DAVCTextureViewComponent),
            n @ 22 ... 23 => Ok(NaluKind::Reserved(n)),
            n @ 24 ... 31 => Ok(NaluKind::Unspecified(n)),
            _ => Err(()),
        }
    }
}

impl Into<u8> for NaluKind {
    fn into(self) -> u8 {
        use self::NaluKind::*;

        match self {
            Unspecified(n) => n,
            Reserved(n) => n,
            CodedSliceNonIdr => 1,
            CodedSliceDataPartitionA => 2,
            CodedSliceDataPartitionB => 3,
            CodedSliceDataPartitionC => 4,
            CodedSliceIdr => 5,
            SupplementalEnhancementInformation => 6,
            SequenceParameterSet => 7,
            PictureParameterSet => 8,
            AccessUnitDelimiter => 9,
            EndOfSequence => 10,
            EndOfStream => 11,
            FillerData => 12,
            SequenceParameterSetExtension => 13,
            PrefixNALUnit => 14,
            SubsetSequenceParameterSet => 15,
            DepthParameterSet => 16,
            CodedSliceOfAnAuxiliaryCodedPictureWithoutPartitioning => 19,
            CodedSliceExtension => 20,
            CodedSliceExtensionForADepthViewComponentOrA3DAVCTextureViewComponent => 21,
        }
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaluRefIdc {
    DISPOSABLE,
    LOW,
    HIGH,
    HIGHEST,
}

impl Into<u8> for NaluRefIdc {
    fn into(self) -> u8 {
        match self {
            NaluRefIdc::DISPOSABLE => 0,
            NaluRefIdc::LOW => 1,
            NaluRefIdc::HIGH => 2,
            NaluRefIdc::HIGHEST => 3,
        }
    }
}

impl TryFrom<u8> for NaluRefIdc {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NaluRefIdc::DISPOSABLE),
            1 => Ok(NaluRefIdc::LOW),
            2 => Ok(NaluRefIdc::HIGH),
            3 => Ok(NaluRefIdc::HIGHEST),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaluHeader {
    nal_ref_idc: NaluRefIdc,       // 2 bits, 0 .. 3
    nal_unit_type: NaluKind,       // 5 bits, 0 .. 31
}

impl NaluHeader {
    pub fn new(nal_ref_idc: NaluRefIdc, nalu_kind: NaluKind) -> Result<Self, ()> {
        match nalu_kind {
            NaluKind::CodedSliceIdr => {
                // nal_ref_idc shall not be equal to 0 for NAL units with nal_unit_type equal to 5.
                if nal_ref_idc == NaluRefIdc::DISPOSABLE {
                    return Err(());
                }
            },
            NaluKind::SupplementalEnhancementInformation
            | NaluKind::AccessUnitDelimiter
            | NaluKind::EndOfSequence
            | NaluKind::EndOfStream
            | NaluKind::FillerData => {
                // nal_ref_idc shall be equal to 0 for all NAL units having nal_unit_type equal to 6, 9, 10, 11, or 12.
                if nal_ref_idc != NaluRefIdc::DISPOSABLE {
                    return Err(());
                }
            },
            _ => { }
        }

        Ok(Self {
            nal_ref_idc: nal_ref_idc,
            nal_unit_type: nalu_kind,
        })
    }

    pub fn forbidden_zero_bit(&self) -> u8 {
        0
    }

    pub fn nal_ref_idc(&self) -> NaluRefIdc {
        self.nal_ref_idc
    }

    pub fn nal_unit_type(&self) -> NaluKind {
        self.nal_unit_type
    }
}

impl TryFrom<u8> for NaluHeader {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let forbidden_zero_bit = value >> 7;
        let nal_ref_idc = ( value >> 5 ) & 0b011;
        let nal_unit_type = value & 0b00011111;

        if forbidden_zero_bit != 0 {
            return Err(());
        }

        if let Ok(ref_idc) = NaluRefIdc::try_from(nal_ref_idc) {
            if let Ok(kind) = NaluKind::try_from(nal_unit_type) {
                return NaluHeader::new(ref_idc, kind)
            }
        }
        
        return Err(())
    }
}

impl Into<u8> for NaluHeader {
    fn into(self) -> u8 {
        let forbidden_zero_bit: u8 = 0;
        let nal_ref_idc: u8 = self.nal_ref_idc.into();
        let nal_unit_type: u8 = self.nal_unit_type.into();
        forbidden_zero_bit | (nal_ref_idc << 5) | nal_unit_type
    }
}

#[derive(Clone)]
pub struct Nalu<RBSP: RawByteSequencePayload + 'static> {
    header: NaluHeader,
    payload: RBSP,
}

impl<RBSP: RawByteSequencePayload> Nalu<RBSP> {
    pub fn new(header: NaluHeader, payload: RBSP) -> Self {
        Self { header, payload }
    }

    pub fn ref_idc(&self) -> NaluRefIdc {
        self.header.nal_ref_idc
    }

    pub fn kind(&self) -> NaluKind {
        self.header.nal_unit_type
    }

    pub fn payload(&self) -> &RBSP {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut RBSP {
        &mut self.payload
    }
}

impl<RBSP: RawByteSequencePayload> fmt::Debug for Nalu<RBSP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nalu {{ ref_idc: {:?}, kind: {:?}, payload: {:?} }}",
            self.ref_idc(),
            self.kind(),
            self.payload())
    }
}