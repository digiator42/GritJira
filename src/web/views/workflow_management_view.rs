// src/web/views/workflow_management_view.rs
use maud::{Markup, html};
use crate::models::{ProjectModel, WorkflowStepModel};

pub fn workflow_management_view(
    project_id: i32,
    project_name: &str,
    workflows: &[WorkflowStepModel],
) -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200 max-w-4xl mx-auto" {
            // Header
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                div {
                    h1 class="text-xl font-bold text-white tracking-wide" { 
                        "Workflow Management" 
                    }
                    p class="text-xxs text-gray-400" {
                        (format!("{} - Configure workflow steps", project_name))
                    }
                }
                a href={"/jira/settings?project_id=" (project_id)}
                    hx-get={"/jira/settings?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="text-xxs text-blue-400 hover:underline" {
                    "← Back to Settings"
                }
            }

            // Tabs
            div class="flex gap-2 border-b border-gray-800" {
                a href="/jira/settings"
                    hx-get="/jira/settings"
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm whitespace-nowrap transition" {
                    "⚙️ General"
                }
                a href={"/jira/settings/users?project_id=" (project_id)}
                    hx-get={"/jira/settings/users?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm whitespace-nowrap transition" {
                    "👥 Users"
                }
                a href={"/jira/settings/workflow?project_id=" (project_id)}
                    hx-get={"/jira/settings/workflow?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm whitespace-nowrap transition" {
                    "📋 Workflow"
                }
            }

            // Workflow Steps
            div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-3" {
                div class="flex justify-between items-center" {
                    h2 class="text-sm font-bold text-purple-400" { 
                        "Workflow Steps " 
                        span class="text-xxs text-gray-500" { (format!("({})", workflows.len())) }
                    }
                    button
                        hx-post={"/jira/projects/" (project_id) "/workflow/add"}
                        hx-target="#workflow-list"
                        hx-swap="beforeend"
                        hx-on--after-request="this.closest('.bg-gray-900').querySelector('input:last-child')?.focus()"
                        class="text-xxs bg-purple-600 hover:bg-purple-500 text-white px-3 py-1.5 rounded transition" {
                        "+ Add Step"
                    }
                }
                
                div id="workflow-list" class="space-y-2" {
                    @if workflows.is_empty() {
                        div class="text-center py-8 text-gray-500" {
                            p { "No workflow steps configured." }
                            p class="text-xxs mt-1" { "Click 'Add Step' to create your first workflow column." }
                        }
                    } @else {
                        @for step in workflows {
                            div class="flex items-center justify-between bg-gray-950 border border-gray-800 rounded-lg p-2 hover:border-gray-700 transition" {
                                div class="flex items-center gap-3 flex-1" {
                                    span class="text-xxs text-gray-500 w-6" { (step.position) }
                                    input type="text"
                                        value=(step.name)
                                        class="flex-1 bg-transparent text-sm text-white focus:outline-none focus:border-blue-500 border border-transparent rounded px-2 py-1"
                                        hx-patch={"/jira/projects/" (project_id) "/workflow/" (step.id)}
                                        hx-trigger="change"
                                        hx-target="this"
                                        hx-swap="outerHTML"
                                        placeholder="Step name...";
                                    span class={@if step.is_completed { "text-green-400" } @else { "text-gray-500" } } {
                                        @if step.is_completed { "✅" } @else { "⬜" }
                                    }
                                }
                                div class="flex items-center gap-2" {
                                    button
                                        hx-post={"/jira/projects/" (project_id) "/workflow/" (step.id) "/toggle"}
                                        hx-target="closest div"
                                        hx-swap="outerHTML"
                                        class={@if step.is_completed { "text-green-400 hover:text-green-300" } @else { "text-gray-500 hover:text-gray-400" } } {
                                        @if step.is_completed { "⬜" } @else { "✅" }
                                    }
                                    button
                                        hx-delete={"/jira/projects/" (project_id) "/workflow/" (step.id)}
                                        hx-target="closest div"
                                        hx-swap="outerHTML"
                                        hx-confirm={"Delete workflow step '" (step.name) "'?"}
                                        class="text-red-400 hover:text-red-300 text-xxs transition" {
                                        "✕"
                                    }
                                }
                            }
                        }
                    }
                }

                // Info about workflow
                div class="mt-3 p-3 bg-gray-800/50 rounded-lg border border-gray-700" {
                    p class="text-xxs text-gray-400" {
                        "💡 "
                        "Workflow steps define the columns on your board. "
                        "Issues move through steps in order (from lowest to highest position)."
                    }
                }
            }
        }
    }
}