package main

import (
	"flag"
	"github.com/fionera/adsb/gen/pb"
	"google.golang.org/protobuf/proto"
	"log"
	"net"
)

var (
	listenAddr string
)

func main() {
	flag.StringVar(&listenAddr, "target-addr", "", "The address:port to forward to")
	flag.Parse()

	if listenAddr == "" {
		log.Fatal("invalid addr")
	}

	conn, err := net.Listen("tcp", listenAddr)
	if err != nil {
		log.Fatal(err)
	}

	for {
		accept, err := conn.Accept()
		if err != nil {
			log.Println(err)
			continue
		}

		go func() {
			var frame pb.Frame

			data := make([]byte, 4096)
			for {
				frame.Reset()

				length, err := accept.Read(data)
				if err != nil {
					log.Println(err)
					return
				}

				if err := proto.Unmarshal(data[:length], &frame); err != nil {
					return
				}

				log.Println(net.IP(frame.SrcIP).String())
			}
		}()
	}
}
