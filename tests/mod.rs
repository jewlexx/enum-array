#[cfg(test)]
mod tests {
    use enum_array::EnumArray;

    #[derive(Debug, EnumArray, PartialEq, Eq)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn test_array_match() {
        let array = [TestEnum::A, TestEnum::B, TestEnum::C];
        let enum_array = TestEnum::to_array();

        assert_eq!(array, enum_array);
    }
}