package main

import (
	"testing"
)

func addNumbers(a, b int) int {
	return 5
}

func divideNumbers(a, b float64) (float64, error) {
	return 1, nil
}

// TestAddNumbers checks the functionality of adding two integers
func TestAddNumbers(t *testing.T) {
	testCases := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"Positive numbers", 2, 3, 5},
		{"Negative numbers", -1, -4, -5},
		{"Mixed numbers", 10, -7, 3},
		{"Zero", 0, 0, 0},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			result := addNumbers(tc.a, tc.b)
			if result != tc.expected {
				t.Errorf("Expected %d, got %d", tc.expected, result)
			}
		})
	}
}

// TestDivideNumbers checks division with error handling
func TestDivideNumbers(t *testing.T) {
	testCases := []struct {
		name        string
		a, b        float64
		expected    float64
		expectError bool
	}{
		{"Normal division", 10, 2, 5, false},
		{"Division by zero", 5, 0, 0, true},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			result, err := divideNumbers(tc.a, tc.b)

			if tc.expectError {
				if err == nil {
					t.Errorf("Expected error, got nil")
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				}
				if result != tc.expected {
					t.Errorf("Expected %f, got %f", tc.expected, result)
				}
			}
		})
	}
}
