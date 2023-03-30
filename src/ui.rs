use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use std::time::{Duration, Instant};
use crate::app::{App, Ticked};

/// The struct containing your application and your terminal.
///
/// An application is any struct which implements the [`App`]
/// trait and (optionally) the [`Ticked`] trait. 
/// 
/// You should create your UI using the `new` method 
/// (if you do not want it to be ticked), or, if you 
/// have an application struct which implements Ticked,
/// you should use `new_ticked` which allows you to 
/// supply a tickrate for your application.
///
/// You run these UIs using their respective `run` and 
/// `run_ticked` methods, depending on what kind of app
/// you supplied. Your code will panic if you use the
/// wrong run method for your UI (i.e. `new_ticked` and
/// `run` used together or vice versa).
/// 
/// # Examples
///
/// This example creates an app that immediately closes after running.
/// We can see this in the `draw` function where the variable which
/// we have assigned the open/closed state of the program to
/// is set to true immediately. The main loop of a UI only runs while
/// `is_closed` is returning false.
/// 
/// TODO: Update the example to be functional in some way rather than
/// immediately closing.
/// ```
/// use tui_wrapper::{ui::UI, app::App};
/// use tui::{backend::Backend, Frame};
/// use crossterm::event::Event;
/// 
/// struct MyApp(bool);
/// impl App for MyApp {
///     #[allow(unused)]
///     fn draw(&mut self, f: &mut Frame<impl Backend>) {
///         // Write tui-rs code for drawing to screen
///         self.0 = true;
///     }
/// 
///     #[allow(unused)]
///     fn on_input_received(&mut self, event: Event) {
///         // Write logic for when an event is received
///     }
///         
///     fn is_closed(&self) -> bool {
///         // Return what indicates if the UI should close.
///         // In this case, is it just the first
///         // field of our tuple-struct
///         self.0 
///     }
/// }
///     
/// let app = MyApp(false);
/// let mut ui = UI::new(app).unwrap();
/// ui.run().expect("There was an error running the app");
/// ui.destroy_app().expect("Setting the terminal back to normal encountered an error!");
/// ```
pub struct UI<A>
where
    A: App,
{
    /// The [`tui`] Terminal interface which is passed into
    /// crossterm commands.
    terminal: Terminal<CrosstermBackend<Stdout>>,
    /// How often your application struct's `on_tick` method is called.
    tick_rate: Option<Duration>,
    /// See [`App`] and [`Ticked`].
    app: A,
}

impl<A: App> UI<A> {
    /// This function creates a new [`UI`] instance, taking in a
    /// struct which implements [`App`].
    /// 
    /// It initializes the terminal by entering an alternate
    /// screen, and enabling mouse capture. This function should
    /// not be used with an application struct which also 
    /// implements [`Ticked`], in which case the function
    /// `new_ticked` should be used instead.
    pub fn new(app: A) -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(UI {
            terminal,
            tick_rate: None,
            app,
        })
    }

    /// This function runs an application that has been created
    /// using `new`, and will panic if used with a UI made with 
    /// `new_ticked`.
    pub fn run(&mut self) -> io::Result<()> {
        if let Some(_) = self.tick_rate {
            eprintln!("Hey! You shouldn't use `run` in conjunction with `new_ticked`. Use the functions");
            eprintln!("in their respective pairs, which are: `new` + `run`, and `new_ticked` + `run_ticked`.");
            panic!("`new`/`new_ticked` not used with respective `run`/`run_ticked` pair");
        }

        while !self.app.is_closed() {
            self.terminal.draw(|f| self.app.draw(f))?;
            let event = event::read()?;
            self.app.on_input_received(event);
        }
        Ok(())
    }
    
    /// This function leaves the alternate screen of the terminal
    /// and disables mouse capturing events. Use this after your
    /// app's main loop is completed.
    /// 
    /// Keep in mind that this function can be used for both 
    /// [`Ticked`] and not [`Ticked`] application structs,
    /// unlike the `new`/`run` pairs which have variants for 
    /// application structs which are [`Ticked`].
    pub fn destroy_app(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            DisableMouseCapture,
            LeaveAlternateScreen
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl<A: App + Ticked> UI<A> {
    /// This function creates a new UI, taking a tick rate
    /// value (the time between each `on_tick` function's
    /// calling), and an app struct which implements [`App`]
    /// and [`Ticked`].
    /// 
    /// It initializes the terminal by entering an alternate
    /// screen, and enabling mouse capture. This function 
    /// should not be used with an application struct that
    /// does not implement [`Ticked`], in which case the 
    /// function `new` should be used instead.
    pub fn new_ticked(app: A, tick_rate: Duration) -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(UI {
            terminal,
            tick_rate: Some(tick_rate),
            app,
        })
    }

    /// This function runs an application that has been created
    /// using `new_ticked`, and will panic if used with a UI made with 
    /// `new`.
    pub fn run_ticked(&mut self) -> io::Result<()> {
        let tr = match self.tick_rate {
            None => {
                eprintln!("Hey! You shouldn't use `run_ticked` in conjunction with `new`. Use the functions");
                eprintln!("in their respective pairs, which are: `new` + `run`, and `new_ticked` + `run_ticked`.");
                panic!("`new`/`new_ticked` not used with respective `run`/`run_ticked` pair");
            }
            Some(tr) => tr,
        };
        
        let mut last_tick = Instant::now();
        while !self.app.is_closed() {
            self.terminal.draw(|f| self.app.draw(f))?;
            let timeout = tr
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            if event::poll(timeout)? {
                let event = event::read()?;
                self.app.on_input_received(event);
            }
            if last_tick.elapsed() >= self.tick_rate.unwrap() {
                self.app.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }
}