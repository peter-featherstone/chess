package main

import (
	"fmt"

	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/widget"
)

func main() {
	a := app.New()
	w := a.NewWindow("Peter Chess")

	board := NewBoard()

	w.SetContent(widget.NewLabel("Hello World!"))

	content := ""
	for _, square := range board.Squares {
		content += fmt.Sprintf("Square: %d:%d", square.xPos, square.yPos)
	}

	w.SetContent(widget.NewLabel(content))
	w.ShowAndRun()
}
