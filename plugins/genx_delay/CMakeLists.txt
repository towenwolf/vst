cmake_minimum_required(VERSION 3.22)
project(GenXDelay VERSION 1.0.0 LANGUAGES C CXX)

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

# Define the plugin
juce_add_plugin(GenXDelay
    COMPANY_NAME "trwolf"
    IS_SYNTH FALSE
    NEEDS_MIDI_INPUT FALSE
    NEEDS_MIDI_OUTPUT FALSE
    IS_MIDI_EFFECT FALSE
    EDITOR_WANTS_KEYBOARD_FOCUS FALSE
    COPY_PLUGIN_AFTER_BUILD TRUE
    PLUGIN_MANUFACTURER_CODE Trwf
    PLUGIN_CODE GxDl
    FORMATS AU VST3 Standalone
    PRODUCT_NAME "GenX Delay"
    BUNDLE_ID "com.trwolf.genxdelay"
    AU_MAIN_TYPE kAudioUnitType_Effect
    VST3_CATEGORIES Fx Delay
)

# Embedded font binary data
juce_add_binary_data(GenXDelayData
    SOURCES
        assets/fonts/Righteous-Regular.ttf
        assets/fonts/JosefinSans-Variable.ttf
        assets/fonts/BebasNeue-Regular.ttf
        assets/fonts/DSEG14Classic-Regular.ttf
        assets/fonts/DSEG7Classic-Regular.ttf
)

# Plugin source files
target_sources(GenXDelay
    PRIVATE
        src/PluginProcessor.cpp
        src/PluginEditor.cpp
        src/DelayLine.cpp
        src/Modulation.cpp
        src/Filters.cpp
        src/Saturation.cpp
        src/Ducker.cpp
)

# Include directories
target_include_directories(GenXDelay
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
)

# JUCE modules
target_link_libraries(GenXDelay
    PRIVATE
        GenXDelayData
        juce::juce_audio_utils
        juce::juce_dsp
    PUBLIC
        juce::juce_recommended_config_flags
        juce::juce_recommended_lto_flags
        juce::juce_recommended_warning_flags
)

# Compile definitions
target_compile_definitions(GenXDelay
    PUBLIC
        JUCE_WEB_BROWSER=0
        JUCE_USE_CURL=0
        JUCE_VST3_CAN_REPLACE_VST2=0
        JUCE_DISPLAY_SPLASH_SCREEN=0
)

# Copy to system plugin folders (macOS)
if(APPLE)
    set_target_properties(GenXDelay PROPERTIES
        XCODE_ATTRIBUTE_INSTALL_PATH "$(HOME)/Library/Audio/Plug-Ins"
    )
endif()
