fn main() {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut n1 :usize = (m-1) as usize;
        let mut n2 :usize = (n-1) as usize;
        let mut cur :usize = nums1.len() -1;

        while(n1>=0 && n2>=0) {
            if nums1[n1] > nums2[n2] {
                nums1[cur--] =  nums1[n1--];
            } else {
                nums1[cur--] = nums2[n2--];
            }
                
        }
    }
}
