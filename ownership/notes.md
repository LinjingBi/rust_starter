# ownership

A set of rules that govern how a rust program manages memory. some examples:
some languages may garbage collection that regularly looks for no-longer-used memory as the program runs; as for others, programmers must explicitly allocate and free the memory. As for rust, memory is managed through a system of ownership with a set of rules that compiler checks. if any of the rules are violated, the program won't compile. 

## the stack and heap
stack: first in last out. all data stored on the stack must have a known, fixed size. 
heap: use stack to store pointer, lenght, capacity. and content is stored in heap.