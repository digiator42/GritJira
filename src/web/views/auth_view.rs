// src/web/views/auth_view.rs
use maud::{Markup, html};

pub fn login_page_view() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-900 via-gray-950 to-gray-900 text-white" {
            // Background decorative elements
            div class="absolute inset-0 overflow-hidden" {
                div class="absolute -top-40 -right-40 w-80 h-80 bg-blue-500/10 rounded-full blur-3xl" {}
                div class="absolute -bottom-40 -left-40 w-80 h-80 bg-purple-500/10 rounded-full blur-3xl" {}
            }

            div class="relative w-full max-w-md p-8 space-y-8 bg-gray-900/80 backdrop-blur-xl border border-gray-800/80 rounded-2xl shadow-2xl" {
                // Logo and header
                div class="text-center space-y-4" {
                    div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-2xl shadow-lg shadow-blue-500/30 mb-4" {
                        span class="text-3xl" { "⚡" }
                    }
                    div {
                        h1 class="text-2xl font-bold text-white tracking-wide" { "Welcome Back" }
                        p class="text-gray-400 text-sm mt-2" { "Sign in to your GritJira account" }
                    }
                }

                form
                    hx-post="/api/v1/auth/login"
                    hx-ext="json-enc"
                    hx-target="#auth-error"
                    hx-swap="innerHTML"
                    hx-on--after-request="handleLoginResponse(event)"
                    class="space-y-5"
                {
                    div id="auth-error" class="hidden bg-red-500/10 border border-red-500/30 text-red-400 text-sm p-3 rounded-lg" {}

                    div class="space-y-2" {
                        label class="block text-sm font-medium text-gray-300" { "Email Address" }
                        input type="email" name="email" required placeholder="you@example.com" class="w-full px-4 py-3 bg-gray-950/50 border border-gray-800 rounded-xl text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/50 transition-all duration-200";
                    }

                    div class="space-y-2" {
                        label class="block text-sm font-medium text-gray-300" { "Password" }
                        input type="password" name="password" required placeholder="••••••••" class="w-full px-4 py-3 bg-gray-950/50 border border-gray-800 rounded-xl text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/50 transition-all duration-200";
                        div class="flex items-center justify-between" {
                            a href="#" class="text-sm text-blue-400 hover:text-blue-300 hover:underline" { "Forgot password?" }
                        }
                    }

                    button type="submit" class="w-full py-3 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-semibold rounded-xl shadow-lg shadow-blue-900/50 transition-all duration-200 hover:shadow-xl hover:shadow-blue-900/60 hover:scale-[1.02] active:scale-[0.98]" {
                        "Sign In"
                    }
                }

                // Divider
                div class="relative" {
                    div class="absolute inset-0 flex items-center" {
                        div class="w-full border-t border-gray-800" {}
                    }
                    div class="relative flex justify-center text-sm" {
                        span class="px-2 bg-gray-900/80 text-gray-500" { "Or continue with" }
                    }
                }

                // Social login buttons
                div class="grid grid-cols-2 gap-3" {
                    button class="flex items-center justify-center gap-2 py-3 bg-gray-950/50 border border-gray-800 rounded-xl hover:bg-gray-800/50 transition-colors" {
                        span class="text-lg" { "🔐" }
                        span class="text-sm text-gray-300" { "SSO" }
                    }
                    button class="flex items-center justify-center gap-2 py-3 bg-gray-950/50 border border-gray-800 rounded-xl hover:bg-gray-800/50 transition-colors" {
                        span class="text-lg" { "🔑" }
                        span class="text-sm text-gray-300" { "API Key" }
                    }
                }

                // Footer
                div class="text-center pt-4 border-t border-gray-800/50" {
                    p class="text-gray-400 text-sm" { "Don't have an account? " }
                    a href="/jira/register" class="text-blue-400 hover:text-blue-300 font-medium hover:underline" { "Sign up" }
                }
            }

            // Login handler script
            script {
                (maud::PreEscaped(r#"
                    function handleLoginResponse(event) {
                        const response = JSON.parse(event.detail.xhr.responseText);
                        if (response.success) {
                            window.location.href = '/jira/dashboard';
                        } else {
                            const errorDiv = document.getElementById('auth-error');
                            if (errorDiv) {
                                errorDiv.textContent = response.message || 'Login failed';
                                errorDiv.classList.remove('hidden');
                            }
                        }
                    }
                "#))
            }
        }
    }
}

pub fn register_page_view() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-900 via-gray-950 to-gray-900 text-white" {
            // Background decorative elements
            div class="absolute inset-0 overflow-hidden" {
                div class="absolute -top-40 -right-40 w-80 h-80 bg-emerald-500/10 rounded-full blur-3xl" {}
                div class="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-500/10 rounded-full blur-3xl" {}
            }

            div class="relative w-full max-w-md p-8 space-y-8 bg-gray-900/80 backdrop-blur-xl border border-gray-800/80 rounded-2xl shadow-2xl" {
                // Logo and header
                div class="text-center space-y-4" {
                    div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-emerald-500 to-green-600 rounded-2xl shadow-lg shadow-emerald-500/30 mb-4" {
                        span class="text-3xl" { "⚡" }
                    }
                    div {
                        h1 class="text-2xl font-bold text-white tracking-wide" { "Create Account" }
                        p class="text-gray-400 text-sm mt-2" { "Join GritJira and start managing your projects" }
                    }
                }

                form
                    hx-post="/api/v1/auth/register"
                    hx-target="#auth-error"
                    hx-ext="json-enc"
                    hx-swap="innerHTML"
                    hx-on--after-request="handleRegisterResponse(event)"
                    class="space-y-5"
                {
                    div id="auth-error" class="hidden bg-red-500/10 border border-red-500/30 text-red-400 text-sm p-3 rounded-lg" {}

                    div class="space-y-2" {
                        label class="block text-sm font-medium text-gray-300" { "Full Name" }
                        input type="text" name="name" required placeholder="John Doe" class="w-full px-4 py-3 bg-gray-950/50 border border-gray-800 rounded-xl text-white placeholder-gray-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500/50 transition-all duration-200";
                    }

                    div class="space-y-2" {
                        label class="block text-sm font-medium text-gray-300" { "Email Address" }
                        input type="email" name="email" required placeholder="you@example.com" class="w-full px-4 py-3 bg-gray-950/50 border border-gray-800 rounded-xl text-white placeholder-gray-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500/50 transition-all duration-200";
                    }

                    div class="space-y-2" {
                        label class="block text-sm font-medium text-gray-300" { "Password" }
                        input type="password" name="password" required placeholder="••••••••" class="w-full px-4 py-3 bg-gray-950/50 border border-gray-800 rounded-xl text-white placeholder-gray-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500/50 transition-all duration-200";
                        p class="text-xxs text-gray-500" { "Must be at least 8 characters" }
                    }

                    button type="submit" class="w-full py-3 bg-gradient-to-r from-emerald-600 to-green-600 hover:from-emerald-500 hover:to-green-500 text-white font-semibold rounded-xl shadow-lg shadow-emerald-900/50 transition-all duration-200 hover:shadow-xl hover:shadow-emerald-900/60 hover:scale-[1.02] active:scale-[0.98]" {
                        "Create Account"
                    }
                }

                // Divider
                div class="relative" {
                    div class="absolute inset-0 flex items-center" {
                        div class="w-full border-t border-gray-800" {}
                    }
                    div class="relative flex justify-center text-sm" {
                        span class="px-2 bg-gray-900/80 text-gray-500" { "Or continue with" }
                    }
                }

                // Social signup buttons
                div class="grid grid-cols-2 gap-3" {
                    button class="flex items-center justify-center gap-2 py-3 bg-gray-950/50 border border-gray-800 rounded-xl hover:bg-gray-800/50 transition-colors" {
                        span class="text-lg" { "🔐" }
                        span class="text-sm text-gray-300" { "SSO" }
                    }
                    button class="flex items-center justify-center gap-2 py-3 bg-gray-950/50 border border-gray-800 rounded-xl hover:bg-gray-800/50 transition-colors" {
                        span class="text-lg" { "🔑" }
                        span class="text-sm text-gray-300" { "API Key" }
                    }
                }

                // Footer
                div class="text-center pt-4 border-t border-gray-800/50" {
                    p class="text-gray-400 text-sm" { "Already have an account? " }
                    a href="/jira/login" class="text-emerald-400 hover:text-emerald-300 font-medium hover:underline" { "Sign in" }
                }
            }

            script {
                (maud::PreEscaped(r#"
                    function handleRegisterResponse(event) {
                        const response = JSON.parse(event.detail.xhr.responseText);
                        if (response.success) {
                            window.location.href = '/jira/dashboard';
                        } else {
                            const errorDiv = document.getElementById('auth-error');
                            if (errorDiv) {
                                errorDiv.textContent = response.message || 'Registration failed';
                                errorDiv.classList.remove('hidden');
                            }
                        }
                    }
                "#))
            }
        }
    }
}
