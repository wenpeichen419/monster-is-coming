use macroquad::prelude::*;

//游戏结束的结构体
pub struct GameOver {
    //游戏结束界面的背景
    background: Texture2D,
    //游戏持续时间，单位为秒
    game_time: f32,
    //玩家在游戏中收集的血包数量
    health_packs_collected: usize,
    //玩家被怪物攻击的次数
    monster_attacks: usize,
}

impl GameOver {
    pub async fn new(game_time: f32, health_packs_collected: usize, monster_attacks: usize) -> Self {
        //游戏结束界面的背景图片
        let background = load_texture("assets/game_over_background.png").await.unwrap();
        
        //返回Gameover实例
        Self {
            background,
            game_time,
            health_packs_collected,
            monster_attacks,
        }
    }

    pub fn update(&self) -> bool {
        // 检查玩家是否按下空格键，如果按空格键，则重新开始
        is_key_pressed(KeyCode::Space)
    }

    //负责绘制游戏结束界面的背景、标题、统计信息和重新开始提示等内容的函数
    pub fn draw(&self) {
        // step1---绘制白色背景图
        draw_texture(&self.background, 0.0, 0.0, WHITE);

        // step2---添加半透明黑色背景板
        draw_rectangle(
            0.0, 
            0.0, 
            screen_width(), 
            screen_height(), 
            Color::new(0.0, 0.0, 0.0, 0.7) // 70%不透明度的黑色
        );

        // step3---绘制游戏标题
        let title = "GAME OVER";
        let title_size = 60.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        
        // 描边效果
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0 + 2.0,
            120.0 + 2.0,
            title_size,
            BLACK,
        );
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            120.0,
            title_size,
            YELLOW,
        );

        // step4---游戏统计信息
        let stats = format!(
            "Game Duration: {:.1} seconds\n\
             Health Packs Collected: {}\n\
             Times Attacked: {}",
            self.game_time, 
            self.health_packs_collected, 
            self.monster_attacks
        );
        
        let stats_size = 36.0;
        let stats_lines = stats.split('\n').collect::<Vec<_>>();
        
        for (i, line) in stats_lines.iter().enumerate() {
            let line_width = measure_text(line, None, stats_size as u16, 1.0).width;
            
            draw_text(
                line,
                screen_width() / 2.0 - line_width / 2.0 + 1.0,
                screen_height() / 2.0 - 30.0 + (i as f32 * 50.0) + 1.0,
                stats_size,
                BLACK,
            );
            draw_text(
                line,
                screen_width() / 2.0 - line_width / 2.0,
                screen_height() / 2.0 - 30.0 + (i as f32 * 50.0),
                stats_size,
                WHITE,
            );
        }

        // step5---重新开始游戏的提示
        let restart_text = "Press SPACE to Restart";
        let restart_size = 30.0;
        let restart_width = measure_text(restart_text, None, restart_size as u16, 1.0).width;
        
        
        draw_text(
            restart_text,
            screen_width() / 2.0 - restart_width / 2.0 + 1.0,
            screen_height() - 80.0 + 1.0,
            restart_size,
            BLACK,
        );
        draw_text(
            restart_text,
            screen_width() / 2.0 - restart_width / 2.0,
            screen_height() - 80.0,
            restart_size,
            GREEN,
        );
    }
}