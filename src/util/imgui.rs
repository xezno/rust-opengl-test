use imgui::{sys::*, FontConfig, FontSource};

pub fn imgui_init() -> imgui::Context {
    let mut imgui = imgui::Context::create();

    //
    // Imgui setup
    //
    let font_data = include_bytes!("../../content/fonts/Roboto-Regular.ttf");
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: font_data,
        config: Some(FontConfig::default()),
        size_pixels: 14.0,
    }]);

    let io = imgui.io_mut();
    io.config_docking_with_shift = true;
    io.config_docking_always_tab_bar = true;
    io.config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    // MESS AHEAD
    imgui.style_mut().colors[ImGuiCol_Text as usize] = [1.00, 1.00, 1.00, 1.00];
    imgui.style_mut().colors[ImGuiCol_TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    imgui.style_mut().colors[ImGuiCol_WindowBg as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_ChildBg as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_PopupBg as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_Border as usize] = [0.43, 0.43, 0.50, 0.50];
    imgui.style_mut().colors[ImGuiCol_BorderShadow as usize] = [0.00, 0.00, 0.00, 0.00];
    imgui.style_mut().colors[ImGuiCol_FrameBg as usize] = [0.25, 0.25, 0.25, 1.00];
    imgui.style_mut().colors[ImGuiCol_FrameBgHovered as usize] = [0.38, 0.38, 0.38, 1.00];
    imgui.style_mut().colors[ImGuiCol_FrameBgActive as usize] = [0.67, 0.67, 0.67, 0.39];
    imgui.style_mut().colors[ImGuiCol_TitleBg as usize] = [0.08, 0.08, 0.09, 1.00];
    imgui.style_mut().colors[ImGuiCol_TitleBgActive as usize] = [0.08, 0.08, 0.09, 1.00];
    imgui.style_mut().colors[ImGuiCol_TitleBgCollapsed as usize] = [0.00, 0.00, 0.00, 0.51];
    imgui.style_mut().colors[ImGuiCol_MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.00];
    imgui.style_mut().colors[ImGuiCol_ScrollbarBg as usize] = [0.02, 0.02, 0.02, 0.53];
    imgui.style_mut().colors[ImGuiCol_ScrollbarGrab as usize] = [0.31, 0.31, 0.31, 1.00];
    imgui.style_mut().colors[ImGuiCol_ScrollbarGrabHovered as usize] = [0.41, 0.41, 0.41, 1.00];
    imgui.style_mut().colors[ImGuiCol_ScrollbarGrabActive as usize] = [0.51, 0.51, 0.51, 1.00];
    imgui.style_mut().colors[ImGuiCol_CheckMark as usize] = [0.11, 0.64, 0.92, 1.00];
    imgui.style_mut().colors[ImGuiCol_SliderGrab as usize] = [0.11, 0.64, 0.92, 1.00];
    imgui.style_mut().colors[ImGuiCol_SliderGrabActive as usize] = [0.08, 0.50, 0.72, 1.00];
    imgui.style_mut().colors[ImGuiCol_Button as usize] = [0.25, 0.25, 0.25, 1.00];
    imgui.style_mut().colors[ImGuiCol_ButtonHovered as usize] = [0.38, 0.38, 0.38, 1.00];
    imgui.style_mut().colors[ImGuiCol_ButtonActive as usize] = [0.67, 0.67, 0.67, 0.39];
    imgui.style_mut().colors[ImGuiCol_Header as usize] = [0.22, 0.22, 0.22, 1.00];
    imgui.style_mut().colors[ImGuiCol_HeaderHovered as usize] = [0.25, 0.25, 0.25, 1.00];
    imgui.style_mut().colors[ImGuiCol_HeaderActive as usize] = [0.67, 0.67, 0.67, 0.39];
    imgui.style_mut().colors[ImGuiCol_Separator as usize] =
        imgui.style_mut().colors[ImGuiCol_Border as usize];
    imgui.style_mut().colors[ImGuiCol_SeparatorHovered as usize] = [0.41, 0.42, 0.44, 1.00];
    imgui.style_mut().colors[ImGuiCol_SeparatorActive as usize] = [0.26, 0.59, 0.98, 0.95];
    imgui.style_mut().colors[ImGuiCol_ResizeGrip as usize] = [0.00, 0.00, 0.00, 0.00];
    imgui.style_mut().colors[ImGuiCol_ResizeGripHovered as usize] = [0.29, 0.30, 0.31, 0.67];
    imgui.style_mut().colors[ImGuiCol_ResizeGripActive as usize] = [0.26, 0.59, 0.98, 0.95];
    imgui.style_mut().colors[ImGuiCol_Tab as usize] = [0.08, 0.08, 0.09, 0.83];
    imgui.style_mut().colors[ImGuiCol_TabHovered as usize] = [0.33, 0.34, 0.36, 0.83];
    imgui.style_mut().colors[ImGuiCol_TabActive as usize] = [0.23, 0.23, 0.24, 1.00];
    imgui.style_mut().colors[ImGuiCol_TabUnfocused as usize] = [0.08, 0.08, 0.09, 1.00];
    imgui.style_mut().colors[ImGuiCol_TabUnfocusedActive as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    imgui.style_mut().colors[ImGuiCol_TextSelectedBg as usize] = [0.26, 0.59, 0.98, 0.35];
    imgui.style_mut().colors[ImGuiCol_DragDropTarget as usize] = [0.11, 0.64, 0.92, 1.00];
    imgui.style_mut().colors[ImGuiCol_NavHighlight as usize] = [0.26, 0.59, 0.98, 1.00];
    imgui.style_mut().colors[ImGuiCol_NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    imgui.style_mut().colors[ImGuiCol_NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    imgui.style_mut().colors[ImGuiCol_ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
    imgui.style_mut().frame_rounding = 32f32;
    imgui.style_mut().grab_rounding = imgui.style_mut().frame_rounding;

    return imgui;
}
