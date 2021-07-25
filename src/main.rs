mod ui;
use ui::ui::UI;

mod cmd;
use cmd::cmd::Args;

mod db;
use db::db as DB;

fn main() {
    let args = Args::parse();
    let db_text = DB::read(&args.file_path);

    let mut ui = UI::new(args.x_pos, args.y_pos, args.width, args.height);
    ui.build();

    ui.set_text(&db_text);
    ui.signal_content_changed(move |text| {
        DB::write(args.file_path.as_str(), &text);
    });

    ui.run();
}
