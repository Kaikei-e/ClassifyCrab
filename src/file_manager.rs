pub mod file_searcher;
mod file_calculator;

use file_searcher::zip_searcher;

mod inspector{
    zip_searcher!();

    file_calculator!();
}

mod extractor{
    fn zip_extractor(){}
}

mod classifier{
    fn file_classifier(){}
}



