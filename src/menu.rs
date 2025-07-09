use macroquad::prelude::*;


#[derive(Clone, Copy)]
//游戏难度不同，怪物数量不同，怪物越多，玩家生存的难度越大
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Menu {
    background: Texture2D,
    selected_difficulty: GameDifficulty,
}

impl Menu {
    pub async fn new() -> Self {
        let background = load_texture("assets/menu_background.png").await.unwrap();
        
        //游戏是默认简单难度的
        Self {
            background,
            selected_difficulty: GameDifficulty::Easy, 
        }
    }

    pub fn update(&mut self) -> Option<GameDifficulty> {
        // 难度的选择
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) {
            self.selected_difficulty = match self.selected_difficulty {
                GameDifficulty::Easy => GameDifficulty::Medium,
                GameDifficulty::Medium => GameDifficulty::Hard,
                GameDifficulty::Hard => GameDifficulty::Easy,
            };
        }

        if is_key_pressed(KeyCode::Enter) {
            return Some(self.selected_difficulty);
        }

        None
    }

    pub fn draw(&self) {
        // step1--绘制背景
        draw_texture(&self.background, 0.0, 0.0, WHITE);

        // step2---显式难度的文字
        let difficulty_text = match self.selected_difficulty {
            GameDifficulty::Easy => "Easy (1 Monster)",
            GameDifficulty::Medium => "Medium (3 Monsters)",
            GameDifficulty::Hard => "Hard (5 Monsters)",
        };
        let diff_size = 30.0;
        let diff_width = measure_text(difficulty_text, None, diff_size as u16, 1.0).width;
        
        // step3---(调整UI的步骤)对页面显式的关于游戏难度的文字进行描边
        draw_text(
            difficulty_text,
            screen_width() / 2.0 - diff_width / 2.0 + 1.0,
            screen_height() / 2.0 + 1.0,
            diff_size,
            BLACK,
        );
        draw_text(
            difficulty_text,
            screen_width() / 2.0 - diff_width / 2.0,
            screen_height() / 2.0,
            diff_size,
            GREEN,
        );

        // step4---显示游戏开始的文字提示
        let start_text = "Press ENTER to Begin";
        let start_size = 25.0;
        let start_width = measure_text(start_text, None, start_size as u16, 1.0).width;

        draw_text(
            start_text,
            screen_width() / 2.0 - start_width / 2.0 + 1.0,
            screen_height() - 100.0 + 1.0,
            start_size,
            BLACK,
        );
        draw_text(
            start_text,
            screen_width() / 2.0 - start_width / 2.0,
            screen_height() - 100.0,
            start_size,
            Color::new(0.4, 0.8, 1.0, 1.0), 
        );

        // step5---显示提示用户进行难度选择的文字
        let hint_text = "Use LEFT/RIGHT to change difficulty";
        let hint_size = 20.0;
        let hint_width = measure_text(hint_text, None, hint_size as u16, 1.0).width;
        draw_text(
            hint_text,
            screen_width() / 2.0 - hint_width / 2.0,
            screen_height() - 50.0,
            hint_size,
            Color::new(0.4, 0.8, 1.0, 1.0), 
        );

        // step6---显示提示用户游戏可进行暂停的文字
        let pause_hint = "Click the Pause button at the top-right corner to pause the game";
        let pause_size = 18.0;
        let pause_width = measure_text(pause_hint, None, pause_size as u16, 1.0).width;
        draw_text(
            pause_hint,
            screen_width() / 2.0 - pause_width / 2.0,
            screen_height() - 20.0,
            pause_size,
            Color::new(0.8, 0.4, 0.0, 1.0),
        );
    }
}
