use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use rustbox::Key;
use context::{BulletContext, EditorContext};
use editor_view::ViewState;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum WidgetKey {
  MainEditor,
  InfoBar,
  Search
}

trait WidgetRegistry<T: Widget, S: EditorContext> {
  fn new(context: S) -> Self;
  fn register_widget(&mut self, widget: T);
  fn activate_widget(&mut self, widget_key: WidgetKey);
  fn forward_input(&mut self, key: Key);
}

#[derive(Clone, Debug)]
pub struct SimpleWidgetRegistry<T, S> {
  widgets: HashMap<WidgetKey, T>,
  context: Rc<RefCell<S>>
}

impl<T, S> WidgetRegistry<T, S> for SimpleWidgetRegistry<T, S>
  where T: Widget,
        S: EditorContext {

  fn new(context: S) -> Self {
    SimpleWidgetRegistry {
      widgets: HashMap::new(),
      context: Rc::new(RefCell::new(context))
    }
  }

  /// Make Bullet aware of a widget, allowing it to be displayed,
  /// activated, and for it to receive user input.
  fn register_widget(&mut self, widget: T) {
    self.widgets.insert(widget.get_key(), widget);
  }

  /// Activates a widget keyed on widget_key, deactivating 
  /// all other widgets.
  fn activate_widget(&mut self, widget_key: WidgetKey) {
    for (_, widget) in self.widgets.iter_mut() {
      if widget.get_key() == widget_key {
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
        widget.handle_input(input, self.context.borrow_mut());
      }
    }
  }

}

trait Widget {
  fn new(key: WidgetKey) -> Self;
  fn get_key(&self) -> WidgetKey;
  fn is_active(&self) -> bool;
  fn activate(&mut self);
  fn deactivate(&mut self);
  fn handle_input(&self, input: Key, context: RefMut<EditorContext>);
}

trait Renderable {
  fn render(element: &Self, context: &ViewState);
  fn is_displayed(&self) -> bool;
  fn show(&mut self);
  fn hide(&mut self);
}

pub struct SearchWidget {
  pub search_text: String,
  pub num_matches: usize,
  pub is_displayed: bool,
  pub is_active: bool
}

impl Widget for SearchWidget {

  fn new(key: WidgetKey) -> Self {
    SearchWidget {
      is_displayed: false,
      is_active: false,
      num_matches: 0,
      search_text: "".to_string()
    }
  }

  fn get_key(&self) -> WidgetKey {
    WidgetKey::Search
  }

  fn is_active(&self) -> bool {
    self.is_active
  }

  fn activate(&mut self) {
    self.is_active = true
  }

  fn deactivate(&mut self) {
    self.is_active = false
  }

  fn handle_input(&self, input: Key, context: RefMut<EditorContext>) {
    
  }

}

impl Renderable for SearchWidget {
  fn render(element: &Self, context: &ViewState) {
    
  }

  fn is_displayed(&self) -> bool {
    self.is_displayed
  }

  fn show(&mut self) {
    self.is_displayed = true;
  }

  fn hide(&mut self) {
    self.is_displayed = false;
  }
}