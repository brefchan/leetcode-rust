mod maximize_the_confusion_of_an_exam;
mod maximum_acerage_subarray;
use maximize_the_confusion_of_an_exam::max_consecutive_answers;
use maximum_acerage_subarray::find_max_average;
fn main() {
    println!("{}", find_max_average([0, 1, 1, 3, 3].to_vec(), 4));
}
