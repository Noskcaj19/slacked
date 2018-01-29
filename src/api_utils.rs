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

pub fn get_public_channels<R>(
    client: &R,
    token: &str,
    request: &slack_api::channels::ListRequest,
) -> Vec<slack_api::Channel>
where
    R: slack_api::requests::SlackWebRequestSender,
{
    let response = match slack_api::channels::list(client, token, request) {
        Ok(response) => response,
        Err(_) => return Vec::new(),
    };
    response.channels.unwrap_or(vec![])
}

pub fn get_private_channels<R>(
    client: &R,
    token: &str,
    request: &slack_api::groups::ListRequest,
) -> Vec<slack_api::Group>
where
    R: slack_api::requests::SlackWebRequestSender,
{
    let response = match slack_api::groups::list(client, token, request) {
        Ok(response) => response,
        Err(_) => return Vec::new(),
    };
    let mut groups = response.groups.unwrap_or(vec![]);

    groups.retain(|group| !group.is_mpim.clone().unwrap());
    groups
}
