fn main() {
    // Load .env file if it exists (for local development)
    // In production/CI, environment variables are set by the build system
    if let Err(_) = dotenvy::dotenv() {
        println!("cargo:warning=No .env file found (this is OK for CI/production builds)");
    }

    // Forward POSTHOG_API_KEY to the crate as a compile-time env var.
    // dotenvy only sets the build script's own env; cargo:rustc-env is needed
    // for env!() in the crate code to see it.
    let posthog_key = std::env::var("POSTHOG_API_KEY").unwrap_or_default();
    println!("cargo:rustc-env=POSTHOG_API_KEY={}", posthog_key);

    // Tell Cargo to rerun this build script if POSTHOG_API_KEY changes
    println!("cargo:rerun-if-env-changed=POSTHOG_API_KEY");

    tauri_build::build()
}
