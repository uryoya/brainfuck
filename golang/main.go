package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
)

const (
	MEM_SIZE = 3000
)

func main() {
	file, err := getCmdArg()
	if err != nil {
		panic(err)
	}
	script, err := ioutil.ReadFile(file)
	if err != nil {
		panic(err)
	}

	memory := [MEM_SIZE]byte{0}
	pointer := 0
	idx := 0
	for idx < len(script) {
		switch script[idx] {
		case '>':
			pointer += 1
			if pointer >= MEM_SIZE {
				panic("ぬるぽ")
			}
		case '<':
			pointer -= 1
			if pointer < 0 {
				panic("ぬるぽ")
			}
		case '+':
			memory[pointer] += 1
		case '-':
			memory[pointer] -= 1
		case '.':
			fmt.Printf("%c", memory[pointer])
		case ',':
			// pass
		case '[':
			if memory[pointer] == 0 {
				roop := 1
				for roop > 0 {
					idx += 1
					if script[idx] == '[' {
						roop += 1
					} else if script[idx] == ']' {
						roop -= 1
					}
				}
			}
		case ']':
			if memory[pointer] != 0 {
				roop := 1
				for roop > 0 {
					idx -= 1 // TODO: idxがMEM_SIZEを超える時のエラー処理がない
					if script[idx] == ']' {
						roop += 1
					} else if script[idx] == '[' {
						roop -= 1
					}
				}
			}
		}
		idx += 1
	}
	fmt.Println("")
}

func getCmdArg() (string, error) {
	if len(os.Args) < 2 {
		return "", errors.New("The following arguments are required: file")
	}
	return os.Args[1], nil
}
