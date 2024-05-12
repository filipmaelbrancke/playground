package net.maelbrancke.filip

import org.web3j.crypto.Hash
import kotlin.math.floor

class MerkleTree(private val items: List<ByteArray>) {

    constructor(vararg items: ByteArray) : this(items.toList())
    constructor(vararg items: String) : this(items.map { it.toByteArray() })

    private val leaves = items.map(Hash::sha3).toSortedSet(byteArrayComparator).toList()
    private val layers = initializeLayers(leaves)

    fun root(): ByteArray {
        return  layers.last().first()
    }

    fun hexRoot(): String {
        return bytesToHex(root())
    }

    fun containsLeaf(item: ByteArray): Boolean = leaves.binarySearch(item, byteArrayComparator) >= 0

    fun containsItem(item: ByteArray): Boolean = containsLeaf(Hash.sha3(item))

    fun proofLeaf(leaf: ByteArray): List<ByteArray> {
        var currentIndex = leaves.binarySearch(leaf, byteArrayComparator)
        if (currentIndex < 0) {
            throw NoSuchElementException("Item $leaf not found in Merkle tree")
        }
        val proof = mutableListOf<ByteArray>()
        layers.forEachIndexed { index, layer ->
            val oppositeItemFromPair = getOppositeItemFromPair(currentIndex, layer)
            if (oppositeItemFromPair != null) {
                proof.add(oppositeItemFromPair)
            }
            currentIndex = floor(currentIndex / 2.0).toInt()
        }
        return proof
    }

    fun proofItem(item: ByteArray): List<ByteArray> = proofLeaf(Hash.sha3(item))

    private fun getOppositeItemFromPair(index: Int, layer: List<ByteArray>): ByteArray? {
        val oppositeInPairIndex = if (index % 2 == 0) index + 1 else index - 1
        return if (oppositeInPairIndex < layer.size) {
            layer[oppositeInPairIndex]
        } else {
            null
        }
    }

    override fun toString(): String {
        val sb = StringBuilder()

        val output = sb.toString()
        return super.toString()
    }

    private fun buildTreeDescvription(buffer: StringBuilder, prefix: String, childPrefix: String) {
        buffer.append(prefix)
        layers.reversed().forEachIndexed { index, byteArrays ->

        }
    }

    private fun initializeLayers(leaves: List<ByteArray>): List<List<ByteArray>> {
        if (leaves.isEmpty()) {
            return emptyList()
        }
        val layers = mutableListOf<List<ByteArray>>()
        layers.add(leaves)

        // add layers until root
        while (layers[layers.size - 1].size > 1) {
            layers.add(calculateNextLayer(layers[layers.size - 1]))
        }

        return layers
    }

    private fun calculateNextLayer(leaves: List<ByteArray>): List<ByteArray> {
        val nextLayer = mutableListOf<ByteArray>()
        leaves.forEachIndexed { index, bytes ->
            if (index % 2 == 0) {
                nextLayer.add(combinedHash(bytes, leaves.elementAtOrNull(index + 1)))
            }
        }
        return nextLayer
    }

    companion object {

        fun verify(proof: List<ByteArray>, root: ByteArray, leaf: ByteArray): Boolean {
            var hash = leaf
            proof.forEach {
                if (byteArrayComparator.compare(hash, it) < 0) {
                    hash = combinedHash(hash, it)
                } else {
                    hash = combinedHash(it, hash)
                }
            }
            return byteArrayComparator.compare(hash, root) == 0
        }

         val byteArrayComparator = Comparator<ByteArray> { left, right ->
            var comparatorReturn = 0
            for (i in left.indices) {
                val comparison = left[i].toInt().toChar().compareTo(right[i].toInt().toChar())
                if (comparison != 0) {
                    comparatorReturn = comparison
                    break
                }
            }
            comparatorReturn
        }

        private fun combinedHash(left: ByteArray, right: ByteArray?): ByteArray {
            /*if (right == null) {
                return left
            }*/
            val sortedByteArrays = listOf(left, right ?: left).sortedWith(byteArrayComparator)
            return Hash.sha3(sortedByteArrays.first().plus(sortedByteArrays.last()))
        }
    }
}
