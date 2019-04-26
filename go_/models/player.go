package models

import (
    "bufio"
    "fmt"
    "strings"
    "strconv"
)

type Player struct {
    name string
    token string
}

func (p *Player) Prompt() {
    fmt.Printf("%s, place your \"%s\": ", p.name, p.token)
}

func (p *Player) PlayToken(r *bufio.Reader, board *Board) (row int, col int) {
    var err error
    board.Print()

    for {
        p.Prompt()
        choice, _ := r.ReadString('\n')
        col, err = strconv.Atoi(strings.TrimSpace(choice))

        if err != nil { continue }

        col--
        row, err = board.Receive(col, p.token)

        if err == nil { break }
    }

    return
}

func NewPlayer(r *bufio.Reader, ordinal string, token string) Player {
    fmt.Printf("%v player: ", ordinal)
    name, _ := r.ReadString('\n')
    name = strings.TrimSpace(name)

    return Player{name, token}
}
