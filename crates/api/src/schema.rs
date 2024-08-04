#[cynic::schema("linear")]
mod schema {}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);

impl DateTime {
    pub fn parse(&self) -> Result<time::OffsetDateTime, time::error::Parse> {
        time::OffsetDateTime::parse(&self.0, &time::format_description::well_known::Rfc3339)
    }
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum PaginationOrderBy {
    #[cynic(rename = "createdAt")]
    CreatedAt,
    #[cynic(rename = "updatedAt")]
    UpdatedAt,
}

#[derive(cynic::QueryFragment, serde::Serialize, Debug)]
pub struct User {
    pub id: cynic::Id,
    pub name: String,
    pub display_name: String,
    pub url: String,
    pub is_me: bool,
}

pub mod viewer {
    use super::{schema, User};

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct QueryViewer {
        pub viewer: User,
    }
}

pub mod team_list {
    use super::{schema, DateTime, PaginationOrderBy};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct TeamListVariables {
        pub first: i32,
        pub order: PaginationOrderBy,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "TeamListVariables")]
    pub struct TeamList {
        #[arguments(first: $first, orderBy: $order)]
        pub teams: TeamConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct TeamConnection {
        pub page_info: PageInfo,
        pub nodes: Vec<Team>,
    }

    #[derive(cynic::QueryFragment, serde::Serialize, Debug)]
    pub struct Team {
        pub id: cynic::Id,
        pub created_at: DateTime,
        pub updated_at: DateTime,
        pub name: String,
        pub description: Option<String>,
        pub key: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
    }
}

pub mod projects_list {
    use super::{schema, DateTime, PaginationOrderBy, User};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct ProjectListVariables {
        pub first: i32,
        pub include_archived: bool,
        pub oder: PaginationOrderBy,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "ProjectListVariables")]
    pub struct ProjectList {
        #[arguments(first: $first, orderBy: $oder, includeArchived: $include_archived)]
        pub projects: ProjectConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ProjectConnection {
        pub page_info: PageInfo,
        pub nodes: Vec<Project>,
    }

    #[derive(cynic::QueryFragment, serde::Serialize, Debug)]
    pub struct Project {
        pub id: cynic::Id,
        pub created_at: DateTime,
        pub updated_at: DateTime,
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub description: String,
        pub slug_id: String,
        pub status: ProjectStatus,
        pub creator: Option<User>,
        pub priority: i32,
        pub url: String,
    }

    #[derive(cynic::QueryFragment, serde::Serialize, Debug)]
    pub struct ProjectStatus {
        pub id: cynic::Id,
        pub name: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
    }
}

pub mod issue_list {
    use super::{schema, DateTime, PaginationOrderBy, User};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct IssueListVariables {
        pub first: i32,
        pub order: PaginationOrderBy,
        pub sort: Option<Vec<IssueSortInput>>,
        pub filter: Option<IssueFilter>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "IssueListVariables")]
    pub struct IssueList {
        #[arguments(first: $first, sort: $sort, orderBy: $order, filter: $filter)]
        pub issues: IssueConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct IssueConnection {
        pub page_info: PageInfo,
        pub nodes: Vec<Issue>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PageInfo {
        pub end_cursor: Option<String>,
    }

    #[derive(cynic::QueryFragment, serde::Serialize, Debug)]
    pub struct Issue {
        pub id: cynic::Id,
        pub identifier: String,
        pub title: String,
        pub description: Option<String>,
        pub branch_name: String,
        pub canceled_at: Option<DateTime>,
        pub assignee: Option<User>,
        pub completed_at: Option<DateTime>,
        pub created_at: DateTime,
        pub due_date: Option<TimelessDate>,
        pub estimate: Option<f64>,
        pub number: f64,
        pub priority: f64,
        pub priority_label: String,
        pub started_at: Option<DateTime>,
        pub started_triage_at: Option<DateTime>,
        pub state: WorkflowState,
        pub trashed: Option<bool>,
        pub triaged_at: Option<DateTime>,
        pub updated_at: DateTime,
        pub url: String,
    }

    #[derive(cynic::QueryFragment, serde::Serialize, Debug)]
    pub struct WorkflowState {
        pub name: String,
        pub color: String,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    pub enum PaginationNulls {
        #[cynic(rename = "first")]
        First,
        #[cynic(rename = "last")]
        Last,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    pub enum PaginationSortOrder {
        #[cynic(rename = "Ascending")]
        Ascending,
        #[cynic(rename = "Descending")]
        Descending,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    pub enum SlaStatus {
        #[cynic(rename = "Breached")]
        Breached,
        #[cynic(rename = "HighRisk")]
        HighRisk,
        #[cynic(rename = "MediumRisk")]
        MediumRisk,
        #[cynic(rename = "LowRisk")]
        LowRisk,
        #[cynic(rename = "Completed")]
        Completed,
        #[cynic(rename = "Failed")]
        Failed,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct IssueSortInput {
        pub priority: Option<PrioritySort>,
        pub estimate: Option<EstimateSort>,
        pub title: Option<TitleSort>,
        pub label: Option<LabelSort>,
        pub sla_status: Option<SlaStatusSort>,
        pub created_at: Option<CreatedAtSort>,
        pub updated_at: Option<UpdatedAtSort>,
        pub completed_at: Option<CompletedAtSort>,
        pub due_date: Option<DueDateSort>,
        pub cycle: Option<CycleSort>,
        pub milestone: Option<MilestoneSort>,
        pub assignee: Option<AssigneeSort>,
        pub project: Option<ProjectSort>,
        pub team: Option<TeamSort>,
        pub manual: Option<ManualSort>,
        pub workflow_state: Option<WorkflowStateSort>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct WorkflowStateSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ManualSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct TeamSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct MilestoneSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct UpdatedAtSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct SlaStatusSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct LabelSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct TitleSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct PrioritySort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
        pub no_priority_first: Option<bool>,
    }

    #[derive(cynic::InputObject, Debug, Default)]
    pub struct IssueFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub number: Option<NumberComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub title: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub description: Option<NullableStringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub priority: Option<NullableNumberComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub estimate: Option<EstimateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub started_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub triaged_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub completed_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub canceled_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub auto_closed_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub auto_archived_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub due_date: Option<NullableTimelessDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub snoozed_until_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub assignee: Option<NullableUserFilter>,
        // pub last_applied_template: Option<NullableTemplateFilter>,
        // pub source_metadata: Option<SourceMetadataComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub creator: Option<NullableUserFilter>,
        // pub parent: Option<NullableIssueFilter>,
        // pub snoozed_by: Option<NullableUserFilter>,
        // pub labels: Option<IssueLabelCollectionFilter>,
        // pub subscribers: Option<UserCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub team: Option<TeamFilter>,
        // pub project_milestone: Option<NullableProjectMilestoneFilter>,
        // pub comments: Option<CommentCollectionFilter>,
        // pub cycle: Option<NullableCycleFilter>,
        // pub project: Option<NullableProjectFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub state: Option<WorkflowStateFilter>,
        // pub children: Option<IssueCollectionFilter>,
        // pub attachments: Option<AttachmentCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub searchable_content: Option<ContentComparator>,
        // pub has_related_relations: Option<RelationExistsComparator>,
        // pub has_duplicate_relations: Option<RelationExistsComparator>,
        // pub has_blocked_by_relations: Option<RelationExistsComparator>,
        // pub has_blocking_relations: Option<RelationExistsComparator>,
        // pub sla_status: Option<SlaStatusComparator>,
        // pub reactions: Option<ReactionCollectionFilter>,
        // pub needs: Option<CustomerNeedCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<IssueFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<IssueFilter>>,
    }

    impl IssueFilter {
        pub fn new_and(filters: Vec<Self>) -> Self {
            Self {
                and: Some(filters),
                ..Default::default()
            }
        }

        pub fn creator_username(name: String) -> Self {
            Self {
                creator: Some(NullableUserFilter::username(name)),
                ..Default::default()
            }
        }

        pub fn assignee_username(name: String) -> Self {
            Self {
                assignee: Some(NullableUserFilter::username(name)),
                ..Default::default()
            }
        }

        pub fn assignee_me() -> Self {
            Self {
                assignee: Some(NullableUserFilter::me()),
                ..Default::default()
            }
        }

        pub fn team_key(key: String) -> Self {
            Self {
                team: Some(TeamFilter {
                    key: Some(StringComparator::eq(key)),
                    ..Default::default()
                }),
                ..Default::default()
            }
        }

        pub fn search(term: String) -> Self {
            Self {
                searchable_content: Some(ContentComparator {
                    contains: Some(term),
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
    }

    #[derive(cynic::InputObject, Debug, Default)]
    pub struct NullableUserFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub display_name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub email: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub active: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub admin: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_me: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
        // NOTE: commented out because it causes cynic to overflow the stack.
        // pub assigned_issues: Option<IssueCollectionFilter>,
        // pub and: Option<Vec<NullableUserFilter>>,
        // pub or: Option<Vec<NullableUserFilter>>,
    }

    impl NullableUserFilter {
        pub fn username(name: String) -> Self {
            Self {
                name: Some(StringComparator {
                    eq: Some(name),
                    ..Default::default()
                }),
                ..Default::default()
            }
        }

        pub fn me() -> Self {
            Self {
                is_me: Some(BooleanComparator::is_true()),
                ..Default::default()
            }
        }
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct IssueCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub number: Option<NumberComparator>,
        pub title: Option<StringComparator>,
        pub description: Option<NullableStringComparator>,
        pub priority: Option<NullableNumberComparator>,
        pub estimate: Option<EstimateComparator>,
        pub started_at: Option<NullableDateComparator>,
        pub triaged_at: Option<NullableDateComparator>,
        pub completed_at: Option<NullableDateComparator>,
        pub canceled_at: Option<NullableDateComparator>,
        pub auto_closed_at: Option<NullableDateComparator>,
        pub auto_archived_at: Option<NullableDateComparator>,
        pub due_date: Option<NullableTimelessDateComparator>,
        pub snoozed_until_at: Option<NullableDateComparator>,
        pub assignee: Option<Box<NullableUserFilter>>,
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub source_metadata: Option<SourceMetadataComparator>,
        pub creator: Option<Box<NullableUserFilter>>,
        pub parent: Option<NullableIssueFilter>,
        pub snoozed_by: Option<Box<NullableUserFilter>>,
        pub labels: Option<IssueLabelCollectionFilter>,
        pub subscribers: Option<UserCollectionFilter>,
        pub team: Option<TeamFilter>,
        pub project_milestone: Option<NullableProjectMilestoneFilter>,
        pub comments: Option<CommentCollectionFilter>,
        pub cycle: Option<NullableCycleFilter>,
        pub project: Option<NullableProjectFilter>,
        pub state: Option<WorkflowStateFilter>,
        pub children: Option<Box<IssueCollectionFilter>>,
        pub attachments: Option<AttachmentCollectionFilter>,
        pub searchable_content: Option<ContentComparator>,
        pub has_related_relations: Option<RelationExistsComparator>,
        pub has_duplicate_relations: Option<RelationExistsComparator>,
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        pub has_blocking_relations: Option<RelationExistsComparator>,
        pub sla_status: Option<SlaStatusComparator>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Option<CustomerNeedCollectionFilter>,
        pub some: Option<Box<IssueFilter>>,
        pub every: Option<Box<IssueFilter>>,
        pub length: Option<NumberComparator>,

        // NOTE: commented out because it causes cynic to overflow the stack.
        pub and: Option<Vec<IssueCollectionFilter>>,
        pub or: Option<Vec<IssueCollectionFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableIssueFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub number: Option<NumberComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub title: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub description: Option<NullableStringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub priority: Option<NullableNumberComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub estimate: Option<EstimateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub started_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub triaged_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub completed_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub canceled_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub auto_closed_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub auto_archived_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub due_date: Option<NullableTimelessDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub snoozed_until_at: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub assignee: Option<Box<NullableUserFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub source_metadata: Option<SourceMetadataComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub creator: Option<Box<NullableUserFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub parent: Option<Box<NullableIssueFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub snoozed_by: Option<Box<NullableUserFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub labels: Option<IssueLabelCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub subscribers: Option<UserCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub team: Option<TeamFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub project_milestone: Option<NullableProjectMilestoneFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub comments: Option<CommentCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub cycle: Option<NullableCycleFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub project: Option<NullableProjectFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub state: Option<WorkflowStateFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub children: Option<Box<IssueCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<AttachmentCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub searchable_content: Option<ContentComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub has_duplicate_relations: Option<RelationExistsComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub sla_status: Option<SlaStatusComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub reactions: Option<ReactionCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub needs: Option<CustomerNeedCollectionFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<NullableIssueFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<NullableIssueFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct SlaStatusComparator {
        pub eq: Option<SlaStatus>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub neq: Option<SlaStatus>,
        #[cynic(rename = "in", skip_serializing_if = "Option::is_none")]
        pub in_: Option<Vec<SlaStatus>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub nin: Option<Vec<SlaStatus>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct WorkflowStateFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub description: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub position: Option<NumberComparator>,
        #[cynic(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub team: Option<TeamFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub issues: Option<Box<IssueCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<WorkflowStateFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<WorkflowStateFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableCycleFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub number: Option<NumberComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub starts_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub ends_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub completed_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_active: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_in_cooldown: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_next: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_previous: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_future: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub is_past: Option<BooleanComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub team: Option<TeamFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub issues: Option<Box<IssueCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<NullableCycleFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<NullableCycleFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableProjectMilestoneFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub target_date: Option<NullableDateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<NullableProjectMilestoneFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<NullableProjectMilestoneFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct IssueLabelCollectionFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub creator: Option<Box<NullableUserFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub team: Option<NullableTeamFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub parent: Option<IssueLabelFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<IssueLabelCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<IssueLabelCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub some: Option<IssueLabelFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub every: Option<IssueLabelFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct IssueLabelFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub creator: Option<Box<NullableUserFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub team: Option<NullableTeamFilter>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub parent: Option<Box<IssueLabelFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<IssueLabelFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<IssueLabelFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableTeamFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub key: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub description: Option<NullableStringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub issues: Option<Box<IssueCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<NullableTeamFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<NullableTeamFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct SourceMetadataComparator {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub eq: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub neq: Option<String>,
        #[cynic(rename = "in", skip_serializing_if = "Option::is_none")]
        pub in_: Option<Vec<String>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub nin: Option<Vec<String>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<SubTypeComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct SubTypeComparator {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub eq: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub neq: Option<String>,
        #[cynic(rename = "in", skip_serializing_if = "Option::is_none")]
        pub in_: Option<Vec<String>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub nin: Option<Vec<String>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub null: Option<bool>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableTimelessDateComparator {
        pub eq: Option<TimelessDateOrDuration>,
        pub neq: Option<TimelessDateOrDuration>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<TimelessDateOrDuration>>,
        pub nin: Option<Vec<TimelessDateOrDuration>>,
        pub null: Option<bool>,
        pub lt: Option<TimelessDateOrDuration>,
        pub lte: Option<TimelessDateOrDuration>,
        pub gt: Option<TimelessDateOrDuration>,
        pub gte: Option<TimelessDateOrDuration>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct EstimateSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct EstimateComparator {
        pub eq: Option<f64>,
        pub neq: Option<f64>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<f64>>,
        pub nin: Option<Vec<f64>>,
        pub null: Option<bool>,
        pub lt: Option<f64>,
        pub lte: Option<f64>,
        pub gt: Option<f64>,
        pub gte: Option<f64>,
        pub or: Option<Vec<NullableNumberComparator>>,
        pub and: Option<Vec<NullableNumberComparator>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct DueDateSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CycleSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
        pub current_cycle_first: Option<bool>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CreatedAtSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CompletedAtSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CommentCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub body: Option<StringComparator>,
        pub user: Option<UserFilter>,
        pub issue: Option<Box<NullableIssueFilter>>,
        pub project_update: Option<ProjectUpdateFilter>,
        pub parent: Option<NullableCommentFilter>,
        pub document_content: Option<DocumentContentFilter>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Option<CustomerNeedCollectionFilter>,
        pub and: Option<Vec<CommentCollectionFilter>>,
        pub or: Option<Vec<CommentCollectionFilter>>,
        pub some: Option<CommentFilter>,
        pub every: Option<CommentFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CommentFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub body: Option<StringComparator>,
        pub user: Option<UserFilter>,
        pub issue: Option<Box<NullableIssueFilter>>,
        pub project_update: Option<ProjectUpdateFilter>,
        pub parent: Option<NullableCommentFilter>,
        pub document_content: Option<DocumentContentFilter>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Option<CustomerNeedCollectionFilter>,
        pub and: Option<Vec<CommentFilter>>,
        pub or: Option<Vec<CommentFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectUpdateFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub user: Option<UserFilter>,
        pub project: Option<ProjectFilter>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub and: Option<Vec<ProjectUpdateFilter>>,
        pub or: Option<Vec<ProjectUpdateFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub state: Option<StringComparator>,
        pub status: Option<ProjectStatusFilter>,
        pub priority: Option<NullableNumberComparator>,
        pub searchable_content: Option<ContentComparator>,
        pub completed_at: Option<NullableDateComparator>,
        pub start_date: Option<NullableDateComparator>,
        pub target_date: Option<NullableDateComparator>,
        pub health: Option<StringComparator>,
        pub has_related_relations: Option<RelationExistsComparator>,
        pub has_depended_on_by_relations: Option<RelationExistsComparator>,
        pub has_depends_on_relations: Option<RelationExistsComparator>,
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        pub has_blocking_relations: Option<RelationExistsComparator>,
        pub project_updates: Option<ProjectUpdatesCollectionFilter>,
        pub creator: Option<UserFilter>,
        pub lead: Option<Box<NullableUserFilter>>,
        pub members: Option<UserCollectionFilter>,
        pub issues: Option<Box<IssueCollectionFilter>>,
        pub roadmaps: Option<RoadmapCollectionFilter>,
        pub initiatives: Option<InitiativeCollectionFilter>,
        pub project_milestones: Option<ProjectMilestoneCollectionFilter>,
        pub completed_project_milestones: Option<ProjectMilestoneCollectionFilter>,
        pub next_project_milestone: Option<ProjectMilestoneFilter>,
        pub accessible_teams: Option<TeamCollectionFilter>,
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub needs: Option<CustomerNeedCollectionFilter>,
        pub and: Option<Vec<ProjectFilter>>,
        pub or: Option<Vec<ProjectFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectStatusFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub description: Option<StringComparator>,
        pub position: Option<NumberComparator>,
        #[cynic(rename = "type")]
        pub type_: Option<StringComparator>,
        pub projects: Option<ProjectCollectionFilter>,
        pub and: Option<Vec<ProjectStatusFilter>>,
        pub or: Option<Vec<ProjectStatusFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub state: Option<StringComparator>,
        pub status: Option<Box<ProjectStatusFilter>>,
        pub priority: Option<NullableNumberComparator>,
        pub searchable_content: Option<ContentComparator>,
        pub completed_at: Option<NullableDateComparator>,
        pub start_date: Option<NullableDateComparator>,
        pub target_date: Option<NullableDateComparator>,
        pub health: Option<StringComparator>,
        pub has_related_relations: Option<RelationExistsComparator>,
        pub has_depended_on_by_relations: Option<RelationExistsComparator>,
        pub has_depends_on_relations: Option<RelationExistsComparator>,
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        pub has_blocking_relations: Option<RelationExistsComparator>,
        pub project_updates: Option<ProjectUpdatesCollectionFilter>,
        pub creator: Option<UserFilter>,
        pub lead: Option<Box<NullableUserFilter>>,
        pub members: Option<UserCollectionFilter>,
        pub issues: Option<Box<IssueCollectionFilter>>,
        pub roadmaps: Option<RoadmapCollectionFilter>,
        pub initiatives: Option<InitiativeCollectionFilter>,
        pub project_milestones: Option<ProjectMilestoneCollectionFilter>,
        pub completed_project_milestones: Option<ProjectMilestoneCollectionFilter>,
        pub next_project_milestone: Option<ProjectMilestoneFilter>,
        pub accessible_teams: Option<TeamCollectionFilter>,
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub needs: Option<CustomerNeedCollectionFilter>,
        pub and: Option<Vec<ProjectCollectionFilter>>,
        pub or: Option<Vec<ProjectCollectionFilter>>,
        pub some: Option<Box<ProjectFilter>>,
        pub every: Option<Box<ProjectFilter>>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CustomerNeedCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub priority: Option<NumberComparator>,
        pub project: Option<NullableProjectFilter>,
        pub issue: Option<Box<NullableIssueFilter>>,
        pub comment: Option<NullableCommentFilter>,
        pub customer: Option<CustomerFilter>,
        pub and: Option<Vec<CustomerNeedCollectionFilter>>,
        pub or: Option<Vec<CustomerNeedCollectionFilter>>,
        pub some: Option<CustomerNeedFilter>,
        pub every: Option<CustomerNeedFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CustomerNeedFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub priority: Option<NumberComparator>,
        pub project: Option<NullableProjectFilter>,
        pub issue: Option<Box<NullableIssueFilter>>,
        pub comment: Option<NullableCommentFilter>,
        pub customer: Option<CustomerFilter>,
        pub and: Option<Vec<CustomerNeedFilter>>,
        pub or: Option<Vec<CustomerNeedFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct CustomerFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slack_channel_id: Option<StringComparator>,
        pub domains: Option<StringArrayComparator>,
        pub external_ids: Option<StringArrayComparator>,
        pub owner: Option<UserFilter>,
        pub needs: Option<Box<CustomerNeedCollectionFilter>>,
        pub and: Option<Vec<CustomerFilter>>,
        pub or: Option<Vec<CustomerFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct StringArrayComparator {
        pub length: Option<NumberComparator>,
        pub every: Option<Vec<StringItemComparator>>,
        pub some: Option<Vec<StringItemComparator>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct StringItemComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        pub eq_ignore_case: Option<String>,
        pub neq_ignore_case: Option<String>,
        pub starts_with: Option<String>,
        pub starts_with_ignore_case: Option<String>,
        pub not_starts_with: Option<String>,
        pub ends_with: Option<String>,
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        pub contains_ignore_case: Option<String>,
        pub not_contains: Option<String>,
        pub not_contains_ignore_case: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableCommentFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub body: Option<StringComparator>,
        pub user: Option<UserFilter>,
        pub issue: Option<Box<NullableIssueFilter>>,
        pub project_update: Option<Box<ProjectUpdateFilter>>,
        pub parent: Option<Box<NullableCommentFilter>>,
        pub document_content: Option<DocumentContentFilter>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Option<Box<CustomerNeedCollectionFilter>>,
        pub null: Option<bool>,
        pub and: Option<Vec<NullableCommentFilter>>,
        pub or: Option<Vec<NullableCommentFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ReactionCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub emoji: Option<StringComparator>,
        pub custom_emoji_id: Option<Idcomparator>,
        pub and: Option<Vec<ReactionCollectionFilter>>,
        pub or: Option<Vec<ReactionCollectionFilter>>,
        pub some: Option<ReactionFilter>,
        pub every: Option<ReactionFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ReactionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub emoji: Option<StringComparator>,
        pub custom_emoji_id: Option<Idcomparator>,
        pub and: Option<Vec<ReactionFilter>>,
        pub or: Option<Vec<ReactionFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct DocumentContentFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub project: Option<Box<ProjectFilter>>,
        pub document: Option<DocumentFilter>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct DocumentFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub title: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub creator: Option<UserFilter>,
        pub project: Option<Box<ProjectFilter>>,
        pub initiative: Option<InitiativeFilter>,
        pub and: Option<Vec<DocumentFilter>>,
        pub or: Option<Vec<DocumentFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableProjectFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub state: Option<StringComparator>,
        pub status: Option<Box<ProjectStatusFilter>>,
        pub priority: Option<NullableNumberComparator>,
        pub searchable_content: Option<ContentComparator>,
        pub completed_at: Option<NullableDateComparator>,
        pub start_date: Option<NullableDateComparator>,
        pub target_date: Option<NullableDateComparator>,
        pub health: Option<StringComparator>,
        pub has_related_relations: Option<RelationExistsComparator>,
        pub has_depended_on_by_relations: Option<RelationExistsComparator>,
        pub has_depends_on_relations: Option<RelationExistsComparator>,
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        pub has_blocking_relations: Option<RelationExistsComparator>,
        pub project_updates: Option<ProjectUpdatesCollectionFilter>,
        pub creator: Option<UserFilter>,
        pub lead: Option<Box<NullableUserFilter>>,
        pub members: Option<UserCollectionFilter>,
        pub issues: Option<Box<IssueCollectionFilter>>,
        pub roadmaps: Option<RoadmapCollectionFilter>,
        pub initiatives: Option<InitiativeCollectionFilter>,
        pub project_milestones: Option<ProjectMilestoneCollectionFilter>,
        pub completed_project_milestones: Option<ProjectMilestoneCollectionFilter>,
        pub next_project_milestone: Option<ProjectMilestoneFilter>,
        pub accessible_teams: Option<TeamCollectionFilter>,
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub needs: Option<Box<CustomerNeedCollectionFilter>>,
        pub null: Option<bool>,
        pub and: Option<Vec<NullableProjectFilter>>,
        pub or: Option<Vec<NullableProjectFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableTemplateFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub null: Option<bool>,
        pub and: Option<Vec<NullableTemplateFilter>>,
        pub or: Option<Vec<NullableTemplateFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct TeamCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub and: Option<Vec<TeamCollectionFilter>>,
        pub or: Option<Vec<TeamCollectionFilter>>,
        pub some: Option<TeamFilter>,
        pub every: Option<TeamFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug, Default)]
    pub struct TeamFilter {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub id: Option<Idcomparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub name: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub key: Option<StringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub description: Option<NullableStringComparator>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub issues: Option<Box<IssueCollectionFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub and: Option<Vec<TeamFilter>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub or: Option<Vec<TeamFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectMilestoneCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub target_date: Option<NullableDateComparator>,
        pub and: Option<Vec<ProjectMilestoneCollectionFilter>>,
        pub or: Option<Vec<ProjectMilestoneCollectionFilter>>,
        pub some: Option<ProjectMilestoneFilter>,
        pub every: Option<ProjectMilestoneFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectMilestoneFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub target_date: Option<NullableDateComparator>,
        pub and: Option<Vec<ProjectMilestoneFilter>>,
        pub or: Option<Vec<ProjectMilestoneFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct InitiativeCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub status: Option<StringComparator>,
        pub creator: Option<UserFilter>,
        pub and: Option<Vec<InitiativeCollectionFilter>>,
        pub or: Option<Vec<InitiativeCollectionFilter>>,
        pub some: Option<InitiativeFilter>,
        pub every: Option<InitiativeFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct InitiativeFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub status: Option<StringComparator>,
        pub creator: Option<UserFilter>,
        pub and: Option<Vec<InitiativeFilter>>,
        pub or: Option<Vec<InitiativeFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct RoadmapCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub creator: Option<UserFilter>,
        pub and: Option<Vec<RoadmapCollectionFilter>>,
        pub or: Option<Vec<RoadmapCollectionFilter>>,
        pub some: Option<RoadmapFilter>,
        pub every: Option<RoadmapFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct RoadmapFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub slug_id: Option<StringComparator>,
        pub creator: Option<UserFilter>,
        pub and: Option<Vec<RoadmapFilter>>,
        pub or: Option<Vec<RoadmapFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct UserCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub display_name: Option<StringComparator>,
        pub email: Option<StringComparator>,
        pub active: Option<BooleanComparator>,
        pub assigned_issues: Option<Box<IssueCollectionFilter>>,
        pub admin: Option<BooleanComparator>,
        pub is_me: Option<BooleanComparator>,
        pub and: Option<Vec<UserCollectionFilter>>,
        pub or: Option<Vec<UserCollectionFilter>>,
        pub some: Option<UserFilter>,
        pub every: Option<UserFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectUpdatesCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub health: Option<StringComparator>,
        pub and: Option<Vec<ProjectUpdatesCollectionFilter>>,
        pub or: Option<Vec<ProjectUpdatesCollectionFilter>>,
        pub some: Option<ProjectUpdatesFilter>,
        pub every: Option<ProjectUpdatesFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct ProjectUpdatesFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub health: Option<StringComparator>,
        pub and: Option<Vec<ProjectUpdatesFilter>>,
        pub or: Option<Vec<ProjectUpdatesFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct RelationExistsComparator {
        pub eq: Option<bool>,
        pub neq: Option<bool>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableDateComparator {
        pub eq: Option<DateTimeOrDuration>,
        pub neq: Option<DateTimeOrDuration>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<DateTimeOrDuration>>,
        pub nin: Option<Vec<DateTimeOrDuration>>,
        pub null: Option<bool>,
        pub lt: Option<DateTimeOrDuration>,
        pub lte: Option<DateTimeOrDuration>,
        pub gt: Option<DateTimeOrDuration>,
        pub gte: Option<DateTimeOrDuration>,
    }

    #[derive(cynic::InputObject, Debug, Default)]
    pub struct ContentComparator {
        pub contains: Option<String>,
        pub not_contains: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableNumberComparator {
        pub eq: Option<f64>,
        pub neq: Option<f64>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<f64>>,
        pub nin: Option<Vec<f64>>,
        pub null: Option<bool>,
        pub lt: Option<f64>,
        pub lte: Option<f64>,
        pub gt: Option<f64>,
        pub gte: Option<f64>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct UserFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub display_name: Option<StringComparator>,
        pub email: Option<StringComparator>,
        pub active: Option<BooleanComparator>,
        pub assigned_issues: Option<Box<IssueCollectionFilter>>,
        pub admin: Option<BooleanComparator>,
        pub is_me: Option<BooleanComparator>,
        pub and: Option<Vec<UserFilter>>,
        pub or: Option<Vec<UserFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct BooleanComparator {
        pub eq: Option<bool>,
        pub neq: Option<bool>,
    }

    impl BooleanComparator {
        pub fn is_true() -> Self {
            Self {
                eq: Some(true),
                neq: None,
            }
        }

        pub fn is_false() -> Self {
            Self {
                eq: Some(false),
                neq: None,
            }
        }
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct AttachmentCollectionFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub title: Option<StringComparator>,
        pub subtitle: Option<NullableStringComparator>,
        pub url: Option<StringComparator>,
        pub creator: Option<Box<NullableUserFilter>>,
        pub source_type: Option<SourceTypeComparator>,
        pub and: Option<Vec<AttachmentCollectionFilter>>,
        pub or: Option<Vec<AttachmentCollectionFilter>>,
        pub some: Option<AttachmentFilter>,
        pub every: Option<AttachmentFilter>,
        pub length: Option<NumberComparator>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NumberComparator {
        pub eq: Option<f64>,
        pub neq: Option<f64>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<f64>>,
        pub nin: Option<Vec<f64>>,
        pub lt: Option<f64>,
        pub lte: Option<f64>,
        pub gt: Option<f64>,
        pub gte: Option<f64>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct AttachmentFilter {
        pub id: Option<Idcomparator>,
        pub created_at: Option<DateComparator>,
        pub updated_at: Option<DateComparator>,
        pub title: Option<StringComparator>,
        pub subtitle: Option<NullableStringComparator>,
        pub url: Option<StringComparator>,
        pub creator: Option<Box<NullableUserFilter>>,
        pub source_type: Option<SourceTypeComparator>,
        pub and: Option<Vec<AttachmentFilter>>,
        pub or: Option<Vec<AttachmentFilter>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct SourceTypeComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        pub eq_ignore_case: Option<String>,
        pub neq_ignore_case: Option<String>,
        pub starts_with: Option<String>,
        pub starts_with_ignore_case: Option<String>,
        pub not_starts_with: Option<String>,
        pub ends_with: Option<String>,
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        pub contains_ignore_case: Option<String>,
        pub not_contains: Option<String>,
        pub not_contains_ignore_case: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NullableStringComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        pub null: Option<bool>,
        pub eq_ignore_case: Option<String>,
        pub neq_ignore_case: Option<String>,
        pub starts_with: Option<String>,
        pub starts_with_ignore_case: Option<String>,
        pub not_starts_with: Option<String>,
        pub ends_with: Option<String>,
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        pub contains_ignore_case: Option<String>,
        pub not_contains: Option<String>,
        pub not_contains_ignore_case: Option<String>,
    }

    #[derive(cynic::InputObject, Debug, Default)]
    pub struct StringComparator {
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub eq: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub neq: Option<String>,
        #[cynic(rename = "in", skip_serializing_if = "Option::is_none")]
        pub in_: Option<Vec<String>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub nin: Option<Vec<String>>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub eq_ignore_case: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub neq_ignore_case: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub starts_with: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub starts_with_ignore_case: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub not_starts_with: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub ends_with: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub not_ends_with: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub contains: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub contains_ignore_case: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub not_contains: Option<String>,
        #[cynic(skip_serializing_if = "Option::is_none")]
        pub not_contains_ignore_case: Option<String>,
    }

    impl StringComparator {
        pub fn eq(value: String) -> Self {
            Self {
                eq: Some(value),
                ..Default::default()
            }
        }
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct DateComparator {
        pub eq: Option<DateTimeOrDuration>,
        pub neq: Option<DateTimeOrDuration>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<DateTimeOrDuration>>,
        pub nin: Option<Vec<DateTimeOrDuration>>,
        pub lt: Option<DateTimeOrDuration>,
        pub lte: Option<DateTimeOrDuration>,
        pub gt: Option<DateTimeOrDuration>,
        pub gte: Option<DateTimeOrDuration>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "IDComparator")]
    pub struct Idcomparator {
        pub eq: Option<cynic::Id>,
        pub neq: Option<cynic::Id>,
        #[cynic(rename = "in")]
        pub in_: Option<Vec<cynic::Id>>,
        pub nin: Option<Vec<cynic::Id>>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct AssigneeSort {
        pub nulls: Option<PaginationNulls>,
        pub order: Option<PaginationSortOrder>,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct DateTimeOrDuration(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct TimelessDate(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct TimelessDateOrDuration(pub String);
}
