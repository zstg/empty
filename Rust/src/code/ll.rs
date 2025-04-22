struct Node {
    data: i32,
    next: Option< Box<Node> >,
}

struct LL {
    head: Option < Box<Node> >,
}

impl LL {
    fn new() -> Self {
        LL { head:None }
    }

    fn push(mut self, data:i32) {
        let new = Box::new(
            Node{
                data: data, 
                next: self.head.take()
            }
        );
        // because Node.next can return NULL (since it's an Option), we use unwrap() to take the returned  value sanely
        self.head = Some(new);
    }

    fn push_pos(mut self, data:i32, pos: i8) {
        if self.head.is_none() // if the head is empty, i.e if the LL is empty
        || pos <= 0 {
            self.push(data); return;
        }

        let mut current = self.head;
        let mut index = 0;
    }

    fn pop(mut self) -> Option<i32> {
        self.head.take()
            .map(|node| {
                self.head = node.next;
                node.data
            })
    }

    fn is_empty(self) -> bool {
        return self.head.is_none();
    }
}
