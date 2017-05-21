// This is your actual server application. By default it just runs the app you
// created with the `routes!` macro. You can edit it to have it do additional
// set-up or tear-down as necesary.

extern crate cargonauts;
extern crate colors;

fn main() {
    cargonauts::serve(colors::routes).unwrap();
}