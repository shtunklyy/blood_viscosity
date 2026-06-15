use plotters::prelude::*;
use nalgebra::DVector;

pub fn draw(
    exp_rpm: &DVector<f64>,
    exp_hep: &DVector<f64>,
    exp_cit: &DVector<f64>,
    err_hep: &DVector<f64>,
    err_cit: &DVector<f64>,
    dense_rpm: &[f64],
    dense_hep: &[f64],
    dense_cit: &[f64],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("hemodynamic.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("blood viscosity", ("calibri", 30).into_font())
        .margin(20)
        .x_label_area_size(70)
        .y_label_area_size(80)
        .build_cartesian_2d(0f64..160f64, 0f64..450f64)?;

    chart.configure_mesh()
        .x_desc("Rotor Speed, w (RPM)")
        .y_desc("Apparent Viscosity, eff (mPa*s)")
        .x_label_style(("calibri", 20).into_font())
        .y_label_style(("calibri", 20).into_font())
        .axis_desc_style(("calibri", 24).into_font())
        .disable_x_mesh()
        .draw()?;

    let hep_theory_data: Vec<(f64, f64)> = dense_rpm.iter().zip(dense_hep.iter()).map(|(&x, &y)| (x, y)).collect();
    chart.draw_series(LineSeries::new(hep_theory_data, &BLUE))?
        .label("Theory: Heparin")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    let cit_theory_data: Vec<(f64, f64)> = dense_rpm.iter().zip(dense_cit.iter()).map(|(&x, &y)| (x, y)).collect();
    chart.draw_series(LineSeries::new(cit_theory_data, &RED))?
        .label("Theory: Citrate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    let hep_exp_data: Vec<(f64, f64, f64)> = exp_rpm.iter()
        .zip(exp_hep.iter())
        .zip(err_hep.iter())
        .map(|((&x, &y), &err)| (x, y, err))
        .collect();

    chart.draw_series(hep_exp_data.iter().map(|&(x, y, err)| {
        ErrorBar::new_vertical(x, y - err, y, y + err, BLUE.filled(), 5)
    }))?;

    let hep_points: Vec<(f64, f64)> = hep_exp_data.iter().map(|&(x, y, _)| (x, y)).collect();
    chart.draw_series(PointSeries::of_element(
        hep_points.clone(),
        6,
        &BLUE,
        &|c, s, st| return EmptyElement::at(c) + Circle::new((0,0), s, st.filled()),
    ))?.label("Exp: Heparin").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE.mix(0.5)));

    chart.draw_series(LineSeries::new(hep_points, &BLUE.mix(0.3)))?;
    let cit_exp_data: Vec<(f64, f64, f64)> = exp_rpm.iter()
        .zip(exp_cit.iter())
        .zip(err_cit.iter())
        .map(|((&x, &y), &err)| (x, y, err))
        .collect();

    chart.draw_series(cit_exp_data.iter().map(|&(x, y, err)| {
        ErrorBar::new_vertical(x, y - err, y, y + err, RED.filled(), 5)
    }))?;

    let cit_points: Vec<(f64, f64)> = cit_exp_data.iter().map(|&(x, y, _)| (x, y)).collect();
    chart.draw_series(PointSeries::of_element(
        cit_points.clone(),
        6,
        &RED,
        &|c, s, st| return EmptyElement::at(c) + Circle::new((0,0), s, st.filled()),
    ))?.label("Exp: Citrate").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.5)));

    chart.draw_series(LineSeries::new(cit_points, &RED.mix(0.3)))?;
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("calibri", 18).into_font())
        .draw()?;

    root.present()?;
    println!("plot in 'hemodynamic.png'");
    Ok(())
}
