use eframe::egui;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "inter_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/MFWS-Inter-Regular.ttf")),
    );
    fonts.font_data.insert(
        "jetbrains_mono_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf")),
    );
    fonts.font_data.insert(
        "source_han_sans_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/SourceHanSansSC-Regular.otf")),
    );
    fonts.font_data.insert(
        "material_design_icons_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/MaterialDesignIcons.ttf")),
    );
    fonts.font_data.insert(
        "material_symbols_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/MaterialSymbolsOutlined.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "inter_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "source_han_sans_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(2, "material_design_icons_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "material_symbols_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "jetbrains_mono_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(1, "source_han_sans_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

pub fn setup_custom_styles(ctx: &egui::Context) {
    let mut style: egui::Style = (*ctx.style()).clone();
    style.visuals.window_rounding = 0.0.into();
    style.visuals.menu_rounding = 0.0.into();
    style.visuals.widgets.noninteractive.rounding = 0.0.into();
    style.visuals.widgets.inactive.rounding = 0.0.into();
    style.visuals.widgets.hovered.rounding = 0.0.into();
    style.visuals.widgets.active.rounding = 0.0.into();
    style.visuals.widgets.open.rounding = 0.0.into();
    style.visuals.slider_trailing_fill = true;
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(0xBD, 0xBD, 0xBD));
    // style.animation_time = 1.0;
    ctx.set_style(style);
}
