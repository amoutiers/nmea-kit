# nmea-kit

Bidirectional NMEA 0183 parser/encoder with AIS message decoding, written in Rust.

- **25 NMEA sentence types** — parse and encode with checksum
- **9 AIS message types** — decode Class A/B position, base station, safety broadcasts, AtoN, long range
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
use nmea_kit::nmea::NmeaEncodable;
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
              (Mwd, Apb..)   (pass-through)  (Types 1-5,14,18,19,21,24,27)
```

**Frame layer** validates checksum, strips tag blocks, extracts talker ID and sentence type. Shared by both NMEA and AIS.

**NMEA content** uses `FieldReader`/`FieldWriter` for sequential field parsing and encoding. Each sentence type is a standalone struct with `parse()`, `encode()`, and `to_sentence()`. Parsing is lenient: `parse()` always returns `Some` for known types, mapping missing or malformed fields to `None`. This is intentional for marine instruments that often produce partial data.

**AIS content** decodes 6-bit ASCII armor into a bitstream, handles multi-fragment reassembly, and extracts typed fields. Read-only (transmitting AIS requires certified hardware).

## Supported types

### NMEA 0183 sentences (bidirectional) — [full coverage list](SENTENCES.md)

| Category | Sentences |
|----------|-----------|
| Position | RMC, GGA, GLL, GNS |
| Satellites | GBS, GST |
| Wind | MWD, MWV |
| Heading | HDT, HDG, HDM |
| Course & Speed | VBW, VLW, VTG, VHW |
| Depth | DPT, DBT, DBS, DBK |
| Steering | ROT, RSA |
| Environment | MTW, XDR¹ |
| Waypoints & Routes | RMB |
| Time | ZDA |

¹ `Xdr` has an additional `to_sentences() -> Vec<String>` method that automatically splits many measurements into multiple sentences to stay within the 82-character NMEA line limit.

### AIS messages (read-only) — [full type list](SENTENCES.md#ais-message-types-decoded-from-aivdmaivdo)

| Type(s) | Struct | Description |
|---------|--------|-------------|
| 1, 2, 3 | `PositionReport` | Class A position report |
| 4 | `BaseStationReport` | Base station UTC + position |
| 5 | `StaticVoyageData` | Static and voyage data (Class A) |
| 14 | `SafetyBroadcast` | Safety-related broadcast message |
| 18 | `PositionReport` | Class B standard position |
| 19 | `PositionReport` | Class B+ extended position |
| 21 | `AidToNavigation` | Aid-to-navigation report (buoys, beacons) |
| 24 | `StaticDataReport` | Static data report (Class B) |
| 27 | `LongRangePosition` | Long range position (satellite AIS, 1/10° precision) |

### Key improvements over existing crates

| Issue | `nmea` 0.7 / `ais` 0.12 | `nmea-kit` |
|-------|--------------------------|------------|
| NMEA sentence coverage | ~10 types, rest manual | 25 types, all typed |
| AIS message coverage | ~5 types | 9 types (1-5, 14, 18, 19, 21, 24, 27) |
| Encoding | Read-only | Bidirectional (parse + encode) |
| Error distinction | Can't tell unsupported vs malformed | Frame errors vs content errors |
| AIS lat/lon precision | `f32` (11m error) | `f64` |
| AIS sentinels | 91/181/511 leak to caller | Filtered to `None` at decode |
| Tag blocks | Manual stripping | Built into frame layer |
| Dependencies | `nom` (AIS) | Zero |

## Features

```toml
[dependencies]
nmea-kit = "0.2"
```

| Feature | Default | Enables |
|---------|---------|---------|
| `nmea` | yes | All 25 NMEA sentence types |
| `ais` | yes | AIS message decoding |
| `positioning` | via `nmea` | GGA, GLL, RMC, GNS |
| `speed` | via `nmea` | VTG, VHW, VBW, RMC |
| `heading` | via `nmea` | HDG, HDM, HDT |
| `wind` | via `nmea` | MWD, MWV |
| `depth` | via `nmea` | DBT, DBS, DBK, DPT |
| `dbk`, `dbs`, `dbt`, `dpt`, `gbs`, `gga`, `gll`, `gns`, `gst`, `hdg`, `hdm`, `hdt`, `mtw`, `mwd`, `mwv`, `rmb`, `rmc`, `rot`, `rsa`, `vbw`, `vhw`, `vlw`, `vtg`, `xdr`, `zda` | via `nmea` | Individual sentence types |

Use a group feature for common use cases:

```toml
# Only positioning sentences (GGA, GLL, RMC, GNS), no AIS
nmea-kit = { version = "0.2", default-features = false, features = ["positioning"] }
```

Cherry-pick individual sentences you need:

```toml
nmea-kit = { version = "0.2", default-features = false, features = ["rmc", "mwd"] }
```

NMEA-only (no AIS, all sentences):

```toml
nmea-kit = { version = "0.2", default-features = false, features = ["nmea"] }
```

## Adding a new sentence type

Create `src/nmea/sentences/xyz.rs`:

```rust
use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Xyz {
    pub some_value: Option<f32>,
    pub some_flag: Option<char>,
}

impl Xyz {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            some_value: r.f32(),
            some_flag: r.char(),
        })
    }
}

impl NmeaEncodable for Xyz {
    const SENTENCE_TYPE: &str = "XYZ";

    fn encode(&self) -> Vec<String> {
        let mut w = FieldWriter::new();
        w.f32(self.some_value);
        w.char(self.some_flag);
        w.finish()
    }
}
```

Then add `mod xyz; pub use xyz::*;` (feature-gated) to `sentences/mod.rs`, one line to the `nmea_sentences!` invocation in `nmea/mod.rs`, and `xyz = []` to `[features]` in `Cargo.toml`.

## License

MIT OR Apache-2.0
