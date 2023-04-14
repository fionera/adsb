package main

import (
	"flag"
	"github.com/fionera/adsb/gen/pb"
	"github.com/google/gopacket"
	"github.com/google/gopacket/afpacket"
	"github.com/google/gopacket/layers"
	"google.golang.org/protobuf/proto"
	"log"
	"net"
	"time"
)

var (
	targetPort    int
	targetAddress string

	currFrame pb.Frame
)

func main() {
	flag.IntVar(&targetPort, "target-port", 30_005, "The port to listen for")
	flag.StringVar(&targetAddress, "target-addr", "", "The address:port to forward to")
	flag.Parse()

	if len(flag.Args()) == 0 {
		log.Fatal("please give me interfaces")
	}

	var args = []any{afpacket.SocketRaw, afpacket.TPacketVersion3}
	for _, name := range flag.Args() {
		args = append(args, afpacket.OptInterface(name))
	}

	handle, err := afpacket.NewTPacket(args...)
	if err != nil {
		log.Fatalf("opening interface: %v", err)
	}

reconn:
	conn, err := net.Dial("tcp", targetAddress)
	if err != nil {
		log.Println(err)
		time.Sleep(1 * time.Second)
		goto reconn
	}

	for {
		currFrame.Reset()

		data, _, err := handle.ZeroCopyReadPacketData()
		if err != nil {
			log.Fatal(err)
		}

		packet := gopacket.NewPacket(data, layers.LayerTypeIPv4, gopacket.Default)
		var dstPort int
		for _, layer := range packet.Layers() {
			switch layer.(type) {
			case *layers.IPv4:
				currFrame.SrcIP = layer.(*layers.IPv4).SrcIP

			case *layers.TCP:
				dstPort = int(layer.(*layers.TCP).DstPort)
			}
		}

		if dstPort != targetPort {
			continue
		}

		currFrame.Data = data

		bytes, err := proto.Marshal(&currFrame)
		if err != nil {
			log.Println("failed marshal", err)
			continue
		}

		if _, err := conn.Write(bytes); err != nil {
			log.Println(err)
			// jump to reconnect
			goto reconn
		}
	}
}
