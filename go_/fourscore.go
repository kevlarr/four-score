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
        playRound(&player, &board)
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

func playRound(player *models.Player, board *models.Board) {
    var row int
    board.Print()

    for {
        player.Prompt()
        choice, _   := reader.ReadString('\n')
        column, err := strconv.Atoi(strings.TrimSpace(choice))

        if err != nil { continue }

        row, err = board.Receive(column - 1, player.Token())

        if err == nil { break }
    }

    fmt.Printf("row: %v\n", row)
}
