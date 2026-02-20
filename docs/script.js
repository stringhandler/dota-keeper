// Fetch the latest release information from GitHub API
async function loadLatestRelease() {
    try {
        const res = await fetch(
            'https://api.github.com/repos/stringhandler/dota-keeper/releases/latest'
        );

        if (!res.ok) {
            console.warn('Failed to fetch latest release, using fallback');
            return;
        }

        const data = await res.json();

        // Update version numbers
        document.querySelectorAll('.version').forEach(el => {
            el.textContent = data.tag_name;
        });

        // Update download links with actual asset URLs
        data.assets.forEach(asset => {
            const name = asset.name.toLowerCase();

            // Windows .msi.zip
            if (name.endsWith('.msi.zip')) {
                const windowsBtn = document.querySelector('#dl-windows');
                if (windowsBtn) {
                    windowsBtn.href = asset.browser_download_url;
                }
            }
            // Linux AppImage
            else if (name.endsWith('.appimage.tar.gz')) {
                const linuxBtn = document.querySelector('#dl-linux');
                if (linuxBtn) {
                    linuxBtn.href = asset.browser_download_url;
                }
            }
            // macOS Apple Silicon (aarch64)
            else if (name.includes('aarch64') && name.endsWith('.app.tar.gz')) {
                const macosArmBtn = document.querySelector('#dl-macos-arm');
                if (macosArmBtn) {
                    macosArmBtn.href = asset.browser_download_url;
                }
            }
            // macOS Intel (x64)
            else if (name.includes('x64') && name.endsWith('.app.tar.gz')) {
                const macosX64Btn = document.querySelector('#dl-macos-x64');
                if (macosX64Btn) {
                    macosX64Btn.href = asset.browser_download_url;
                }
            }
        });

        console.log(`Successfully loaded release ${data.tag_name} with ${data.assets.length} assets`);
    } catch (error) {
        console.error('Error fetching release information:', error);
        // Fallback URLs are already in the HTML
    }
}

// Load release information when page loads
document.addEventListener('DOMContentLoaded', loadLatestRelease);
