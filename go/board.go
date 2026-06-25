package main

type Board struct {
	Squares []Square
}

type Square struct {
	xPos  int
	yPos  int
	Piece *Piece
}

type Piece struct {
	Type   string
	Colour string
}

func NewBoard() Board {
	return Board{
		Squares: []Square{{xPos: 1, yPos: 1, Piece: &Piece{Type: "rook", Colour: "white"}}},
	}
}
