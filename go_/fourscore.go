package main

import (
    "fourscore/models"
    "bufio"
    "fmt"
    "os"
)

func main() {
    fmt.Println("Welcome to FourScore!\n")

    reader := bufio.NewReader(os.Stdin)

    first := models.NewPlayer(reader, "First", "x")
    second := models.NewPlayer(reader, "Second", "o")
    board := models.NewBoard()

    players := []models.Player{first, second}
    current := 0

    var player models.Player

    for {
        player = players[current]

        board.Print()
        player.Prompt()

        current = 1 - current

        break
    }
}
