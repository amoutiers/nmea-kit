# nmea-kit

Bidirectional NMEA 0183 parser/encoder with AIS message decoding, written in Rust.

- **18 NMEA sentence types** — parse and encode with checksum
- **7 AIS message types** — decode Class A/B position reports and static data
- **Shared frame layer** — handles `$` (NMEA) and `!` (AIS) framing, IEC 61162-450 tag blocks
- **Zero dependencies**
- **No `nom`, no proc-macro** — `FieldReader`/`FieldWriter` helpers for clean sequential parsing

## Quick start

### Parse an NMEA sentence

```rust
use nmea_kit::{parse_frame, NmeaSentence};

let frame = parse_frame("$IIMWD,046.,T,046.,M,10.1,N,05.2,M*43").unwrap();
let sentence = NmeaSentence::parse(&frame);

match sentence {
    NmeaSentence::Mwd(mwd) => {
        println!("True wind dir: {:?}°", mwd.wind_dir_true);
        println!("Wind speed: {:?} kts", mwd.wind_speed_kts);
    }
    _ => {}
}
```

### Encode and send an NMEA sentence

```rust
use nmea_kit::nmea::sentences::Dbt;

let dbt = Dbt {
    depth_feet: Some(7.7),
    depth_meters: Some(2.3),
    depth_fathoms: Some(1.3),
};

let sentence = dbt.to_sentence("SD");
// "$SDDBT,7.7,f,2.3,M,1.3,F*05\r\n"
```

### Decode AIS messages

```rust
use nmea_kit::parse_frame;
use nmea_kit::ais::{AisParser, AisMessage};

let mut parser = AisParser::new();
let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").unwrap();

if let Some(AisMessage::Position(pos)) = parser.decode(&frame) {
    println!("MMSI: {}, lat: {:?}, lon: {:?}", pos.mmsi, pos.latitude, pos.longitude);
}
```

## Architecture

```
raw line ──→ parse_frame() ──→ NmeaFrame { prefix, talker, sentence_type, fields }
                                    |
                     +--------------+--------------+
                     v              v               v
               $ + known      $ + unknown     ! (AIVDM/AIVDO)
                     |              |               |
                     v              v               v
              Typed struct    Raw fields      AisMessage enum
              (Mwd, Apb..)   (pass-through)  (Types 1-5,18,19,24)
```

**Frame layer** validates checksum, strips tag blocks, extracts talker ID and sentence type. Shared by both NMEA and AIS.

**NMEA content** uses `FieldReader`/`FieldWriter` for sequential field parsing and encoding. Each sentence type is a standalone struct with `parse()`, `encode()`, and `to_sentence()`. Parsing is lenient: `parse()` always returns `Some` for known types, mapping missing or malformed fields to `None`. This is intentional for marine instruments that often produce partial data.

**AIS content** decodes 6-bit ASCII armor into a bitstream, handles multi-fragment reassembly, and extracts typed fields. Read-only (transmitting AIS requires certified hardware).

## Supported types

### NMEA 0183 sentences (bidirectional)

| Category | Sentences |
|----------|-----------|
| Position | RMC, GGA, GLL, GNS |
| Satellites | GBS, GST |
| Navigation | RMB |
| Wind | MWD, MWV |
| Heading | HDT, HDG, HDM, ROT |
| Speed | VTG, VHW |
| Depth | DPT, DBT, DBS |

### AIS messages (read-only)

| File | Type | Description |
|------|------|-------------|
| `position_a.rs` | 1, 2, 3 | Class A position report |
| `voyage_a.rs` | 5 | Static and voyage data |
| `position_b.rs` | 18 | Class B standard position |
| `position_b_ext.rs` | 19 | Class B+ extended position |
| `static_b.rs` | 24 | Class B static data report |

### Key improvements over existing crates

| Issue | `nmea` 0.7 / `ais` 0.12 | `nmea-kit` |
|-------|--------------------------|------------|
| Sentence coverage | ~10 types, rest manual | 18 types, all typed |
| Encoding | Read-only | Bidirectional (parse + encode) |
| Error distinction | Can't tell unsupported vs malformed | Frame errors vs content errors |
| AIS lat/lon precision | `f32` (11m error) | `f64` |
| AIS sentinels | 91/181/511 leak to caller | Filtered to `None` at decode |
| Tag blocks | Manual stripping | Built into frame layer |
| Dependencies | `nom` (AIS) | Zero |

## Features

```toml
[dependencies]
nmea-kit = "0.1"
```

| Feature | Default | Description |
|---------|---------|-------------|
| `nmea` | yes | All 18 NMEA sentence types |
| `ais` | yes | AIS message decoding |
| `dbs`, `dbt`, `dpt`, `gbs`, `gga`, `gll`, `gns`, `gst`, `hdg`, `hdm`, `hdt`, `mwd`, `mwv`, `rmb`, `rmc`, `rot`, `vhw`, `vtg` | via `nmea` | Individual sentence types |


Cherry-pick only the sentences you need (no AIS, minimal code):

```toml
[dependencies]
nmea-kit = { version = "0.1", default-features = false, features = ["rmc", "mwd"] }
```

NMEA-only (no AIS, all sentences, zero dependencies):

```toml
[dependencies]
nmea-kit = { version = "0.1", default-features = false, features = ["nmea"] }
```

## Adding a new sentence type

Create `src/nmea/sentences/xyz.rs`:

```rust
use crate::nmea::field::{FieldReader, FieldWriter};

#[derive(Debug, Clone, PartialEq)]
pub struct Xyz {
    pub some_value: Option<f32>,
    pub some_flag: Option<char>,
}

impl Xyz {
    pub const SENTENCE_TYPE: &str = "XYZ";

    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            some_value: r.f32(),
            some_flag: r.char(),
        })
    }

    pub fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.some_value);
        w.char(self.some_flag);
        w.finish()
    }

    pub fn to_sentence(&self, talker: &str) -> String {
        let fields = self.encode();
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        crate::encode_frame('$', talker, Self::SENTENCE_TYPE, &field_refs)
    }
}
```

Then add `mod xyz; pub use xyz::*;` to `sentences/mod.rs` and a variant to `NmeaSentence` in `nmea/mod.rs`.

## License

MIT OR Apache-2.0
