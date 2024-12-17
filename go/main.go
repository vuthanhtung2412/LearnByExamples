package main // only main package can be executed

import (
	m2 "tung-example/my_packages"
	m1 "tung-example/my_package"
)

func main() {
	m1.Hello()
  m1.Main()
  m2.Hello()
  m2.Main()
}
