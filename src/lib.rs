use anyhow::Result;

pub async fn run(request_url: &str) -> Result<String> {
    let response = reqwest::get(request_url).await?;
    // let response = dbg!(response);

    let response_url = response.url().to_string();
    // let response_url = dbg!(response_url);

    // let response_text = response.text().await?;
    // dbg!(response_text);

    if request_url == response_url {
        let header_target = response.headers().get("target");
        // dbg!(header_target);
        match header_target {
            Some(header_target) => {
                return Ok(header_target.to_str().unwrap().to_string());
            }
            None => {
                todo!()
            }
        }
    }

    Ok(response_url)
}

#[tokio::test]
async fn run_test() {
    let github_jerryshell = "https://github.com/jerryshell";
    assert_eq!(
        run("https://bit.ly/3A7vZoF").await.unwrap(),
        github_jerryshell
    );
    assert_eq!(
        run("https://reurl.cc/7eDRj5").await.unwrap(),
        github_jerryshell
    );
}
