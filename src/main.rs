use submillisecond::{extract::path::Path, router, Application};

fn index() -> &'static str {
    "Hello :D"
}

fn greet(Path((first, last, age)): Path<(String, String, u32)>) -> String {
    format!("welcome {first} {last}. You are {age} years old.")
}

fn parent_child() -> &'static str {
    "parent -> child :)"
}

fn not_found() -> &'static str {
    "cannot find endpoint"
}

fn main() -> std::io::Result<()> {
    Application::new(router! {
        // Standard
        GET "/" => index

        // With args
        GET "/users/:first/:last/:age" => greet

        // Nested
        "/parent" => {
            GET "/child" => parent_child
        }

        // catch
        _ => not_found
    })
    .serve("0.0.0.0:3000")
}
