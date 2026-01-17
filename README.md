# FFMPEG Stream Audio
Configuration wrapper around FFMPEG to steam audio

## Configuration

### UI

| Option         | Type   | Description                             |
|----------------|--------|-----------------------------------------|
| port           | u16    | Port the UI webserver will listen on    |
| listen_address | String | Address the UI webserver with listen on |
| pico_css_color | String | Colour PicoCSS will use for the UI      |


### FFMPEG
#### General

| Option      | Type    | Description        |
|-------------|---------|--------------------|
| hide_banner | boolean | Hide FFMPEG banner |
| overwrite   | boolean | Overwrite output   |


#### Input

| Option         | Type   | Description                                 |
|----------------|--------|---------------------------------------------|
| input          | String | Input file or device                        |
| input_type     | String | Input type ie alsa                          |
| sample_rate    | u32    | Input sample frequency                      |
| channels       | u8     | Number of input channels                    |
| channel_layout | String | Channel layout ie. mono, stereo             |


#### Output

| Option        | Type   | Description                       |
|---------------|--------|-----------------------------------|
| channels      | String | Number of output channels         |
| container     | String | Container type ie. ogg            |
| output        | String | Output file or destination        |
| sample_rate   | u32    | Output sample frequency           |
| sample_format | String | Output sample format ie. s16, s32 |
| content_type  | String | Content type ie. application/ogg  |

### Example Configuration
```toml
[ui]
port = 8080
listen_address = "0.0.0.0"
pico_css_color = "Indigo"

[ffmpeg.general]
hide_banner = true
overwrite = true

[ffmpeg.input]
input = "hw:2,0"
input_type = "alsa"

[ffmpeg.output]
output = "/path/to/output.flac"
channels = 2
sample_rate = 48000
container = "ogg"

[ffmpeg.output.codec]
codec = "flac"
compression_level = 2
```
