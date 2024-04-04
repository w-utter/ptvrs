#[cfg(test)]
pub mod test {
    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use ptvrs::*;

    static CLIENT: Lazy<Client> = Lazy::new(|| {
        dotenv().ok();
        Client::new(
            std::env::var("DEVID").expect("could not find DEVID env var"),
            std::env::var("KEY").expect("could not find KEY env var"),
        )
    });

    //
    #[tokio::test]
    pub async fn test() {
        println!(
            "{:?}",
            CLIENT
                .runs_id(
                    2,
                    RunsIdOpts {
                        expand: vec![ExpandOptions::All].into(),
                        ..Default::default()
                    }
                )
                .await
        );
    }
}
