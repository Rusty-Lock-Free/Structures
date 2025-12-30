use std::sync::atomic::{AtomicPtr, Ordering};

struct Node {
    value : i32,
    next : * const Node,
}

struct Stack {
    head : AtomicPtr<Node>
}

impl Stack {
    fn new() -> Stack {
        Stack { head : AtomicPtr::new(std::ptr::null_mut()) }
    }
    fn push(& self, value : i32) {
        loop{
            let old : * mut Node = self.head.load(std::sync::atomic::Ordering::Acquire); //ensures writes to node x in head by thread that initialized x are done before reading head.
            let new : * mut Node = Box::into_raw(Box::new(Node{value, next : old})); //allocate and initialize new node to push onto the stack
            if self.head.compare_exchange(old, new, Ordering::Release, Ordering::Relaxed).is_ok() { break; } // release on success to publish, relaxed on failure because no synchronization required.
        }
    }

    // fn pop(& self) -> Option<i32> {
    //
    // }
}