use maud::{Markup, html};

pub fn login_page_view() -> Markup {
    html! {
        div class="flex min-h-screen items-center justify-center bg-gray-900 text-white" {
            div class="w-full max-w-md p-8 space-y-6 bg-gray-800 rounded-lg shadow-md" {
                h2 class="text-2xl font-bold text-center" { "Sign in to GritJira" }

                form
                    hx-post="/auth/login"
                    hx-ext="json-enc"
                    hx-target="#auth-error"
                    hx-swap="innerHTML"
                    class="space-y-4"
                {
                    div id="auth-error" class="text-red-400 text-sm" {}

                    div {
                        label class="block text-sm font-medium" { "Email Address" }
                        input type="email" name="email" required class="w-full px-3 py-2 mt-1 bg-gray-700 rounded border border-gray-600 text-white focus:outline-none focus:border-blue-500";
                    }

                    div {
                        label class="block text-sm font-medium" { "Password" }
                        input type="password" name="password" required class="w-full px-3 py-2 mt-1 bg-gray-700 rounded border border-gray-600 text-white focus:outline-none focus:border-blue-500";
                    }

                    button type="submit" class="w-full py-2 font-semibold bg-blue-600 rounded hover:bg-blue-500 transition-colors" {
                        "Login"
                    }
                }
            }
        }
    }
}

pub fn register_page_view() -> Markup {
    html! {
        div class="flex min-h-screen items-center justify-center bg-gray-900 text-white" {
            div class="w-full max-w-md p-8 space-y-6 bg-gray-800 rounded-lg shadow-md" {
                h2 class="text-2xl font-bold text-center" { "Create GritJira Account" }

                form 
                    hx-post="/auth/register" 
                    hx-target="#auth-error" 
                    hx-ext="json-enc"
                    hx-swap="innerHTML" 
                    class="space-y-4" 
                {
                    div id="auth-error" class="text-red-400 text-sm" {}

                    div {
                        label class="block text-sm font-medium" { "Full Name" }
                        input type="text" name="name" required class="w-full px-3 py-2 mt-1 bg-gray-700 rounded border border-gray-600 text-white focus:outline-none focus:border-blue-500";
                    }

                    div {
                        label class="block text-sm font-medium" { "Email Address" }
                        input type="email" name="email" required class="w-full px-3 py-2 mt-1 bg-gray-700 rounded border border-gray-600 text-white focus:outline-none focus:border-blue-500";
                    }

                    div {
                        label class="block text-sm font-medium" { "Password" }
                        input type="password" name="password" required class="w-full px-3 py-2 mt-1 bg-gray-700 rounded border border-gray-600 text-white focus:outline-none focus:border-blue-500";
                    }

                    button type="submit" class="w-full py-2 font-semibold bg-green-600 rounded hover:bg-green-500 transition-colors" {
                        "Register"
                    }
                }
            }
        }
    }
}
