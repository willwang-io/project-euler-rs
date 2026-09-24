macro_rules! run_problem {
    ($path:literal) => {
        #[path = $path]
        mod selected_problem;

        fn main() {
            selected_problem::run();
        }
    };
}

run_problem!("problems/0101-0200/p0104.rs");
