use activitypub::actor;
use actor as internal_actor;
use database;
use tests::utils::create_local_test_actor;
use tests::utils::create_remote_test_actor;
use tests::utils::delete_test_actor;

#[test]
fn get_err_by_preferred_username() {
    let json_object = actor::get_json_by_preferred_username(String::from("希"));
    assert_eq!(
        json_object["error"].to_string(),
        format!("\"{}\"", "User not found.")
    );
}

#[test]
fn get_json_by_preferred_username() {
    let test_actor: internal_actor::Actor = create_local_test_actor("ap_actor_test_json");

    let email = test_actor.email.clone();
    let password = test_actor.password.clone();
    let actor_uri = test_actor.actor_uri.clone();
    let username = test_actor
        .username
        .clone()
        .unwrap_or_else(|| test_actor.preferred_username.clone());
    let preferred_username = test_actor.preferred_username.clone();
    let summary = test_actor
        .summary
        .clone()
        .unwrap_or_else(|| String::from(""));
    let local = test_actor.local.clone();

    let json_object = actor::get_json_by_preferred_username(preferred_username.clone());

    delete_test_actor(test_actor);
    assert_eq!(
        json_object["id"].to_string(),
        format!("\"{}\"", actor_uri.clone())
    );
    assert_eq!(
        json_object["summary"].to_string(),
        format!("\"{}\"", summary)
    );
    assert_eq!(
        json_object["following"].to_string(),
        format!("\"{}/following\"", actor_uri.clone())
    );
    assert_eq!(
        json_object["followers"].to_string(),
        format!("\"{}/followers\"", actor_uri.clone())
    );
    assert_eq!(
        json_object["inbox"].to_string(),
        format!("\"{}/inbox\"", actor_uri.clone())
    );
    assert_eq!(
        json_object["outbox"].to_string(),
        format!("\"{}/outbox\"", actor_uri.clone())
    );
    assert_eq!(
        json_object["preferredUsername"].to_string(),
        format!("\"{}\"", preferred_username)
    );
    assert_eq!(json_object["name"].to_string(), format!("\"{}\"", username));
    assert_eq!(json_object["url"].to_string(), format!("\"{}\"", actor_uri));
}

#[test]
fn add_follow() {
    let database = database::establish_connection();
    let test_actor = create_local_test_actor("cb21a906-0827-4dbd-a34f-f923fc0e38fb");
    let test_follower_1 = create_remote_test_actor("0fdba399-d603-433c-934b-e774b8262698");
    let test_follower_1_uri = test_follower_1.actor_uri.clone();

    actor::add_follow(&test_actor.actor_uri, &test_follower_1_uri, "");
    let test_actor = internal_actor::get_actor_by_uri(&database, &test_actor.actor_uri).unwrap();

    let activitypub_followers: serde_json::Value = test_actor.followers["activitypub"].clone();
    let follow_data: Vec<serde_json::Value> =
        serde_json::from_value(activitypub_followers).unwrap_or_else(|_| vec![]);
    delete_test_actor(test_actor);
    delete_test_actor(test_follower_1);

    assert_eq!(follow_data[0]["href"], test_follower_1_uri);
}

#[test]
fn remove_follow() {
    let database = database::establish_connection();
    let test_actor = create_local_test_actor("5eca9b7a-a545-4d2b-b28f-3e6a960d3a6d");
    let test_follower_1 = create_remote_test_actor("bdecd8a8-8aa6-4d21-9e44-c0e9d258d471");
    let test_follower_1_uri = test_follower_1.actor_uri.clone();

    actor::add_follow(&test_actor.actor_uri, &test_follower_1_uri, "");
    actor::remove_follow(&test_actor.actor_uri, &test_follower_1_uri);
    let test_actor = internal_actor::get_actor_by_uri(&database, &test_actor.actor_uri).unwrap();

    let activitypub_followers: serde_json::Value = test_actor.followers["activitypub"].clone();
    let follow_data: Vec<serde_json::Value> =
        serde_json::from_value(activitypub_followers).unwrap_or_else(|_| vec![]);
    delete_test_actor(test_actor);
    delete_test_actor(test_follower_1);

    assert_eq!(follow_data.len(), 0);
}
