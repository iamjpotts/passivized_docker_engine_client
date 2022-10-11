#[path = "test_utils/lib.rs"]
mod test_utils;

use std::str::FromStr;

use http::Uri;
use test_utils::images::web;
use test_utils::{content_type, random_name};
use passivized_docker_engine_client::client::DOCKER_ENGINE_VERSION;
use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::errors::DecUseError;

const MISSING_IMAGE: &str = "does_not_exist";
const MISSING_TAG: &str = "does_not_exist";

fn mockito_client() -> DockerEngineClient {
    DockerEngineClient::with_server(format!("http://{}", mockito::server_address()))
        .unwrap()
}

#[tokio::test]
async fn test_bad_image_list_responses() {
    mockito::reset();

    let dec = mockito_client();

    let path = format!("/{}/images/json", DOCKER_ENGINE_VERSION);

    {
        let error = dec.images().list().await.unwrap_err();

        if let DecUseError::ApiNotImplemented { uri } = error {
            assert_eq!(path, Uri::from_str(&uri).unwrap().path());
        }
        else {
            panic!("Did not expect {}", error)
        }
    }

    mockito::reset();

    let _m = mockito::mock("GET", path.as_str()).with_status(404).create();

    {
        let error = dec.images().list().await.unwrap_err();

        match error {
            DecUseError::ApiNotFound { uri} => {
                assert_eq!(path, Uri::from_str(&uri).unwrap().path());
            }
            _ => {
                panic!("Did not expect {}", error)
            }
        }
    }

    mockito::reset();

    let _m = mockito::mock("GET", path.as_str())
        .with_status(200)
        .with_header("Content-Type", content_type::JSON)
        .with_body("not json")
        .create();

    {
        let error = dec.images().list().await.unwrap_err();

        match error {
            DecUseError::UnparseableJsonResponse { status, text, parse_error } => {
                assert_eq!(200, status);
                assert_eq!("not json", &text);
                assert_eq!("expected ident at line 1 column 2", &parse_error.to_string());
            }
            _ => {
                panic!("Did not expect {}", error)
            }
        }
    }
}

#[tokio::test]
async fn test_pull_and_list_images() {
    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let images = dec.images().list()
        .await
        .unwrap();

    let found = images
        .iter()
        .find(|i| i.repo_tags.contains(&format!("{}:{}", web::IMAGE, web::TAG)))
        .unwrap();

    assert_eq!(web::HASH, found.id);
    assert!(found.size > web::MIN_SIZE);
}

#[tokio::test]
async fn test_pull_not_found() {
    let dec = DockerEngineClient::new()
        .unwrap();

    let error = dec.images().pull(MISSING_IMAGE, MISSING_TAG)
        .await
        .unwrap_err();

    if let DecUseError::NotFound { message} = error {
        assert!(message.contains("does not exist"));
        assert!(message.contains(MISSING_IMAGE));
    }
    else {
        panic!("Did not expect {}", error)
    }
}

#[tokio::test]
async fn test_tag_and_untag() {
    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let image_name = format!("{}:{}", web::IMAGE, web::TAG);

    let new_repo = random_name(web::IMAGE);
    let new_tag = random_name(web::TEST_TAG_PREFIX);

    dec.images().tag(image_name, &new_repo, &new_tag)
        .await
        .unwrap();

    let new_image = format!("{}:{}", new_repo, new_tag);

    let found = dec.images().list()
        .await
        .unwrap()
        .iter()
        .filter(|item| item.repo_tags.contains(&new_image))
        .next()
        .is_some();

    assert!(found, "{}", new_image);

    // Verify untag defaults to foo:latest, and will return a not found error when appropriate

    let untag_error = dec.images().untag(&new_repo)
        .await
        .unwrap_err();

    if let DecUseError::NotFound { message } = untag_error {
        let expected = format!("No such image: {}:latest", new_repo);
        assert_eq!(expected, message);
    }
    else {
        panic!("Unexpected untag error: {:?}", untag_error);
    }

    // Verify successful untag

    dec.images().untag(&new_image)
        .await
        .unwrap();

    let found_after_untag = dec.images().list()
        .await
        .unwrap()
        .iter()
        .filter(|item| item.repo_tags.contains(&new_image))
        .next()
        .is_some();

    assert!(!found_after_untag, "{}", new_image);
}

#[tokio::test]
async fn test_tag_source_not_found() {
    let dec = DockerEngineClient::new()
        .unwrap();

    let old_image_name = random_name("does_not_exist");
    let new_repo = random_name("will_not_be_created");

    let tag_error = dec.images().tag(&old_image_name, &new_repo, "latest")
        .await
        .unwrap_err();

    if let DecUseError::NotFound { message } = tag_error {
        let expected = format!("No such image: {}:latest", old_image_name);
        assert_eq!(expected, message);
    }
    else {
        panic!("Unexpected tag error: {:?}", tag_error);
    }
}
