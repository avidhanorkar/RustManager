use axum::{
    debug_handler,
    extract::{Path, State}, http::StatusCode, Json
};
use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use crate::middleware::auth_middleware::Claims;
use crate::models::{user_model::User};

#[derive(Deserialize)]
pub struct TaskRequest {
    pub taskname: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub task_id: ObjectId,
    pub taskname: String,
    pub user_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<ObjectId>,
    pub taskname: String,
    pub status: String,
    pub user_id: String,
}

pub async fn create_task(
    State(db): State<Database>,
    claims: Claims,
    Json(mut payload): Json<TaskRequest>,
) -> Result<Json<TaskResponse>, (StatusCode, String)> {
    let user_id = claims.user_id.clone();

    if payload.taskname.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Task name cannot be empty".to_string(),
        ));
    }

    if payload.status.is_empty() {
        payload.status = "Pending".to_string();
    }

    let collection: Collection<Task> = db.collection("task");
    let user_collection = db.collection::<mongodb::bson::Document>("user");

    let new_task = Task {
        task_id: None,
        taskname: payload.taskname.clone(),
        status: payload.status.clone(),
        user_id: user_id.clone(),
    };

    match collection.insert_one(&new_task).await {
        Ok(insert_result) => {
            // Extract the ObjectId
            let inserted_id = insert_result.inserted_id.as_object_id().ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get inserted task ID".to_string(),
                )
            })?;

            // Convert user_id to ObjectId
            let user_obj_id = ObjectId::parse_str(&user_id).map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid user ID format".to_string(),
                )
            })?;

            // Update the user doc to push the task_id
            let update_result = user_collection
                .update_one(
                    doc! { "_id": user_obj_id },
                    doc! { "$push": { "tasks": inserted_id.clone() } },
                )
                .await;

            if let Err(e) = update_result {
                eprintln!("Failed to update user with task ID: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Task created but failed to link with user".to_string(),
                ));
            }

            return Ok(Json(TaskResponse {
                task_id: inserted_id,
                taskname: new_task.taskname,
                status: new_task.status,
                user_id: new_task.user_id,
            }));
        }
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
        }
    }
}

pub async fn update_task(
    State(db): State<Database>,
    claims: Claims,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskRequest>,
) -> Result<Json<TaskResponse>, (StatusCode, String)> {
    if payload.taskname.is_empty() || payload.status.is_empty() || task_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Task name and status and id are required".to_string(),
        ));
    }

    let collection: Collection<Task> = db.collection("task");
    let user = claims.user_id;

    let obj_id = match ObjectId::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid Task Id".to_string())),
    };

    let update_doc = doc! {
        "$set": {
            "taskname": &payload.taskname,
            "status": &payload.status
        }
    };
    let filter = doc! {"_id": obj_id};

    match collection.find_one(filter.clone()).await {
        Ok(Some(task)) => {
            if task.user_id == user {

                match collection.update_one(filter.clone(), update_doc).await {
                    Ok(_) => {
                        match collection.find_one(filter).await{
                            Ok(Some(updated_task)) => {
                                Ok(Json(TaskResponse {
                                task_id: obj_id,
                                taskname: (&updated_task.taskname).to_string(),
                                user_id: (&updated_task.user_id).to_string(),
                                status: (&updated_task.status).to_string(),
                            }))
                            }
                            _ =>
                                Err((
                                     StatusCode::INTERNAL_SERVER_ERROR,
                                    "Update Not Found".to_string()
                                )),
                            
                        }
                    }
                    Err(e) => {
                        println!("Error updating task: {}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to update the task".to_string(),
                        ));
                    }
                }
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Not authorized to update this task".to_string(),
                ));
            }
        } 
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                "The task id is not valid or there is not task with this id".to_string()
            ))
        }
        Err(e) => {
            println!("There is some error in finding the task to update: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "There is some error in finding the task to update".to_string(),
            ));
        }
    }
}

#[debug_handler]
pub async fn all_for_user(
    State(db): State<Database>,
    claims: Claims
) -> Result<Json<Vec<TaskResponse>>, (StatusCode, String)> {
    let user_id = claims.user_id.clone();
    let user_collection: Collection<User> = db.collection("user");
    let task_collection: Collection<Task> = db.collection("task");

    let user_obj_id = match ObjectId::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid user ID".to_string()
            ));
        }
    };

    let filter = doc! { "_id": user_obj_id };

    match user_collection.find_one(filter).await {
        Ok(Some(user)) => {
            let mut result = Vec::new();

            for task_obj_id in user.tasks {
                let task_filter = doc! { "_id": task_obj_id };

                match task_collection.find_one(task_filter).await {
                    Ok(Some(task_found)) => {
                        let task_response = TaskResponse {
                            task_id: task_obj_id,
                            taskname: task_found.taskname,
                            status: task_found.status,
                            user_id: user_id.clone(),
                        };
                        result.push(task_response);
                    }
                    Ok(None) => continue,
                    Err(e) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to fetch task: {}", e),
                        ));
                    }
                }
            }

            Ok(Json(result))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                "User not found".to_string()
            ))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e)
            ))
        }
    }
}
