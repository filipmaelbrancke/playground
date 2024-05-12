package net.maelbrancke.filip

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.collections.shouldContain
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.ints.shouldBeGreaterThan
import io.kotest.matchers.ints.shouldBeLessThan
import io.kotest.matchers.shouldBe
import org.web3j.crypto.Hash

class MerkleTreeTest : FunSpec({

    val merkleTree = MerkleTree("test1", "test2", "test3", "test4", "test5", "test6")

    test("merkleTree root") {
        merkleTree.hexRoot() shouldBe "fc22780392fdea00dfd1f25d3092874a21f53ea9476670649002bb1bde7fa28e"
    }

    test("contains item") {
        merkleTree.containsItem("test1".toByteArray()) shouldBe true
        merkleTree.containsItem("test2".toByteArray()) shouldBe true
        merkleTree.containsItem("test3".toByteArray()) shouldBe true
        merkleTree.containsItem("test4".toByteArray()) shouldBe true
        merkleTree.containsItem("test5".toByteArray()) shouldBe true
        merkleTree.containsItem("test6".toByteArray()) shouldBe true
        merkleTree.containsItem("test7".toByteArray()) shouldBe false
    }

    test("proof leaf") {
        val proof = merkleTree.proofLeaf(Hash.sha3("test1".toByteArray()))
        proof shouldHaveSize 3
        proof shouldContain Hash.sha3("test2".toByteArray())
    }

    test("proof item") {
        val proof = merkleTree.proofItem("test1".toByteArray())
        proof shouldHaveSize 3
        proof shouldContain Hash.sha3("test2".toByteArray())
    }

    test("verify proof") {
        val test1Proof = merkleTree.proofItem("test1".toByteArray())
        MerkleTree.verify(test1Proof, merkleTree.root(), Hash.sha3("test1".toByteArray())) shouldBe true
        MerkleTree.verify(test1Proof, merkleTree.root(), Hash.sha3("test2".toByteArray())) shouldBe false

        val test2Proof = merkleTree.proofItem("test2".toByteArray())
        MerkleTree.verify(test2Proof, merkleTree.root(), Hash.sha3("test2".toByteArray())) shouldBe true
        MerkleTree.verify(test2Proof, merkleTree.root(), Hash.sha3("test3".toByteArray())) shouldBe false
    }

    test("byteArrayComparison") {
        MerkleTree.byteArrayComparator.compare(byteArrayOf(1, 2, 3), byteArrayOf(1, 2, 3)) shouldBe 0
        MerkleTree.byteArrayComparator.compare(
            byteArrayOf(0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65),
            byteArrayOf(0x74, 0x72, 0x65, 0x65)
        ) shouldBeLessThan 0
        MerkleTree.byteArrayComparator.compare(
            byteArrayOf(0x74, 0x72, 0x65, 0x65),
            byteArrayOf(0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65)
        ) shouldBeGreaterThan 0
    }

})
