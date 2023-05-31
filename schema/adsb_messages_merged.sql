-- The backing table for an aggregated storage for asdb_messages

CREATE TABLE adsb_messages_merged
(
    dateTime       DateTime,
    hex            String,
    callsign       String,
    type Enum8( 'adsb_icao' = 1, 'adsb_icao_nt' = 2, 'adsr_icao' = 3, 'tisb_icao' = 4, 'adsc' = 5, 'mlat' = 6, 'unknown' = 7, 'mode_s' = 8, 'adsb_other' = 9, 'adsr_other' = 10, 'tisb_other' = 11, 'tisb_trackfile' = 12 ),
    lat Nullable(Decimal32(6)),
    lon Nullable(Decimal32(6)),
    pos_type Enum8( 'INVALID' = 0, 'MESSAGE' = 1, 'MLAT' = 2 ),
    pos_nic Nullable(UInt32),
    pos_rc Nullable(UInt32),
    baro_rate Nullable(Int32),
    geom_rate Nullable(Int32),
    baro_alt Nullable(Int32),
    geom_alt Nullable(Int32),
    nav_altitude_mcp Nullable(UInt32),
    nav_altitude_fms Nullable(UInt32),
    nav_qnh Nullable(Decimal64(6)),
    nav_heading Nullable(Decimal32(6)),
    squawk Nullable(UInt32),
    gs Nullable(Decimal64(6)),
    mach Nullable(Decimal32(6)),
    roll Nullable(Decimal32(6)),
    track Nullable(Decimal32(6)),
    track_rate Nullable(Decimal32(6)),
    mag_heading Nullable(Decimal32(6)),
    true_heading   Decimal32(6),
    wind_direction Decimal32(6),
    wind_speed     Decimal32(6),
    oat            Decimal64(6),
    tat            Decimal64(6),
    tas Nullable(UInt32),
    ias Nullable(UInt32),
    category       String,
    nav_modes Nullable(UInt8),
    emergency Enum8( 'NONE' = 0, 'GENERAL' = 1, 'LIFEGUARD' = 2, 'MINFUEL' = 3, 'NORDO' = 4, 'UNLAWFUL' = 5, 'DOWNED' = 6, 'RESERVED' = 7 ),
    airground Enum8( 'INVALID' = 0, 'GROUND' = 1, 'AIRBORNE' = 2, 'UNCERTAIN' = 3 ),
    nav_altitude_src Enum8( 'INVALID' = 0, 'UNKNOWN' = 1, 'AIRCRAFT' = 2, 'MCP' = 3, 'FMS' = 4 ),
    sil_type Enum8( 'INVALID' = 0, 'UNKNOWN' = 1, 'PER_SAMPLE' = 2, 'PER_HOUR' = 3 ),
    sil Nullable(UInt32),
    adsb_version Nullable(Int32),
    adsr_version Nullable(Int32),
    tisb_version Nullable(Int32),
    nac_p Nullable(UInt32),
    nac_v Nullable(UInt32),
    gva Nullable(UInt32),
    sda Nullable(UInt32),
    nic_a Nullable(UInt32),
    nic_c Nullable(UInt32),
    nic_baro Nullable(UInt32),
    alert Nullable(UInt32),
    spi Nullable(UInt32),
    signal         Int32
) ENGINE = ReplacingMergeTree() ORDER BY (dateTime, hex);