package main // only main package can be executed

import (
	ms "tung-example/my_packages"
	m "tung-example/my_package"
  // mi "tung-example/inner_main" // error inner_main is a program not an importable package
)

func main() {
	m.Hello()
  m.Main()
  ms.Hello()
  ms.Main()
}
