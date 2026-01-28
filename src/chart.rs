/*
  このコードはGPT-5.1を利用して出力しました。
*/
use plotters::prelude::*;

/// マルチライン折れ線グラフを SVG に出力する
///
/// `series` は以下形式：
/// [
///   ("step1", vec![(x, y), (x, y), ...]),
///   ("step1c_dp", vec![(x, y), (x, y), ...]),
///   ("step3", vec![(x, y), ...]),
/// ]
pub fn draw_multi_line_chart(
    path: &str, // "multi_line.svg"
    series: &[(&str, Vec<(f64, f64)>)],
    title: &str,
    x_label: &str,
    y_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if series.is_empty() {
        return Ok(());
    }

    // ----------------- 軸の最小・最大を求める -----------------

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for (_, pts) in series {
        for (x, y) in pts {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
    }

    // Plotters の都合で min==max だと描けないため少し広げる
    if (min_x - max_x).abs() < f64::EPSILON {
        min_x -= 1.0;
        max_x += 1.0;
    }
    if (min_y - max_y).abs() < f64::EPSILON {
        min_y -= 1.0;
        max_y += 1.0;
    }

    // ----------------- SVG の描画領域作成 -----------------

    let root = SVGBackend::new(path, (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;

    // ----------------- 色リスト（必要に応じて追加可能） -----------------

    let colors = vec![&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

    // ----------------- 複数シリーズの描画 -----------------

    for (idx, (name, pts)) in series.iter().enumerate() {
        let color = colors[idx % colors.len()];

        chart
            .draw_series(LineSeries::new(pts.clone(), color.stroke_width(3)))?
            .label(*name)
            .legend(move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color.stroke_width(3)));
    }

    // ----------------- 凡例（レジェンド）表示 -----------------

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    root.present()?;
    Ok(())
}
