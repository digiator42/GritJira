use maud::{Markup, html};

pub fn create_sprint_modal(project_id: i32) -> Markup {
    html! {
        div id="create-sprint-modal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4" onclick="if(event.target===this)this.remove()" {
            div class="bg-gray-900 border border-gray-800/60 rounded-xl w-full max-w-md p-6 space-y-4" onclick="event.stopPropagation()" {
                h3 class="text-sm font-bold text-white" { "Create New Sprint" }
                form hx-post={"/api/v1/sprints/projects/" (project_id)}
                      hx-ext="json-enc"
                      hx-target="#sprint-list"
                      hx-swap="innerHTML"
                      hx-on--after-request="document.getElementById('create-sprint-modal').remove()" {
                    input type="text" name="name" placeholder="Sprint name" required class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2 text-sm" {};
                    input type="text" name="goal" placeholder="Sprint goal" class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2 text-sm" {};
                    button type="submit" class="w-full bg-blue-600 hover:bg-blue-500 text-white font-mono text-sm py-2 rounded-lg" {
                        "Create Sprint"
                    }
                }
            }
        }
    }
}
