 /* main.rs- A Rust port of an old Gnomemm version of the wc program
* Copyright (C) 2017 Paulo Pinto
*
* This library is free software; you can redistribute it and/or
* modify it under the terms of the GNU Lesser General Public
* License as published by the Free Software Foundation; either
* version 2 of the License, or (at your option) any later version.
*
* This library is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
* Lesser General Public License for more details.
*
* You should have received a copy of the GNU Lesser General Public
* License along with this library; if not, write to the
* Free Software Foundation, Inc., 59 Temple Place - Suite 330,
* Boston, MA 02111-1307, USA.
*/
extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Label, Menu, MenuBar, MenuItem, IconSize, Image, AboutDialog, Toolbar, ToolButton,
    ToolbarStyle, SeparatorToolItem, FileChooserDialog, FileChooserAction, ResponseType,
    MessageDialog, MessageType, ButtonsType, DIALOG_MODAL};

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// The application window state
#[derive(Debug)]
struct GWCApp {
    /// the window label used to display the counters
    msg_label: Option<Label>,

    /// a kind of handle for the Gtk+ window
    window : Option<Window>

}

impl GWCApp {

    /// Provides a new instance of the GWC application
    pub fn new() -> GWCApp {
        GWCApp { msg_label: None, window: None }
    }

    /// Responsible for initializing the application state, including the whole UI
    pub fn init(&mut self) {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let win = Window::new(WindowType::Toplevel);
        win.set_title("GWC");
        win.set_position(gtk::WindowPosition::Center);
        win.set_size_request(400, 400);

        win.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        // create the application menu
        let menu_bar = self.init_menus();
        v_box.pack_start(&menu_bar, false, false, 0);

        // followed by the toolbar
        let tool_bar = self.init_toolbar();
        v_box.pack_start(&tool_bar, false, false, 0);

        // Create the text label for showing the word count
        let file_counter = Label::new("No file counted");
        v_box.pack_start(&file_counter, true, true, 0);

        win.add(&v_box);

        self.window = Some(win);
        self.msg_label = Some(file_counter)
    }

    /// Displays the application window
    pub fn show(&self) {
        if let Some(ref win) = self.window {
            win.show_all()
        } else {
            panic!("Window has not been properly initialized");
        }
    }

    ///  Called when the user selects the
    /// File->Open option
    fn on_menu_open(&self) {
        let filesel = FileChooserDialog::new(Some("Choose a file"), None::<&Window>,
                                                    FileChooserAction::Open);
        filesel.add_buttons(&[
            ("Open", ResponseType::Ok.into()),
            ("Cancel", ResponseType::Cancel.into())
        ]);

        filesel.set_select_multiple(true);
        filesel.run();
        let file = filesel.get_filename();
        filesel.destroy();

        if let Some(filename) = file {
            self.process_file(filename);
        }
    }



    ///  Called when the user presses the
    /// Ok button on the FileSelection dialog
    fn process_file (&self, filename : PathBuf) {
        println!("Files: {:?}", filename);

        if let Some ((words, lines, bytes)) = count_words(&filename) {
            let msg = format!("The file {:?}, has {} lines, {} words and {} bytes", filename, words, lines, bytes);
            if let Some(ref lbl) = self.msg_label {
                lbl.set_text(&msg.to_string());
            }
        } else {
            let msg = format!("Could not open file {:?}", filename);
            let dialog = MessageDialog::new(None::<&Window>, DIALOG_MODAL,
                MessageType::Error, ButtonsType::Ok, &msg);
            dialog.run();
            dialog.destroy();
        }
    }

    /// Creates the application menus
    fn init_menus (&self) -> MenuBar {
        let menu = Menu::new();
        let menu_bar = MenuBar::new();
        let file = MenuItem::new_with_label("File");
    
        let quit = MenuItem::new_with_label("Quit");
        let file_item = MenuItem::new();
        let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let file_image = Image::new_from_icon_name("document-open", IconSize::Menu.into());
        let file_label = Label::new(Some("File"));

        file_box.pack_start(&file_image, false, false, 0);
        file_box.pack_start(&file_label, true, true, 0);
        file_item.add(&file_box);

        menu.append(&file_item);
        menu.append(&quit);
        file.set_submenu(Some(&menu));
        menu_bar.append(&file);

        // Extras menu
        let extras_menu = Menu::new();
        let extras = MenuItem::new_with_label("Extras");
        let about = MenuItem::new_with_label("About");

        extras_menu.append(&about);
        extras.set_submenu(Some(&extras_menu));
        menu_bar.append(&extras);

        file_item.connect_activate( |_| {
            //self.on_menu_open();
        });

        quit.connect_activate(|_| {
            gtk::main_quit();
        });

        about.connect_activate(move |_| {
            let p = AboutDialog::new();
            p.set_authors(&["Paulo Pinto"]);
            p.set_website_label(Some("A simple GUI version of wc"));
            p.set_website(Some("https://www.progtools.org"));
            p.set_authors(&["Paulo Pinto"]);
            p.set_title("About GWC");
            p.run();
            p.destroy();
        });

        menu_bar
    }

    /// Creates the application toolbar
    fn init_toolbar(&self) -> Toolbar {
        let toolbar = Toolbar::new();
        toolbar.set_style(ToolbarStyle::Both);

        let open_btn_image = Image::new_from_icon_name("document-open", IconSize::LargeToolbar.into());
        let open_btn = ToolButton::new(Some(&open_btn_image), "Open");
        toolbar.insert(&open_btn, 0);

        let sep = SeparatorToolItem::new();
        toolbar.insert(&sep, 1);

        let quit_btn_image = Image::new_from_icon_name("application-exit", IconSize::LargeToolbar.into());
        let quit_btn = ToolButton::new(Some(&quit_btn_image), "Quit");
        toolbar.insert(&quit_btn, 2);

        open_btn.connect_clicked(|_| {
            //self.on_menu_open();
        });

        quit_btn.connect_clicked(|_| {
            gtk::main_quit();
        });

        toolbar
    } 
}

/// Counts the number of words, lines and bytes on the given file
fn count_words(filename: &PathBuf) -> Option<(usize, usize, usize)> {
    let mut words = 0;
    let mut lines = 0;
    let mut bytes = 0;

    // Open the path in read-only mode, returns `io::Result<File>`
    if let Ok(file) = File::open(&filename) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(row) = line {
                lines += 1;
                bytes += row.len();
                words += row.split_whitespace().count();
            }
            else {
                // IO error no need to carry on
                return None
            }
        }       
    } else {
        return None
    }
    Some((words, lines, bytes))
}

/// Application entry point
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut window = GWCApp::new();

    window.init();
    window.show();

    gtk::main();
}
