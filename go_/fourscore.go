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
    var win, draw bool

    for {
        player = players[current]

        board.Print()
        row, col := player.PlayToken(reader, &board)
        win, draw = board.Inspect(row, col)

        if win || draw { break }

        current = 1 - current
    }
    board.Print()

    if win {
        fmt.Printf("nice job, %s :)\n", player.Name())
    } else {
        fmt.Println("draw :(")
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
