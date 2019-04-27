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

func (b *Board) Inspect(row, col int) (win bool, draw bool) {
    win =
        b.wonVertical(row, col) || b.wonHorizontal(row, col) ||
        b.wonDiagonal(row, col) || b.wonAntidiagonal(row, col)

    if win { return }

    for _, v := range b.rows[0] {
        if v == defaultCell { return }
    }

    draw = true
    return
}

func (b *Board) wonVertical(row, col int) bool {
    if row > 3 { return false }

    return b.count(row, 1, col, 0) > 2
}

func (b *Board) wonHorizontal(row, col int) bool {
    return b.count(row, 0, col, 1) + b.count(row, 0, col, -1) > 2
}

func (b *Board) wonDiagonal(row, col int) bool { // Direction: \
    return b.count(row, 1, col, 1) + b.count(row, -1, col, -1) > 2
}

func (b *Board) wonAntidiagonal(row, col int) bool { // Direction: /
    return b.count(row, -1, col, 1) + b.count(row, 1, col, -1) > 2
}

func (b *Board) count(row, dRow, col, dCol int) (count int) {
    token, count := b.rows[row][col], 0

    for r, c := row + dRow, col + dCol; b.inBounds(r, c); {
        if b.rows[r][c] != token { break }
        r, c = r + dRow, c + dCol
        count++
    }
    return
}

func (b *Board) inBounds(row, col int) bool {
    return row >= 0 && col >= 0 && row < 7 && col < 7
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
