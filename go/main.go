package main

import "fmt"

func main() {
	board := NewBoard()

	for _, square := range board.Squares {
		fmt.Printf("Square: %d:%d", square.xPos, square.yPos)
	}
}
