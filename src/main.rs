use prompt::run;

fn main() {
    if run().is_err() {
        println!(); // print an empty line in case of an error
    };
}
