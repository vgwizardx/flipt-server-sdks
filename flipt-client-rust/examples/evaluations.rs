// cargo run --example evaluations

use std::collections::HashMap;

use anyhow::Result;
use flipt::evaluation::models::{BatchEvaluationRequest, EvaluationRequest};
use flipt::FliptClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = FliptClient::default();

    let mut context: HashMap<String, String> = HashMap::new();

    context.insert("fizz".into(), "buzz".into());

    let variant_result = client
        .evaluation
        .variant(&EvaluationRequest {
            namespace_key: "default".into(),
            flag_key: "flag1".into(),
            entity_id: "entity".into(),
            context: context.clone(),
        })
        .await
        .unwrap();

    print!("{:?}", variant_result);

    let boolean_result = client
        .evaluation
        .boolean(&EvaluationRequest {
            namespace_key: "default".into(),
            flag_key: "flag_boolean".into(),
            entity_id: "entity".into(),
            context: context.clone(),
        })
        .await
        .unwrap();

    print!("{:?}", boolean_result);

    let requests: Vec<EvaluationRequest> = vec![
        EvaluationRequest {
            namespace_key: "default".into(),
            flag_key: "flag1".into(),
            entity_id: "entity".into(),
            context: context.clone(),
        },
        EvaluationRequest {
            namespace_key: "default".into(),
            flag_key: "flag_boolean".into(),
            entity_id: "entity".into(),
            context: context.clone(),
        },
    ];

    let batch_result = client
        .evaluation
        .batch(&BatchEvaluationRequest { requests })
        .await
        .unwrap();

    print!("{:?}", batch_result);

    Ok(())
}
