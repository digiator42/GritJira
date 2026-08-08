// src/web/views/dashboard_view.rs
use maud::{html, Markup};

pub fn dashboard_view() -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200" {
            // Dashboard Header
            div class="flex justify-between items-center border-b border-gray-800/60 pb-4" {
                div {
                    h1 class="text-2xl font-bold text-white tracking-wide" { "Dashboard" }
                    p class="text-gray-400 text-sm mt-1" { "Overview of your project activity" }
                }
                div class="flex items-center gap-2" {
                    button class="bg-gray-800/50 hover:bg-gray-700/50 text-gray-300 px-3 py-2 rounded-lg transition-colors flex items-center gap-2" {
                        span { "📊" }
                        span { "Export Report" }
                    }
                    button class="bg-blue-600 hover:bg-blue-500 text-white px-3 py-2 rounded-lg transition-colors flex items-center gap-2" {
                        span { "🔄" }
                        span { "Refresh" }
                    }
                }
            }

            // Metrics Grid
            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4" {
                (crate::web::components::card::metric_card("Total Issues", "47", Some("+12%"), "📋", "blue"))
                (crate::web::components::card::metric_card("In Progress", "12", Some("+5%"), "🔄", "orange"))
                (crate::web::components::card::metric_card("Completed", "28", Some("+8%"), "✅", "green"))
                (crate::web::components::card::metric_card("Story Points", "156", Some("+15%"), "⭐", "purple"))
            }

            // Recent Activity & Sprint Status
            div class="grid grid-cols-1 lg:grid-cols-2 gap-6" {
                // Recent Activity
                div class="bg-gray-900/80 backdrop-blur-sm border border-gray-800/80 rounded-xl p-5" {
                    div class="flex items-center justify-between mb-4" {
                        h2 class="text-sm font-bold text-white uppercase tracking-wider" { "Recent Activity" }
                        a href="#" class="text-blue-400 hover:underline text-xxs" { "View All" }
                    }
                    div class="space-y-3" {
                        div class="flex items-start gap-3 p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                            div class="w-8 h-8 bg-blue-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                                span class="text-blue-400 text-sm" { "📝" }
                            }
                            div class="flex-1 min-w-0" {
                                p class="text-sm text-gray-200" { "New issue created: PROJ-47" }
                                p class="text-xxs text-gray-500 mt-1" { "2 minutes ago by John Doe" }
                            }
                        }
                        div class="flex items-start gap-3 p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                            div class="w-8 h-8 bg-emerald-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                                span class="text-emerald-400 text-sm" { "✅" }
                            }
                            div class="flex-1 min-w-0" {
                                p class="text-sm text-gray-200" { "PROJ-45 moved to Done" }
                                p class="text-xxs text-gray-500 mt-1" { "15 minutes ago by Jane Smith" }
                            }
                        }
                        div class="flex items-start gap-3 p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                            div class="w-8 h-8 bg-amber-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                                span class="text-amber-400 text-sm" { "💬" }
                            }
                            div class="flex-1 min-w-0" {
                                p class="text-sm text-gray-200" { "Comment added to PROJ-42" }
                                p class="text-xxs text-gray-500 mt-1" { "1 hour ago by Mike Johnson" }
                            }
                        }
                    }
                }

                // Sprint Status
                div class="bg-gray-900/80 backdrop-blur-sm border border-gray-800/80 rounded-xl p-5" {
                    div class="flex items-center justify-between mb-4" {
                        h2 class="text-sm font-bold text-white uppercase tracking-wider" { "Current Sprint" }
                        span class="text-xxs bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 px-2 py-1 rounded-full" { "Active" }
                    }
                    div class="space-y-4" {
                        div {
                            div class="flex justify-between mb-2" {
                                span class="text-sm text-gray-300" { "Sprint Progress" }
                                span class="text-sm text-emerald-400 font-semibold" { "65%" }
                            }
                            div class="w-full bg-gray-800 rounded-full h-2" {
                                div class="bg-gradient-to-r from-emerald-500 to-green-500 h-2 rounded-full transition-all duration-500" style="width: 65%" {}
                            }
                        }
                        div class="grid grid-cols-3 gap-3 mt-4" {
                            div class="text-center p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                                p class="text-2xl font-bold text-white" { "12" }
                                p class="text-xxs text-gray-500 mt-1" { "To Do" }
                            }
                            div class="text-center p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                                p class="text-2xl font-bold text-blue-400" { "8" }
                                p class="text-xxs text-gray-500 mt-1" { "In Progress" }
                            }
                            div class="text-center p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                                p class="text-2xl font-bold text-emerald-400" { "4" }
                                p class="text-xxs text-gray-500 mt-1" { "Done" }
                            }
                        }
                        div class="flex items-center justify-between mt-4 pt-4 border-t border-gray-800/50" {
                            div class="flex items-center gap-2" {
                                span class="text-gray-400 text-sm" { "📅" }
                                span class="text-gray-300 text-sm" { "3 days remaining" }
                            }
                            a href="/jira/board" class="text-blue-400 hover:underline text-sm" { "View Board" }
                        }
                    }
                }
            }

            // Team Performance
            div class="bg-gray-900/80 backdrop-blur-sm border border-gray-800/80 rounded-xl p-5" {
                div class="flex items-center justify-between mb-4" {
                    h2 class="text-sm font-bold text-white uppercase tracking-wider" { "Team Performance" }
                    select class="bg-gray-950/50 border border-gray-800 rounded-lg px-3 py-1.5 text-sm text-gray-300 focus:outline-none focus:border-blue-500" {
                        option { "This Week" }
                        option { "This Month" }
                        option { "This Quarter" }
                    }
                }
                div class="space-y-3" {
                    div class="flex items-center gap-4 p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                        div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-full flex items-center justify-center text-sm font-bold text-white" {
                            "JD"
                        }
                        div class="flex-1" {
                            div class="flex items-center justify-between mb-1" {
                                span class="text-sm text-gray-200" { "John Doe" }
                                span class="text-sm text-emerald-400 font-semibold" { "12 issues" }
                            }
                            div class="w-full bg-gray-800 rounded-full h-1.5" {
                                div class="bg-gradient-to-r from-blue-500 to-indigo-500 h-1.5 rounded-full" style="width: 85%" {}
                            }
                        }
                    }
                    div class="flex items-center gap-4 p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                        div class="w-10 h-10 bg-gradient-to-br from-purple-500 to-pink-600 rounded-full flex items-center justify-center text-sm font-bold text-white" {
                            "JS"
                        }
                        div class="flex-1" {
                            div class="flex items-center justify-between mb-1" {
                                span class="text-sm text-gray-200" { "Jane Smith" }
                                span class="text-sm text-emerald-400 font-semibold" { "9 issues" }
                            }
                            div class="w-full bg-gray-800 rounded-full h-1.5" {
                                div class="bg-gradient-to-r from-purple-500 to-pink-500 h-1.5 rounded-full" style="width: 72%" {}
                            }
                        }
                    }
                    div class="flex items-center gap-4 p-3 bg-gray-950/50 rounded-lg border border-gray-800/50" {
                        div class="w-10 h-10 bg-gradient-to-br from-amber-500 to-orange-600 rounded-full flex items-center justify-center text-sm font-bold text-white" {
                            "MJ"
                        }
                        div class="flex-1" {
                            div class="flex items-center justify-between mb-1" {
                                span class="text-sm text-gray-200" { "Mike Johnson" }
                                span class="text-sm text-emerald-400 font-semibold" { "7 issues" }
                            }
                            div class="w-full bg-gray-800 rounded-full h-1.5" {
                                div class="bg-gradient-to-r from-amber-500 to-orange-500 h-1.5 rounded-full" style="width: 58%" {}
                            }
                        }
                    }
                }
            }
        }
    }
}