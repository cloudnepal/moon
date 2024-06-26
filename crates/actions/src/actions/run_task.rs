use moon_action::{Action, ActionStatus, RunTaskNode};
use moon_action_context::{ActionContext, TargetState};
use moon_app_context::AppContext;
use moon_common::color;
use moon_project_graph::ProjectGraph;
use moon_task_runner::TaskRunner;
use std::sync::Arc;
use tracing::{instrument, warn};

#[instrument(skip(action, action_context, app_context, project_graph))]
pub async fn run_task(
    action: &mut Action,
    action_context: Arc<ActionContext>,
    app_context: Arc<AppContext>,
    project_graph: Arc<ProjectGraph>,
    node: &RunTaskNode,
) -> miette::Result<ActionStatus> {
    let project_id = node
        .target
        .get_project_id()
        .expect("Project required for running tasks!");
    let project = project_graph.get(project_id)?;
    let task = project.get_task(&node.target.task_id)?;

    // Must be set before running the task in case it fails and
    // and error is bubbled up the stack
    action.allow_failure = task.options.allow_failure;

    // If the task is persistent, set the status early since it "never finshes",
    // and the runner will error about a missing hash if it's a dependency
    if task.is_persistent() {
        action_context.set_target_state(&task.target, TargetState::Passthrough);
    }

    let operations = TaskRunner::new(&app_context, &project, task)?
        .run(&action_context, &action.node)
        .await?
        .operations;

    action.flaky = operations.is_flaky();
    action.status = operations.get_final_status();
    action.operations = operations;

    if action.has_failed() && action.allow_failure {
        warn!(
            "Task {} has failed, but is marked to allow failures, continuing pipeline",
            color::label(&task.target),
        );
    }

    Ok(action.status)
}
