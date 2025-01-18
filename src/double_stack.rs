use std::fmt;

pub struct DoubleStack<T : Copy> {
    stack: Vec<Vec<T>>
}

impl<T: Copy> DoubleStack<T> {
    pub fn new() -> DoubleStack<T> {
        DoubleStack {
            stack: vec![Vec::new()]
        }
    }

    /*
        Simplest case, push onto whatever is the most recent
        stack frame
     */
    pub fn push_simple(&mut self, value: T) {
        let top_index = self.stack.len() - 1;
        
        let top = &mut self.stack[top_index];

        top.push(value);
    }

    /*
        Push the value by creating a new stack frame, and 
        add it as an element
     */
    pub fn push_new_layer(&mut self, value: T) {
        let new_layer = vec![value];

        if self.stack.len() == 1 && self.stack[0].len() == 0 {
            self.stack.pop();
        }

        self.stack.push(new_layer);
    }

    /*
        No case for wanting to pop an entire frame at once,
        since it may lose data.

        When a frame loses all elements, want to destroy
        the stack frame of empty elements.
     */
    pub fn pop(&mut self) -> T {
        let top_index = self.stack.len() - 1;

        let val = self.stack[top_index].pop().unwrap();

        // If that made stack frame empty, remove it
        if self.stack[top_index].len() == 0 {
            self.stack.pop();
        }

        val
    }

    pub fn is_empty(&self) -> bool {
        self.stack.len() <= 0
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl<T: Copy + fmt::Display> fmt::Display for DoubleStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack_str = String::new();

        let mut i = 0;
        for frame in &self.stack {
            stack_str.push_str(&format!("LAYER: {i} ["));
            for item in frame {
                stack_str.push_str(&format!("{item}, "));
            }
            stack_str.push_str("] \n");

            i = i + 1;
        }

        write!(f, "{stack_str}")
    }
}

