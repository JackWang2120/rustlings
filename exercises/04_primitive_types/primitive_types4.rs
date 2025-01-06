fn main() {
    // You can optionally experiment here.
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        /*在 Rust 中，切片（slice）是一种对数组或向量的一部分的引用。切片本身并不拥有数据，它只是引用了原始数组或向量的一部分。因此，切片必须是一个引用类型。*/
        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];//在前面加上 &，你创建了一个对这个切片的引用。这样你就可以在不拥有原始数据的情况下操作切片。

        assert_eq!([2, 3, 4], nice_slice);
    }
}
