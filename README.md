# ADSB

Collection of DB schema, lookup source and general documentation

## Tools

Most of the tools are written in Rust. If you want to run them I recommend setting up the current stable toolchain
via https://rustup.rs.

For building the Go Protobuf files you need:
```
go install github.com/bufbuild/buf/cmd/buf@v1.17.0
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.2
go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.28
```
afterwards you can generate the proto files with `buf generate`

### Running a tool

To run a specific tool you can use:
`cargo run --bin squawkcodes`

Listing all available tools is possible by not supplying a `--bin` argument to cargo.

## Protocols

### BEAST

https://wiki.jetvision.de/wiki/Mode-S_Beast:Data_Output_Formats

### readsb json

https://github.com/wiedehopf/readsb/blob/dev/README-json.md

## Coordinates

### Decimal degrees precision

Six (6) decimal places precision in coordinates using decimal degrees notation is at a 10 cm (or 0.1 meters) resolution.
Each .000001 difference in coordinate decimal degree is approximately 10 cm in length. For example, the imagery of
Google Earth and Google Maps is typically at the 1-meter resolution, and some places have a higher resolution of 1 inch
per pixel. One meter resolution can be represented using 5 decimal places so more than 6 decimal places are extraneous
for that resolution. The distance between longitudes at the equator is the same as latitude, but the distance between
longitudes reaches zero at the poles as the lines of meridian converge at that point.

For millimeter (mm) precision then represent lat/lon with 8 decimal places in decimal degrees format. Since most
applications don't need that level of precision 6 decimal places is sufficient for most cases.

In the other direction, whole decimal degrees represent a distance of ~111 km (or 60 nautical miles) and a 0.1 decimal
degree difference represents a ~11 km distance.

Here is a table of # decimal places difference in latitude with the delta degrees and the estimated distance in meters
using 0,0 as the starting point.

| Decimal places | Decimal degrees | Distance (meters) | Notes  |
|----------------|-----------------|-------------------|--------| 
| 0              | 1.0             | 110,574.3         | 111 km | 
| 1              | 0.1             | 11,057.43         | 11 km  | 
| 2              | 0.01            | 1,105.74          | 1 km   | 
| 3              | 0.001           | 110.57            | 110 m  | 	
| 4              | 0.0001          | 11.06             | 11m    | 
| 5              | 0.00001         | 1.11              | 1m     | 
| 6              | 0.000001        | 0.11              | 11 cm  | 
| 7              | 0.0000001       | 0.01              | 1 cm   | 
| 8              | 0.00000001      | 0.001             | 1 mm   |
