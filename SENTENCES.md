# NMEA 0183 sentence coverage

Non-obsolete sentences from the [NMEA 0183 standard](https://gpsd.gitlab.io/gpsd/NMEA.html). Checked = supported by nmea-kit.

## Position & Navigation

- [ ] DTM — Datum Reference
- [x] GGA — Global Positioning System Fix Data
- [x] GLL — Geographic Position, Latitude/Longitude
- [x] GNS — GNSS Fix Data
- [ ] GTD — Geographic Location in Time Differences
- [ ] RMA — Recommended Minimum Navigation Information
- [ ] RMB — Recommended Minimum Navigation Information (to waypoint)
- [x] RMC — Recommended Minimum Navigation Information

## Satellites

- [ ] GBS — GPS Satellite Fault Detection
- [ ] GRS — GPS Range Residuals
- [ ] GSA — GPS DOP and Active Satellites
- [ ] GST — GPS Pseudorange Noise Statistics
- [ ] GSV — Satellites in View

## Wind

- [x] MWD — Wind Direction & Speed
- [x] MWV — Wind Speed and Angle
- [ ] VWR — Relative Wind Speed and Angle

## Heading

- [x] HDG — Heading, Deviation & Variation
- [x] HDM — Heading, Magnetic
- [x] HDT — Heading, True

## Speed

- [ ] OSD — Own Ship Data
- [ ] VBW — Dual Ground/Water Speed
- [x] VHW — Water Speed and Heading
- [ ] VLW — Distance Traveled through Water
- [ ] VPW — Speed, Measured Parallel to Wind
- [x] VTG — Track Made Good and Ground Speed

## Depth

- [ ] DBK — Depth Below Keel
- [x] DBS — Depth Below Surface
- [x] DBT — Depth Below Transducer
- [x] DPT — Depth of Water

## Autopilot & Steering

- [ ] APA — Autopilot Sentence "A"
- [ ] APB — Autopilot Sentence "B"
- [ ] HSC — Heading Steering Command
- [ ] ROT — Rate of Turn
- [ ] RSA — Rudder Sensor Angle

## Waypoints & Routes

- [ ] AAM — Waypoint Arrival Alarm
- [ ] BWC — Bearing & Distance to Waypoint, Great Circle
- [ ] BWR — Bearing & Distance to Waypoint, Rhumb Line
- [ ] BWW — Bearing, Waypoint to Waypoint
- [ ] R00 — Waypoints in Active Route
- [ ] RTE — Routes
- [ ] WCV — Waypoint Closure Velocity
- [ ] WNC — Distance, Waypoint to Waypoint
- [ ] WPL — Waypoint Location
- [ ] XTE — Cross-Track Error, Measured
- [ ] XTR — Cross-Track Error, Dead Reckoning
- [ ] ZFO — UTC & Time from Origin Waypoint
- [ ] ZTG — UTC & Time to Destination Waypoint

## Environment

- [ ] MTW — Mean Temperature of Water
- [ ] XDR — Transducer Measurement

## Time

- [ ] ZDA — Time & Date

## Radar & Targets

- [ ] RSD — Radar System Data
- [ ] TLB — Target Label
- [ ] TLL — Target Latitude and Longitude
- [ ] TTM — Tracked Target Message

## Communication

- [ ] ALM — GPS Almanac Data
- [ ] FSI — Frequency Set Information
- [ ] MSK — Control for a Beacon Receiver
- [ ] MSS — Beacon Receiver Status
- [ ] RLM — Return Link Message
- [ ] SFI — Scanning Frequency Information
- [ ] STN — Multiple Data ID

## Engine

- [ ] RPM — Revolutions

## Drift

- [ ] VDR — Set and Drift

## Trawl / Fishing

- [ ] HFB — Trawl Headrope to Footrope and Bottom
- [ ] ITS — Trawl Door Spread 2 Distance
- [ ] TDS — Trawl Door Spread Distance
- [ ] TFI — Trawl Filling Indicator
- [ ] TPC — Trawl Position Cartesian Coordinates
- [ ] TPR — Trawl Position Relative Vessel
- [ ] TPT — Trawl Position True
