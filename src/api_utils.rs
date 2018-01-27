use slack_api;

pub struct UserIds {
    pub name: String,
    pub id: String,
}

pub fn get_users<R>(
    client: &R,
    token: &str,
    request: &slack_api::users::ListRequest,
) -> Vec<UserIds>
where
    R: slack_api::requests::SlackWebRequestSender,
{
    let response = match slack_api::users::list(client, token, request) {
        Ok(response) => response,
        Err(_) => return Vec::new(),
    };
    let members = match response.members {
        Some(members) => members,
        None => return Vec::new(),
    };

    members
        .iter()
        .map(|member| UserIds {
            name: member.name.clone().unwrap(),
            id: member.id.clone().unwrap(),
        })
        .collect()
}
