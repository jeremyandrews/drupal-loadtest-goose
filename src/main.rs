use rand::Rng;

use goose::GooseState;
use goose::goose::{GooseTaskSet, GooseClient, GooseTask};

fn main() {
    GooseState::initialize()
        .register_taskset(GooseTaskSet::new("AnonBrowsingUser")
            .set_weight(4)
            .register_task(GooseTask::new(drupal_loadtest_front_page)
                .set_weight(15)
                .set_name("(Anon) front page")
            )
            .register_task(GooseTask::new(drupal_loadtest_node_page)
                .set_weight(10)
                .set_name("(Anon) node page")
            )
            .register_task(GooseTask::new(drupal_loadtest_profile_page)
                .set_weight(3)
                .set_name("(Anon) user page")
            )
        )
        .register_taskset(GooseTaskSet::new("AuthBrowsingUser")
            .set_weight(1)
            .register_task(GooseTask::new(drupal_loadtest_login)
                .set_on_start()
                .set_name("(Auth) login")
            )
            .register_task(GooseTask::new(drupal_loadtest_front_page)
                .set_weight(15)
                .set_name("(Auth) front page")
            )
            .register_task(GooseTask::new(drupal_loadtest_node_page)
                .set_weight(10)
                .set_name("(Auth) node page")
            )
            .register_task(GooseTask::new(drupal_loadtest_profile_page)
                .set_weight(3)
                .set_name("(Auth) user page")
            )
            .register_task(GooseTask::new(drupal_loadtest_post_comment)
                .set_weight(3)
                .set_name("(Auth) comment form")
            )
        )
        .execute();
}

/// View the front page.
fn drupal_loadtest_front_page(client: &mut GooseClient) {
    let _response = client.get("/");
    // @TODO: static assets
}

/// View a node from 1 to 10,000, created by preptest.sh.
fn drupal_loadtest_node_page(client: &mut GooseClient) {
    let nid = rand::thread_rng().gen_range(1, 10_000);
    let _response = client.get(format!("/node/{}", &nid).as_str());
}

/// View a profile from 3 to 5,002, created by preptest.sh.
fn drupal_loadtest_profile_page(client: &mut GooseClient) {
    let uid = rand::thread_rng().gen_range(3, 5_002);
    let _response = client.get(format!("/user/{}", &uid).as_str());
}

/// Log in.
fn drupal_loadtest_login(client: &mut GooseClient) {
    let response = client.get("/user");
    match response {
        Ok(r) => {
            match r.text() {
                Ok(user_page) => {
                    println!("{}", user_page);
                }
                // User page shouldn't be empty.
                Err(_) => client.set_failure(),
            }
        }
        Err(_) => (),
    }
}

/// Post a comment.
fn drupal_loadtest_post_comment(client: &mut GooseClient) {
    let nid = rand::thread_rng().gen_range(1, 10_000);
    let _comment_form = client.get(format!("/comment/reply/{}", &nid).as_str());
}
