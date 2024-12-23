package main

import "fmt"

func main() { // This is private so can't be accessed anw
	fmt.Println("this is inner main")
}

func Main() {
  main()
}

func Hello() {
	fmt.Println("this is inner main hello")
}
