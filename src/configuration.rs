use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fmt::Debug;
use strum::{AsRefStr, EnumString};

// General Options
const FORMAT_OPT: &str = "-f";
const INPUT_OPT: &str = "-i";

// Audio Options
const CODEC_OPT: &str = "-codec:a";
const SAMPLE_RATE_OPT: &str = "-ar";
const CHANNELS_OPT: &str = "-ac";
const CHANNEL_LAYOUT_OPT: &str = "-channel_layout";
const SAMPLE_FORMAT: &str = "-sample_fmt";
const CONTENT_TYPE_OPT: &str = "-content_type";

// FLAC Options
const COMPRESSION_LEVEL_OPT: &str = "-compression_level";

// PICO CSS
const PICO_CSS_CDN_BASE: &str = "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/";

pub trait CommandConfig: Debug + Send {
    fn to_vec(&self) -> Vec<OsString>;
}

#[typetag::deserialize(tag = "codec")]
trait AudioCodec: CommandConfig + Send {}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum PicoCssColour {
    Amber,
    Blue,
    Cyan,
    Fuchsia,
    Green,
    Grey,
    Indigo,
    Jade,
    Lime,
    Orange,
    Pink,
    Pumpkin,
    Purple,
    Red,
    Sand,
    Slate,
    Violet,
    Yellow,
    Zinc,
}

impl PicoCssColour {
    pub fn get_pico_css_name(&self) -> String {
        format!("pico.classless.{}.min.css", self.as_ref())
    }
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    ffmpeg: StreamConfig,
    #[serde(default)]
    ui: UiConfig,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct UiConfig {
    port: u16,
    listen_address: String,
    pico_css_color: PicoCssColour,
}

#[derive(Deserialize, Debug)]
pub struct StreamConfig {
    general: GeneralConfig,
    input: StreamInput,
    output: StreamOutput,
}
#[derive(Deserialize, Debug, Default)]
struct GeneralConfig {
    overwrite: bool,
    hide_banner: bool,
}

#[derive(Deserialize, Debug)]
struct StreamInput {
    input: String,
    input_type: String,
    sample_rate: Option<u32>,
    channels: Option<u8>,
    channel_layout: Option<String>,
    //    sample_format: Option<String>,
    codec: Option<String>,
}

#[derive(Deserialize, Debug)]
struct StreamOutput {
    output: String,
    channels: Option<u8>,
    sample_rate: u32,
    sample_format: Option<String>,
    codec: Box<dyn AudioCodec + Send>,

    container: Option<String>,
    content_type: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Flac {
    compression_level: Option<u8>,
}

#[derive(Deserialize, Debug)]
struct PulseCodeModulation {
    encoder: String,
}

impl TryFrom<&str> for Configuration {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(toml::from_str(value)?)
    }
}

impl Configuration {
    pub fn ffmpeg(&self) -> &StreamConfig {
        &self.ffmpeg
    }

    pub fn ui(&self) -> &UiConfig {
        &self.ui
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            listen_address: "0.0.0.0".to_string(),
            pico_css_color: PicoCssColour::Indigo,
        }
    }
}

impl UiConfig {
    pub fn get_stylesheet_href(&self) -> String {
        format!(
            "{}{}",
            PICO_CSS_CDN_BASE,
            self.pico_css_color.get_pico_css_name()
        )
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn listen_address(&self) -> &str {
        &self.listen_address
    }
}

impl CommandConfig for GeneralConfig {
    fn to_vec(&self) -> Vec<OsString> {
        let overwrite = match self.overwrite {
            true => "-y",
            false => "-n",
        };
        let mut general = vec![overwrite.into()];
        if self.hide_banner {
            general.push("-hide_banner".into());
        }
        general
    }
}

impl CommandConfig for StreamConfig {
    fn to_vec(&self) -> Vec<OsString> {
        let mut config = self.general.to_vec();
        config.extend(self.input.to_vec());
        config.extend(self.output.to_vec());
        config
    }
}

impl CommandConfig for StreamInput {
    fn to_vec(&self) -> Vec<OsString> {
        let mut input = Vec::new();

        if let Some(channels) = self.channels {
            input.push(CHANNELS_OPT.into());
            input.push(channels.to_string().into());
        }

        if let Some(layout) = &self.channel_layout {
            input.push(CHANNEL_LAYOUT_OPT.into());
            input.push(layout.into());
        }

        if let Some(codec) = &self.codec {
            input.push(CODEC_OPT.into());
            input.push(codec.into());
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

        if let Some(sample_format) = &self.sample_format {
            value.push(SAMPLE_FORMAT.into());
            value.push(sample_format.as_str().into());
        }

        if let Some(channels) = self.channels {
            value.push(CHANNELS_OPT.into());
            value.push(channels.to_string().into());
        }

        value.extend(self.codec.to_vec());

        if let Some(content_type) = &self.content_type {
            value.push(CONTENT_TYPE_OPT.into());
            value.push(OsString::from(content_type));
        }

        if let Some(container) = &self.container {
            value.push(FORMAT_OPT.into());
            value.push(container.as_str().into());
        }

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

#[typetag::deserialize(name = "pcm")]
impl AudioCodec for PulseCodeModulation {}

impl CommandConfig for PulseCodeModulation {
    fn to_vec(&self) -> Vec<OsString> {
        vec![CODEC_OPT.into(), format!("pcm_{}", &self.encoder).into()]
    }
}
