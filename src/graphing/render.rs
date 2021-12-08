use std::{env, path::Path, io, fs::read_to_string};

use charts::Chart;

pub fn render_chart_svg(chart: Chart, title: String) -> io::Result<String> {
    let temp_dir = env::temp_dir();
    let file_name = title + ".svg";
    let file_path = Path::new(&file_name);
    let full_path = temp_dir.join(file_path);

    chart.save(&full_path);
    
    read_to_string(full_path)
}
