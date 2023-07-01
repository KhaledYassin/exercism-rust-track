use ocr_numbers as ocr;
fn main() {
    #[rustfmt::skip]
    // let input = " _ \n".to_string() +
    //                     "| |\n" +
    //                     "|_|\n" +
    //                     "   ";
    let input = "    _  _ \n".to_string() +
                        "  | _| _|\n" +
                        "  ||_  _|\n" +
                        "         \n" +
                        "    _  _ \n" +
                        "|_||_ |_ \n" +
                        "  | _||_|\n" +
                        "         \n" +
                        " _  _  _ \n" +
                        "  ||_||_|\n" +
                        "  ||_| _|\n" +
                        "         ";

    println!("{:?}", ocr::convert(&input))
}
