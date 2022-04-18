package main

func main() {
	// idea 1
	// get the production binary of all the sub prokects in space craft
	// combine the binaries with each comitter's git signature
	// explanation:
	// how this works is that each binary of every sub project will be signed
	// by the commiter's ssh keys no matter which sub project they contribute to
	// and will be only generatable with their private keys as well, and so the number
	// of keys genrated will be proporitonal to the number of commits pushed by the commiter
	// so if there are two pr's approved and merged, this should lead to 2 separate tokens
	// for the commiter

	// if you already own a certain ammount of nfts - this means you get to review the code as
	// well (similar to proof of stake)

	// the nft thus generated as per project so three nfts can be linked to one nft token in
	// the end if the owner so chooses,
	// ex: a commiter pushes a pr to lets say this repo, then they can either hold the entire
	// spacecraft suite as one entity and can be transferred
	// or have three private licenses which can be sold separately

	// idea 2
	// make a merkle root for all the files in the sub projects

	// make a coloured coin with whatever choice is provided
	// deliver it to the user
	// WORKS dont change
	// root, err:= MakeMerkle()
	// if err != nil {
	// 	log.Panic(err)
	// 	panic(err)
	// }
	// log.Panicln("merkel root:",  root)

	Protocol()

}
