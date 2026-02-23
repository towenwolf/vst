cmake_minimum_required(VERSION 3.22)
project(GenXTemplate VERSION 1.0.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Fetch JUCE
include(FetchContent)
FetchContent_Declare(
    JUCE
    GIT_REPOSITORY https://github.com/juce-framework/JUCE.git
    GIT_TAG 8.0.4
    GIT_SHALLOW TRUE
)
FetchContent_MakeAvailable(JUCE)

# ─────────────────────────────────────────────────────────────────────────────
# TEMPLATE INSTRUCTIONS:
#   1. Replace "GenXTemplate" with your plugin target name (e.g. "GenXFilter")
#   2. Update PRODUCT_NAME, PLUGIN_CODE, BUNDLE_ID, VST3_CATEGORIES
#   3. Add your DSP source files to target_sources
#   4. Add any plugin-specific binary data (icons, images) if needed
# ─────────────────────────────────────────────────────────────────────────────

# Define the plugin
juce_add_plugin(GenXTemplate
    COMPANY_NAME "trwolf"
    IS_SYNTH FALSE
    NEEDS_MIDI_INPUT FALSE
    NEEDS_MIDI_OUTPUT FALSE
    IS_MIDI_EFFECT FALSE
    EDITOR_WANTS_KEYBOARD_FOCUS FALSE
    COPY_PLUGIN_AFTER_BUILD TRUE
    PLUGIN_MANUFACTURER_CODE Trwf
    PLUGIN_CODE GxTp                      # CHANGE: unique 4-char code per plugin
    FORMATS AU VST3 Standalone
    PRODUCT_NAME "GenX Template"          # CHANGE: your plugin display name
    BUNDLE_ID "com.trwolf.genxtemplate"   # CHANGE: unique bundle ID
    AU_MAIN_TYPE kAudioUnitType_Effect
    VST3_CATEGORIES Fx                    # CHANGE: Delay, Reverb, Dynamics, EQ, etc.
)

# Embedded font binary data
juce_add_binary_data(GenXTemplateData
    SOURCES
        assets/fonts/BebasNeue-Regular.ttf
        assets/fonts/Inter-Variable.ttf
        assets/fonts/JetBrainsMono-Regular.ttf
)

# Plugin source files
target_sources(GenXTemplate
    PRIVATE
        src/PluginProcessor.cpp
        src/PluginEditor.cpp
)

# Include directories
target_include_directories(GenXTemplate
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
)

# JUCE modules
target_link_libraries(GenXTemplate
    PRIVATE
        GenXTemplateData
        juce::juce_audio_utils
        juce::juce_dsp
    PUBLIC
        juce::juce_recommended_config_flags
        juce::juce_recommended_lto_flags
        juce::juce_recommended_warning_flags
)

# Compile definitions
target_compile_definitions(GenXTemplate
    PUBLIC
        JUCE_WEB_BROWSER=0
        JUCE_USE_CURL=0
        JUCE_VST3_CAN_REPLACE_VST2=0
        JUCE_DISPLAY_SPLASH_SCREEN=0
)

# Copy to system plugin folders (macOS)
if(APPLE)
    set_target_properties(GenXTemplate PROPERTIES
        XCODE_ATTRIBUTE_INSTALL_PATH "$(HOME)/Library/Audio/Plug-Ins"
    )
endif()
