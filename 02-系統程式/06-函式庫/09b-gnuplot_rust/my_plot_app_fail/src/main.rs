use gnuplot::{Figure, Caption, Color, ColorType, AxesCommon};

fn main() {
    // 準備數據
    let x = (0..100).map(|i| i as f64 * 0.1).collect::<Vec<_>>();
    let y = x.iter().map(|&v| v.sin()).collect::<Vec<_>>();

    let mut fg = Figure::new();
    
    fg.axes2d()
        .set_title("Rust Gnuplot 範例", &[])
        .set_x_label("X 軸", &[])
        .set_y_label("Y 軸", &[])
        .lines(
            &x, 
            &y, 
            // 修正這裡：將 Color("red") 改為 Color(ColorType::RGBString("red"))
            &[Caption("sin(x)"), Color(ColorType::RGBString("red"))]
        );

    // 顯示圖表
    fg.show().unwrap();
}