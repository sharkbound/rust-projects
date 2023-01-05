///
/// video resource:
/// https://www.youtube.com/watch?v=8_HPKGZGM5I
///

fn main() {}

fn inline_if_pattern_matching() {
    // inline w/ if statement
    let value = Some(9);
    if let Some(val2) = value {
        // code here if VALUE is Some(val2)
    } else {
        // code here if VALUE is None
    }
}

fn inline_variable_pattern_matching() {
    // inline w/ variable
    let value = Some(9);
    let x = if let Some(val1) = value { val1 + 1 } else { -1 };
}

fn number_range_pattern_matching() {
    let num = 9;
    match num {
        0..=3 => { /*0-3*/ }
        4..=6 => { /*4-6*/ }
        7..=9 => { /*7-9*/ }
        _ => { /*not in any other pattern or range above*/ }
    }
}

fn destructure_struct_enum_pattern_matching() {
    enum ShapeWithNamed {
        Circle { radius: i32 },
        Rect { width: i32, height: i32 },
    }

    // destructuring enum with named values w/ match
    match (ShapeWithNamed::Rect { height: 10, width: 20 }) {
        ShapeWithNamed::Rect { width, height } => {
            // code here, can use width/height
        }
        ShapeWithNamed::Circle { radius } => {
            // code here, can use radius
        }
    }
}

fn destruct_tuple_enum_pattern_matching() {
    enum ShapeWithTuple {
        Circle(i32),
        Rect(i32, i32),
    }

    // destructuring tuple enum with values w/ match
    match ShapeWithTuple::Rect(10, 20) {
        ShapeWithTuple::Rect(x, y) => {
            // code here, can use width/height
        }
        ShapeWithTuple::Circle(radius) => {
            // code here, can use radius
        }
    }
}

// can use & on &(x, y) and &(i32, i32)
fn function_args_destructuring((x, y): (i32, i32)) {
    // can use `x` and `y`
}


struct Item {
    id: i32,
    name: String,
}

// destructuring a struct into its values, can use `..` to ignore other values
fn function_args_struct_destructuring(Item { name, id }: Item) {}


