use fibula::run;

fn main() {
    pollster::block_on(run()).expect("TODO: panic message");
}
