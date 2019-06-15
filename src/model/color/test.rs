use super::*;

#[test]
fn test_color_cross() {
    let samples: Vec<(&str, &str, &str)> = vec![
        // with black is black
        ("#000000", "#000000", "#000000"),
        ("#ff0000", "#000000", "#000000"),
        ("#00ff00", "#000000", "#000000"),
        ("#0000ff", "#000000", "#000000"),
        // with white is itself
        ("#000000", "#ffffff", "#000000"),
        ("#ff0000", "#ffffff", "#ff0000"),
        ("#00ff00", "#ffffff", "#00ff00"),
        ("#0000ff", "#ffffff", "#0000ff"),
        // with other primary color is black
        ("#ff0000", "#00ff00", "#000000"),
        ("#00ff00", "#0000ff", "#000000"),
        ("#0000ff", "#ff0000", "#000000"),
    ];

    for s in &samples {
        let x = Color::parse(s.0).unwrap().unwrap();
        let y = Color::parse(s.1).unwrap().unwrap();
        let to_be = Color::parse(s.2).unwrap().unwrap();
        let actual = x.cross(&y);

        assert_eq!(
            actual, to_be,
            "cross '{}' and '{}' should be '{}', but '{}'",
            x, y, to_be, actual
        );
        
        let actual_swap = y.cross(&x);

        assert_eq!(
            actual, actual_swap,
            "should x.closs(&y) == y.cross(&x), but '{}' and '{}'",
            actual, actual_swap
        )
    }
}
