# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.
#
# For more details, see
#   http://developer.android.com/guide/developing/tools/proguard.html

# If your project uses WebView with JS, uncomment the following
# and specify the fully qualified class name to the JavaScript interface
# class:
#-keepclassmembers class fqcn.of.javascript.interface.for.webview {
#   public *;
#}

# Uncomment this to preserve the line number information for
# debugging stack traces.
#-keepattributes SourceFile,LineNumberTable

# If you keep the line number information, uncomment this to
# hide the original source file name.
#-renamesourcefileattribute SourceFile

# ── Tauri / wry / tao JNI surface ──────────────────────────────────────────
# tao (ndk_glue.onActivityCreate) and the Tauri plugin framework call into
# these classes/methods reflectively by name over JNI. R8 cannot see those
# call sites, so without these rules it strips/renames members such as
# WryActivity.getId(), producing a JavaException -> abort on launch.
# Required before re-enabling isMinifyEnabled in build.gradle.kts.
-keep class com.volthawk.dota_keeper.** { *; }
-keep class app.tauri.** { *; }
-keepclassmembers class * extends app.tauri.plugin.Plugin { *; }