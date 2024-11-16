# FFMPEG Stream Audio



## Configuration

### General

| Option      | Type    | Description        |
|-------------|---------|--------------------|
| hide_banner | boolean | Hide FFMPEG banner |
| overwrite   | boolean | Overwrite output   |


### Input

| Option         | Type   | Description                                 |
|----------------|--------|---------------------------------------------|
| input          | String | Input file or device                        |
| input_type     | String | Input type ie alsa                          |
| sample_rate    | u32    | Input sample frequency                      |
| sample_format  | String | Input sample format ie. s16le, u16le, f32le |
| channels       | u8     | Number of input channels                    |
| channel_layout | String | Channel layout ie. mono, stereo             |



### Output

| Option       | Type   | Description                      |
|--------------|--------|----------------------------------|
| channels     | String | Number of output channels        |
| container    | String | Container type ie. ogg           |
| output       | String | Output file or destination       |
| sample_rate  | u32    | Output sample frequency          |
| content_type | String | Content type ie. application/ogg |

### Example Configuration
```toml
[general]
hide_banner = true
overwrite = true
log_level = "info"

[input]
input = "hw:2,0"
input_type = "alsa"

[output]
output = "/path/to/output.flac"
channels = 2
sample_rate = 48000
container = "ogg"

[output.codec]
codec = "flac"
compression_level = 2
```
