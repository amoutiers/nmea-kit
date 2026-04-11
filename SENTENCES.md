# NMEA 0183 sentence coverage

Sentences from the [NMEA 0183 standard](https://gpsd.gitlab.io/gpsd/NMEA.html) and [go-nmea](https://github.com/adrianmo/go-nmea). Checked = supported by nmea-kit.

## Position

- [ ] DTM — Datum Reference
- [x] GGA — Global Positioning System Fix Data
- [x] GLL — Geographic Position, Latitude/Longitude
- [x] GNS — GNSS Fix Data
- [ ] GTD — Geographic Location in Time Differences
- [ ] RMA — Recommended Minimum Navigation Information
- [x] RMC — Recommended Minimum Navigation Information

## Satellites

- [x] GBS — GPS Satellite Fault Detection
- [ ] GRS — GPS Range Residuals
- [ ] GSA — GPS DOP and Active Satellites
- [x] GST — GPS Pseudorange Noise Statistics
- [ ] GSV — Satellites in View

## Heading

- [x] HDG — Heading, Deviation & Variation
- [x] HDM — Heading, Magnetic
- [x] HDT — Heading, True
- [x] THS — True Heading and Status

## Course & Speed

- [ ] OSD — Own Ship Data
- [x] VBW — Dual Ground/Water Speed
- [x] VHW — Water Speed and Heading
- [x] VLW — Distance Traveled through Water
- [ ] VPW — Speed, Measured Parallel to Wind
- [x] VTG — Track Made Good and Ground Speed

## Wind

- [x] MWD — Wind Direction & Speed
- [x] MWV — Wind Speed and Angle
- [ ] VWR — Relative Wind Speed and Angle
- [ ] VWT — True Wind Speed and Angle

## Depth

- [x] DBK — Depth Below Keel
- [x] DBS — Depth Below Surface
- [x] DBT — Depth Below Transducer
- [x] DPT — Depth of Water

## Steering

- [ ] APA — Autopilot Sentence "A"
- [ ] APB — Autopilot Sentence "B"
- [ ] HSC — Heading Steering Command
- [x] ROT — Rate of Turn
- [x] RSA — Rudder Sensor Angle

## Waypoints & Routes

- [ ] AAM — Waypoint Arrival Alarm
- [ ] BEC — Bearing & Distance to Waypoint, Dead Reckoning
- [ ] BOD — Bearing, Waypoint to Waypoint
- [ ] BWC — Bearing & Distance to Waypoint, Great Circle
- [ ] BWR — Bearing & Distance to Waypoint, Rhumb Line
- [ ] BWW — Bearing, Waypoint to Waypoint
- [ ] R00 — Waypoints in Active Route
- [x] RMB — Recommended Minimum Navigation Information (to waypoint)
- [ ] RTE — Routes
- [ ] WCV — Waypoint Closure Velocity
- [ ] WNC — Distance, Waypoint to Waypoint
- [ ] WPL — Waypoint Location
- [ ] XTE — Cross-Track Error, Measured
- [ ] XTR — Cross-Track Error, Dead Reckoning
- [ ] ZFO — UTC & Time from Origin Waypoint
- [ ] ZTG — UTC & Time to Destination Waypoint

## Environment

- [ ] MDA — Meteorological Composite
- [ ] MTA — Air Temperature
- [x] MTW — Mean Temperature of Water
- [x] XDR — Transducer Measurement

## Time

- [x] ZDA — Time & Date

## Targets

- [ ] RSD — Radar System Data
- [ ] TLB — Target Label
- [ ] TLL — Target Latitude and Longitude
- [ ] TTD — Tracked Target Data
- [ ] TTM — Tracked Target Message

## AIS Sentences

- [ ] ABM — Addressed Binary Message
- [ ] BBM — Broadcast Binary Message
- [ ] VSD — AIS Voyage Static Data

## AIS Message Types (decoded from AIVDM/AIVDO)

- [x] Type 1 — Class A Position Report (under way, engine)
- [x] Type 2 — Class A Position Report (at anchor)
- [x] Type 3 — Class A Position Report (special maneuver)
- [x] Type 4 — Base Station Report (UTC + position)
- [x] Type 5 — Static and Voyage Related Data (Class A)
- [x] Type 6 — Addressed Binary Message
- [x] Type 7 — Binary Acknowledge
- [x] Type 8 — Binary Broadcast Message
- [x] Type 9 — Standard SAR Aircraft Position Report
- [ ] Type 10 — UTC/Date Inquiry
- [x] Type 11 — UTC/Date Response
- [x] Type 12 — Addressed Safety-Related Message
- [x] Type 13 — Safety-Related Acknowledge
- [x] Type 14 — Safety-Related Broadcast Message
- [x] Type 15 — Interrogation
- [ ] Type 16 — Assignment Mode Command
- [ ] Type 17 — DGNSS Broadcast Binary Message
- [x] Type 18 — Standard Class B Position Report
- [x] Type 19 — Extended Class B Position Report
- [ ] Type 20 — Data Link Management
- [x] Type 21 — Aid-to-Navigation Report
- [ ] Type 22 — Channel Management
- [ ] Type 23 — Group Assignment Command
- [x] Type 24 — Static Data Report (Class B)
- [ ] Type 25 — Single Slot Binary Message
- [ ] Type 26 — Multiple Slot Binary Message
- [x] Type 27 — Long Range Position Report (satellite AIS)

## Safety & Alarms

- [ ] ACK — Acknowledge Alarm
- [ ] ACN — Alert Command
- [ ] ALA — Alert
- [ ] ALC — Cyclic Alert List
- [ ] ALF — Alert
- [ ] ALR — Alert Response
- [ ] ARC — Alert Response Command
- [ ] DOR — Door Status
- [ ] DSC — Digital Selective Calling Information
- [ ] DSE — DSC Extended
- [ ] EVE — Event
- [ ] FIR — Fire Detection
- [ ] HBT — Heartbeat Supervision

## Vessel Systems

- [ ] RPM — Revolutions
- [ ] VDR — Set and Drift

## Communication

- [ ] ALM — GPS Almanac Data
- [ ] FSI — Frequency Set Information
- [ ] MSK — Control for a Beacon Receiver
- [ ] MSS — Beacon Receiver Status
- [ ] RLM — Return Link Message
- [ ] SFI — Scanning Frequency Information
- [ ] STN — Multiple Data ID
- [x] TXT — Text Transmission

## Trawl / Fishing

- [ ] HFB — Trawl Headrope to Footrope and Bottom
- [ ] ITS — Trawl Door Spread 2 Distance
- [ ] TDS — Trawl Door Spread Distance
- [ ] TFI — Trawl Filling Indicator
- [ ] TPC — Trawl Position Cartesian Coordinates
- [ ] TPR — Trawl Position Relative Vessel
- [ ] TPT — Trawl Position True

## Proprietary

- [ ] PASHR — RT300 Inertial Attitude (Hemisphere/Ashtech)
- [ ] PCDIN — NMEA 2000 via SeaSmart bridge
- [ ] PGRME — Garmin Estimated Position Error
- [ ] PGRMT — Garmin Sensor Status
- [ ] PHTRO — Vessel Pitch and Roll (Xsens)
- [ ] PKLDS — Keel Depth Sentence
- [ ] PKLID — Keel ID
- [ ] PKLSH — Keel Shallow
- [ ] PKNDS — Knudsen Depth
- [ ] PKNID — Knudsen ID
- [ ] PKNSH — Knudsen Shallow
- [ ] PKWDWPL — Waypoint (unknown vendor)
- [ ] PMTK — MediaTek GPS Command
- [ ] PRDID — RDI Doppler Instrument Data
- [ ] PSKPDPT — Skipper Depth
- [ ] PSONCMS — SMC IMU Data (SBG Systems)
