pub enum List {
    Empty,
    Elem(i32, Box<List>), // To bound the List size as Box is of defined size
}
