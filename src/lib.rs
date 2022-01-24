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
    let test_url_list = vec![
        "https://bit.ly/3A7vZoF",
        "https://reurl.cc/7eDRj5",
        "https://t.hk.uy/aGVK",
        "https://cn.hk.uy/R94",
        "https://x88.ltd/crP",
        "https://dwz.date/fmMe",
        "https://dwz.win/awDE",
        "https://tinyurl.com/y8x2m3le",
    ];
    for test_url in test_url_list {
        assert_eq!(
            run(test_url).await.unwrap(),
            "https://github.com/jerryshell"
        );
    }

    assert_eq!(
        run("https://kutt.appinn.net/eY1NJh").await.unwrap(),
        "http://www.nlc.cn/dsb_zyyfw/sytc/sp/"
    );

    assert_eq!(
        run("https://dllzff.cn/Quwzwuso").await.unwrap(),
        "https://comm.tencentcs.com/sms/wxmp.html?t=weixin%3A%2F%2Fdl%2Fbusiness%2F%3Ft%3Dr59IGRxz51d"
    );

    assert_eq!(
        run("https://pingan.com/rrjx").await.unwrap(),
        "https://b.pingan.com.cn/creditcard/life/marketing/tt8/tt8.html?cid=ci0000001&outerid=rrjxqd001"
    );
}
