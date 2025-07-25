use std::fmt::{self, Debug, Display};

struct SinglyLinkedListNode<T>
where
    T: Display + Copy + PartialEq,
{
    data: T,
    next: Option<Box<SinglyLinkedListNode<T>>>,
}

// class functions
impl<T> SinglyLinkedListNode<T>
where
    T: Display + Copy + PartialEq,
{
    fn new(data: T) -> SinglyLinkedListNode<T> {
        Self {
            data,
            next: Option::None,
        }
    }
}

// instance functions
impl<T> SinglyLinkedListNode<T>
where
    T: Display + Copy + PartialEq,
{
    fn set_next(&mut self, next: Option<Box<SinglyLinkedListNode<T>>>) {
        self.next = next;
    }

    fn equals(&self, other: &SinglyLinkedListNode<T>) -> bool {
        self.data == other.data
    }
}

impl<T> fmt::Display for SinglyLinkedListNode<T>
where
    T: Display + Copy + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
struct SinglyLinkedList<T>
where
    T: Display + Copy + PartialEq,
{
    head: Option<Box<SinglyLinkedListNode<T>>>,
    count: u32,
}

// class functions
impl<T> SinglyLinkedList<T>
where
    T: Display + Copy + PartialEq,
{
    pub fn new() -> SinglyLinkedList<T> {
        Self {
            head: Option::None,
            count: 0,
        }
    }
}

// instance functions
impl<T> SinglyLinkedList<T>
where
    T: Display + Copy + PartialEq,
{
    pub fn add_front(&mut self, data: T) {
        let mut new_node = SinglyLinkedListNode::new(data);
        new_node.set_next(self.head.take());
        let new_node_ptr = Box::new(new_node);
        self.head = Option::Some(new_node_ptr);
        self.count += 1;
    }

    pub fn remove_front(&mut self) {
        if let Some(mut old_head) = self.head.take() {
            self.head = old_head.next.take();
            self.count -= 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            self.count -= 1;
            old_head.data
        })
    }

    pub fn add_at(&mut self, data: T, index: u32) {
        if index > self.count {
            return;
        }

        if index == 0 {
            self.add_front(data);
            return;
        }

        if index == self.count {
            self.add_back(data);
            return;
        }

        let mut current_index: u32 = 0;
        let mut current_node = self.head.as_mut().unwrap();

        while current_index < index - 1 {
            match current_node.next.as_mut() {
                Some(next) => {
                    current_node = next;
                    current_index += 1;
                }
                None => return,
            }
        }

        let mut new_node = Box::new(SinglyLinkedListNode::new(data));
        new_node.set_next(current_node.next.take());
        current_node.set_next(Some(new_node));
        self.count += 1;
    }

    pub fn add_back(&mut self, data: T) {
        let new_node = Box::new(SinglyLinkedListNode::new(data));

        match self.head {
            Some(_) => {
                let mut current = self.head.as_mut().unwrap();

                while current.next.is_some() {
                    let temp = current.next.as_mut().unwrap();
                    current = temp;
                }
                current.set_next(Option::Some(new_node));
            }
            None => {
                self.head = Option::Some(new_node);
            }
        }
        self.count += 1;
    }

    pub fn remove_back(&mut self) {
        let _ = self.pop_back();
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(last) if last.next.is_none() => {
                let last = self.head.take().unwrap();
                self.head = None;
                self.count -= 1;
                Some(last.data)
            }
            Some(_) => {
                let mut current = self.head.as_mut().unwrap();
                while let Some(next) = current.next.as_ref() {
                    if next.next.is_none() {
                        let last = current.next.take().unwrap();
                        self.count -= 1;
                        return Some(last.data);
                    }
                    current = current.next.as_mut().unwrap();
                }
                None
            }
        }
    }

    pub fn peek_front(&self) -> Option<T> {
        if let Some(value) = &self.head {
            Some(value.data)
        } else {
            None
        }
    }

    pub fn peek_back(&self) -> Option<T> {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.next.is_none() {
                return Some(node.data);
            }
            current = &node.next;
        }

        None
    }

    pub fn remove_at(&mut self, index: u32) {
        if index >= self.count {
            return;
        }

        if index == 0 {
            self.remove_front();
            return;
        }

        if index == self.count - 1 {
            self.remove_back();
            return;
        }

        let mut current_index: u32 = 0;
        let mut current_node = self.head.as_mut().unwrap();

        while current_index < index - 1
            && let Some(_) = current_node.next
        {
            match current_node.next.as_mut() {
                Some(next) => {
                    current_node = next;
                    current_index += 1;
                }
                None => return,
            }
        }

        let mut removed_node = current_node.next.take().unwrap();
        current_node.set_next(removed_node.next.take());
        self.count -= 1;
    }

    pub fn equals(&self, other: &SinglyLinkedList<T>) -> bool {
        if self.count != other.count {
            return false;
        }

        let mut self_node = &self.head;
        let mut other_node = &other.head;

        while let (Some(self_next), Some(other_next)) = (self_node, other_node) {
            if !self_next.equals(other_next) {
                return false;
            }

            self_node = &self_next.next;
            other_node = &other_next.next;
        }

        self_node.is_none() && other_node.is_none()
    }
}

impl<T> fmt::Display for SinglyLinkedList<T>
where
    T: Display + Copy + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count == 0 {
            return write!(f, "{}", "Empty");
        }
        let mut result_str = String::from("(head)");
        let mut current = &self.head;

        while let Some(node) = current {
            result_str += &format!(" {} ->", node.data);
            current = &node.next;
        }

        write!(f, "{}", result_str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_list_with_values<T>(values: &[T]) -> SinglyLinkedList<T>
    where
        T: Display + Copy + PartialEq,
    {
        let mut list = SinglyLinkedList::new();
        for &value in values.iter().rev() {
            list.add_front(value);
        }
        list
    }

    #[test]
    fn test_empty_list_display() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!("Empty", list.to_string())
    }

    #[test]
    fn test_list_display() {
        let list: SinglyLinkedList<i32> = get_list_with_values::<i32>(&[1, 4, 2]);
        assert_eq!("(head) 1 -> 4 -> 2 ->", list.to_string())
    }

    #[test]
    fn test_peek_front() {
        let list: SinglyLinkedList<i32> = get_list_with_values::<i32>(&[1, 4, 2]);
        let front: Option<i32> = list.peek_front();
        assert_eq!(list.count, 3);
        assert!(front.is_some());
        assert_eq!(front.unwrap(), 1);
    }

    #[test]
    fn test_peek_front_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let front: Option<i32> = list.peek_front();
        assert_eq!(list.count, 0);
        assert!(front.is_none());
    }

    #[test]
    fn test_peek_back() {
        let list: SinglyLinkedList<i32> = get_list_with_values::<i32>(&[1, 4, 2]);
        let front: Option<i32> = list.peek_back();
        assert_eq!(list.count, 3);
        assert!(front.is_some());
        assert_eq!(front.unwrap(), 2);
    }

    #[test]
    fn test_peek_back_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let front: Option<i32> = list.peek_back();
        assert_eq!(list.count, 0);
        assert!(front.is_none());
    }

    #[test]
    fn test_empty_list() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let head = list.peek_front();
        assert_eq!(list.count, 0);
        assert!(head.is_none());
    }

    #[test]
    fn test_add_front() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1, 4, 2]);
        list.add_front(5);
        let head = list.peek_front();
        assert_eq!(list.count, 4);
        assert_eq!(head.unwrap(), 5);
    }

    #[test]
    fn test_add_front_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        list.add_front(1);
        let head = list.peek_front();
        assert_eq!(list.count, 1);
        assert_eq!(head.unwrap(), 1);
    }

    #[test]
    fn test_remove_front() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1, 4, 2]);
        list.remove_front();
        let head: Option<i32> = list.peek_front();
        assert_eq!(list.count, 2);
        assert_eq!(head.unwrap(), 4);
    }

    #[test]
    fn test_remove_front_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        list.remove_front();
        let head = list.peek_front();
        assert_eq!(list.count, 0);
        assert!(head.is_none());
    }

    #[test]
    fn test_pop_front() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1, 4, 2]);
        let popped: Option<i32> = list.pop_front();
        let head = list.peek_front();
        assert_eq!(popped.unwrap(), 1);
        assert_eq!(list.count, 2);
        assert_eq!(head.unwrap(), 4);
    }

    #[test]
    fn test_pop_front_single_element() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1]);
        let popped: Option<i32> = list.pop_front();
        let head = list.peek_front();
        assert_eq!(popped.unwrap(), 1);
        assert_eq!(list.count, 0);
        assert!(head.is_none());
    }

    #[test]
    fn test_pop_front_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let popped: Option<i32> = list.pop_front();
        let head = list.peek_front();
        assert!(popped.is_none());
        assert_eq!(list.count, 0);
        assert!(head.is_none());
    }

    #[test]
    fn test_add_at() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1, 3]);
        let expected_list = get_list_with_values(&[0, 1, 2, 3, 4, 5]);
        list.add_at(2, 1);
        list.add_at(0, 0);
        list.add_at(5, 4);
        list.add_at(4, 4);
        assert!(list.equals(&expected_list));
    }

    #[test]
    fn test_add_back() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1, 4, 2]);
        list.add_back(5);
        let tail = list.peek_back();
        assert_eq!(list.count, 4);
        assert_eq!(tail.unwrap(), 5);
    }

    #[test]
    fn test_add_back_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        list.add_back(1);
        let tail = list.peek_back();
        assert_eq!(list.count, 1);
        assert_eq!(tail.unwrap(), 1);
    }

    #[test]
    fn test_pop_back() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1, 4, 2]);
        let popped: Option<i32> = list.pop_back();
        let tail = list.peek_back();
        assert_eq!(popped.unwrap(), 2);
        assert_eq!(list.count, 2);
        assert_eq!(tail.unwrap(), 4);
    }

    #[test]
    fn test_pop_back_single_element() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[1]);
        let popped: Option<i32> = list.pop_back();
        let tail = list.peek_back();
        assert_eq!(popped.unwrap(), 1);
        assert_eq!(list.count, 0);
        assert!(tail.is_none());
    }

    #[test]
    fn test_pop_back_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let popped: Option<i32> = list.pop_back();
        let tail = list.peek_back();
        assert!(popped.is_none());
        assert_eq!(list.count, 0);
        assert!(tail.is_none());
    }

    #[test]
    fn test_remove_at() {
        let mut list: SinglyLinkedList<i32> = get_list_with_values(&[0, 1, 2, 3, 4, 5]);
        let expected_list = get_list_with_values(&[1, 3]);
        list.remove_at(2);
        list.remove_at(3);
        list.remove_at(3);
        list.remove_at(0);
        assert!(list.equals(&expected_list));
    }
}
