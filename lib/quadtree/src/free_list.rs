use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct FreeElement<T> {
    pub element: T,
    next: i32
}

pub struct FreeList<T> {

    pub data: Vec::<FreeElement<T>>,
    pub first_free: i32,
}


impl<T> FreeList<T> {

    pub fn new() -> Self {
        FreeList {
            data: Vec::new(),
            first_free: -1
        }
    }

    pub fn insert(&mut self, element: T) -> i32 {
        if self.first_free != -1 {
            let index = self.first_free;
            self.first_free = self.data[self.first_free as usize].next;
            self.data[index as usize].element = element;
            return index;
        }
        else {
            let fe = FreeElement {
                element,
                next: -1
            };

            self.data.push(fe);
            return (self.data.len() - 1) as i32;
        }

    }

    pub fn erase(&mut self, n: i32) {

        self.data[n as usize].next = self.first_free;
        self.first_free = n;
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.first_free = -1;

    }

    pub fn range(&self) -> i32 {
        (self.data.len() - 1)as i32
    }
}


impl<T> Index<i32> for FreeList<T> {
    type Output = FreeElement<T>;

    fn index<'a>(&'a self, i: i32) -> &'a FreeElement<T> {
        &self.data[i as usize]
    }
}


impl<T> IndexMut<i32> for FreeList<T> {
    fn index_mut<'a>(&'a mut self, i: i32) -> &'a mut FreeElement<T> {
        &mut self.data[i as usize]
    }
}



#[cfg(test)]
mod test {





}
