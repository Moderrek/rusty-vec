mod moderr {
    pub struct Vec<T> {
        pub data: *mut T,
        pub length: usize,
        pub capacity: usize,
    }
    
    pub struct VecIterator<'a, T> {
        vec: &'a Vec<T>,
        index: usize,
        reverse: bool,
        end: bool
    }
    
    impl<'a, T> Iterator for VecIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            if self.index < self.vec.length {
                let value = unsafe { self.vec.data.add(self.index).as_ref() };
                if self.reverse {
                    if self.index == 0 {
                        self.end = true;
                        return value;
                    }
                    self.index -= 1;
                } else {
                    self.index += 1;
                }
                value
            } else {
                None
            }
        }
    }
    
    pub struct VecIteratorMut<'a, T> {
        vec: &'a mut Vec<T>,
        index: usize,
        reverse: bool,
        end: bool,
    }
    
    impl<'a, T> Iterator for VecIteratorMut<'a, T> {
        type Item = &'a mut T;
    
        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            if self.index < self.vec.length {
                let value = unsafe { self.vec.data.add(self.index).as_mut() };
                if self.reverse {
                    if self.index == 0 {
                        self.end = true;
                        return value;
                    }
                    self.index -= 1;
                } else {
                    self.index += 1;
                }
                value
            } else {
                None
            }
        }
    }
    
    impl<T> Default for Vec<T> {
        fn default() -> Self {
            Vec {
                data: std::ptr::null_mut(),
                length: 0,
                capacity: 0,
            }
        }
    }
    
    #[allow(dead_code)]
    impl<T> Vec<T> {
        pub fn new() -> Self {
            Vec::default()
        }
    
        pub fn with_capacity(capacity: usize) -> Self {
            let data = unsafe {
                let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
                std::alloc::alloc(layout) as *mut T
            };
    
            Vec {
                data,
                length: 0,
                capacity,
            }
        }
    
        pub fn push(&mut self, value: T) {
            if self.length == self.capacity {
                self.extend();
            }
    
            unsafe {
                let end = self.data.add(self.length);
                std::ptr::write(end, value);
            }
    
            self.length += 1;
        }
    
        pub fn pop(&mut self) -> Option<T> {
            if self.length == 0 {
                return None;
            }
    
            self.length -= 1;
            unsafe {
                let end = self.data.add(self.length);
                Some(std::ptr::read(end))
            }
        }
    
        pub fn len(&self) -> usize {
            self.length
        }
    
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
    
        pub fn capacity(&self) -> usize {
            self.capacity
        }
    
        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= self.length {
                return None;
            }
    
            unsafe { Some(&*self.data.add(index)) }
        }
    
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            if index >= self.length {
                return None;
            }
    
            unsafe { Some(&mut *self.data.add(index)) }
        }
    
        fn extend(&mut self) {
            let new_cap = if self.capacity == 0 { 1 } else { self.capacity * 2 };
            let new_data = unsafe {
                let layout = std::alloc::Layout::array::<T>(new_cap).unwrap();
                std::alloc::alloc(layout) as *mut T
            };
    
            for i in 0..self.length {
                unsafe {
                    let src = self.data.add(i);
                    let dst = new_data.add(i);
                    std::ptr::copy(src, dst, 1);
                }
            }
    
            if self.capacity != 0 {
                unsafe {
                    let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                    std::alloc::dealloc(self.data as *mut u8, layout);
                }
            }
    
            self.data = new_data;
            self.capacity = new_cap;
        }
    
        pub fn clear(&mut self) {
            for i in 0..self.length {
                unsafe {
                    let ptr = self.data.add(i);
                    std::ptr::drop_in_place(ptr);
                }
            }
    
            self.length = 0;
        }
    
        pub fn remove(&mut self, index: usize) -> Option<T> {
            if index >= self.length {
                return None;
            }
    
            unsafe {
                let ptr = self.data.add(index);
                let value = std::ptr::read(ptr);
                std::ptr::copy(ptr.add(1), ptr, self.length - index - 1);
                self.length -= 1;
                Some(value)
            }
        }
    
        pub fn insert(&mut self, index: usize, value: T) -> Option<()> {
            if index > self.length {
                return None;
            }
    
            if self.length == self.capacity {
                self.extend();
            }
    
            unsafe {
                let ptr = self.data.add(index);
                std::ptr::copy(ptr, ptr.add(1), self.length - index);
                std::ptr::write(ptr, value);
                self.length += 1;
            }
    
            Some(())
        }
    
        pub fn shrink_to_fit(&mut self) {
            if self.capacity == 0 {
                return;
            }
    
            if self.length == 0 {
                unsafe {
                    let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                    std::alloc::dealloc(self.data as *mut u8, layout);
                }
    
                self.data = std::ptr::null_mut();
                self.capacity = 0;
            } else {
                let new_data = unsafe {
                    let layout = std::alloc::Layout::array::<T>(self.length).unwrap();
                    std::alloc::alloc(layout) as *mut T
                };
    
                for i in 0..self.length {
                    unsafe {
                        let src = self.data.add(i);
                        let dst = new_data.add(i);
                        std::ptr::copy(src, dst, 1);
                    }
                }
    
                unsafe {
                    let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                    std::alloc::dealloc(self.data as *mut u8, layout);
                }
    
                self.data = new_data;
                self.capacity = self.length;
            }
        }
    
        pub fn as_slice(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.data, self.length) }
        }
    
        pub fn as_mut_slice(&mut self) -> &mut [T] {
            unsafe { std::slice::from_raw_parts_mut(self.data, self.length) }
        }
    
        pub fn get_unchecked(&self, index: usize) -> &T {
            unsafe { &*self.data.add(index) }
        }
    
        pub fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
            unsafe { &mut *self.data.add(index) }
        }
    
        pub fn iter(&self) -> VecIterator<T> {
            VecIterator {
                vec: self,
                index: 0,
                reverse: false,
                end: false
            }
        }
    
        pub fn iter_mut(&mut self) -> VecIteratorMut<T> {
            VecIteratorMut {
                vec: self,
                index: 0,
                reverse: false,
                end: false
            }
        }

        pub fn reverse(&self) -> VecIterator<T> {
            let index = self.length - 1;
            VecIterator { vec: self, index, reverse: true, end: false }
        }

        pub fn reverse_mut(&mut self) -> VecIteratorMut<T> {
            let index = self.length - 1;
            VecIteratorMut { vec: self, index, reverse: true, end: false }
        }

        pub fn to_std_vec(&self) -> std::vec::Vec<T>
        where
            T: Clone,
        {
            let mut vec = std::vec::Vec::with_capacity(self.length);
            for i in 0..self.length {
                vec.push(self.get_unchecked(i).clone());
            }
    
            vec
        }
    }
    
    impl<T> Drop for Vec<T> {
        fn drop(&mut self) {
            for i in 0..self.length {
                unsafe {
                    let ptr = self.data.add(i);
                    std::ptr::drop_in_place(ptr);
                }
            }
    
            if self.capacity != 0 {
                unsafe {
                    let layout = std::alloc::Layout::array::<i32>(self.capacity).unwrap();
                    std::alloc::dealloc(self.data as *mut u8, layout);
                }
            }
        }
    }
    
    impl<T> std::fmt::Debug for Vec<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_list().entries(self.as_slice()).finish()
        }
    }
    
    impl<T: Clone> Clone for Vec<T> {
        fn clone(&self) -> Self {
            let mut vec = Vec::with_capacity(self.length);
            for i in 0..self.length {
                vec.push(self.get_unchecked(i).clone());
            }
    
            vec
        }
    }
}



fn main() {
    let start = std::time::Instant::now();
    let mut std_vec = std::vec::Vec::new();
    std_vec.push("Hello".to_string());
    std_vec.push("World".to_string());
    std_vec.push("Rust".to_string());
    std_vec.push("Vec".to_string());
    std_vec.push("Alloc".to_string());

    println!("std::vec = {:?} created in {:?}", std_vec, start.elapsed());
    println!("std::vec Len = {}, cap = {}", std_vec.len(), std_vec.capacity());

    let start = std::time::Instant::now();
    let mut vec = moderr::Vec::new();
    vec.push("Hello".to_string());
    vec.push("World".to_string());
    vec.push("Rust".to_string());
    vec.push("Vec".to_string());
    vec.push("Alloc".to_string());

    println!("moderr::Vec = {:?} created in {:?}", vec, start.elapsed());
    println!("moderr::Vec Len = {}, cap = {}", vec.len(), vec.capacity());

    println!("moderr::Vec");
    for i in vec.iter() {
        println!("{}", i);
    }
    println!("moderr::Vec reverse");
    for i in vec.reverse() {
        println!("{}", i);
    }
}
