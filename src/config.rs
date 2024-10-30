use std::ffi::OsString;
use crate::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StreamConfig {
    input: StreamInput,
    output: StreamOutput,
}

#[derive(Deserialize, Debug)]
struct StreamInput {
    input: String,
    input_type: String,
}

#[derive(Deserialize, Debug)]
struct StreamOutput {
    output: String,
    channels: String,
    sample_rate: String,
    codec: String,
    compression_level: String,
    container: String,
    content_type: Option<String>,
}

impl TryFrom<&str> for StreamConfig {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(toml::from_str(value)?)
    }
}

impl StreamConfig {
    pub fn to_vec(self) -> Vec<OsString> {
        let mut value = vec![
            "-f".into(),
            self.input.input_type.into(),
            "-i".into(),
            self.input.input.into(),
            "-ac".into(),
            self.output.channels.into(),
            "-ar".into(),
            self.output.sample_rate.into(),
            "-c:a".into(),
            self.output.codec.into(),
            "-compression_level".into(),
            self.output.compression_level.into(),
        ];
        if self.output.content_type.is_some() {
            value.push("-content_type".into());
            value.push(self.output.content_type.unwrap().into())
        }
        
        value.push("-f".into());
        value.push(self.output.container.into());
        value.push(self.output.output.into());
        
        value
    }
}
