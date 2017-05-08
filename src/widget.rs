use std::collections::HashMap;
use std::rc::Rc;

use rustbox::Key;
use context::{BulletContext, EditorContext};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum WidgetKey {
  MainEditor,
  InfoBar,
  Search
}

trait WidgetRegistry<T: Widget, S: EditorContext> {
  fn new(context: S) -> Self;
  fn register_widget(&mut self, widget: T);
  fn forward_input(&mut self, key: Key);
  // fn get_active_widget(&self) -> Option<T>;
  // fn get_widget_by_key(&self, key: &WidgetKey) -> Option<&T>;
  // fn get_all_widgets(&self) -> Vec<&T>;
}

#[derive(Clone, Debug)]
pub struct SimpleWidgetRegistry<T, S> {
  active_widget: Option<T>,
  widgets: HashMap<WidgetKey, T>,
  context: Rc<S>
}

impl<T, S> WidgetRegistry<T, S> for SimpleWidgetRegistry<T, S>
  where T: Widget,
        S: EditorContext {

  fn new(context: S) -> Self {
    SimpleWidgetRegistry {
      active_widget: None,
      widgets: HashMap::new(),
      context: Rc::new(context)
    }
  }

  /// Make Bullet aware of a widget, allowing it to be displayed,
  /// activated, and for it to receive user input.
  fn register_widget(&mut self, widget: T) {
    self.widgets.insert(widget.get_key(), widget);
  }

  /// Forwards user input to the active widget, which will handle the
  /// input as it sees fit.
  fn forward_input(&mut self, input: Key) {
    self.widgets.iter()
                .filter(|&(name, widg)| widg.is_active())
                .map(|(name, widg)| widg.handle_input(input));
  }

}

trait Widget {
  fn new(key: WidgetKey) -> Self;
  fn get_key(&self) -> WidgetKey;
  fn handle_input(&self, input: Key);
  fn is_displayed(&self) -> bool;
  fn show(&mut self);
  fn hide(&mut self);
}

