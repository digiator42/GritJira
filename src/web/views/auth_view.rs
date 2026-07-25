// src/web/views/auth_view.rs
use maud::{Markup, html};

pub fn login_page_view() -> Markup {
    html! {
        div class="flex min-h-screen items-center justify-center bg-gray-900 text-white" {
            div class="w-full max-w-md p-8 space-y-6 bg-gray-800 rounded-lg shadow-md" {
                h2 class="text-2xl font-bold text-center" { "Sign in to GritJira" }

                form
                    hx-post="/api/v1/auth/login"
                    hx-ext="json-enc"
                    hx-target="#auth-error"
                    hx-swap="innerHTML"
                    hx-on--after-request="handleLoginResponse(event)"
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

                // Login handler script
                script {
                    (maud::PreEscaped(r#"
                        function handleLoginResponse(event) {
                            const response = JSON.parse(event.detail.xhr.responseText);
                            if (response.success) {
                                // Redirect to the board page
                                window.location.href = '/jira/board';
                            } else {
                                // Show error message
                                const errorDiv = document.getElementById('auth-error');
                                if (errorDiv) {
                                    errorDiv.textContent = response.message || 'Login failed';
                                }
                            }
                        }
                    "#))
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
                    hx-post="/api/v1/auth/register"
                    hx-target="#auth-error"
                    hx-ext="json-enc"
                    hx-swap="innerHTML"
                    hx-on--after-request="handleRegisterResponse(event)"
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

                script {
                    (maud::PreEscaped(r#"
                        function handleRegisterResponse(event) {
                            const response = JSON.parse(event.detail.xhr.responseText);
                            if (response.success) {
                                // Redirect to login page or board
                                window.location.href = '/jira/board';
                            } else {
                                const errorDiv = document.getElementById('auth-error');
                                if (errorDiv) {
                                    errorDiv.textContent = response.message || 'Registration failed';
                                }
                            }
                        }
                    "#))
                }
            }
        }
    }
}
