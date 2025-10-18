package main

import (
    "fmt"
    "os"
    "github.com/msteinert/pam"
)

func main() {
    if len(os.Args) != 3 {
        fmt.Println("Usage: auth-helper <username> <password>")
        os.Exit(2)
    }
    username := os.Args[1]
    password := os.Args[2]

    tx, err := pam.StartFunc("login", username, func(s pam.Style, msg string) (string, error) {
        switch s {
        case pam.PromptEchoOff:
            return password, nil
        case pam.PromptEchoOn:
            return username, nil
        default:
            return "", nil
        }
    })
    if err != nil {
        fmt.Println("ERR")
        os.Exit(2)
    }

    if err := tx.Authenticate(0); err != nil {
        fmt.Println("FAIL")
        os.Exit(1)
    }

    fmt.Println("OK")
    os.Exit(0)
}
