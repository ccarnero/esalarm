use reqwest::{Url};
use std::sync::mpsc;
use std::{thread, time::Duration};

mod query_response;

fn fetch_url_get_body(
  tupla: (String, String),
) -> Result<query_response::QueryResponse, reqwest::Error> {
  let (address, query) = tupla;
  let url = Url::parse(address.as_str()).unwrap();
  reqwest::blocking::Client::new()
    .get(url)
    .header("Content-Type", "application/json")
    .body(query)
    .send()?
    .json::<query_response::QueryResponse>()
}

// TODO: formatear este mensaje
// Error en Produccion:
// http link para `Ir a Kibana`
// 2020-06-03T01:49:17.066Z
// LogGroup: /aws/lambda/realtime-notifications-v306-send-email-notification
fn create_message(query_results: &query_response::QueryResponse) -> String {
  let results = query_results.hits.hit_list.clone();
  let blocks: String = results
    .into_iter()
    .map(|h| {
      let as_code = format!("```{}```", h.source.message);
      format!(
        r#"{{"type": "section", "text":{{"type": "mrkdwn", "text": {:?}}}}},"#,
        as_code
      )
    })
    .collect::<Vec<String>>()
    .join("");

  format!(r#"{{"blocks": [{}]}}"#, blocks)
}

pub fn get_worker_thread(tx: mpsc::Sender<String>, setting: (u64, String, String)) {
  let (duration, url, query) = setting;
  thread::spawn(move || loop {
    let resultados = fetch_url_get_body((url.clone(), query.clone()));

    let message = match resultados {
      Err(err) => err.to_string(),
      Ok(result) => create_message(&result),
    };

    tx.send(message);
    thread::sleep(Duration::from_millis(duration));
  });
}
