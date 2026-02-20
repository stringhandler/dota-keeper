/* ================================
   DOTA KEEPER - INTERACTIVE EFFECTS
   WebGL background, smooth scrolling,
   and dynamic animations
   ================================ */

// ================================
// WEBGL ANIMATED BACKGROUND
// ================================
class WebGLBackground {
    constructor() {
        this.canvas = document.getElementById('webgl-bg');
        if (!this.canvas) return;

        this.gl = this.canvas.getContext('webgl', { alpha: true, antialias: true });
        if (!this.gl) {
            console.warn('WebGL not supported, falling back to canvas gradient');
            this.fallbackGradient();
            return;
        }

        this.time = 0;
        this.mouse = { x: 0.5, y: 0.5 };
        this.targetMouse = { x: 0.5, y: 0.5 };

        this.init();
        this.resize();
        this.animate();

        window.addEventListener('resize', () => this.resize());
        window.addEventListener('mousemove', (e) => this.onMouseMove(e));
    }

    init() {
        const gl = this.gl;

        // Vertex shader
        const vertexShaderSource = `
            attribute vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        `;

        // Fragment shader with animated gradient mesh
        const fragmentShaderSource = `
            precision highp float;
            uniform float u_time;
            uniform vec2 u_resolution;
            uniform vec2 u_mouse;

            // Noise function
            vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
            vec2 mod289(vec2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
            vec3 permute(vec3 x) { return mod289(((x*34.0)+1.0)*x); }

            float snoise(vec2 v) {
                const vec4 C = vec4(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
                vec2 i  = floor(v + dot(v, C.yy));
                vec2 x0 = v - i + dot(i, C.xx);
                vec2 i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
                vec4 x12 = x0.xyxy + C.xxzz;
                x12.xy -= i1;
                i = mod289(i);
                vec3 p = permute(permute(i.y + vec3(0.0, i1.y, 1.0)) + i.x + vec3(0.0, i1.x, 1.0));
                vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);
                m = m*m;
                m = m*m;
                vec3 x = 2.0 * fract(p * C.www) - 1.0;
                vec3 h = abs(x) - 0.5;
                vec3 ox = floor(x + 0.5);
                vec3 a0 = x - ox;
                m *= 1.79284291400159 - 0.85373472095314 * (a0*a0 + h*h);
                vec3 g;
                g.x = a0.x * x0.x + h.x * x0.y;
                g.yz = a0.yz * x12.xz + h.yz * x12.yw;
                return 130.0 * dot(m, g);
            }

            void main() {
                vec2 st = gl_FragCoord.xy / u_resolution.xy;
                vec2 pos = st * 2.0 - 1.0;
                pos.x *= u_resolution.x / u_resolution.y;

                // Animate with time and mouse
                float t = u_time * 0.2;
                vec2 mouseInfluence = (u_mouse - 0.5) * 0.5;

                // Create flowing gradient with noise
                float noise1 = snoise(pos * 2.0 + vec2(t, t * 0.5) + mouseInfluence);
                float noise2 = snoise(pos * 3.0 - vec2(t * 0.7, t * 0.3) - mouseInfluence * 0.5);
                float noise3 = snoise(pos * 1.5 + vec2(sin(t) * 0.5, cos(t) * 0.5));

                // Combine noises for complex pattern
                float pattern = (noise1 + noise2 * 0.5 + noise3 * 0.3) * 0.5 + 0.5;

                // Dota 2 inspired color palette - gold/navy/purple
                vec3 color1 = vec3(0.04, 0.06, 0.1);      // Dark navy
                vec3 color2 = vec3(0.06, 0.08, 0.16);     // Medium navy
                vec3 color3 = vec3(0.96, 0.77, 0.19);     // Gold
                vec3 color4 = vec3(0.66, 0.33, 0.97);     // Purple

                // Create gradient mesh
                vec3 finalColor = mix(color1, color2, st.y);
                finalColor = mix(finalColor, color3 * 0.15, pattern * 0.3);
                finalColor += color4 * 0.08 * pow(noise2 * 0.5 + 0.5, 2.0);

                // Add radial gradient from center
                float dist = length(pos);
                float radial = 1.0 - smoothstep(0.0, 1.5, dist);
                finalColor += color3 * radial * 0.1;

                // Vignette
                float vignette = smoothstep(1.0, 0.3, dist);
                finalColor *= 0.5 + 0.5 * vignette;

                gl_FragColor = vec4(finalColor, 1.0);
            }
        `;

        // Compile shaders
        const vertexShader = this.compileShader(gl.VERTEX_SHADER, vertexShaderSource);
        const fragmentShader = this.compileShader(gl.FRAGMENT_SHADER, fragmentShaderSource);

        // Create program
        this.program = gl.createProgram();
        gl.attachShader(this.program, vertexShader);
        gl.attachShader(this.program, fragmentShader);
        gl.linkProgram(this.program);

        if (!gl.getProgramParameter(this.program, gl.LINK_STATUS)) {
            console.error('Program link failed:', gl.getProgramInfoLog(this.program));
            this.fallbackGradient();
            return;
        }

        gl.useProgram(this.program);

        // Create buffer
        const positions = new Float32Array([
            -1, -1,
             1, -1,
            -1,  1,
             1,  1,
        ]);

        const buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

        // Get attribute and uniform locations
        this.positionLocation = gl.getAttribLocation(this.program, 'position');
        this.timeLocation = gl.getUniformLocation(this.program, 'u_time');
        this.resolutionLocation = gl.getUniformLocation(this.program, 'u_resolution');
        this.mouseLocation = gl.getUniformLocation(this.program, 'u_mouse');

        gl.enableVertexAttribArray(this.positionLocation);
        gl.vertexAttribPointer(this.positionLocation, 2, gl.FLOAT, false, 0, 0);
    }

    compileShader(type, source) {
        const gl = this.gl;
        const shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);

        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            console.error('Shader compile failed:', gl.getShaderInfoLog(shader));
            gl.deleteShader(shader);
            return null;
        }

        return shader;
    }

    resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
        if (this.gl) {
            this.gl.viewport(0, 0, this.canvas.width, this.canvas.height);
        }
    }

    onMouseMove(e) {
        this.targetMouse.x = e.clientX / window.innerWidth;
        this.targetMouse.y = 1.0 - (e.clientY / window.innerHeight);
    }

    animate() {
        if (!this.gl || !this.program) return;

        const gl = this.gl;
        this.time += 0.016; // ~60fps

        // Smooth mouse movement
        this.mouse.x += (this.targetMouse.x - this.mouse.x) * 0.05;
        this.mouse.y += (this.targetMouse.y - this.mouse.y) * 0.05;

        // Set uniforms
        gl.uniform1f(this.timeLocation, this.time);
        gl.uniform2f(this.resolutionLocation, this.canvas.width, this.canvas.height);
        gl.uniform2f(this.mouseLocation, this.mouse.x, this.mouse.y);

        // Draw
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

        requestAnimationFrame(() => this.animate());
    }

    fallbackGradient() {
        // Canvas 2D fallback for browsers without WebGL
        const ctx = this.canvas.getContext('2d');
        if (!ctx) return;

        const animate = () => {
            const gradient = ctx.createRadialGradient(
                this.canvas.width / 2,
                this.canvas.height / 2,
                0,
                this.canvas.width / 2,
                this.canvas.height / 2,
                Math.max(this.canvas.width, this.canvas.height)
            );

            gradient.addColorStop(0, 'rgba(244, 196, 48, 0.1)');
            gradient.addColorStop(0.5, 'rgba(106, 84, 194, 0.05)');
            gradient.addColorStop(1, 'rgba(10, 14, 26, 0)');

            ctx.fillStyle = gradient;
            ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

            requestAnimationFrame(animate);
        };

        this.resize();
        animate();
    }
}

// ================================
// SCROLL ANIMATIONS
// ================================
class ScrollAnimations {
    constructor() {
        this.observeElements();
    }

    observeElements() {
        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        entry.target.classList.add('fade-in-up');
                        observer.unobserve(entry.target);
                    }
                });
            },
            { threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
        );

        // Observe cards and sections
        const elements = document.querySelectorAll('.download-card, .feature-card, .section-header');
        elements.forEach((el, index) => {
            el.style.opacity = '0';
            el.style.animationDelay = `${index * 0.1}s`;
            observer.observe(el);
        });
    }
}

// ================================
// SMOOTH SCROLL TO ANCHOR
// ================================
function setupSmoothScroll() {
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            if (href === '#' || !href) return;

            const target = document.querySelector(href);
            if (target) {
                e.preventDefault();
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
}

// ================================
// PARALLAX EFFECT ON SCREENSHOT
// ================================
function setupParallax() {
    const screenshot = document.querySelector('.screenshot-img');
    if (!screenshot) return;

    window.addEventListener('scroll', () => {
        const scrolled = window.pageYOffset;
        const rate = scrolled * 0.3;
        screenshot.style.transform = `translateY(${rate}px)`;
    });
}

// ================================
// DYNAMIC PLATFORM DETECTION
// ================================
function detectPlatform() {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    let detectedPlatform = 'windows';

    if (platform.includes('mac') || userAgent.includes('mac')) {
        detectedPlatform = 'macos';
    } else if (platform.includes('linux') || userAgent.includes('linux')) {
        detectedPlatform = 'linux';
    }

    // Highlight detected platform card
    const cards = document.querySelectorAll('.download-card');
    cards.forEach(card => {
        const cardPlatform = card.getAttribute('data-platform');
        if (cardPlatform === detectedPlatform) {
            card.classList.add('featured');
            // Ensure it's visible
            card.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
        }
    });
}

// ================================
// GITHUB RELEASE FETCHER
// ================================
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

            // Windows .msi (NOT .msi.zip)
            if (name.endsWith('.msi')) {
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
            // macOS Apple Silicon (aarch64) - check before x64 to avoid conflicts
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

        // Update release date if available
        if (data.published_at) {
            const date = new Date(data.published_at);
            const formattedDate = date.toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
            document.querySelectorAll('.version-date').forEach(el => {
                el.textContent = `• Released ${formattedDate}`;
            });
        }

        console.log(`Successfully loaded release ${data.tag_name} with ${data.assets.length} assets`);
    } catch (error) {
        console.error('Error fetching release information:', error);
        // Fallback URLs are already in the HTML
    }
}

// ================================
// BUTTON RIPPLE EFFECT
// ================================
function setupRippleEffect() {
    const buttons = document.querySelectorAll('.btn-download, .btn-primary, .btn-secondary');

    buttons.forEach(button => {
        button.addEventListener('click', function(e) {
            const rect = this.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;

            const ripple = document.createElement('span');
            ripple.style.cssText = `
                position: absolute;
                left: ${x}px;
                top: ${y}px;
                width: 0;
                height: 0;
                border-radius: 50%;
                background: rgba(255, 255, 255, 0.5);
                transform: translate(-50%, -50%);
                pointer-events: none;
                animation: ripple-effect 0.6s ease-out;
            `;

            this.appendChild(ripple);

            setTimeout(() => ripple.remove(), 600);
        });
    });

    // Add ripple animation to stylesheet
    if (!document.querySelector('#ripple-style')) {
        const style = document.createElement('style');
        style.id = 'ripple-style';
        style.textContent = `
            @keyframes ripple-effect {
                to {
                    width: 400px;
                    height: 400px;
                    opacity: 0;
                }
            }
        `;
        document.head.appendChild(style);
    }
}

// ================================
// INITIALIZE EVERYTHING
// ================================
document.addEventListener('DOMContentLoaded', () => {
    // Initialize WebGL background
    new WebGLBackground();

    // Initialize scroll animations
    new ScrollAnimations();

    // Setup smooth scrolling
    setupSmoothScroll();

    // Setup parallax effect
    setupParallax();

    // Detect user's platform
    detectPlatform();

    // Load latest release from GitHub
    loadLatestRelease();

    // Setup button ripple effects
    setupRippleEffect();

    // Log welcome message
    console.log('%c⚔️ DOTA KEEPER ⚔️', 'font-size: 24px; font-weight: bold; color: #f4c430;');
    console.log('%cTrack your matches. Set your goals. Dominate the game.', 'font-size: 14px; color: #94a3b8;');
    console.log('%cGitHub: https://github.com/stringhandler/dota-keeper', 'font-size: 12px; color: #f4c430;');
});

// ================================
// PERFORMANCE OPTIMIZATION
// ================================
// Reduce animations on low-end devices
if (navigator.hardwareConcurrency && navigator.hardwareConcurrency < 4) {
    document.documentElement.style.setProperty('--reduced-motion', '1');
}

// Lazy load images
if ('loading' in HTMLImageElement.prototype) {
    const images = document.querySelectorAll('img[loading="lazy"]');
    images.forEach(img => {
        img.src = img.dataset.src || img.src;
    });
} else {
    // Fallback for browsers that don't support lazy loading
    const script = document.createElement('script');
    script.src = 'https://cdnjs.cloudflare.com/ajax/libs/lazysizes/5.3.2/lazysizes.min.js';
    document.body.appendChild(script);
}
