use plotly::common::Mode;
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};

fn main() {
    // 1. 準備數據
    let x_values = vec![1, 2, 3, 4, 5];
    let y_values_1 = vec![10, 15, 13, 17, 12];
    let y_values_2 = vec![16, 5, 11, 9, 14];

    // 2. 建立第一條線：折線圖 (Lines)
    let trace1 = Scatter::new(x_values.clone(), y_values_1)
        .name("數據 A")
        .mode(Mode::LinesMarkers); // 同時顯示線和點

    // 3. 建立第二條線：散點圖 (Markers)
    let trace2 = Scatter::new(x_values, y_values_2)
        .name("數據 B")
        .mode(Mode::Markers); // 只顯示點

    // 4. 設定圖表佈局 (Layout)
    let layout = Layout::new()
        .title("Plotly.rs 範例圖表")
        .x_axis(Axis::new().title("X 軸名稱"))
        .y_axis(Axis::new().title("Y 軸名稱"));

    // 5. 組合圖表
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    // 6. 顯示圖表
    // 這會產生一個臨時 HTML 並嘗試在你的預設瀏覽器中打開
    plot.show();

    // 或者,如果你只想儲存成檔案而不自動開啟,可以使用:
    // plot.write_html("my_plot.html");
    
    println!("圖表已產生!如果是遠端環境,請檢查專案目錄下的 HTML 檔案。");
}