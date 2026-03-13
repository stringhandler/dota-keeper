package com.volthawk.dota_keeper

import android.graphics.Color
import android.os.Bundle
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }

  // Set WebView background to match the splash screen color so there is no
  // white flash while the app's CSS loads.
  override fun onWebViewCreate(webView: WebView) {
    webView.setBackgroundColor(Color.parseColor("#111827"))
  }
}
