use bevy::app::App;
use bevy::diagnostic::LogDiagnosticsPlugin;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(LogDiagnosticsPlugin::default());
}
