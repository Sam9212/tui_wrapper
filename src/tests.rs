#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    fn it_has_a_bad_pair() {
        use crate::{
            UI,
            App,
            Ticked,
        };
        use tui::{
            backend::Backend,
            Frame,
        };
        use crossterm::event::{KeyCode, KeyEvent};

        struct MyApp {
            should_close: bool,
        }

        impl App for MyApp {
            fn draw(&mut self, _f: &mut Frame<impl Backend>) {
                
            }

            fn on_input_received(&mut self, event: KeyEvent) {
                if event.code == KeyCode::Char('q') {
                    self.should_close = true;
                }
            }

            fn is_closed(&self) -> bool {
                self.should_close
            }
        }

        impl Ticked for MyApp {
            fn on_tick(&mut self) {
                
            }
        }

        let mut bad_ui = UI::new(MyApp { should_close: false }).unwrap();
        // This expect will never be reached as this will panic!
        // You should not use `run_ticked` on a UI which has not been created with `new`!!!
        bad_ui.run_ticked().expect("An error was encountered during initialization of the terminal.");
        bad_ui.destroy_app().expect("An error was encountered uninitializing the terminal.");
    }
}