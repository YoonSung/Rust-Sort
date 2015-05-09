struct QuickSort<'a> {
    arr: &'a mut [i32],
    //_marker: NoCopy
}

impl<'a> QuickSort<'a> {
    pub fn sort(&mut self) {
        //before error. why?
        let length = self.arr.len()-1;
        self._sort(0 as isize, length as isize);
    }

    fn _sort(&mut self, start_index : isize, end_index: isize) {
        if start_index < end_index {
            let pivot = self.partition(start_index as usize, end_index as usize);
            println!("pivot : {}", pivot);
            self._sort(start_index, pivot-1); 
            self._sort(pivot+1, end_index);
        }
    }

    fn partition(&mut self, start_index: usize, end_index: usize) -> isize{
        let pivot_index = start_index;
        let pivot_value = self.arr[pivot_index];

        let mut left_point_index = start_index+1;
        let mut right_point_index = end_index;

        while left_point_index < right_point_index {

            while  self.arr[right_point_index] > pivot_value && left_point_index < right_point_index{
                right_point_index -= 1;
            }
            if left_point_index == right_point_index {
                right_point_index -= 1;
            }

            while self.arr[left_point_index] < pivot_value && left_point_index < right_point_index {
                left_point_index += 1;
            }

            if left_point_index < right_point_index {
                self.swap(left_point_index, right_point_index);
            } 

        }
        println!("arr : {:?}", self.arr);
        println!("pivot_index : {}, pivot_value : {}, left_point_index : {}, right_point_index : {}", pivot_index, pivot_value, left_point_index, right_point_index);

        self.swap(right_point_index, pivot_index);

        return right_point_index as isize;
    }
            
    fn swap(&mut self, x : usize, y : usize) {
        let temp = self.arr[x];
    
        self.arr[x] = self.arr[y];
        self.arr[y] = temp;
    }
}

pub fn sort(array : &mut [i32]) {
    let mut quick_sort = QuickSort{arr : array};
    quick_sort.sort();
}
