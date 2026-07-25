use maud::{html, Markup};

pub fn projects_view(projects: &[crate::models::ProjectModel]) -> Markup {
    html! {
        div class="p-6 space-y-4 font-mono text-xs text-gray-200" {
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                h1 class="text-xl font-bold text-white tracking-wide" { "Projects" }
                span class="text-gray-400" { (format!("{} projects", projects.len())) }
            }

            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" {
                @for project in projects {
                    div class="bg-gray-900 border border-gray-800 rounded-lg p-4 hover:border-gray-700 transition" {
                        div class="flex items-center justify-between" {
                            div class="flex items-center gap-2" {
                                span class="text-xl" { "📁" }
                                h2 class="text-sm font-bold text-white" { (project.name) }
                            }
                            span class="text-xxs bg-blue-950 text-blue-400 border border-blue-800/60 px-2 py-0.5 rounded" {
                                (project.key)
                            }
                        }
                        p class="text-gray-400 text-xxs mt-2" { (project.description.as_deref().unwrap_or("No description")) }
                        div class="mt-3 flex items-center gap-2" {
                            a href={"/jira/board?project_id=" (project.id)}
                               hx-get={"/jira/board?project_id=" (project.id)}
                               hx-target="#main-content"
                               hx-push-url="true"
                               class="text-xs text-blue-400 hover:underline" {
                                "View Board"
                            }
                            a href={"/jira/backlog?project_id=" (project.id)}
                               hx-get={"/jira/backlog?project_id=" (project.id)}
                               hx-target="#main-content"
                               hx-push-url="true"
                               class="text-xs text-gray-400 hover:underline" {
                                "Backlog"
                            }
                        }
                    }
                }
            }
        }
    }
}