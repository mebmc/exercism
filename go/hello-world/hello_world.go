package hello

import "fmt"

const testVersion = 2

// Returns a hello message
func HelloWorld(name string) string {
	if name == "" {
		name = "World"
	}
	return fmt.Sprintf("Hello, %s!", name)
}