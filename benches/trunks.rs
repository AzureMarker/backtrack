use backtrack::trunks::Trunk;
use criterion::{Criterion, criterion_group, criterion_main};

/// Generate benchmark functions given a function name and the config path
macro_rules! benchmark {
    // This signature shows that it takes in 1 or more function names and expressions
    // in the format `function_name: expression` with commas between each set.
    ( $( $fn_name:ident: $file:expr ),+ ) => {
        $(
            fn $fn_name(b: &mut Criterion) {
                benchmark_config($file, b);
            }

        )*

        criterion_group!(benches, $($fn_name),*);
        criterion_main!(benches);
    }
}

/// Read the Trunk config and run the benchmark
fn benchmark_config(filename: &str, c: &mut Criterion) {
    // Read in the trunk once so we don't accidentally benchmark the IO
    let trunk = Trunk::read_from_file(filename).unwrap();

    // Benchmark the function multiple times
    c.bench_function(filename, move |b| b.iter(||
        // black_box makes sure the compiler doesn't optimize away the result of solve()
        criterion::black_box(
            backtrack::solve(trunk.clone())
        )
    ));
}

benchmark!(
    ten_by_six_cannot:              "data/10-6-cannot.txt",
    ten_by_seven_cannot:            "data/10-7-cannot.txt",
    eleven_by_four:                 "data/11-4.txt",
    eleven_by_four_cannot:          "data/11-4-cannot.txt",
    eleven_by_fourteen_full:        "data/11-14-full.txt",
    eleven_by_fourteen_full_b:      "data/11-14-full-B.txt",
    eleven_by_fourteen_full_c:      "data/11-14-full-C.txt",
    eleven_by_fourteen_not_full:    "data/11-14-notfull.txt",
    thirteen_by_ten_full:           "data/13-10-full.txt",
    thirteen_by_ten_not_full:       "data/13-10-notfull.txt",
    default_1:                      "data/default-1.txt",
    default_2:                      "data/default-2.txt",
    default_3:                      "data/default-3.txt"
);
