package main

import (
	"flag"
	"google.golang.org/grpc"
	"log"
	"net"

	"github.com/fionera/adsb/gen/pb"
)

var (
	listenAddr string
)

type server struct {
	pb.UnimplementedFrameStreamerServer
}

func (s *server) SendFrames(conn pb.FrameStreamer_SendFramesServer) error {
	for {
		frame, err := conn.Recv()
		if err != nil {
			return err
		}

		log.Println(net.IP(frame.SrcIP).String())
	}
}

func main() {
	flag.StringVar(&listenAddr, "target-addr", "", "The address:port to forward to")
	flag.Parse()

	if listenAddr == "" {
		log.Fatal("invalid addr")
	}

	s := grpc.NewServer()
	pb.RegisterFrameStreamerServer(s, &server{})

	lis, err := net.Listen("tcp", listenAddr)
	if err != nil {
		log.Fatal(err)
	}

	log.Fatal(s.Serve(lis))
}
