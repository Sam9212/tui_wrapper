use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    Frame,
};

/// A trait which should be implemented by your app struct.
/// 
/// This trait tells the [`UI`](crate::ui::UI) how to draw your app to the screen,
/// what to do when input is received, and how to find the variable
/// containing the open state of your program.
/// 
/// You can also implement the [`Ticked`] trait on your app which
/// allows you to run the [`UI`](crate::ui::UI) in ticked mode (using `run_ticked`).
/// 
/// # Examples
/// 
/// ```
/// use tui_wrapper::{ui::UI, app::App};
/// use tui::{
///     widgets::{Block, BorderType},
///     backend::Backend,
///     Frame,
/// };
/// use crossterm::event::{KeyEvent, KeyCode};
/// 
/// struct HappyTitleApp {
///     is_closed: bool,
/// }
/// 
/// impl App for HappyTitleApp {
///     fn draw(&mut self, f: &mut Frame<impl Backend>) {
///         let block = Block::default()
///             .title("Hi")
///             .border_type(BorderType::Thick);
///         f.render_widget(block, f.size());
///     }
/// 
///     fn on_input_received(&mut self, event: KeyEvent) {
///         if event.code == KeyCode::Char('q') {
///             self.is_closed = true;
///         }
///     }
/// 
///     fn is_closed(&self) -> bool {
///         self.is_closed
///     }
/// }
/// 
/// let mut ui = UI::new(HappyTitleApp { is_closed: false }).unwrap();
/// ui.run().expect("There was an issue with initializing the terminal!");
/// ui.destroy_app().expect("There was an error uninitializing the terminal!");
/// ```
pub trait App {
    /// The funcion called by the [`UI`](crate::ui::UI) every frame of the application.
    /// 
    /// Use the mutable reference to the [`Frame`] provided by this
    /// function to draw [`tui`] widgets to the screen.
    fn draw(&mut self, f: &mut Frame<impl Backend>);
    /// The function called by the [`UI`](crate::ui::UI) every time an input event
    /// is received.
    fn on_input_received(&mut self, event: KeyEvent);
    /// A getter function for anywhere in your code which indicates
    /// if your app is in a state where it should close. You most
    /// likely will want to use this as a getter for a field in your
    /// application struct, rather than to a variable in the main 
    /// body of your program.
    fn is_closed(&self) -> bool;
}

/// A secondary trait that can be implemented by your app struct.
/// 
/// This trait allows you to run code at a fixed rate whilst inputs
/// are not being received.
/// 
/// To use this trait correctly, you must implement it along with
/// the App trait and then use the `new_ticked` & `run_ticked` 
/// associated functions as opposed to the standard `new` and `run`
/// functions.
/// 
/// # Examples
/// 
/// ```
/// use tui_wrapper::{ui::UI, app::{App, Ticked}};
/// use tui::{
///     widgets::{Block, BorderType},
///     backend::Backend,
///     Frame,
/// };
/// use crossterm::event::{KeyEvent, KeyCode};
/// 
/// struct HappyTitleApp {
///     is_closed: bool,
///     ticks: u128,
/// }
/// 
/// impl App for HappyTitleApp {
///     fn draw(&mut self, f: &mut Frame<impl Backend>) {
///         let block = Block::default()
///             .title(format!("Hi :) - {}", self.ticks))
///             .border_type(BorderType::Thick);
///         f.render_widget(block, f.size());
///     }
/// 
///     fn on_input_received(&mut self, event: KeyEvent) {
///         if event.code == KeyCode::Char('q') {
///             self.is_closed = true;
///         }
///     }
/// 
///     fn is_closed(&self) -> bool {
///         self.is_closed
///     }
/// }
/// 
/// impl Ticked for HappyTitleApp {
///     fn on_tick(&mut self) {
///         self.ticks += 1;
///     }
/// }
/// 
/// let mut ui = UI::new(HappyTitleApp { is_closed: false, ticks: 0 }).unwrap();
/// ui.run().expect("There was an issue with initializing the terminal!");
/// ui.destroy_app().expect("There was an error uninitializing the terminal!");
/// ```
pub trait Ticked {
    /// A function called at a fixed interval by [`UI`](crate::ui::UI)s which are [`Ticked`]
    fn on_tick(&mut self);
}