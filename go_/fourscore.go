package main

import (
    "bufio"
    "fmt"
    "fourscore/models"
    "os"
)

var reader = bufio.NewReader(os.Stdin)

func main() {
    fmt.Println("Welcome to FourScore!\n")

    players := createPlayers()
    board   := createBoard()
    current := 0

    var player models.Player

    for {
        player = players[current]

        row, col  := player.PlayToken(reader, &board)
        win, draw := board.Inspect(row, col)

        fmt.Printf("win: %v, draw: %v\n", win, draw)

        current = 1 - current
    }
}

func createPlayers() []models.Player {
    first  := models.NewPlayer(reader, "First", "x")
    second := models.NewPlayer(reader, "Second", "o")

    return []models.Player{first, second}
}

func createBoard() models.Board {
    return models.NewBoard()
}
