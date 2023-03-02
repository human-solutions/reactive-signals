# Goals

- A reactive system that is easy to use and understand.
- Tree and/or graph output of the running system.
- Good performance and low memory footprint.

# Arena Link Tree

An arena-based tree, where a vector contains all the allocated nodes and
each node contains the optional id of their parent, first child and
sibling. This constitutes the minimal information necessary for a tree.

Each node is 12 bytes + the size of the data.

NodeIds are used to identify nodes in the tree. They are 4 bytes with a maximum value of 2^32-1.
