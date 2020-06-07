use reqwest::{blocking, Url};

pub fn notify_slack(message: String) -> Option<bool> {
  let url =
    Url::parse("https://hooks.slack.com/services/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")
      .unwrap();
  println!("\n---\nmensaje:{}", message);
  let response = blocking::Client::new()
    .post(url)
    .header("Content-Type", "application/json")
    .body(message)
    .send();
  let r = response.unwrap();
  println!(
    "status: {:?}\nresponse:{:?}\n---end",
    r.status(),
    r.text()
  );
  Some(true)
}