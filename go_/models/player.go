package models

import (
    "bufio"
    "fmt"
    "strings"
)

type Player struct {
    name string
    token string
}

func (p *Player) Prompt() {
    fmt.Printf("%s, place your \"%s\": ", p.name, p.token)
}

func (p *Player) Token() string {
    return p.token
}

func NewPlayer(r *bufio.Reader, ordinal string, token string) Player {
    fmt.Printf("%v player: ", ordinal)
    name, _ := r.ReadString('\n')
    name = strings.TrimSpace(name)

    return Player{name, token}
}
