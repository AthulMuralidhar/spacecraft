package main

import "testing"

func TestMakeMerkle(t *testing.T) {
	tests := []struct {
		name string
	}{
		// TODO: Add test cases.
		// TODO: verify each content of the merkel tree
		// ex:
		//
		// Verify a specific content in in the tree
		//   vc, err := t.VerifyContent(list[0])
		//   if err != nil {
		// log.Fatal(err)
		//   }
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			MakeMerkle()
		})
	}
}
