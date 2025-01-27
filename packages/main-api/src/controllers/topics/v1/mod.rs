use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::Role;
use dto::*;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    repo: TopicRepository,
}

impl TopicControllerV1 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool);

        repo.create_table().await?;

        let ctrl = TopicControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_topic).post(Self::act_topic_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_topic).get(Self::list_topic))
            .with_state(ctrl.clone()))
    }

    pub async fn act_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(authz): Extension<Option<Authorization>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("list_topic {:?}", body);
        match authz {
            Some(Authorization::Bearer { claims }) if claims.role == Role::Admin => {}
            _ => return Err(ServiceError::Unauthorized),
        }

        match body {
            TopicAction::Create(TopicCreateRequest {
                title,
                content,
                started_at,
                ended_at,
                requirement,
            }) => {
                let topic = ctrl
                    .repo
                    .insert(title, content, started_at, ended_at, requirement)
                    .await?;
                Ok(Json(topic))
            }
        }
    }

    pub async fn act_topic_by_id(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Authorization>>,
        Path(id): Path<String>,
        Json(body): Json<TopicByIdAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("list_topic {:?} {:?}", id, body);
        Ok(Json(Topic::default()))
    }

    pub async fn get_topic(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);
        Ok(Json(Topic::default()))
    }

    pub async fn list_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_authz): Extension<Option<Authorization>>,
        Query(param): Query<TopicParam>,
    ) -> Result<Json<QueryResponse<TopicSummary>>> {
        tracing::debug!("list_topic {:?}", param);

        match param {
            TopicParam::Query(q) => {
                let topics = ctrl.repo.find(&q).await?;
                Ok(Json(topics))
            }
        }
    }
}