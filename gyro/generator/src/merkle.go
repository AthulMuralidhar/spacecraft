package main

import (
	"crypto/sha256"
	"fmt"
	"io/ioutil"
	"log"
	"os"

	"github.com/cbergoon/merkletree"
)

//TestContent implements the Content interface provided by merkletree and represents the content stored in the tree.
type TestContent struct {
	x string
}

//CalculateHash hashes the values of a TestContent
func (t TestContent) CalculateHash() ([]byte, error) {
	h := sha256.New()
	if _, err := h.Write([]byte(t.x)); err != nil {
		return nil, err
	}

	return h.Sum(nil), nil
}

//Equals tests for equality of two Contents
func (t TestContent) Equals(other merkletree.Content) (bool, error) {
	return t.x == other.(TestContent).x, nil
}

func MakeMerkle() ([]byte, error) {
	//Build list of Content to build tree
	var list []merkletree.Content

	// find all the files in the current directory
	files, err := ioutil.ReadDir("./")
	if err != nil {
		log.Panic(err)
		return nil, err
	}

	for _, f := range files {
		fileName := f.Name()
		dat, err := os.ReadFile(fileName)
		if err != nil {
			log.Panic(err.Error())
			return nil, err
		}
		log.Println("printing binary data")
		fmt.Print(string(dat))
		// append to list
		list = append(list, TestContent{x: string(dat)})
	}

	//Create a new Merkle Tree from the list of Content
	t, err := merkletree.NewTree(list)
	if err != nil {
		log.Panic(err.Error())
		return nil, err
	}

	//Get the Merkle Root of the tree
	mr := t.MerkleRoot()
	log.Println("printing merkle root")
	log.Println(mr)

	//Verify the entire tree (hashes for each node) is valid
	vt, err := t.VerifyTree()
	if err != nil {
		log.Panic(err.Error())
		return nil, err
	}
	log.Println("Verify Tree: ", vt)

	//String representation
	log.Println(t)
	// return merkel
	return mr, nil
}
