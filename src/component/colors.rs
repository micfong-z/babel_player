use eframe::egui::Color32;

pub struct MfColors;

#[allow(dead_code)]
impl MfColors {
    pub const WHITE: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
    pub const GRAY_50: Color32 = Color32::from_rgb(0xf5, 0xf5, 0xf5);
    pub const GRAY_100: Color32 = Color32::from_rgb(0xee, 0xee, 0xee);
    pub const GRAY_200: Color32 = Color32::from_rgb(0xe0, 0xe0, 0xe0);
    pub const GRAY_300: Color32 = Color32::from_rgb(0xbd, 0xbd, 0xbd);
    pub const GRAY_400: Color32 = Color32::from_rgb(0x9e, 0x9e, 0x9e);
    pub const GRAY_500: Color32 = Color32::from_rgb(0x80, 0x80, 0x80);
    pub const GRAY_600: Color32 = Color32::from_rgb(0x75, 0x75, 0x75);
    pub const GRAY_700: Color32 = Color32::from_rgb(0x61, 0x61, 0x61);
    pub const GRAY_800: Color32 = Color32::from_rgb(0x42, 0x42, 0x42);
    pub const GRAY_900: Color32 = Color32::from_rgb(0x21, 0x21, 0x21);
    pub const GRAY_950: Color32 = Color32::from_rgb(0x13, 0x13, 0x13);
    pub const BLACK: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);

    pub const RED_50: Color32 = Color32::from_rgb(0xff, 0xe3, 0xe3);
    pub const RED_100: Color32 = Color32::from_rgb(0xfd, 0xd3, 0xd5);
    pub const RED_200: Color32 = Color32::from_rgb(0xfa, 0xb2, 0xb9);
    pub const RED_300: Color32 = Color32::from_rgb(0xf4, 0x94, 0x9d);
    pub const RED_400: Color32 = Color32::from_rgb(0xf0, 0x5e, 0x6c);
    pub const RED_500: Color32 = Color32::from_rgb(0xe4, 0x37, 0x48);
    pub const RED_600: Color32 = Color32::from_rgb(0xb2, 0x2d, 0x39);
    pub const RED_700: Color32 = Color32::from_rgb(0x8c, 0x24, 0x2e);
    pub const RED_800: Color32 = Color32::from_rgb(0x5b, 0x19, 0x1f);
    pub const RED_900: Color32 = Color32::from_rgb(0x36, 0x0e, 0x12);
    pub const RED_950: Color32 = Color32::from_rgb(0x22, 0x08, 0x0a);

    pub const ORANGE_50: Color32 = Color32::from_rgb(0xff, 0xe7, 0xda);
    pub const ORANGE_100: Color32 = Color32::from_rgb(0xfc, 0xd6, 0xc0);
    pub const ORANGE_200: Color32 = Color32::from_rgb(0xf9, 0xbc, 0x98);
    pub const ORANGE_300: Color32 = Color32::from_rgb(0xf6, 0xa3, 0x73);
    pub const ORANGE_400: Color32 = Color32::from_rgb(0xf1, 0x88, 0x4b);
    pub const ORANGE_500: Color32 = Color32::from_rgb(0xec, 0x6f, 0x27);
    pub const ORANGE_600: Color32 = Color32::from_rgb(0xc7, 0x5e, 0x1c);
    pub const ORANGE_700: Color32 = Color32::from_rgb(0xa1, 0x4a, 0x13);
    pub const ORANGE_800: Color32 = Color32::from_rgb(0x63, 0x2b, 0x08);
    pub const ORANGE_900: Color32 = Color32::from_rgb(0x3a, 0x19, 0x04);
    pub const ORANGE_950: Color32 = Color32::from_rgb(0x26, 0x0f, 0x01);

    pub const YELLOW_50: Color32 = Color32::from_rgb(0xff, 0xf4, 0xd5);
    pub const YELLOW_100: Color32 = Color32::from_rgb(0xff, 0xeb, 0xb0);
    pub const YELLOW_200: Color32 = Color32::from_rgb(0xff, 0xe3, 0x8d);
    pub const YELLOW_300: Color32 = Color32::from_rgb(0xff, 0xd6, 0x5c);
    pub const YELLOW_400: Color32 = Color32::from_rgb(0xff, 0xcc, 0x34);
    pub const YELLOW_500: Color32 = Color32::from_rgb(0xff, 0xc1, 0x07);
    pub const YELLOW_600: Color32 = Color32::from_rgb(0xcc, 0x96, 0x0e);
    pub const YELLOW_700: Color32 = Color32::from_rgb(0x97, 0x70, 0x0d);
    pub const YELLOW_800: Color32 = Color32::from_rgb(0x62, 0x43, 0x09);
    pub const YELLOW_900: Color32 = Color32::from_rgb(0x35, 0x24, 0x03);
    pub const YELLOW_950: Color32 = Color32::from_rgb(0x27, 0x1a, 0x01);

    pub const GREEN_50: Color32 = Color32::from_rgb(0xdb, 0xf7, 0xdc);
    pub const GREEN_100: Color32 = Color32::from_rgb(0xc1, 0xec, 0xc2);
    pub const GREEN_200: Color32 = Color32::from_rgb(0x96, 0xe2, 0x9e);
    pub const GREEN_300: Color32 = Color32::from_rgb(0x6e, 0xdb, 0x86);
    pub const GREEN_400: Color32 = Color32::from_rgb(0x2e, 0xbf, 0x57);
    pub const GREEN_500: Color32 = Color32::from_rgb(0x14, 0xae, 0x52);
    pub const GREEN_600: Color32 = Color32::from_rgb(0x0d, 0x88, 0x3e);
    pub const GREEN_700: Color32 = Color32::from_rgb(0x08, 0x68, 0x2f);
    pub const GREEN_800: Color32 = Color32::from_rgb(0x05, 0x49, 0x20);
    pub const GREEN_900: Color32 = Color32::from_rgb(0x03, 0x30, 0x15);
    pub const GREEN_950: Color32 = Color32::from_rgb(0x01, 0x24, 0x0f);

    pub const BLUE_50: Color32 = Color32::from_rgb(0xd9, 0xec, 0xff);
    pub const BLUE_100: Color32 = Color32::from_rgb(0xbe, 0xdf, 0xff);
    pub const BLUE_200: Color32 = Color32::from_rgb(0x8f, 0xc7, 0xff);
    pub const BLUE_300: Color32 = Color32::from_rgb(0x54, 0xa9, 0xfd);
    pub const BLUE_400: Color32 = Color32::from_rgb(0x32, 0x96, 0xfb);
    pub const BLUE_500: Color32 = Color32::from_rgb(0x00, 0x7a, 0xf5);
    pub const BLUE_600: Color32 = Color32::from_rgb(0x01, 0x61, 0xc1);
    pub const BLUE_700: Color32 = Color32::from_rgb(0x02, 0x4b, 0xa0);
    pub const BLUE_800: Color32 = Color32::from_rgb(0x00, 0x34, 0x71);
    pub const BLUE_900: Color32 = Color32::from_rgb(0x01, 0x24, 0x4d);
    pub const BLUE_950: Color32 = Color32::from_rgb(0x00, 0x16, 0x37);

    pub const PURPLE_50: Color32 = Color32::from_rgb(0xec, 0xe1, 0xff);
    pub const PURPLE_100: Color32 = Color32::from_rgb(0xde, 0xcb, 0xff);
    pub const PURPLE_200: Color32 = Color32::from_rgb(0xc2, 0x9e, 0xff);
    pub const PURPLE_300: Color32 = Color32::from_rgb(0xae, 0x7e, 0xff);
    pub const PURPLE_400: Color32 = Color32::from_rgb(0xa0, 0x6c, 0xff);
    pub const PURPLE_500: Color32 = Color32::from_rgb(0x91, 0x54, 0xff);
    pub const PURPLE_600: Color32 = Color32::from_rgb(0x67, 0x36, 0xc0);
    pub const PURPLE_700: Color32 = Color32::from_rgb(0x56, 0x2d, 0xa4);
    pub const PURPLE_800: Color32 = Color32::from_rgb(0x39, 0x1d, 0x70);
    pub const PURPLE_900: Color32 = Color32::from_rgb(0x29, 0x13, 0x53);
    pub const PURPLE_950: Color32 = Color32::from_rgb(0x16, 0x06, 0x33);
}
