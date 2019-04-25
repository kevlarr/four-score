package main

import (
    "bufio"
    "fmt"
    "os"
    "strings"
)

type Player struct {
    name string
    token string
}

func (p *Player) prompt() {
    fmt.Printf("%s, place your \"%s\": ", p.name, p.token)
}

type Board struct {
    rows [][]string
}

func (b *Board) print() {
    fmt.Println("")

    for i := 0; i < len(b.rows); i++ {
        fmt.Printf("  | %s |\n", strings.Join(b.rows[i], " "))
    }
    fmt.Printf("  *%s*\n", strings.Repeat("‾", len(b.rows[0]) * 2 + 1))
}

func NewPlayer(r *bufio.Reader, ordinal string, token string) Player {
    fmt.Printf("%v player: ", ordinal)
    name, _ := r.ReadString('\n')
    name = strings.TrimSpace(name)

    return Player{name, token}
}

func NewBoard() Board {
    var rows [][]string

    for i := 0; i < 7; i++ {
        var row []string

        for j := 0; j < 7; j++ {
            row = append(row, "·")
        }
        rows = append(rows, row)
    }
    return Board{rows}
}

func main() {
    fmt.Println("Welcome to FourScore!\n")

    reader := bufio.NewReader(os.Stdin)

    first := NewPlayer(reader, "First", "x")
    second := NewPlayer(reader, "Second", "o")
    board := NewBoard()

    players := []Player{first, second}
    current := 0

    var player Player

    for {
        player = players[current]

        board.print()
        player.prompt()

        current = 1 - current

        break
    }
}
