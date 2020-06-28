use plotly::common::Mode;
use plotly::common::color::{Rgba, NamedColor};
use plotly::{Plot, Scatter, Layout, Shape, LayoutType, Layer, Line};

fn shapes() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .name("trace1")
        .mode(Mode::Markers);
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .name("trace2")
        .mode(Mode::Lines);
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

    let shape1 = Shape::new()
                 .layout_type(LayoutType::Rect)
                 .x_ref("x".to_string())
                 .y_ref("paper".to_string())
                 .x0(2.0)
                 .y0(0.0)
                 .x1(3.0)
                 .y1(1.0)
                 .fill_color(Rgba::new(211, 211, 211, 1.0))
                 .opacity(0.2)
                 .layer(Layer::Below)
                 .line(Line::new().width(0));
    let shape2 = Shape::new()
                 .layout_type(LayoutType::Circle)
                 .x_ref("x".to_string())
                 .y_ref("y".to_string())
                 .x0(2.0)
                 .y0(13.0)
                 .x1(4.0)
                 .y1(17.0)
                 .opacity(0.2)
                 .fill_color(NamedColor::Blue)
                 .line(Line::new().color(NamedColor::Red));
    let shape3 = Shape::new()
                 .layout_type(LayoutType::Line)
                 .x0(1.5)
                 .y0(16.0)
                 .x1(4.0)
                 .y1(6.0)
                 .line(Line::new()
                       .color("CE2029")
                       .width(4)
                       .dash("dashdot".to_string())
                 );

    let layout = Layout::new().shapes(vec![shape1, shape2, shape3]);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);
    plot.show_png(1024, 680);
    plot.show();
}

fn main() -> std::io::Result<()> {
    shapes();
    Ok(())
}
