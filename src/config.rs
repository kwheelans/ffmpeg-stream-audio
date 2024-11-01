use crate::error::Error;
use serde::Deserialize;
use std::ffi::OsString;
use std::fmt::Debug;

// General Options
const FORMAT_OPT: &str = "-f";
const INPUT_OPT: &str = "-i";

// Audio Options
const CODEC_OPT: &str = "-codec:a";
const SAMPLE_RATE_OPT: &str = "-ar";
const CHANNELS_OPT: &str = "-ac";
const SAMPLE_FORMAT: &str = "-sample_fmt";
const CONTENT_TYPE_OPT: &str = "-content_type";

// FLAC Options
const COMPRESSION_LEVEL_OPT: &str = "-compression_level";

pub trait CommandConfig: Debug {
    fn to_vec(&self) -> Vec<OsString>;
}

#[typetag::deserialize(tag = "codec")]
trait AudioCodec: CommandConfig {}

#[derive(Deserialize, Debug)]
pub struct StreamConfig {
    input: StreamInput,
    output: StreamOutput,
}

#[derive(Deserialize, Debug)]
struct StreamInput {
    input: String,
    input_type: String,
    #[serde(default)]
    overwrite: bool,
    sample_rate: Option<u32>,
    sample_format: Option<String>,
    codec: Option<String>,
}

#[derive(Deserialize, Debug)]
struct StreamOutput {
    output: String,
    channels: Option<String>,
    sample_rate: u32,
    codec: Box<dyn AudioCodec>,

    container: String,
    content_type: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Flac {
    compression_level: Option<u8>,
}

impl TryFrom<&str> for StreamConfig {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(toml::from_str(value)?)
    }
}

impl CommandConfig for StreamConfig {
    fn to_vec(&self) -> Vec<OsString> {
        let mut value = self.input.to_vec();
        value.extend(self.output.to_vec());
        value
    }
}

impl CommandConfig for StreamInput {
    fn to_vec(&self) -> Vec<OsString> {
        let overwrite = match self.overwrite {
            true => "-y",
            false => "-n",
        };
        let mut input = vec!["-hide_banner".into(), overwrite.into()];

        if let Some(codec) = &self.codec {
            input.push(CODEC_OPT.into());
            input.push(codec.into());
        }

        if let Some(sample_format) = &self.sample_format {
            input.push(SAMPLE_FORMAT.into());
            input.push(sample_format.as_str().into());
        }

        input.push(FORMAT_OPT.into());
        input.push(self.input_type.as_str().into());

        if let Some(sample_rate) = self.sample_rate {
            input.push(SAMPLE_RATE_OPT.into());
            input.push(sample_rate.to_string().into());
        }

        input.push(INPUT_OPT.into());
        input.push(self.input.as_str().into());

        input
    }
}

impl CommandConfig for StreamOutput {
    fn to_vec(&self) -> Vec<OsString> {
        let mut value = vec![SAMPLE_RATE_OPT.into(), self.sample_rate.to_string().into()];

        if let Some(channels) = &self.channels {
            value.push(CHANNELS_OPT.into());
            value.push(channels.into());
        }
        
        value.extend(self.codec.to_vec());

        if let Some(content_type) = &self.content_type {
            value.push(CONTENT_TYPE_OPT.into());
            value.push(OsString::from(content_type));
        }

        value.push(FORMAT_OPT.into());
        value.push(self.container.as_str().into());

        value.push(self.output.as_str().into());
        value
    }
}

#[typetag::deserialize(name = "flac")]
impl AudioCodec for Flac {}

impl CommandConfig for Flac {
    fn to_vec(&self) -> Vec<OsString> {
        let mut codec = vec![CODEC_OPT.into(), "flac".into()];

        if let Some(compression) = self.compression_level {
            codec.push(COMPRESSION_LEVEL_OPT.into());
            codec.push(compression.to_string().into())
        }

        codec
    }
}
