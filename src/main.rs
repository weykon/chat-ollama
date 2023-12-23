use app::App;
use reqwest::Error;
mod a_conversition;
mod event_loop;
mod input;
mod view;
mod app;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let web_client = reqwest::Client::new();

    // a_conversition::speak(&client).await?;
    let mut app = App::new(); 


    let runtime_status = event_loop::boost_window(app);



    Ok(())
}
