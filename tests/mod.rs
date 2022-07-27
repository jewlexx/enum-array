#[cfg(test)]
mod tests {
    use enum_array::EnumArray;

    #[derive(EnumArray)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn test_array_match() {
        let array = [TestEnum::A, TestEnum::B, TestEnum::C];

        assert_eq!(array, TestEnum.to_array());
    }
}
