
package net.maelbrancke.filip

import org.web3j.crypto.Hash

fun main() {
    val merkleTree = MerkleTree("685b36169e9092d97f48093432119ca8d9d284fcc0d75dd933870d49ac0bddf2",
        "9ab947332314954a3a581cca3a4ea655511f7755df6ac03db09c442e01b3cfdb",
        "3f2e1a62e9376960d3624ed506afdea29457d8b2d524c472c14f45a3e193a4ae",
        "e022569d04b8f239bfcb1058d52d4aa97041495d39a7dc3190390627320c9909",
        "0de573cba199976f76ad0c63ff158aa459fab3be6f85471b301cbf3e7078c648",
        "3073b3f92ed939b8ece87842df3c458e29a19e02dcf0ee13094c25493836bc98",
        "6864416c3cde8e84cfffab6051a411df877dd0cac90f0d16696aa8eca2777cf3",
        "b2c343548e7c28f12d0ba69721d17290d59af57beceb1eb07900f080ca13a313",
        "6bf28b6e241ca46f474d778b0c5227699e9cecb3d1d58c22f3cc8bdaf1ac434e",
        "c9f90294940f3eb46e65456d38dbe66d03133fa2b0866593d008417ce8c416b8")

    println(merkleTree.hexRoot())
    println(merkleTree.proofItem("685b36169e9092d97f48093432119ca8d9d284fcc0d75dd933870d49ac0bddf2".toByteArray()))
    println(merkleTree.proofItem("6864416c3cde8e84cfffab6051a411df877dd0cac90f0d16696aa8eca2777cf3".toByteArray()))

    val result = MerkleTree.verify(
        merkleTree.proofItem("685b36169e9092d97f48093432119ca8d9d284fcc0d75dd933870d49ac0bddf2".toByteArray()),
        merkleTree.root(),
        Hash.sha3("685b36169e9092d97f48093432119ca8d9d284fcc0d75dd933870d49ac0bddf2".toByteArray()))
    println("verify proof: $result")
}

