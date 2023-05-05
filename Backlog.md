# HNSW algorithms in Rust:

Define the data structures:
a. ProllyTree: A struct representing the Prolly Tree for distributed and versioned data sets.
b. HNSWIndex: A struct representing the HNSW index for organizing the vectors and supporting nearest neighbor queries.
c. Vector: A struct representing a vector in the high-dimensional space.
d. DecentralizedVectorDatabase: A struct encapsulating both the ProllyTree and HNSWIndex for managing the decentralized vector database.

Implement the ProllyTree functionality:
a. Create and initialize a ProllyTree.
b. Implement functions for adding, updating, and deleting data in the ProllyTree.
c. Implement functions for merging and synchronizing ProllyTrees across the network.
d. Implement serialization and deserialization functions for efficient data transfer.

Implement the HNSWIndex functionality:
a. Create and initialize an HNSWIndex.
b. Implement functions for adding and removing vectors in the HNSWIndex.
c. Implement functions for nearest neighbor queries in the HNSWIndex.

Implement the DecentralizedVectorDatabase functionality:
a. Create and initialize a DecentralizedVectorDatabase with a ProllyTree and an HNSWIndex.
b. Implement functions for adding, updating, and deleting vectors in the database.
c. Implement functions for nearest neighbor queries.
d. Implement functions for managing data consistency across the network.

Define a communication protocol for nodes in the decentralized system:
a. Implement functions for sending and receiving data between nodes.
b. Implement functions for handling incoming data (e.g., merging ProllyTrees or updating the HNSWIndex).
c. Implement functions for handling network events, such as node join, leave, and failure.

Implement a main function to start a node in the decentralized system, initialize the DecentralizedVectorDatabase, and handle incoming requests.


# Backlog

- Cleanup commits and tag the first working version.

- Create a demo application that uses the decentralized vector database.
  - Create a Jupiter notebook to:
    1- Create two instances of Hnsw Index since Celestica doesn't support the notion of Collections. One to store image similarities and one to store text similarities.
    2- Load images from a dataset.
    3- Extract features of the images using a pre-trained model and insert them to the Hnsw Index.
    4- Search and display the results.
- Change the identifier type of the Hnsw Index from u64 to String to represent the hash of the image (CID).
- Reorganize the code (Hnsw) into modules. Write some test before doing the migration to make sure that the code is working as expected after the migration.
- Store and Load the flatten index in a IPFS node.
- Introduce the concept of Collections in Celestica. Each user may have different collections of images and text. Each collection is stored in a separate Hnsw Index.
- Access control: Each user can only access his/her own collections.
- Sharding: Each node in the network can store a subset of the collections. This can be achieved either by partitioning the data or using IPLD/ADL to store the data in a distributed manner.
- Add commands new commands to CLI: 
  - use <collection_name>, create <collection_name>, delete <collection_name>, list collections, list nodes, etc.
- Add support to delete vectors from the Hnsw Index.
- Create SDKs for different programming languages.
- Locality-awareness: Each node in the network can store a subset of the collections. This can be achieved either by partitioning the data or using IPLD/ADL to store the data in a distributed manner. Each node should be able to send the part of the query to the node that stores the corresponding collection/segment of the data. The node that receives the query should be able to send the query to the corresponding Hnsw Index and return the results to the calling node. The calling node which can be called Coordinator should be able to aggregate the results and return the final results to the user.
- Monitoring and logging?
    