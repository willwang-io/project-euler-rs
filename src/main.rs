macro_rules! run_problem {
    ($path:literal) => {
        #[path = $path]
        mod selected_problem;

        fn main() {
            selected_problem::run();
        }
    };
}

run_problem!("problems/0001-0100/p0100.rs");
