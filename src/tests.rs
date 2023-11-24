#[cfg(test)]
mod tests {
    use super::super::*;

    fn size_formatter_test(bytes: BYTES, expected: &str, si: bool) {
        let mut buf = String::new();
        // SAFETY: Writing to a string __cannot__ fail
        if si {
            write!(&mut buf, "{:?}", bytes.into_size()).unwrap();
        } else {
            write!(&mut buf, "{}", bytes.into_size()).unwrap();
        }
        assert_eq!(buf, expected);
    }

    #[test]
    fn size_formatter_under_1kib() {
        size_formatter_test(495, "495b", false)
    }

    #[test]
    fn size_formatter_exactly_1_kib() {
        size_formatter_test(1024, "1kb", false)
    }

    #[test]
    fn size_formatter_under_1mib() {
        size_formatter_test(1024 * 512, "512kb", false)
    }

    #[test]
    fn size_formatter_exactly_1mib() {
        size_formatter_test(1024 * 1024, "1mb", false)
    }

    #[test]
    fn size_formatter_under_1gib() {
        size_formatter_test(299 * 1024 * 1024, "299mb", false)
    }

    #[test]
    fn size_formatter_exactly_1gib() {
        size_formatter_test(KIB.pow(3), "1gb", false)
    }

    #[test]
    fn size_formatter_under_1tib() {
        size_formatter_test(KIB.pow(3) * 128, "128gb", false)
    }

    #[test]
    fn size_formatter_exactly_1tib() {
        size_formatter_test(KIB.pow(4), "1tb", false)
    }

    #[test]
    fn size_formatter_under_1pib() {
        size_formatter_test(KIB.pow(4) * 256, "256tb", false)
    }

    #[test]
    fn size_formatter_exactly_1pib() {
        size_formatter_test(KIB.pow(5), "1pb", false)
    }

    #[test]
    fn exactsize_formatter_3pib_2gb_3b() {
        let mut buf = String::new();
        write!(&mut buf, "{}", (3 * PIB + 2 * GIB + 3 * B).into_longsize()).unwrap();
        assert_eq!(buf, "3pb 2gb 3b");
    }
}
