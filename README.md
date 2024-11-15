# FFMPEG Stream Audio



## Configuration

### General

| Option      | Type    | Description        |
|-------------|---------|--------------------|
| hide_banner | boolean | Hide FFMPEG banner |
| overwrite   | boolean | Overwrite output   |


### Input

| Option     | Type   | Description |
|------------|--------|-------------|
| input      | String |             |
| input_type | String |             |



### Output

| Option      | Type   | Description |
|-------------|--------|-------------|
| channels    | String |             |
| container   | String |             |
| output      | String |             |
| sample_rate | u32    |             |

### Example Configuration
```toml
[general]
hide_banner = true
overwrite = true

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
