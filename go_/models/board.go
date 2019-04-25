package models

import (
    "fmt"
    "strings"
)

type Board struct {
    rows [][]string
}

func (b *Board) Print() {
    fmt.Println("")

    for i := 0; i < len(b.rows); i++ {
        fmt.Printf("  | %s |\n", strings.Join(b.rows[i], " "))
    }
    fmt.Printf("  *%s*\n", strings.Repeat("‾", len(b.rows[0]) * 2 + 1))
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
