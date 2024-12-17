package mypackage

import "fmt"

func main() { // This is private so can't be accessed anw
	fmt.Println("this my package from my_packages")
}

func Main() {
  main()
}

func Hello() {
	fmt.Println("this my package hello from my_packages")
}
