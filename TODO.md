# Things to do.
## Complete the Vec API
More work to do with the current API:
* Add benchmark tests
* Implement From Vec for MyVec, which gives Vec into Myvec.
* Macro with default/0 initialized vec.
* Implement FromIterator.
* Implement serialize, deserialize.
* Send, Sync?
* Return errors from operations that can fail.
## Complete the heap API
* Linear time setup from vector
## Integrate with TravisCI.
* Run tests, benchmarks, build.
* Figure kcov.
## New datastrutures
* Frozen set from vector, cache efficient tree like.
* Deque? The new interest.
* Simple nullable array vs circular buffer
* Composable DS: LRU cache?
* Hash, rebalance.. vector of vectors, constant search and insert times.
* Set, can use derefmut methods internally?
