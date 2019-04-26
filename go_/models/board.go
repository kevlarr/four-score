package models

import (
    "errors"
    "fmt"
    "strings"
)

var defaultCell = "·"

type Board struct {
    rows [][]string
}

func (b *Board) Print() {
    fmt.Println("")

    for i := 0; i < len(b.rows); i++ {
        fmt.Printf("  | %s |\n", strings.Join(b.rows[i], " "))
    }
    fmt.Printf("  *%s*\n", strings.Repeat("‾", len(b.rows[0]) * 2 + 1))
    fmt.Print("   ")

    for i, _ := range b.rows[0] {
        fmt.Printf(" %v", i + 1)
    }
    fmt.Println("\n")
}

func (b *Board) Receive(col int, token string) (int, error) {
    if col < 0 || col > 6 {
        return -1, errors.New("Column is out of range")
    }

    for row := 6; row >= 0; row-- {
        if b.rows[row][col] == defaultCell {
            b.rows[row][col] = token
            return row, nil
        }
    }
    return -1, errors.New("Column is full")
}

func NewBoard() Board {
    var rows [][]string

    for i := 0; i < 7; i++ {
        var row []string

        for j := 0; j < 7; j++ {
            row = append(row, defaultCell)
        }
        rows = append(rows, row)
    }
    return Board{rows}
}
