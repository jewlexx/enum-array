#[cfg(test)]
mod tests {
    use enum_array::EnumArray;

    #[derive(Debug, EnumArray, PartialEq, Eq)]
    enum TestEnum {
        RedGreenBlue,
        BlueGreenRed,
        GreenRedBlue,
    }

    #[test]
    fn test_array_match() {
        let array = [
            TestEnum::RedGreenBlue,
            TestEnum::BlueGreenRed,
            TestEnum::GreenRedBlue,
        ];
        let enum_array = TestEnum::to_array();

        assert_eq!(array, enum_array);
    }

    #[test]
    fn test_array_str_match() {
        let array = ["RedGreenBlue", "BlueGreenRed", "GreenRedBlue"];
        let enum_array = TestEnum::to_str_array();

        assert_eq!(array, enum_array);
    }

    #[test]
    fn test_array_snake_match() {
        let array = ["red_green_blue", "blue_green_red", "green_red_blue"];
        let enum_array = TestEnum::to_snake_array();

        assert_eq!(array, enum_array);
    }
}
