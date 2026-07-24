use maud::{html, Markup};

pub fn create_issue_modal() -> Markup {
    html! {
        div id="create-issue-modal" class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4 z-50" {
            div class="bg-gray-900 border border-gray-800 rounded-xl max-w-lg w-full p-6 space-y-4 shadow-2xl" {
                div class="flex justify-between items-center border-b border-gray-800 pb-3" {
                    h3 class="text-sm font-mono font-bold text-white uppercase tracking-wider" { "Create New Issue" }
                    button onclick="document.getElementById('create-issue-modal').remove()" class="text-gray-400 hover:text-white" { "✕" }
                }

                form hx-post="/jira/issues/create"
                     hx-ext="json-enc"
                     hx-target="#main-content"
                     hx-swap="innerHTML"
                     class="space-y-4 font-mono text-xs" {
                    
                    div {
                        label class="block text-gray-400 mb-1" { "Summary" }
                        input type="text" name="summary" required class="w-full bg-gray-950 border border-gray-800 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500";
                    }

                    div class="grid grid-cols-2 gap-3" {
                        div {
                            label class="block text-gray-400 mb-1" { "Type" }
                            select name="issue_type" class="w-full bg-gray-950 border border-gray-800 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500" {
                                option value="task" { "Task" }
                                option value="bug" { "Bug" }
                                option value="story" { "Story" }
                            }
                        }

                        div {
                            label class="block text-gray-400 mb-1" { "Priority (1-5)" }
                            input type="number" name="priority" value="3" min="1" max="5" class="w-full bg-gray-950 border border-gray-800 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500";
                        }
                    }

                    div {
                        label class="block text-gray-400 mb-1" { "Description" }
                        textarea name="description" rows="4" class="w-full bg-gray-950 border border-gray-800 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500" {}
                    }

                    div class="flex justify-end gap-2 pt-2 border-t border-gray-800" {
                        button type="button" onclick="document.getElementById('create-issue-modal').remove()" class="px-4 py-2 bg-gray-800 text-gray-300 rounded hover:bg-gray-700" { "Cancel" }
                        button type="submit" class="px-4 py-2 bg-blue-600 text-white font-semibold rounded hover:bg-blue-500" { "Create Issue" }
                    }
                }
            }
        }
    }
}