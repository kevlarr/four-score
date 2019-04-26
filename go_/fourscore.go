package main

import (
    "bufio"
    "fmt"
    "fourscore/models"
    "os"
    "strconv"
    "strings"
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

        row, col  := playPiece(&player, &board)
        win, draw := board.Inspect(row, col)

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

func playPiece(player *models.Player, board *models.Board) (row int, col int) {
    var err error
    board.Print()

    for {
        player.Prompt()
        choice, _  := reader.ReadString('\n')
        col, err = strconv.Atoi(strings.TrimSpace(choice))

        if err != nil { continue }

        col--
        row, err = board.Receive(col, player.Token())

        if err == nil { break }
    }

    return
}
