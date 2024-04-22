use janus_client;
use url::Url;
use prio::vdaf::prio3::Prio3Histogram;
use janus_messages::{Duration, TaskId};

fn main() {

    let leader_url = Url::parse("https://staging-dap-09-2.api.divviup.org/").unwrap();
    let helper_url = Url::parse("https://staging-dap-09-1.api.divviup.org/").unwrap();
    let vdaf = Prio3Histogram::new_histogram(
        2,
        11,
        8
    );
    let taskid = "rc0jgm1MHH6Q7fcI4ZdNUxas9DAYLcJFK5CL7xUl-gU";
    let task = TaskId::from_str(taskid);

    let client = janus_client::Client::new(
task,
        leader_url,
        helper_url,
        Duration::from_seconds(300),
        vdaf
    );
}
