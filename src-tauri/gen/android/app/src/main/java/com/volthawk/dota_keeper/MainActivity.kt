package com.volthawk.dota_keeper

import android.graphics.Color
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
  private var webViewRef: WebView? = null
  private var insetsBottom = 0f
  private var insetsTop = 0f
  private val handler = Handler(Looper.getMainLooper())

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    // Capture real Android window insets and forward them to the WebView as CSS
    // variables. Android WebView does not automatically populate
    // env(safe-area-inset-*) from window insets (unlike iOS Safari), so we
    // inject the values manually after every inset change.
    ViewCompat.setOnApplyWindowInsetsListener(window.decorView) { view, insets ->
      val navBar = insets.getInsets(WindowInsetsCompat.Type.navigationBars())
      val statusBar = insets.getInsets(WindowInsetsCompat.Type.statusBars())
      val density = resources.displayMetrics.density
      insetsBottom = navBar.bottom / density
      insetsTop = statusBar.top / density
      injectInsets()
      ViewCompat.onApplyWindowInsets(view, insets)
    }
  }

  override fun onResume() {
    super.onResume()
    // Re-inject at multiple delays to ensure at least one attempt lands after
    // the SvelteKit app has fully loaded. 300 ms catches fast resumes;
    // 1 s and 3 s cover cold-start where the Tauri/Wry bundle takes longer.
    handler.postDelayed({ injectInsets() }, 300)
    handler.postDelayed({ injectInsets() }, 1000)
    handler.postDelayed({ injectInsets() }, 3000)
  }

  private fun injectInsets() {
    val wv = webViewRef ?: return
    wv.evaluateJavascript(
      "document.documentElement.style.setProperty('--sab','${insetsBottom}px');" +
      "document.documentElement.style.setProperty('--sat','${insetsTop}px');",
      null
    )
  }

  // Set WebView background to match the splash screen color so there is no
  // white flash while the app's CSS loads.
  override fun onWebViewCreate(webView: WebView) {
    webViewRef = webView
    webView.setBackgroundColor(Color.parseColor("#111827"))
    // Inject insets once the WebView exists — retried at increasing delays so
    // at least one attempt lands after the SvelteKit bundle has executed.
    handler.postDelayed({ injectInsets() }, 300)
    handler.postDelayed({ injectInsets() }, 1000)
    handler.postDelayed({ injectInsets() }, 3000)
  }
}
