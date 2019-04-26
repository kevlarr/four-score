package main

import (
    "bufio"
    "errors"
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
    board.Print()
    player.Prompt()

    choice, _   := reader.ReadString('\n')
    parsed, err := strconv.ParseInt(strings.TrimSpace(choice), 0, 8)

    for err != nil {
        player.Prompt()
        choice, _   = reader.ReadString('\n')
        parsed, err = strconv.ParseInt(strings.TrimSpace(choice), 0, 8)

        if err == nil && (parsed < 1 || parsed > 7) {
            err = errors.New("Must be 1-7")
        }
    }

    fmt.Println(parsed)
}
