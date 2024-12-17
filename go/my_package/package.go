package mypackage

import "fmt"

func main() { // This is private so can't be accessed anw
	fmt.Println("this my package")
}

func Main() {
  main()
}

func Hello() {
	fmt.Println("this my package hello")
}

