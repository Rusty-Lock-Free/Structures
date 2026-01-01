use std::fmt::Debug;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::{mem, ptr};

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
        // Allocate and initialize thread-local node to push onto the stack
        let mut new_box: Box<Node> = Box::new(Node {
            value,
            next: ptr::null(),
        });

        let new_ptr: *mut Node = &mut *new_box;

        loop {
            // Acquire ensures visibility of all writes that happened-before the successful publication of this head pointer.
            let old_ptr: *mut Node = self.head.load(Ordering::Acquire);
            new_box.next = old_ptr; //Safe write :  'new_box' is thread-local

            // Release on success to publish the update to other threads, Relaxed on failure because no synchronization required.
            if self
                .head
                .compare_exchange(old_ptr, new_ptr, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                //CAS succeeded: the stack now owns the node. Prevent reclamation by current thread.
                mem::forget(new_box);
                break;
            }
        }
    }

    // fn pop(& self) -> Option<is32> {
    //Box::new(x) allocates space for x on the heap, moves x into that space, and returns an owning pointer (Box<T>) to it.
    // }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack_values = String::new();
        stack_values.push('[');
        let mut curr: *const Node = self.head.load(Ordering::Acquire);
        while !curr.is_null() {
            unsafe {
                stack_values.push_str(&format!("{} -> ", (*curr).value));
                curr = (*curr).next
            };
        }
        stack_values.push_str("None");
        stack_values.push(']');

        f.debug_struct("Stack")
            .field("head", &stack_values)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_display() {
        let stack = Stack::new();
        for i in 1..=5 {
            stack.push(i);
            dbg!(&stack);
        }
        panic!()
    }
}
