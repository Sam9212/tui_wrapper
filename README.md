# tui_wrapper
A wrapper for ratatui which should make starting projects easier and quicker, with some minor constraints on design. As progress continues I will try to sacrifice as little design adaptability as possible. However, this may not be possible and I may be in future forced to redesign the crate. 

This crate is not on crates.io and will not be published to it until I feel like it's ready.

This crate is dependent on both ratatui and crossterm, so you'll need these in your `cargo.toml`:
```
[dependencies]
crossterm = "0.27"
ratatui = "0.23"
```

## How to use
To use this crate you need to create a struct which implements either one or two traits.
You **must** implement `App` on your struct and may also want to implement `Ticked` depending on your needs.

If your application does not need to do any kind of computation on a regular basis, and only needs to react 
to keypress events, then you only need to implement `App`.

### !! TODO: Make a more featureful example as opposed to empty one

```
use tui_wrapper::{
    ui::UI, app::App
};
use ratatui::{backend::Backend, Frame};
use crossterm::event::{KeyEvent, KeyCode};

struct MyApp(bool);
impl App for MyApp {
    #[allow(unused)]
    fn draw(&mut self, f: &mut Frame<impl Backend>) {
        // Write tui-rs code for drawing to screen
    }

    #[allow(unused)]
    fn on_input_received(&mut self, event: KeyEvent) {
        // Write logic for when an event is received
        match event.code {
            KeyCode::Char('q') => self.closed = true,
        }
    }
        
    fn is_closed(&self) -> bool {
        // Return anything that indicates if the UI should close.
        // In this case, is it just the first
        // field of our tuple-struct
        self.0 
    }
}
    
let app = MyApp(false);
let mut ui = UI::new(app).unwrap();
ui.run().expect("There was an error running the app");
ui.destroy_app().expect("Setting the terminal back to normal encountered an error!");