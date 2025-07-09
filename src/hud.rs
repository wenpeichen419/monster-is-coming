use macroquad::prelude::*;

//绘制玩家生命值条的函数
pub fn draw_health_bar(health: i32) {
    let bar_width = 300.0;
    let bar_height = 20.0;
    //居中显示生命值条
    let bar_x = screen_width() / 2.0 - bar_width / 2.0;
    let bar_y = 10.0;

    let health_ratio = health.clamp(0, 100) as f32 / 100.0;

    draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
    draw_rectangle(bar_x, bar_y, bar_width * health_ratio, bar_height, GREEN);

    //显示血条数值的格式化文本
    let text = format!("HP: {}", health);
    let font_size = 20.0;
    let text_dim = measure_text(&text, None, font_size as u16, 1.0);
    draw_text(&text, bar_x + bar_width / 2.0 - text_dim.width / 2.0, bar_y + bar_height - 4.0, font_size, WHITE);
}

//绘制消息文本的函数
pub fn draw_message(message: &str, color: Color) {
    let font_size = 40.0;  // 从30增大到40
    let text_dim = measure_text(message, None, font_size as u16, 1.0);
    let x = screen_width() / 2.0 - text_dim.width / 2.0;
    let y = screen_height() / 2.0;
    
    // 绘制两次文字制造加粗效果
    draw_text(message, x, y, font_size, color);
    draw_text(message, x+1.0, y+1.0, font_size, color); 
}
