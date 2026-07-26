// src/web/views/user_management_view.rs
use maud::{Markup, html};
use crate::models::{UserModel, ProjectMemberModel};

pub fn user_management_view(
    project_id: i32,
    project_name: &str,
    members: &[ProjectMemberModel],
    available_users: &[UserModel],
) -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200 max-w-4xl mx-auto" {
            // Header
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                div {
                    h1 class="text-xl font-bold text-white tracking-wide" { 
                        "User Management" 
                    }
                    p class="text-xxs text-gray-400" {
                        (format!("{} - Manage project members", project_name))
                    }
                }
                a href={"/jira/settings?project_id=" (project_id)}
                    hx-get={"/jira/settings?project_id=" (project_id)}
                    hx-ext="json-enc"
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
                    class="px-4 py-2 text-sm text-gray-400 hover:text-gray-200 border-b-2 border-transparent hover:border-gray-600 transition" {
                    "⚙️ General"
                }
                a href={"/jira/settings/users?project_id=" (project_id)}
                    hx-get={"/jira/settings/users?project_id=" (project_id)}
                    hx-target="#main-content"
                    hx-swap="innerHTML"
                    class="px-4 py-2 text-sm text-blue-400 border-b-2 border-blue-400 transition" {
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

            // Add Member Section
            div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-3" {
                h2 class="text-sm font-bold text-green-400" { "Add Member" }
                
                form hx-post={"/api/v1/projects/" (project_id) "/members"}
                     hx-ext="json-enc"
                     hx-target="#members-list"
                     hx-swap="innerHTML"
                     hx-on--after-request="this.reset()"
                     class="flex gap-3" {
                    
                    select name="user_id"
                        required
                        class="flex-1 bg-gray-950 border border-gray-800 rounded-lg px-3 py-2 text-sm text-gray-100 focus:outline-none focus:border-blue-500" {
                        option value="" { "Select user..." }
                        @for user in available_users {
                            option value=(user.id) { (user.username) " (" (user.email) ")" }
                        }
                    }
                    
                    select name="role"
                        class="bg-gray-950 border border-gray-800 rounded-lg px-3 py-2 text-sm text-gray-100 focus:outline-none focus:border-blue-500" {
                        option value="Member" { "Member" }
                        option value="Admin" { "Admin" }
                        option value="Viewer" { "Viewer" }
                    }
                    
                    button type="submit"
                        class="bg-green-600 hover:bg-green-500 text-white px-4 py-2 rounded-lg text-xs transition" {
                        "+ Add"
                    }
                }
            }

            // Members List
            div id="members-list" class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-3" {
                h2 class="text-sm font-bold text-gray-400" { 
                    "Current Members " 
                    span class="text-xxs text-gray-500" { (format!("({})", members.len())) }
                }

                @if members.is_empty() {
                    p class="text-gray-500 text-xxs text-center py-4" { "No members in this project" }
                } @else {
                    table class="w-full text-left" {
                        thead class="text-xxs text-gray-500 uppercase tracking-wider border-b border-gray-800" {
                            tr {
                                th class="py-2" { "User" }
                                th class="py-2" { "Role" }
                                th class="py-2" { "Joined" }
                                th class="py-2 text-right" { "Actions" }
                            }
                        }
                        tbody class="divide-y divide-gray-800" {
                            @for member in members {
                                tr class="hover:bg-gray-800/50 transition" {
                                    td class="py-2" {
                                        div class="flex items-center gap-2" {
                                            span class="text-gray-400" { "👤" }
                                            span { (member.username) }
                                        }
                                    }
                                    td {
                                        select
                                            name="role"
                                            hx-patch={"/api/v1/projects/" (project_id) "/members/" (member.id)}
                                            hx-trigger="change"
                                            hx-target="closest tr"
                                            hx-swap="outerHTML"
                                            class="bg-gray-950 border border-gray-800 rounded px-2 py-1 text-xs text-gray-300 focus:outline-none focus:border-blue-500" {
                                            option value="Admin" selected[member.role == "Admin"] { "Admin" }
                                            option value="Member" selected[member.role == "Member"] { "Member" }
                                            option value="Viewer" selected[member.role == "Viewer"] { "Viewer" }
                                        }
                                    }
                                    td class="text-xxs text-gray-500" { (member.joined_at.format("%Y-%m-%d").to_string()) }
                                    td class="text-right" {
                                        @if member.role != "Admin" || members.len() > 1 {
                                            button
                                                hx-delete={"/api/v1/projects/" (project_id) "/members/" (member.id)}
                                                hx-target="closest tr"
                                                hx-swap="outerHTML"
                                                hx-confirm={"Remove " (member.username) " from the project?"}
                                                class="text-red-400 hover:text-red-300 text-xxs transition" {
                                                "✕ Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}