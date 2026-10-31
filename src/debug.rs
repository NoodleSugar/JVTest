use bevy::{color::palettes::css::GREEN, prelude::*};

pub fn debug_plugin(app: &mut App) {
	app.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin {
		config: bevy::dev_tools::fps_overlay::FpsOverlayConfig {
			text_config: TextFont {
				font_size: FontSize::Px(42.0),
				..default()
			},
			text_color: GREEN.into(),
			refresh_interval: core::time::Duration::from_millis(100),
			enabled: true,
			frame_time_graph_config:
				bevy::dev_tools::fps_overlay::FrameTimeGraphConfig {
					enabled: true,
					min_fps: 30.0,
					target_fps: 60.0,
				},
		},
	});
}
