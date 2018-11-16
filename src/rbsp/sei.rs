use crate::bitstream_io::{ BitReader, Endianness, };

use std::fmt;
use std::io::Read;
use std::convert::TryFrom;


// SeiMessage
// SeiMessageKind
pub trait SeiMessage: fmt::Debug {
    fn kind(&self) -> SeiMessageKind;
    fn len(&self) -> usize;
    fn as_bytes(&self) -> &[u8];
}


// SEI Payloads: Annex D ( Page 350 )
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeiMessageKind {
    BufferingPeriod,
    PicTiming,
    PanScanRect,
    FillerPayload,
    UserDataRegisteredItuTT35,
    UserDataUnregistered,
    RecoveryPoint,
    DecRefPicMarkingRepetition,
    SparePic,
    SceneInfo,
    SubSeqInfo,
    SubSeqLayerCharacteristics,
    SubSeqCharacteristics,
    FullFrameFreeze,
    FullFrameFreezeRelease,
    FullFrameSnapshot,
    ProgressiveRefinementSegmentStart,
    ProgressiveRefinementSegmentEnd,
    MotionConstrainedSliceGroupSet,
    FilmGrainCharacteristics,
    DeblockingFilterDisplayPreference,
    StereoVideoInfo,
    PostFilterHint,
    ToneMappingInfo,
    ScalabilityInfo,
    SubPicScalableLayer,
    NonRequiredLayerRep,
    PriorityLayerInfo,
    LayersNotPresent,
    LayerDependencyChange,
    ScalableNesting,
    BaseLayerTemporalHrd,
    QualityLayerIntegrityCheck,
    RedundantPicProperty,
    Tl0DepRepIndex,
    TlSwitchingPoint,
    ParallelDecodingInfo,
    MvcScalableNesting,
    ViewScalabilityInfo,
    MultiviewSceneInfo,
    MultiviewAcquisitionInfo,
    NonRequiredViewComponent,
    ViewDependencyChange,
    OperationPointsNotPresent,
    BaseViewTemporalHrd,
    FramePackingArrangement,
    MultiviewViewPosition,
    DisplayOrientation,
    MvcdScalableNesting,
    MvcdViewScalabilityInfo,
    DepthRepresentationInfo,
    ThreeDimensionalReferenceDisplaysInfo,
    DepthTiming,
    DepthSamplingInfo,
    ConstrainedDepthParameterSetIdentifier,
    GreenMetadata,
    MasteringDisplayColourVolume,
    ColourRemappingInfo,
    AlternativeTransferCharacteristics,
    AlternativeDepthInfo,
    Reserved(u32),
}

impl TryFrom<u32> for SeiMessageKind {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use self::SeiMessageKind::*;

        match value {
            0 => Ok(BufferingPeriod),
            1 => Ok(PicTiming),
            2 => Ok(PanScanRect),
            3 => Ok(FillerPayload),
            4 => Ok(UserDataRegisteredItuTT35),
            5 => Ok(UserDataUnregistered),
            6 => Ok(RecoveryPoint),
            7 => Ok(DecRefPicMarkingRepetition),
            8 => Ok(SparePic),
            9 => Ok(SceneInfo),
            10 => Ok(SubSeqInfo),
            11 => Ok(SubSeqLayerCharacteristics),
            12 => Ok(SubSeqCharacteristics),
            13 => Ok(FullFrameFreeze),
            14 => Ok(FullFrameFreezeRelease),
            15 => Ok(FullFrameSnapshot),
            16 => Ok(ProgressiveRefinementSegmentStart),
            17 => Ok(ProgressiveRefinementSegmentEnd),
            18 => Ok(MotionConstrainedSliceGroupSet),
            19 => Ok(FilmGrainCharacteristics),
            20 => Ok(DeblockingFilterDisplayPreference),
            21 => Ok(StereoVideoInfo),
            22 => Ok(PostFilterHint),
            23 => Ok(ToneMappingInfo),
            24 => Ok(ScalabilityInfo),
            25 => Ok(SubPicScalableLayer),
            26 => Ok(NonRequiredLayerRep),
            27 => Ok(PriorityLayerInfo),
            28 => Ok(LayersNotPresent),
            29 => Ok(LayerDependencyChange),
            30 => Ok(ScalableNesting),
            31 => Ok(BaseLayerTemporalHrd),
            32 => Ok(QualityLayerIntegrityCheck),
            33 => Ok(RedundantPicProperty),
            34 => Ok(Tl0DepRepIndex),
            35 => Ok(TlSwitchingPoint),
            36 => Ok(ParallelDecodingInfo),
            37 => Ok(MvcScalableNesting),
            38 => Ok(ViewScalabilityInfo),
            39 => Ok(MultiviewSceneInfo),
            40 => Ok(MultiviewAcquisitionInfo),
            41 => Ok(NonRequiredViewComponent),
            42 => Ok(ViewDependencyChange),
            43 => Ok(OperationPointsNotPresent),
            44 => Ok(BaseViewTemporalHrd),
            45 => Ok(FramePackingArrangement),
            46 => Ok(MultiviewViewPosition),
            47 => Ok(DisplayOrientation),
            48 => Ok(MvcdScalableNesting),
            49 => Ok(MvcdViewScalabilityInfo),
            50 => Ok(DepthRepresentationInfo),
            51 => Ok(ThreeDimensionalReferenceDisplaysInfo),
            52 => Ok(DepthTiming),
            53 => Ok(DepthSamplingInfo),
            54 => Ok(ConstrainedDepthParameterSetIdentifier),
            56 => Ok(GreenMetadata),
            137 => Ok(MasteringDisplayColourVolume),
            142 => Ok(ColourRemappingInfo),
            147 => Ok(AlternativeTransferCharacteristics),
            181 => Ok(AlternativeDepthInfo),
            n @ _ => Ok(Reserved(n)),
        }
    }
}


impl Into<u32> for SeiMessageKind {
    fn into(self) -> u32 {
        use self::SeiMessageKind::*;

        match self {
            BufferingPeriod => 0u32,
            PicTiming => 1,
            PanScanRect => 2,
            FillerPayload => 3,
            UserDataRegisteredItuTT35 => 4,
            UserDataUnregistered => 5,
            RecoveryPoint => 6,
            DecRefPicMarkingRepetition => 7,
            SparePic => 8,
            SceneInfo => 9,
            SubSeqInfo => 10,
            SubSeqLayerCharacteristics => 11,
            SubSeqCharacteristics => 12,
            FullFrameFreeze => 13,
            FullFrameFreezeRelease => 14,
            FullFrameSnapshot => 15,
            ProgressiveRefinementSegmentStart => 16,
            ProgressiveRefinementSegmentEnd => 17,
            MotionConstrainedSliceGroupSet => 18,
            FilmGrainCharacteristics => 19,
            DeblockingFilterDisplayPreference => 20,
            StereoVideoInfo => 21,
            PostFilterHint => 22,
            ToneMappingInfo => 23,
            ScalabilityInfo => 24,
            SubPicScalableLayer => 25,
            NonRequiredLayerRep => 26,
            PriorityLayerInfo => 27,
            LayersNotPresent => 28,
            LayerDependencyChange => 29,
            ScalableNesting => 30,
            BaseLayerTemporalHrd => 31,
            QualityLayerIntegrityCheck => 32,
            RedundantPicProperty => 33,
            Tl0DepRepIndex => 34,
            TlSwitchingPoint => 35,
            ParallelDecodingInfo => 36,
            MvcScalableNesting => 37,
            ViewScalabilityInfo => 38,
            MultiviewSceneInfo => 39,
            MultiviewAcquisitionInfo => 40,
            NonRequiredViewComponent => 41,
            ViewDependencyChange => 42,
            OperationPointsNotPresent => 43,
            BaseViewTemporalHrd => 44,
            FramePackingArrangement => 45,
            MultiviewViewPosition => 46,
            DisplayOrientation => 47,
            MvcdScalableNesting => 48,
            MvcdViewScalabilityInfo => 49,
            DepthRepresentationInfo => 50,
            ThreeDimensionalReferenceDisplaysInfo => 51,
            DepthTiming => 52,
            DepthSamplingInfo => 53,
            ConstrainedDepthParameterSetIdentifier => 54,
            GreenMetadata => 56,
            MasteringDisplayColourVolume => 137,
            ColourRemappingInfo => 142,
            AlternativeTransferCharacteristics => 147,
            AlternativeDepthInfo => 181,
            Reserved(n) => n,
        }
    }
}

#[derive(Debug)]
pub struct SeiAnyMessage {
    kind: SeiMessageKind,
    size: usize,
    body: Vec<u8>,
}

impl SeiMessage for SeiAnyMessage {
    fn kind(&self) -> SeiMessageKind {
        self.kind
    }

    fn len(&self) -> usize {
        self.size
    }

    fn as_bytes(&self) -> &[u8] {
        &self.body[..]
    }
}


// Syntax: 7.3.2.3 ( Page 69 )
// Semantics: 7.4.2.3 ( Page 105 )
// SEI Payloads: Annex D ( Page 350 )
// SEI
#[derive(Debug)]
pub struct SupplementalEnhancementInformation {
    messages: Vec<Box<dyn SeiMessage>>,
}

impl SupplementalEnhancementInformation {
    pub fn parse<R: Read, E: Endianness>(bitreader: &mut BitReader<R, E>) -> Result<Self, ()> {
        
        let mut messages: Vec<Box<dyn SeiMessage>> = vec![];

        loop {
            let mut payload_type = 0u32;
            let mut payload_size = 0usize;

            let mut ff_byte = bitreader.read::<u8>(8).unwrap();
            while ff_byte == 0xff {
                payload_type += 255;
                ff_byte = bitreader.read::<u8>(8).unwrap();
            }
            payload_type += ff_byte as u32;

            
            ff_byte = bitreader.read::<u8>(8).unwrap();
            while ff_byte == 0xff {
                payload_size += 255;
                ff_byte = bitreader.read::<u8>(8).unwrap();
            }

            payload_size += ff_byte as usize;

            let mut payload_body: Vec<u8> = vec![0u8; payload_size];

            bitreader.read_bytes(&mut payload_body).unwrap();

            let message = SeiAnyMessage {
                kind: SeiMessageKind::try_from(payload_type).unwrap(),
                size: payload_size,
                body: payload_body,
            };

            messages.push(Box::new(message));

            // more_rbsp_data()  msg[offset] != 0x80

        }
        
        Ok(SupplementalEnhancementInformation {
            messages: messages,
        })
    }
}

