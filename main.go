package main

import (
	"fmt"

	"github.com/shirou/gopsutil/v3/cpu"
)

/*
TODO:
- Get correct metrics
- Ensure consistent core ordering (these shouldn't be sorted...)
*/

func main() {
	PrintPercents()
	// PrintCores()
}

func PrintPercents() {
    percents, err := cpu.Percent(0, true)
	if err != nil {
		fmt.Println("Could not get CPUs")
	}
	for _, percent := range percents {
		fmt.Printf("%0.1f%% ", percent)
	}
	fmt.Println()
}

func PrintCores() {
    counts, err := cpu.Counts(true)
	if err != nil {
		fmt.Println("Could not get cores")
	}
	fmt.Println("Cores:", counts)
}
