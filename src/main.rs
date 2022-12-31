use submillisecond::{extract::Path, router, Application, NamedParam};

fn index() -> &'static str {
    "Hello :D"
}

fn greet(
    // Path((users, first, last, age)): Path<(String, String, String, u32)>, // Path(params): Path<std::collections::HashMap<String, String>>,
    Path((first)): Path<(String)>, // Path(params): Path<std::collections::HashMap<String, String>>,
) -> String {
    // format!("Welcome {first} {last}. You are {age} years old.");
    format!("Welcome {first}. You are years old.")
    // "test"
}

#[derive(NamedParam)]
struct Params {
    name: String,
    age: i32,
}

fn user_name_age(Params { name, age }: Params) -> String {
    format!("Hello {name}, You are {age} years old.")
}

fn parent_child() -> &'static str {
    "parent -> child :)"
}

fn not_found() -> &'static str {
    "cannot find endpoint"
}

fn main() -> std::io::Result<()> {
    Application::new(router! {
        // With args
        // GET "/:users/:first/:last/:age" => greet
        GET "/users/:first" => greet

        // Standard
        GET "/" => index

        // Named Params
        GET "/user/:name/:age" => user_name_age
        // Get "/user/:name/:age" => user_name_age

        // Nested
        "/parent" => {
            GET "/child" => parent_child
        }

        // catch
        _ => not_found
    })
    .serve("0.0.0.0:3000")
}
