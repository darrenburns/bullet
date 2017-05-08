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
  fn activate_widget(&mut self, widget_key: &WidgetKey);
  fn forward_input(&mut self, key: Key);
}

#[derive(Clone, Debug)]
pub struct SimpleWidgetRegistry<T, S> {
  widgets: HashMap<WidgetKey, T>,
  context: Rc<S>
}

impl<T, S> WidgetRegistry<T, S> for SimpleWidgetRegistry<T, S>
  where T: Widget,
        S: EditorContext {

  fn new(context: S) -> Self {
    SimpleWidgetRegistry {
      widgets: HashMap::new(),
      context: Rc::new(context)
    }
  }

  /// Make Bullet aware of a widget, allowing it to be displayed,
  /// activated, and for it to receive user input.
  fn register_widget(&mut self, widget: T) {
    self.widgets.insert(widget.get_key(), widget);
  }


  fn activate_widget(&mut self, widget_key: &WidgetKey) {
    for (_, widget) in self.widgets.iter_mut() {
      if widget.get_key() == *widget_key {
        widget.activate();
      } else {
        widget.deactivate();
      }
    }
  }

  /// Forwards user input to the active widget, which will handle the
  /// input as it sees fit.
  fn forward_input(&mut self, input: Key) {
    for (_, widget) in self.widgets.iter() {
      if widget.is_active() {
        widget.handle_input(input);
      }
    }
  }

}

trait Widget {
  fn new(key: WidgetKey) -> Self;
  fn get_key(&self) -> WidgetKey;
  fn is_displayed(&self) -> bool;
  fn show(&mut self);
  fn hide(&mut self);
  fn is_active(&self) -> bool;
  fn activate(&mut self);
  fn deactivate(&mut self);
  fn handle_input(&self, input: Key);
}

