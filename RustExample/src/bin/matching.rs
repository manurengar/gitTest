fn main() {
    // Examples about matching pattern
    let mark = 95;
    let mut grade = 'N';
    match mark {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    }
    println!("{}", grade);

    // Declarations
    let _skin_color = match grade {
        'A' | 'B' => "white",
        'C' => "hispano",
        _ => "Nigger",
    };

    // Declarations using match guards
    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
    enum Grade {
        F,
        D,
        C,
        B,
        A,
        APlus,
    }

    let grade: Grade = Grade::A;
    match grade {
        g if g >= Grade::A => println!("Excellence Award"),
        g if g >= Grade::C => println!("Passed"),
        _ => println!("Failed"),
    }

    // Using structs
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y: 0 } => println!("Starting position"),
        Point { x: 0, y } => println!("On the X axis and y = {y}"),
        Point { x, y: 0 } => println!("On the Y axis and x={x}"),
        Point { x, y } => println!("At position x {x} y {y}"),
    }

    // Ignoring fields we don't care about
    enum CoolGuy {
        Cool,
        NotCool,
    }

    struct User {
        id: u64,
        cool: CoolGuy,
        is_admin: bool,
        username: String,
    }

    let user = User {
        id: 123,
        username: String::from("Niggoloko"),
        cool: CoolGuy::Cool,
        is_admin: true,
    };
    match user {
        User { is_admin: true, .. } => println!("This guy is an admin"),
        User {
            is_admin: false,
            cool: CoolGuy::Cool,
            ..
        } => println!("This guy is cool!"),
        _ => println!("You a bastard!"),
    }

    // Now if we wanted to apply a pattern (like the simple one) to an attribute of a struct
    // We apply a binding
    struct Enemy {
        health: u32,
    }

    let enemy = Enemy { health: 19 };
    match enemy {
        Enemy { health: h @ 1..=20 } => println!("this guy is one hit"),
        Enemy { health: 0 } => println!("Guy is dead"),
        Enemy { health } => println!("It was {health} points of HP"),
    }
}
