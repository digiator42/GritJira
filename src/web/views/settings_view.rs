// src/web/views/settings_view.rs
use maud::{Markup, html};
use crate::models::{ProjectMemberModel, ProjectModel, UserModel, WorkflowStepModel};

pub fn settings_view(
    project_id: i32,
    project: &ProjectModel,
    workflows: &[WorkflowStepModel],
    users: &[UserModel],
    project_members: &[ProjectMemberModel],
) -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200 max-w-4xl mx-auto" {
            // Header
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                div {
                    h1 class="text-xl font-bold text-white tracking-wide" { 
                        "Project Settings" 
                    }
                    p class="text-xxs text-gray-400" {
                        (format!("{} ({})", project.name, project.key))
                    }
                }
                a href={"/jira/board?project_id=" (project_id)}
                    hx-get={"/jira/board?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-push-url="true"
                    class="text-xxs text-blue-400 hover:underline" {
                    "← Back to Board"
                }
            }

            // Settings Tabs
            div class="flex gap-2 border-b border-gray-800" {
                a href="/jira/settings"
                    hx-get="/jira/settings"
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm text-blue-400 border-b-2 border-blue-400 transition" {
                    "⚙️ General"
                }
                a href={"/jira/settings/users?project_id=" (project_id)}
                    hx-get={"/jira/settings/users?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm text-gray-400 hover:text-gray-200 border-b-2 border-transparent hover:border-gray-600 transition" {
                    "👥 Users"
                }
                a href={"/jira/settings/workflow?project_id=" (project_id)}
                    hx-get={"/jira/settings/workflow?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm text-gray-400 hover:text-gray-200 border-b-2 border-transparent hover:border-gray-600 transition" {
                    "📋 Workflow"
                }
            }

            // Content
            div class="space-y-6 mt-6" {
                // Project Info Section
                div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-3" {
                    h2 class="text-sm font-bold text-blue-400 flex items-center gap-2" {
                        span { "ℹ️" }
                        span { "Project Information" }
                    }
                    
                    div class="grid grid-cols-2 gap-4" {
                        div {
                            label class="block text-xxs text-gray-500 uppercase tracking-wider" { "Project Key" }
                            p class="text-sm text-white font-mono" { (project.key) }
                        }
                        div {
                            label class="block text-xxs text-gray-500 uppercase tracking-wider" { "Project Name" }
                            input type="text" 
                                name="name" 
                                value=(project.name)
                                class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-1.5 text-sm text-white focus:outline-none focus:border-blue-500"
                                hx-patch={"/api/v1/projects/" (project.id)}
                                hx-trigger="change"
                                hx-target="#main-content"
                                hx-swap="innerHTML";
                        }
                        div class="col-span-2" {
                            label class="block text-xxs text-gray-500 uppercase tracking-wider" { "Description" }
                            textarea name="description"
                                rows="3"
                                class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-1.5 text-sm text-white resize-y focus:outline-none focus:border-blue-500"
                                hx-patch={"/api/v1/projects/" (project.id)}
                                hx-trigger="change"
                                hx-target="#main-content"
                                hx-swap="innerHTML" {
                                (project.description.as_deref().unwrap_or(""))
                            }
                        }
                    }
                }

                // Workflow Steps Section (Simplified)
                div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-3" {
                    div class="flex justify-between items-center" {
                        h2 class="text-sm font-bold text-purple-400 flex items-center gap-2" {
                            span { "📋" }
                            span { "Workflow Steps" }
                        }
                        a href={"/jira/settings/workflow?project_id=" (project_id)}
                            hx-get={"/jira/settings/workflow?project_id=" (project_id)}
                            hx-target="#main-content"
                            hx-swap="innerHTML"
                            class="text-xxs text-blue-400 hover:underline" {
                            "Manage Workflow →"
                        }
                    }
                    
                    div class="flex flex-wrap gap-2" {
                        @for step in workflows {
                            span class="bg-gray-800 text-gray-300 px-3 py-1 rounded text-xxs" {
                                (step.position) ". " (step.name)
                            }
                        }
                    }
                }

                // Users Section (Simplified)
                div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-3" {
                    div class="flex justify-between items-center" {
                        h2 class="text-sm font-bold text-green-400 flex items-center gap-2" {
                            span { "👥" }
                            span { "Project Members" }
                            span class="text-xxs text-gray-500" { (format!("({})", project_members.len())) }
                        }
                        a href={"/jira/settings/users?project_id=" (project_id)}
                            hx-get={"/jira/settings/users?project_id=" (project_id)}
                            hx-target="#main-content"
                            hx-swap="innerHTML"
                            class="text-xxs text-blue-400 hover:underline" {
                            "Manage Users →"
                        }
                    }
                    
                    div class="flex flex-wrap gap-2" {
                        @if project_members.is_empty() {
                            span class="text-gray-500 text-xxs" { "No members added yet" }
                        } @else {
                            @for member in project_members {
                                div class="flex items-center gap-2 bg-gray-800 rounded-lg px-3 py-1" {
                                    span class="text-xxs text-gray-400" { "👤" }
                                    span class="text-xxs text-gray-300" { 
                                        (member.username) 
                                    }
                                    span class="text-xxs text-gray-500" { (member.role) }
                                }
                            }
                        }
                    }
                }

                // Danger Zone
                div class="bg-red-950/30 border border-red-800/60 rounded-lg p-4 space-y-3" {
                    h2 class="text-sm font-bold text-red-400 flex items-center gap-2" {
                        span { "⚠️" }
                        span { "Danger Zone" }
                    }
                    p class="text-xxs text-gray-400" { 
                        "Deleting a project will permanently remove all associated issues, sprints, and workflow steps." 
                    }
                    button
                        hx-delete={"/api/v1/projects/" (project_id)}
                        hx-confirm="Are you sure you want to delete this project? This action cannot be undone!"
                        hx-target="#main-content"
                        hx-swap="innerHTML"
                        hx-push-url="true"
                        class="bg-red-600 hover:bg-red-500 text-white text-xs px-4 py-2 rounded transition" {
                        "🗑️ Delete Project"
                    }
                }
            }
        }
    }
}