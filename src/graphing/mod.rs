use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};

mod render;
use render::render_chart_svg;

fn create_base_chart(
    title: &str,
    x_domain: Vec<String>,
    y_domain: Vec<f32>,
) -> (Chart, ScaleBand, ScaleLinear) {
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let x = ScaleBand::new()
        .set_domain(x_domain)
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    let y = ScaleLinear::new()
        .set_domain(y_domain)
        .set_range(vec![height - top - bottom, 0]);

    (
        Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title(title.to_owned()),
        x,
        y,
    )
}

pub fn create_bar_graph(
    title: &str,
    x_axis_title: &str,
    y_axis_title: &str,
    points: Vec<(f32, f32)>,
) -> String {
    let mut max: i64 = 0;
    for (x1, y1) in &points {
        if x1 > &(max as f32) {
            max = x1.ceil() as i64;
        }
    }

    let x_domain = (0..(max + 1))
        .map(|x| x as f64)
        .map(|num| {
            let mut str = num.to_string();
            str.truncate(4);
            str
        })
        .collect();
    let y_domain = (0..100).map(|x| x as f32).collect();
    let (chart, x, y) = create_base_chart(title, x_domain, y_domain);

    let data = points
        .into_iter()
        .map(|(x1, y1)| {
            let mut y_format: f32 = y1 as f32;
            y_format = (y_format * 100.0).round() as f32;
            (x1.to_string(), (y_format / 10000.0) as f32)
        })
        .collect();

    println!("{:?}", &data);

    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&data)
        .unwrap();

    let chart = chart
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label(y_axis_title)
        .add_bottom_axis_label(x_axis_title)
        .add_view(&view);

    render_chart_svg(chart, title.to_owned()).unwrap()
}
