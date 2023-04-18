package main

import (
	"context"
	"flag"
	"github.com/fionera/adsb/gen/pb"
	"github.com/google/gopacket"
	"github.com/google/gopacket/afpacket"
	"github.com/google/gopacket/layers"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"log"
)

var (
	targetPort    int
	targetAddress string
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

	for {
		if err := loop(handle); err != nil {
			log.Println(err)
		}
	}

}

func loop(handle *afpacket.TPacket) error {
	conn, err := grpc.Dial(targetAddress, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return err
	}
	defer conn.Close()

	f, err := pb.NewFrameStreamerClient(conn).SendFrames(context.Background())
	if err != nil {
		return err
	}

	for {
		var currFrame pb.Frame

		data, _, err := handle.ZeroCopyReadPacketData()
		if err != nil {
			log.Fatal(err)
		}

		packet := gopacket.NewPacket(data, layers.LayerTypeIPv4, gopacket.NoCopy)
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

		if err := f.Send(&currFrame); err != nil {
			return err
		}
	}
	return nil
}
