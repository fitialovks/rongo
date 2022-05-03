use gtk::{Application, ApplicationWindow, CellRendererText, Label, Notebook, Orientation, Paned, TextView, TreeStore, TreeView, TreeViewColumn};
use gtk::prelude::*;
use mongodb::Client;
use mongodb::options::ClientOptions;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let databases = rt.block_on(async {
        return connect_mongo().await;
    });

    let app = Application::builder()
        .application_id("ru.halcraes.Rongo")
        .build();

    app.connect_activate(move |app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Rongo")
            .build();

        let text = TextView::builder()
            .build();

        let label = Label::builder()
            .label("Connection 1")
            .build();

        let notebook = Notebook::builder()
            .build();

        notebook.append_page(&text, Some(&label));

        let tree = TreeView::builder()
            .headers_visible(false)
            .build();

        let column = TreeViewColumn::builder()
            .build();

        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);

        tree.append_column(&column);

        let tree_model = TreeStore::new(&[String::static_type()]);
        for database in &databases {
            tree_model.insert_with_values(None, None, &[(0, database)]);
        }

        tree.set_model(Some(&tree_model));

        let paned = Paned::builder()
            .orientation(Orientation::Horizontal)
            .build();
        paned.pack1(&tree, true, true);
        paned.pack2(&notebook, true, true);

        win.add(&paned);

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}

async fn connect_mongo() -> Vec<String> {
    println!("Connecting");
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.direct_connection = Some(true);
    println!("Creating client");
    let client = Client::with_options(client_options).unwrap();
    println!("Listing databases");
    return client.list_database_names(None, None).await.unwrap();
}