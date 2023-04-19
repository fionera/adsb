CREATE TABLE adsb_raw
(
    `timestamp` DECIMAL64(3),
    `dateTime`  DateTime,                      --"2023-03-02 23:19:43",

-- hex: the 24-bit ICAO identifier of the aircraft, as 6 hex digits. The identifier may start with '~', this means that the address is a non-ICAO address (e.g. from TIS-B).
    `hex`       String,                        --"c0332c",

-- type: type of underlying messages / best source of current data for this position / aircraft: (the following list is in order of which data is preferentially used)
-- adsb_icao: messages from a Mode S or ADS-B transponder, using a 24-bit ICAO address
-- adsb_icao_nt: messages from an ADS-B equipped "non-transponder" emitter e.g. a ground vehicle, using a 24-bit ICAO address
-- adsr_icao: rebroadcast of ADS-B messages originally sent via another data link e.g. UAT, using a 24-bit ICAO address
-- tisb_icao: traffic information about a non-ADS-B target identified by a 24-bit ICAO address, e.g. a Mode S target tracked by secondary radar
-- adsc: ADS-C (received by monitoring satellite downlinks)
-- mlat: MLAT, position calculated arrival time differences using multiple receivers, outliers and varying accuracy is expected.
-- other: miscellaneous data received via Basestation / SBS format, quality / source is unknown.
-- mode_s: ModeS data from the planes transponder (no position transmitted)
-- adsb_other: messages from an ADS-B transponder using a non-ICAO address, e.g. anonymized address
-- adsr_other: rebroadcast of ADS-B messages originally sent via another data link e.g. UAT, using a non-ICAO address
-- tisb_other: traffic information about a non-ADS-B target using a non-ICAO address
-- tisb_trackfile: traffic information about a non-ADS-B target using a track/file identifier, typically from primary or Mode A/C radar
    `type` Enum8(
        'adsb_icao' = 1,
        'adsb_icao_nt' = 2,
        'adsr_icao' = 3,
        'tisb_icao' = 4,
        'adsc' = 5,
        'mlat' = 6,
        'unknown' = 7, -- its "unknown". not "other"
        'mode_s' = 8,
        'adsb_other' = 9,
        'adsr_other' = 10,
        'tisb_other' = 11,
        'tisb_trackfile' = 12
        ),

-- flight: callsign, the flight name or aircraft registration as 8 chars (2.2.8.2.6)
    `flight` LowCardinality(Nullable(String)), --"ACA599",

-- alt_baro: the aircraft barometric altitude in feet as a number OR "ground" as a string
    `alt_baro`  Int32,

-- alt_geom: geometric (GNSS / INS) altitude in feet referenced to the WGS84 ellipsoid
    `alt_geom`  Int32,

-- gs: ground speed in knots
    `gs`        Decimal32(1),

-- ias: indicated air speed in knots
    `ias` Nullable(Int32),

-- tas: true air speed in knots
    `tas` Nullable(Int32),

-- mach: Mach number
    `mach` Nullable(Decimal32(4)),

-- track: true track over ground in degrees (0-359)
    `track` Nullable(Decimal32(2)),

-- track_rate: Rate of change of track, degrees/second
    `track_rate` Nullable(Decimal32(2)),

-- roll: Roll, degrees, negative is left roll
    `roll` Nullable(Decimal32(2)),

-- mag_heading: Heading, degrees clockwise from magnetic north
    `mag_heading` Nullable(Decimal32(2)),

-- true_heading: Heading, degrees clockwise from true north (usually only transmitted on ground, in the air usually derived from the magnetic heading using magnetic model WMM2020)
    `true_heading` Nullable(Decimal32(2)),

-- baro_rate: Rate of change of barometric altitude, feet/minute
    `baro_rate` Nullable(Int32),

-- geom_rate: Rate of change of geometric (GNSS / INS) altitude, feet/minute
    `geom_rate` Nullable(Int32),

-- squawk: Mode A code (Squawk), encoded as 4 octal digits
    `squawk` Nullable(FixedString(4)),

-- emergency: ADS-B emergency/priority status, a superset of the 7x00 squawks (2.2.3.2.7.8.1.1) (none, general, lifeguard, minfuel, nordo, unlawful, downed, reserved)
    `emergency` Nullable(String),

-- category: emitter category to identify particular aircraft or vehicle classes (values A0 - D7) (2.2.3.2.5.2)
    `category` LowCardinality(String),

-- nav_qnh: altimeter setting (QFE or QNH/QNE), hPa
    `nav_qnh` Nullable(Decimal32(1)),

-- nav_altitude_mcp: selected altitude from the Mode Control Panel / Flight Control Unit (MCP/FCU) or equivalent equipment
    `nav_altitude_mcp` Nullable(Int32),

-- nav_altitude_fms: selected altitude from the Flight Manaagement System (FMS) (2.2.3.2.7.1.3.3)
    `nav_altitude_fms` Nullable(Int32),

-- nav_heading: selected heading (True or Magnetic is not defined in DO-260B, mostly Magnetic as that is the de facto standard) (2.2.3.2.7.1.3.7)
    `nav_heading` Nullable(Decimal32(2)),

-- nav_modes: set of engaged automation modes: 'autopilot', 'vnav', 'althold', 'approach', 'lnav', 'tcas'
    `nav_modes` Array(String),

-- lat, lon: the aircraft position in decimal degrees
    `lat`       Decimal32(6),
    `lon`       Decimal32(6),

-- nic: Navigation Integrity Category (2.2.3.2.7.2.6)
    `nic` Nullable(Int32),

-- rc: Radius of Containment, meters; a measure of position integrity derived from NIC & supplementary bits. (2.2.3.2.7.2.6, Table 2-69)
    `rc` Nullable(Int32),

-- seen_pos: how long ago (in seconds before "now") the position was last updated
    `seen_pos`  Decimal32(3),

-- version: ADS-B Version Number 0, 1, 2 (3-7 are reserved) (2.2.3.2.7.5)
    `version`   UInt8,

-- nic_baro: Navigation Integrity Category for Barometric Altitude (2.2.5.1.35)
    `nic_baro` Nullable(UInt8),

-- nac_p: Navigation Accuracy for Position (2.2.5.1.35)
    `nac_p` Nullable(UInt8),

-- nac_v: Navigation Accuracy for Velocity (2.2.5.1.19)
    `nac_v` Nullable(UInt8),

-- sil: Source Integity Level (2.2.5.1.40)
    `sil` Nullable(UInt8),

-- sil_type: interpretation of SIL: unknown, perhour, persample
    `sil_type` LowCardinality(String),

-- gva: Geometric Vertical Accuracy (2.2.3.2.7.2.8)
    `gva` Nullable(UInt8),

-- sda: System Design Assurance (2.2.3.2.7.2.4.6)
    `sda` Nullable(UInt8),

-- mlat: list of fields derived from MLAT data
    `mlat` Array(String),

-- tisb: list of fields derived from TIS-B data
    `tisb` Array(String),

-- messages: total number of Mode S messages received from this aircraft
    `messages` Nullable(UInt64),

-- seen: how long ago (in seconds before "now") a message was last received from this aircraft
    `seen` Nullable(DECIMAL32(1)),

-- rssi: recent average RSSI (signal power), in dbFS; this will always be negative.
    `rssi`      DECIMAL32(1),

-- alert: Flight status alert bit (2.2.3.2.3.2)
    `alert` Nullable(Int8),

-- spi: Flight status special position identification bit (2.2.3.2.3.2)
    `spi` Nullable(Int8),

-- wd, ws: wind direction and wind speed are calculated from ground track, true heading, true airspeed and ground speed
    `wd` Nullable(Int32),
    `ws` Nullable(Int32),

-- oat, tat: outer/static air temperature (C) and total air temperature (C) are calculated from mach number and true airspeed (typically somewhat inaccurate at lower altitudes / mach numbers below 0.5, calculation is inhibited for mach < 0.395)
    `oat` Nullable(Int8),
    `tat` Nullable(Int8)
) ENGINE = MergeTree()
      PARTITION BY (toDate(dateTime), type)
      ORDER BY (dateTime, type, hex);
