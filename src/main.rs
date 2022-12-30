use submillisecond::{extract::Path, router, Application};

fn index() -> &'static str {
    "Hello :D"
}

fn greet(Path((first, last, age)): Path<(String, String, u32)>) -> String {
    format!("welcome {first} {last}. You are {age} years old.")
}

fn parent_child() -> &'static str {
    "parent -> child :)"
}

fn main() -> std::io::Result<()> {
    Application::new(router! {
        GET "/" => index
        GET "/users/:first/:last/:age" => greet
        "/parent" => {
            GET "/child" => parent_child
        }
    })
    .serve("0.0.0.0:3000")
}
