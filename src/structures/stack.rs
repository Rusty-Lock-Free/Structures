use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node {
    value: i32,
    next: *const Node,
}

struct Stack {
    head: AtomicPtr<Node>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            head: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    fn push(&self, value: i32) {
        // Allocate and initialize new node to push onto the stack
        let new: *mut Node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null(),
        }));

        loop {
            // Acquire ensures visibility of all writes that happened-before the successful publication of this head pointer.
            let old: *mut Node = self.head.load(std::sync::atomic::Ordering::Acquire);
            unsafe {
                (*new).next = old;
            };

            // Release on success to publish the update to other threads, Relaxed on failure because no synchronization required.
            if self
                .head
                .compare_exchange(old, new, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }

    // fn pop(& self) -> Option<is32> {
    //Box::new(x) allocates space for x on the heap, moves x into that space, and returns an owning pointer (Box<T>) to it.
    // }
}
