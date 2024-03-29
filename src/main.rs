fn main() {
    println!("Hello, world!");
}

pub struct PullRequestView;
pub mod pull_request_view {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "PullRequestView";
    pub const QUERY : & 'static str = "query PullRequestView($prNumber: Int!) {\n  repository(owner: \"Microsoft\", name: \"TypeScript\") {\n    pullRequest(number: $prNumber) {\n      commits(first: 250) {\n        pageInfo {\n          hasNextPage\n        }\n        nodes {\n          commit {\n            author {\n              name\n            }\n            message\n            oid\n            authoredDate\n          }\n        }\n      }\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "An ISO-8601 encoded UTC date string."]
    type DateTime = String;
    #[doc = "A Git object ID."]
    type GitObjectID = String;
    #[derive(Deserialize)]
    #[doc = "Information about pagination in a connection."]
    pub struct PullRequestViewRepositoryPullRequestCommitsPageInfo {
        #[doc = "When paginating forwards, are there more items?"]
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
    }
    #[derive(Deserialize)]
    #[doc = "Represents an actor in a Git commit (ie. an author or committer)."]
    pub struct PullRequestViewRepositoryPullRequestCommitsNodesCommitAuthor {
        #[doc = "The name in the Git commit."]
        pub name: Option<String>,
    }
    #[derive(Deserialize)]
    #[doc = "Represents a Git commit."]
    pub struct PullRequestViewRepositoryPullRequestCommitsNodesCommit {
        #[doc = "Authorship details of the commit."]
        pub author: Option<PullRequestViewRepositoryPullRequestCommitsNodesCommitAuthor>,
        #[doc = "The Git commit message"]
        pub message: String,
        #[doc = "The Git object ID"]
        pub oid: GitObjectID,
        #[doc = "The datetime when this commit was authored."]
        #[serde(rename = "authoredDate")]
        pub authored_date: DateTime,
    }
    #[derive(Deserialize)]
    #[doc = "Represents a Git commit part of a pull request."]
    pub struct PullRequestViewRepositoryPullRequestCommitsNodes {
        #[doc = "The Git commit object"]
        pub commit: PullRequestViewRepositoryPullRequestCommitsNodesCommit,
    }
    #[derive(Deserialize)]
    #[doc = "The connection type for PullRequestCommit."]
    pub struct PullRequestViewRepositoryPullRequestCommits {
        #[doc = "Information to aid in pagination."]
        #[serde(rename = "pageInfo")]
        pub page_info: PullRequestViewRepositoryPullRequestCommitsPageInfo,
        #[doc = "A list of nodes."]
        pub nodes: Option<Vec<Option<PullRequestViewRepositoryPullRequestCommitsNodes>>>,
    }
    #[derive(Deserialize)]
    #[doc = "A repository pull request."]
    pub struct PullRequestViewRepositoryPullRequest {
        #[doc = "A list of commits present in this pull request's head branch not present in the base branch."]
        pub commits: PullRequestViewRepositoryPullRequestCommits,
    }
    #[derive(Deserialize)]
    #[doc = "A repository contains the content for a project."]
    pub struct PullRequestViewRepository {
        #[doc = "Returns a single pull request from the current repository by number."]
        #[serde(rename = "pullRequest")]
        pub pull_request: Option<PullRequestViewRepositoryPullRequest>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "prNumber")]
        pub pr_number: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[doc = "Lookup a given repository by the owner and repository name."]
        pub repository: Option<PullRequestViewRepository>,
    }
}
impl graphql_client::GraphQLQuery for PullRequestView {
    type Variables = pull_request_view::Variables;
    type ResponseData = pull_request_view::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: pull_request_view::QUERY,
            operation_name: pull_request_view::OPERATION_NAME,
        }
    }
}
