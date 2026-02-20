fn main() {
    // Load .env file if it exists (for local development)
    // In production/CI, environment variables are set by the build system
    if let Err(_) = dotenvy::dotenv() {
        println!("cargo:warning=No .env file found (this is OK for CI/production builds)");
    }

    // Tell Cargo to rerun this build script if POSTHOG_API_KEY changes
    println!("cargo:rerun-if-env-changed=POSTHOG_API_KEY");

    tauri_build::build()
}
