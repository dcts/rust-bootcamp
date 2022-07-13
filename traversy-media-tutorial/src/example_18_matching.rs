pub fn run() {
  // MATCHING DOESNT CONSUME! 
  // Think of it as SWITCH statement
  //
  // IF does not consume (it only reads the data)
  // so why should SWITCH consume? ;)

  // Matching Enums (inline)
  let dir: Direction = Direction::Down;
  match dir {
    Direction::Right => println!("moving right"), // comma required
    Direction::Left => println!("moving left"), // comma required
    _ => println!("Invalid move. Only left and right supported.")
  }

  // Matching Enums (with codeblock)
  match dir {
    Direction::Right => {
      // do somethings for RIGHT
    }
    Direction::Left => {
      // do something for LEFT
    }
    _ => {
      // do something for all other cases
    }
  }

  // Match can return something
  let match_result = match dir {
    Direction::Right => "right",
    Direction::Left => "left",
    Direction::Up => "up",
    Direction::Down => "down",
  }; // semicolon here since this is an expression

  // MATCHING OVER OTHER VARIABLE TYPES
  // boolean
  let my_bool = true;
  match my_bool {
    true => println!("this bool is true"),
    false => println!("this bool is false")
  }
  // numeric values
  let my_num = 2.33;
  match my_num {
    1 => println!("this num is 1"),
    1..2 => println!("this num is between 1 and 2 (excluding 2)"),
    1..=2 => println!("this num is between 1 and 2 (including 2)"),
  }
  // String
  let my_string = String::from("A");
  match my_string {
    String::from("A") => println!("hello from A"),
    String::from("b") => println!("hello from b"),
    // only one pipe for OR
    String::from("a") | String::from("b") => println!("hello from A or b"),
    _ => println!("string doesnt match. Default behavior")
  }
  // String literals
  let user_name = "Andy";
	match user_name {
		"Andy" => println!("Hello Andy!"),
		_ => println!("Hello everyone else!"),
	}

  // chars
  let my_char = 'x';
  match my_char {
    'a'..='z' => println!("lowercase alphabetic char"),
    'A'..='Z' => println!("uppercase alphabetic char"),
		_ => println!("all other chars"),
  }

  // matching multiple values => pack them in a tuple
  let dir1 = Symbol::Heart;
  let dir2 = Symbol::Cross;
  match (dir1, dir2) {
    (Symbol::Heart, Symbol::Cross) => println!("Heart + Cross"),
    (Symbol::Heart, Symbol::Heart) => println!("Heart + Heart"),
    (Symbol::Cross, Symbol::Heart) => println!("Cross + Heart"),
    (Symbol::Cross, Symbol::Cross) => println!("Cross + Cross"),
  }
  // Alternatively you can also chain matches
  match dir1 {
    Symbol::Heart => match dir2 {
      Symbol::Cross => println!("Heart + Cross"),
      Symbol::Heart => println!("Heart + Heart"),
    }
    Symbol::Cross => match dir2 {
      Symbol::Heart => println!("Cross + Heart"),
      Symbol::Cros => println!("Cross + Cross"),
    }
  }

  // use if inside match, to add conditional from "outside"
  let env = "prod";
  let performance = 99.5;
  match env {
    "prod" if performance < 80 => println!("WARN : performance less than 80%"),
    "prod" if performance < 95 => println!("WARN : performance between 80-95%"),
    "prod" if performance >= 95 => println!("current performance {}", performance),
    "env" => println!("dev environment doesnt measure performance"),
    _ => println!("nonmatching string literal. Please use an enum next time! ;)"),
  }

  // alternative 1 (using if else in block)
  match env {
    "prod" => {
      if performance < 80 {
        println!("WARN : performance less than 80%");
      } else if performance < 95 {
        println!("WARN : performance between 80-95%");
      } else {
        println!("current performance {}", performance);
      }
    }
    "env" => println!("dev environment doesnt measure performance"),
    _ => println!("nonmatching string literal. Please use an enum next time! ;)"),
  }

  // alternative 2 (nesting match)
  match env {
    "prod" => match performance {
      0..80 => println!("WARN : performance less than 80%"),
      80..95 => println!("WARN : performance between 80-95%"),
      _ => println!("current performance {}", performance)
    }
    "env" => println!("dev environment doesnt measure performance"),
    _ => println!("nonmatching string literal. Please use an enum next time! ;)"),
  }
}

enum Symbol {
  Heart,
  Cross
}
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
