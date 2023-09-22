use wgpu::{CompareFunction, TextureFormat, AstcBlock, AstcChannel, StencilOperation, StencilFaceState, ColorWrites, BlendState, BlendComponent, BlendFactor, BlendOperation};

use crate::renderer::{GpuCompareFunction, GpuTextureFormat, GpuStencilOperation, GpuStencilFaceState, GpuColorWrite, GpuBlendState, GpuBlendComponent, GpuBlendFactor, GpuBlendOperation};

impl Into<CompareFunction> for GpuCompareFunction {
    fn into(self) -> CompareFunction {
        match self {
            GpuCompareFunction::Never => CompareFunction::Never,
            GpuCompareFunction::Less => CompareFunction::Less,
            GpuCompareFunction::Equal => CompareFunction::Equal,
            GpuCompareFunction::LessEqual => CompareFunction::LessEqual,
            GpuCompareFunction::Greater => CompareFunction::Greater,
            GpuCompareFunction::NotEqual => CompareFunction::NotEqual,
            GpuCompareFunction::GreaterEqual => CompareFunction::GreaterEqual,
            GpuCompareFunction::Always => CompareFunction::Always,
        }
    }
}

impl Into<TextureFormat> for GpuTextureFormat {
    fn into(self) -> TextureFormat {
        match self {
            GpuTextureFormat::R8unorm => TextureFormat::R8Unorm,
            GpuTextureFormat::R8snorm => TextureFormat::R8Snorm,
            GpuTextureFormat::R8uint => TextureFormat::R8Uint,
            GpuTextureFormat::R8sint => TextureFormat::R8Sint,
            GpuTextureFormat::R16uint => TextureFormat::R16Uint,
            GpuTextureFormat::R16sint => TextureFormat::R16Sint,
            GpuTextureFormat::R16float => TextureFormat::R16Float,
            GpuTextureFormat::Rg8unorm => TextureFormat::Rg8Unorm,
            GpuTextureFormat::Rg8snorm => TextureFormat::Rg8Snorm,
            GpuTextureFormat::Rg8uint => TextureFormat::Rg8Uint,
            GpuTextureFormat::Rg8sint => TextureFormat::Rg8Sint,
            GpuTextureFormat::R32uint => TextureFormat::R32Uint,
            GpuTextureFormat::R32sint => TextureFormat::R32Sint,
            GpuTextureFormat::R32float => TextureFormat::R32Float,
            GpuTextureFormat::Rg16uint => TextureFormat::Rg16Uint,
            GpuTextureFormat::Rg16sint => TextureFormat::Rg16Sint,
            GpuTextureFormat::Rg16float => TextureFormat::Rg16Float,
            GpuTextureFormat::Rgba8unorm => TextureFormat::Rgba8Unorm,
            GpuTextureFormat::Rgba8unormsrgb => TextureFormat::Rgba8UnormSrgb,
            GpuTextureFormat::Rgba8snorm => TextureFormat::Rgba8Snorm,
            GpuTextureFormat::Rgba8uint => TextureFormat::Rgba8Uint,
            GpuTextureFormat::Rgba8sint => TextureFormat::Rgba8Sint,
            GpuTextureFormat::Bgra8unorm => TextureFormat::Bgra8Unorm,
            GpuTextureFormat::Bgra8unormsrgb => TextureFormat::Bgra8UnormSrgb,
            GpuTextureFormat::Rgb9e5ufloat => TextureFormat::Rgb9e5Ufloat,
            GpuTextureFormat::Rgb10a2uint => panic!("Unsupported Texture Format"),
            GpuTextureFormat::Rgb10a2unorm => TextureFormat::Rgb10a2Unorm,
            GpuTextureFormat::Rg11b10ufloat => TextureFormat::Rg11b10Float,
            GpuTextureFormat::Rg32uint => TextureFormat::Rg32Uint,
            GpuTextureFormat::Rg32sint => TextureFormat::Rg32Sint,
            GpuTextureFormat::Rg32float => TextureFormat::Rg32Float,
            GpuTextureFormat::Rgba16uint => TextureFormat::Rgba16Uint,
            GpuTextureFormat::Rgba16sint => TextureFormat::Rgba16Sint,
            GpuTextureFormat::Rgba16float => TextureFormat::Rgba16Float,
            GpuTextureFormat::Rgba32uint => TextureFormat::Rgba32Uint,
            GpuTextureFormat::Rgba32sint => TextureFormat::Rgba32Sint,
            GpuTextureFormat::Rgba32float => TextureFormat::Rgba32Float,
            GpuTextureFormat::Stencil8 => TextureFormat::Stencil8,
            GpuTextureFormat::Depth16unorm => TextureFormat::Depth16Unorm,
            GpuTextureFormat::Depth24plus => TextureFormat::Depth24Plus,
            GpuTextureFormat::Depth24plusstencil8 => TextureFormat::Depth24PlusStencil8,
            GpuTextureFormat::Depth32float => TextureFormat::Depth32Float,
            GpuTextureFormat::Depth32floatstencil8 => TextureFormat::Depth32FloatStencil8,
            GpuTextureFormat::Bc1rgbaunorm => TextureFormat::Bc1RgbaUnorm,
            GpuTextureFormat::Bc1rgbaunormsrgb => TextureFormat::Bc1RgbaUnormSrgb,
            GpuTextureFormat::Bc2rgbaunorm => TextureFormat::Bc2RgbaUnorm,
            GpuTextureFormat::Bc2rgbaunormsrgb => TextureFormat::Bc2RgbaUnormSrgb,
            GpuTextureFormat::Bc3rgbaunorm => TextureFormat::Bc3RgbaUnorm,
            GpuTextureFormat::Bc3rgbaunormsrgb => TextureFormat::Bc3RgbaUnormSrgb,
            GpuTextureFormat::Bc4runorm => TextureFormat::Bc4RUnorm,
            GpuTextureFormat::Bc4rsnorm => TextureFormat::Bc4RSnorm,
            GpuTextureFormat::Bc5rgunorm => TextureFormat::Bc5RgUnorm,
            GpuTextureFormat::Bc5rgsnorm => TextureFormat::Bc5RgSnorm,
            GpuTextureFormat::Bc6hrgbufloat => TextureFormat::Bc6hRgbUfloat,
            GpuTextureFormat::Bc6hrgbfloat => TextureFormat::Bc6hRgbFloat,
            GpuTextureFormat::Bc7rgbaunorm => TextureFormat::Bc7RgbaUnorm,
            GpuTextureFormat::Bc7rgbaunormsrgb => TextureFormat::Bc7RgbaUnormSrgb,
            GpuTextureFormat::Etc2rgb8unorm => TextureFormat::Etc2Rgb8Unorm,
            GpuTextureFormat::Etc2rgb8unormsrgb => TextureFormat::Etc2Rgb8UnormSrgb,
            GpuTextureFormat::Etc2rgb8a1unorm => TextureFormat::Etc2Rgb8A1Unorm,
            GpuTextureFormat::Etc2rgb8a1unormsrgb => TextureFormat::Etc2Rgb8A1UnormSrgb,
            GpuTextureFormat::Etc2rgba8unorm => TextureFormat::Etc2Rgba8Unorm,
            GpuTextureFormat::Etc2rgba8unormsrgb => TextureFormat::Etc2Rgba8UnormSrgb,
            GpuTextureFormat::Eacr11unorm => TextureFormat::EacR11Unorm,
            GpuTextureFormat::Eacr11snorm => TextureFormat::EacR11Snorm,
            GpuTextureFormat::Eacrg11unorm => TextureFormat::EacRg11Unorm,
            GpuTextureFormat::Eacrg11snorm => TextureFormat::EacRg11Snorm,
            GpuTextureFormat::Astc4x4unorm => TextureFormat::Astc {
                block: AstcBlock::B4x4,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc4x4unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B4x4,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc5x4unorm => TextureFormat::Astc {
                block: AstcBlock::B5x4,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc5x4unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B5x4,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc5x5unorm => TextureFormat::Astc {
                block: AstcBlock::B5x5,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc5x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B5x5,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc6x5unorm => TextureFormat::Astc {
                block: AstcBlock::B6x5,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc6x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B6x5,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc6x6unorm => TextureFormat::Astc {
                block: AstcBlock::B6x6,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc6x6unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B6x6,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc8x5unorm => TextureFormat::Astc {
                block: AstcBlock::B8x5,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc8x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B8x5,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc8x6unorm => TextureFormat::Astc {
                block: AstcBlock::B8x6,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc8x6unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B8x6,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc8x8unorm => TextureFormat::Astc {
                block: AstcBlock::B8x8,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc8x8unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B8x8,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc10x5unorm => TextureFormat::Astc {
                block: AstcBlock::B10x5,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc10x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x5,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc10x6unorm => TextureFormat::Astc {
                block: AstcBlock::B10x6,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc10x6unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x6,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc10x8unorm => TextureFormat::Astc {
                block: AstcBlock::B10x8,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc10x8unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x8,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc10x10unorm => TextureFormat::Astc {
                block: AstcBlock::B10x10,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc10x10unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x10,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc12x10unorm => TextureFormat::Astc {
                block: AstcBlock::B12x10,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc12x10unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B12x10,
                channel: AstcChannel::UnormSrgb
            },
            GpuTextureFormat::Astc12x12unorm => TextureFormat::Astc {
                block: AstcBlock::B12x12,
                channel: AstcChannel::Unorm
            },
            GpuTextureFormat::Astc12x12unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B12x12,
                channel: AstcChannel::UnormSrgb
            },
        }
    }
}

impl Into<StencilOperation> for GpuStencilOperation {
    fn into(self) -> StencilOperation {
        match self {
            GpuStencilOperation::Keep => StencilOperation::Keep,
            GpuStencilOperation::Zero => StencilOperation::Zero,
            GpuStencilOperation::Replace => StencilOperation::Replace,
            GpuStencilOperation::Invert => StencilOperation::Invert,
            GpuStencilOperation::IncrementClamp => StencilOperation::IncrementClamp,
            GpuStencilOperation::DecrementClamp => StencilOperation::DecrementClamp,
            GpuStencilOperation::IncrementWrap => StencilOperation::IncrementWrap,
            GpuStencilOperation::DecrementWrap => StencilOperation::DecrementWrap
        }
    }
}

impl Into<StencilFaceState> for GpuStencilFaceState {
    fn into(self) -> StencilFaceState {
        StencilFaceState {
            compare: match self.compare {
                None => CompareFunction::Always,
                Some(compare) => compare.into()
            },
            fail_op: match self.fail_op {
                None => StencilOperation::default(),
                Some(fail_op) => fail_op.into()
            },
            depth_fail_op: match self.depth_fail_op {
                None => StencilOperation::default(),
                Some(depth_fail_op) => depth_fail_op.into()
            },
            pass_op: match self.pass_op {
                None => StencilOperation::default(),
                Some(pass_op) => pass_op.into()
            },
        }
    }
}

impl Into<BlendState> for GpuBlendState {
    fn into(self) -> BlendState {
        BlendState {
            color: self.color.into(),
            alpha: self.alpha.into(),
        }
    }
}

impl Into<BlendComponent> for GpuBlendComponent {
    fn into(self) -> BlendComponent {
        BlendComponent {
            src_factor: match self.src_factor {
                None => BlendFactor::One,
                Some(src_factor) => src_factor.into()
            },
            dst_factor: match self.dst_factor {
                None => BlendFactor::Zero,
                Some(src_factor) => src_factor.into()
            },
            operation: match self.operation {
                None => BlendOperation::Add,
                Some(operation) => operation.into()
            },
        }
    }
}

impl Into<BlendFactor> for GpuBlendFactor {
    fn into(self) -> BlendFactor {
        match self {
            GpuBlendFactor::Zero => BlendFactor::Zero,
            GpuBlendFactor::One => BlendFactor::One,
            GpuBlendFactor::Src => BlendFactor::Src,
            GpuBlendFactor::OneMinusSrc => BlendFactor::OneMinusSrc,
            GpuBlendFactor::SrcAlpha => BlendFactor::SrcAlpha,
            GpuBlendFactor::OneMinusSrcAlpha => BlendFactor::OneMinusSrcAlpha,
            GpuBlendFactor::Dst => BlendFactor::Dst,
            GpuBlendFactor::OneMinusDst => BlendFactor::OneMinusDst,
            GpuBlendFactor::DstAlpha => BlendFactor::DstAlpha,
            GpuBlendFactor::OneMinusDstAlpha => BlendFactor::OneMinusDstAlpha,
            GpuBlendFactor::SrcAlphaSaturated => BlendFactor::SrcAlphaSaturated,
            GpuBlendFactor::Constant => BlendFactor::Constant,
            GpuBlendFactor::OneMinusConstant => BlendFactor::OneMinusConstant,
        }
    }
}

impl Into<BlendOperation> for GpuBlendOperation {
    fn into(self) -> BlendOperation {
        match self {
            GpuBlendOperation::Add => BlendOperation::Add,
            GpuBlendOperation::Subtract => BlendOperation::Subtract,
            GpuBlendOperation::ReverseSubtract => BlendOperation::ReverseSubtract,
            GpuBlendOperation::Min => BlendOperation::Min,
            GpuBlendOperation::Max => BlendOperation::Max,
        }
    }
}

impl Into<ColorWrites> for GpuColorWrite {
    fn into(self) -> ColorWrites {
        unsafe { std::mem::transmute(self.as_array()[0]) }
    }
}
