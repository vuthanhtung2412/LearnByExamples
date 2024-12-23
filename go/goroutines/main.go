// The example below demonstrate how go do asynchrounous operations
package main

import (
    "fmt"
    "sync"
    "time"
)

func main() {
    // Create a channel and a WaitGroup
    ch := make(chan int)
    var wg sync.WaitGroup
    
    // Add 3 to the WaitGroup for our 3 goroutines
    wg.Add(3)

    // Goroutine 1 - will block on channel receive
    go func() {
        defer wg.Done()
        fmt.Println("Goroutine 1: Starting")
        value := <-ch  // This blocks
        fmt.Println("Goroutine 1: Received", value)
    }()

    // Goroutine 2 - CPU-bound work
    go func() {
        defer wg.Done()
        fmt.Println("Goroutine 2: Starting")
        sum := 0
        for i := 0; i < 1000000; i++ {
            sum += i
        }
        fmt.Println("Goroutine 2: Finished calculation")
    }()

    // Goroutine 3 - I/O bound (simulated)
    go func() {
        defer wg.Done()
        fmt.Println("Goroutine 3: Starting")
        time.Sleep(100 * time.Millisecond)  // Simulating I/O
        fmt.Println("Goroutine 3: Finished I/O")
        ch <- 42  // Unblocks Goroutine 1
    }()

    fmt.Println("goroutines declared")

    // Wait for all goroutines to complete
    wg.Wait()
    fmt.Println("All goroutines completed")
}
