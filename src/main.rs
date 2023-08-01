
mod dico;

#[tokio::main]
async fn main() {

    let application = dico::Application::load("books".to_string());

    println!("{:?}", application);
}