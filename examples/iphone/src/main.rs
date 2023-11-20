use std::sync::RwLock;

use cacao::uikit::{
    App, AppDelegate, Scene, SceneConfig, SceneConnectionOptions, SceneSession, Window,
    WindowSceneDelegate,
};

use cacao::layout::LayoutConstraint;
use cacao::view::{View, ViewController, ViewDelegate};

#[derive(Default)]
struct TestApp;

impl AppDelegate for TestApp {
    fn config_for_scene_session(
        &self,
        session: SceneSession,
        _options: SceneConnectionOptions,
    ) -> SceneConfig {
        SceneConfig::new("Default Configuration", session.role())
    }
}
pub struct RootView {
    pub green: View,
    pub blue: View,
}

impl Default for RootView {
    fn default() -> Self {
        RootView {
            green: View::new(),
            blue: View::new(),
        }
    }
}

impl ViewDelegate for RootView {
    const NAME: &'static str = "RootView";

    fn did_load(&mut self, _view: View) {
        LayoutConstraint::activate(&[]);
    }
}

#[derive(Default)]
pub struct WindowScene {
    pub window: RwLock<Option<Window>>,
    pub root_view_controller: RwLock<Option<ViewController<RootView>>>,
}

impl WindowSceneDelegate for WindowScene {
    fn will_connect(&self, scene: Scene, _session: SceneSession, _options: SceneConnectionOptions) {
        let bounds = scene.get_bounds();
        let mut window = Window::new(bounds);
        window.set_window_scene(scene);

        let root_view_controller = ViewController::new(RootView::default());
        window.set_root_view_controller(&root_view_controller);
        window.show();

        {
            let mut w = self.window.write().unwrap();
            *w = Some(window);

            let mut vc = self.root_view_controller.write().unwrap();
            *vc = Some(root_view_controller);
        }
    }
}

fn main() {
    App::new(TestApp::default(), || Box::new(WindowScene::default())).run();
}
