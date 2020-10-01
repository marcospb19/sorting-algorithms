use rand::seq::SliceRandom;

fn main() {
    let len = 32;
    let mut v: Vec<i32> = (1..=len).collect();

    let mut sorted = v.clone();
    sorted.sort();

    v.shuffle(&mut rand::thread_rng());

    sort::heap_sort(&mut v[..]); // !
    println!("{:?}", std::mem::size_of::<usize>());

    assert_eq!(v, sorted);
    println!("Sorted!");
}

#[allow(dead_code)]
mod sort {
    pub fn heap_sort(v: &mut [i32]) {
        fn sift_up(v: &mut [i32], i: usize) {
            if i == 0 {
                return;
            }
            let parent_index = (i - 1) / 2;
            if v[i] > v[parent_index] {
                v.swap(i, parent_index);
                println!("{:?}", v);
                sift_up(v, parent_index);
            }
        }

        // fn sift_down(v: &mut [i32], i: usize) {
        //     let left = i * 2 + 1;
        //     if left < v.len() {
        //         if v[i] < v[left] {
        //             v.swap(i, left);
        //             println!("{:?} down left", v);
        //             sift_down(v, left);
        //         }

        //         let right = i * 2 + 2;
        //         if right < v.len() {
        //             if v[i] < v[right] {
        //                 v.swap(i, right);
        //                 println!("{:?} down right", v);
        //                 sift_down(v, right);
        //             }
        //         }
        //     }
        // }

        // fn assert_heap(v: &mut [i32]) -> bool {
        //     for i in 1..v.len() {
        //         if v[i] > v[(i - 1) / 2] {
        //             eprintln!("{:?} {}", i, (i - 1) / 2);
        //             return false;
        //         }
        //     }
        //     true
        // }

        // Creates heap in-place
        for i in (1..v.len()).rev() {
            sift_up(v, i);
        }
        println!("Heap created");

        // for i in (1..v.len()).rev() {
        //     let index = (i - 1) / 2;
        //     if v[i] > v[index] {
        //         println!("{} {} error {} {}", i, index, v[i], v[index]);
        //     }
        // }

        // // Sorting
        // for j in (1..v.len()).rev() {
        //     v.swap(0, j);
        //     println!("{:?} j picked", v);
        //     sift_down(&mut v[..j], 0);
        // }

        println!("Heap: {:?}", v);
    }

    // -
    // -
    // -
    // -
    // -
    // -

    pub fn bubble_sort(v: &mut [i32]) {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                if v[i] > v[j] {
                    v.swap(i, j);
                }
            }
        }
    }

    pub fn insertion_sort_v1(v: &mut [i32]) {
        for index in 1..v.len() {
            let mut i = index;
            while i > 0 && v[i] < v[i - 1] {
                v.swap(i, i - 1);
                i -= 1;
            }
        }
    }

    pub fn insertion_sort_v2(v: &mut [i32]) {
        for index in 1..v.len() {
            let save = v[index];
            let mut i = index;

            while i > 0 && save < v[i - 1] {
                v[i] = v[i - 1];
                i -= 1;
            }
            v[i] = save;
        }
    }

    pub fn insertion_sort_v3(v: &mut [i32]) {
        for slice in 1..v.len() {
            let save = v[slice];

            let index = v[0..slice].binary_search(&save).unwrap_or_else(|i| i);

            for j in (index + 1..=slice).rev() {
                v[j] = v[j - 1];
            }
            v[index] = save;
        }
    }

    pub fn merge_sort(v: &mut [i32]) {
        if v.len() <= 16 {
            insertion_sort_v3(v);
            return;
        }

        let index = v.len() / 2;
        merge_sort(&mut v[..index]);
        merge_sort(&mut v[index..]);

        let mut i = 0;
        let mut j = index;

        let mut temp: Vec<i32> = Vec::new();
        temp.reserve(v.len());

        loop {
            if v[i] < v[j] {
                temp.push(v[i]);
                i += 1;

                if i == index {
                    while j < v.len() {
                        temp.push(v[j]);
                        j += 1;
                    }
                    break;
                }
            } else {
                temp.push(v[j]);
                j += 1;

                if j == v.len() {
                    while i < index {
                        temp.push(v[i]);
                        i += 1;
                    }
                    break;
                }
            }
        }

        for i in 0..v.len() {
            v[i] = temp[i];
        }
    }

    pub fn quick_sort(v: &mut [i32]) {
        if v.len() <= 1 {
            // insertion_sort_v3(v);
            return;
        }

        let choose_pivot = |_v: &[i32]| -> usize { 0 };
        let pivot_index = choose_pivot(v);

        v.swap(0, pivot_index);
        let pivot = v[0];

        let mut lo = 1;
        let mut hi = v.len() - 1;

        loop {
            while lo <= hi && v[lo] < pivot {
                lo += 1;
            }
            while lo <= hi && v[hi] > pivot {
                hi -= 1;
            }

            if lo < hi {
                v.swap(lo, hi);
            } else {
                v.swap(0, hi);
                quick_sort(&mut v[..hi]);
                quick_sort(&mut v[lo..]);
                break;
            }
        }
    }
}
