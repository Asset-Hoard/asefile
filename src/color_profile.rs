use crate::{reader::AseReader, AsepriteParseError, Result};

#[derive(Debug)]
pub struct ColorProfile {
    pub profile_type: ColorProfileType,
    pub fixed_gamma: Option<f64>,
    pub icc_profile: Option<Vec<u8>>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorProfileType {
    None,
    Srgb,
    ICC,
}

pub(crate) fn parse_chunk(data: &[u8]) -> Result<ColorProfile> {
    let mut reader = AseReader::new(data);
    let profile_type = reader.word()?;
    let flags = reader.word()?;
    let raw_gamma = reader.dword()?;
    reader.skip_reserved(8)?;

    let profile_type = parse_color_profile_type(profile_type)?;
    let fixed_gamma = if flags & 1 != 0 {
        Some(raw_gamma as f64 / 65536.0)
    } else {
        None
    };

    let icc_profile = if profile_type == ColorProfileType::ICC {
        let icc_length = reader.dword()? as usize;
        Some(reader.take_bytes(icc_length)?)
    } else {
        None
    };

    Ok(ColorProfile {
        profile_type,
        fixed_gamma,
        icc_profile,
    })
}

fn parse_color_profile_type(id: u16) -> Result<ColorProfileType> {
    match id {
        0x0000 => Ok(ColorProfileType::None),
        0x0001 => Ok(ColorProfileType::Srgb),
        0x0002 => Ok(ColorProfileType::ICC),
        _ => Err(AsepriteParseError::UnsupportedFeature(format!(
            "Unknown color profile type: {}",
            id
        ))),
    }
}
